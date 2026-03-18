use std::{cmp, collections::VecDeque, iter, ops::Range};

use typst::{
    WorldExt, compile,
    introspection::Tag,
    layout::{Abs, FrameItem, PagedDocument, Point, Rect},
    syntax::Span,
};

use crate::{
    renderer::{
        RenderTarget,
        paged::{BoundFrameItem, FrameItemsChunk, PagedRender},
        remove_errornous_block, sync_source_context, try_mark_errornous,
    },
    state::{SourceContext, TypstState},
    world::MnemoWorld,
    wrappers::{TypstDiagnostic, TypstFileId},
};

/// Chunks a Typst document into renderable blocks by frame items, handling diagnostics and error convergence.
#[typst_macros::time]
pub fn chunk_by_items<'a>(
    id: &TypstFileId,
    text: &'a str,
    prelude: &'a str,
    render_target: RenderTarget,
    state: &'a mut TypstState,
) -> PagedRender<'a> {
    let (ir, mut ast_blocks) = sync_source_context(id, text, prelude, render_target, state);

    let mut document = None;
    let mut convergence = 0_u8;

    let mut diagnostics = Vec::new();
    let mut compiled_warnings = None;

    let context = state.source_context_map.get_mut(id).unwrap();

    context
        .main_source_mut(&mut state.world)
        .unwrap()
        .replace(&ir);

    let mut chunks = Vec::new();
    let mut tooltips = Vec::new();

    while document.is_none() {
        let compiled = compile::<PagedDocument>(&state.world);
        compiled_warnings = Some(compiled.warnings);

        // crate::log!("[DOING A THING]");

        (chunks, tooltips, document) = match compiled.output {
            Ok(document) => {
                let mut sink = BoundFrameSink::default();
                let mut bound_frame_items = Vec::new();

                for page in &document.pages {
                    for frame_item in page.frame.items() {
                        let frame_block =
                            bound_frame(frame_item, None, &mut sink, context, &state.world);
                        bound_frame_items.extend(frame_block);
                    }
                }

                let mut bound_frame_items = bound_frame_items.into_iter().peekable();

                let mut chunks = Vec::with_capacity(ast_blocks.len());
                let mut ast_blocks = ast_blocks.iter().peekable();
                let mut remaining_items = Vec::<BoundFrameItem>::new();

                while let Some(ast_block) = ast_blocks.next() {
                    let aux_source = context.aux_source(&state.world).unwrap();

                    let aux_range = &ast_block.range;
                    let aux_lines = aux_source.lines();
                    let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start).unwrap();
                    let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end).unwrap();
                    let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                    // let main_range_start = context.map_aux_to_main_from_left(aux_range.start);
                    let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
                    // let main_range = main_range_start..main_range_end;

                    let mut chunk_items = VecDeque::<BoundFrameItem>::new();
                    let mut deferred_items = Vec::<BoundFrameItem>::new();

                    let mut block_start_width = None;
                    let mut block_start_height = None;
                    let mut block_end_width = None;
                    let mut block_end_height = None;

                    while let Some(frame_block) = bound_frame_items.peek() {
                        if let Some(range) = &frame_block.range {
                            if range.end <= main_range_end {
                                let frame_block = bound_frame_items.next().unwrap();

                                // crate::log!("{frame_block:#?}");

                                match block_start_width {
                                    Some(width) if width < frame_block.bounds.min.x => {}
                                    _ => block_start_width = Some(frame_block.bounds.min.x),
                                }

                                match block_start_height {
                                    Some(height) if height < frame_block.bounds.min.y => {}
                                    _ => block_start_height = Some(frame_block.bounds.min.y),
                                }

                                match block_end_width {
                                    Some(width) if width > frame_block.bounds.max.x => {}
                                    _ => block_end_width = Some(frame_block.bounds.max.x),
                                }

                                match block_end_height {
                                    Some(height) if height > frame_block.bounds.max.y => {}
                                    _ => block_end_height = Some(frame_block.bounds.max.y),
                                }

                                chunk_items.extend(deferred_items.drain(..));
                                chunk_items.push_back(frame_block);
                            } else {
                                break;
                            }
                        } else {
                            let frame_block = bound_frame_items.next().unwrap();
                            deferred_items.push(frame_block);
                        }
                    }

                    let block_start_width = block_start_width.unwrap_or_default().to_pt();
                    let block_start_height = block_start_height.unwrap_or_default().to_pt();
                    let block_end_width = block_end_width.unwrap_or_default().to_pt();
                    let block_end_height = block_end_height.unwrap_or_default().to_pt();

                    match context.height {
                        Some(height) if block_start_height >= height => {
                            break;
                        }
                        _ => {}
                    }

                    if ast_block.is_inline {
                        let length = remaining_items.len();
                        chunk_items.reserve(length.saturating_add(1));

                        for remaining in remaining_items.drain(..).rev() {
                            chunk_items.push_front(remaining);
                        }
                    }

                    remaining_items.extend(deferred_items.drain(..));

                    // crate::log!("start width: {block_start_width}");
                    // crate::log!("end width: {block_end_width}");

                    let block_width = block_end_width - block_start_width;
                    let block_height = block_end_height - block_start_height;

                    if block_width <= 0_f64 || block_height <= 0_f64 {
                        continue;
                    }

                    chunks.push(FrameItemsChunk {
                        items: chunk_items,
                        range: aux_range_utf16,
                        width: block_width,
                        height: block_height,
                        x_offset: block_start_width,
                        y_offset: block_start_height,
                    });
                }

                if !remaining_items.is_empty()
                    && let Some(chunk) = chunks.last_mut()
                {
                    let length = remaining_items.len();
                    chunk.items.reserve(length.saturating_add(1));

                    for remaining in remaining_items.drain(..).rev() {
                        chunk.items.push_front(remaining);
                    }
                }

                (chunks, sink.tooltips, Some(document))
            }
            Err(source_diagnostics) => {
                convergence += 1;
                if convergence >= 32 {
                    crate::error!("COULD NOT CONVERGE ‼️");

                    break;
                }

                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics.clone(),
                    context,
                    &mut state.world,
                ));

                crate::error!("[ERRORS]: {diagnostics:?}");

                // let marked =
                //     try_mark_errornous(source_diagnostics.clone(), context, &mut state.world);

                // if marked.is_some() {
                //     continue;
                // }

                let indicies = remove_errornous_block(
                    &ast_blocks,
                    source_diagnostics,
                    context,
                    &mut state.world,
                );

                if indicies.is_empty() {
                    crate::error!("NO ERROR BLOCKS FOUND ‼️");

                    break;
                } else {
                    for idx in indicies.iter().rev() {
                        ast_blocks.remove(*idx);
                    }
                }

                (Vec::new(), Vec::new(), None)
            }
        };
    }

    if let Some(warnings) = compiled_warnings {
        diagnostics.extend(TypstDiagnostic::from_diagnostics(
            warnings,
            &context,
            &state.world,
        ));
    }

    context
        .main_source_mut(&mut state.world)
        .unwrap()
        .replace(&ir);

    // convert tooltips into Vec<FrameItemsChunk>
    let tooltips = tooltips
        .into_iter()
        .filter_map(|items| {
            let mut block_start_width = None;
            let mut block_start_height = None;
            let mut block_end_width = None;
            let mut block_end_height = None;

            for block in &items {
                match block_start_height {
                    Some(height) if height < block.bounds.min.y => {}
                    _ => block_start_height = Some(block.bounds.min.y),
                }

                match block_end_height {
                    Some(height) if height > block.bounds.max.y => {}
                    _ => block_end_height = Some(block.bounds.max.y),
                }

                match block.item {
                    FrameItem::Tag(..) => {}
                    _ => {
                        match block_start_width {
                            Some(width) if width < block.bounds.min.x => {}
                            _ => block_start_width = Some(block.bounds.min.x),
                        }

                        match block_end_width {
                            Some(width) if width > block.bounds.max.x => {}
                            _ => block_end_width = Some(block.bounds.max.x),
                        }
                    }
                }
            }

            let block_start_width = block_start_width?.to_pt();
            let block_start_height = block_start_height?.to_pt();
            let block_end_width = block_end_width?.to_pt();
            let block_end_height = block_end_height?.to_pt();

            let main_range = items
                .iter()
                .filter_map(|item| item.range.clone())
                .fold(None::<Range<usize>>, |range, item_range| {
                    Some(match range {
                        Some(range) => {
                            let start = cmp::min(range.start, item_range.start);
                            let end = cmp::max(range.end, item_range.end);

                            start..end
                        }
                        None => item_range,
                    })
                })
                .unwrap_or(0..0);

            let aux_start = context.map_main_to_aux_from_left(main_range.start);
            let aux_end = context.map_main_to_aux_from_right(main_range.end);

            let aux_source = context.aux_source(&state.world)?;

            let aux_lines = aux_source.lines();
            let aux_start_utf16 = aux_lines.byte_to_utf16(aux_start)?;
            let aux_end_utf16 = aux_lines.byte_to_utf16(aux_end)?;
            let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

            // crate::log!("main_range: {main_range:?}");
            // crate::log!("aux_range: {:?}", aux_start..aux_end);

            Some(FrameItemsChunk {
                items: VecDeque::from(items),
                range: aux_range_utf16,
                width: block_end_width - block_start_width,
                height: block_end_height - block_start_height,
                x_offset: block_start_width,
                y_offset: block_start_height,
            })
        })
        .collect();

    PagedRender {
        chunks,
        tooltips,
        diagnostics,
        document,
        context,
    }
}

/// Recursively bounds a frame item, producing frame blocks with position and range.
// #[comemo::memoize]
#[typst_macros::time]
fn bound_frame(
    frame_item: &(Point, FrameItem),
    parent_point: Option<Point>,
    sink: &mut BoundFrameSink,
    context: &SourceContext,
    world: &MnemoWorld,
) -> Box<[BoundFrameItem]> {
    let (point, item) = frame_item;

    let bounds = match &item {
        FrameItem::Text(text) => {
            Rect::new(
                Point::new(point.x, cmp::max(point.y - text.size, Abs::zero())),
                Point::new(point.x + text.width(), point.y),
            )
        }
        FrameItem::Group(group) => {
            if group.transform.is_identity() {
                let point = if let Some(parnet_point) = parent_point {
                    parnet_point + *point
                } else {
                    *point
                };

                return group
                    .frame
                    .items()
                    .flat_map(|frame_item| {
                        bound_frame(frame_item, Some(point), sink, context, world)
                    })
                    .collect::<Box<[_]>>();
            }

            let (range, bounds) = group
                .frame
                .items()
                .flat_map(|frame_item| bound_frame(frame_item, None, sink, context, world))
                .fold(
                    (
                        None::<Range<usize>>,
                        Rect::new(Point::zero(), Point::zero()),
                    ),
                    |(range, mut bounds), frame_block| {
                        let range = match (range, frame_block.range) {
                            (Some(range), Some(block_range)) => {
                                let start = cmp::min(range.start, block_range.start);
                                let end = cmp::max(range.end, block_range.end);

                                Some(start..end)
                            }
                            (Some(range), None) => Some(range),
                            (None, Some(block_range)) => Some(block_range),
                            (None, None) => None,
                        };

                        bounds.min.x = cmp::min(bounds.min.x, frame_block.bounds.min.x);
                        bounds.min.y = cmp::min(bounds.min.y, frame_block.bounds.min.y);
                        bounds.max.x = cmp::max(bounds.max.x, frame_block.bounds.max.x);
                        bounds.max.y = cmp::max(bounds.max.y, frame_block.bounds.max.y);

                        // sink.process_tooltips(frame_block);

                        (range, bounds)
                    },
                );

            let mut item = BoundFrameItem {
                range,
                bounds,
                item: item.clone(),
                point: point.clone(),
            };

            if let Some(point) = parent_point {
                item.point.x += point.x;
                item.point.y += point.y;
                item.bounds.min.x += point.x;
                item.bounds.min.y += point.y;
                item.bounds.max.x += point.x;
                item.bounds.max.y += point.y;
            }

            sink.process_tooltips(&item);

            return Box::from_iter(iter::once(item));
        }
        FrameItem::Shape(shape, _span) => {
            let bbox = shape.geometry.bbox_size();
            Rect::new(*point, Point::new(bbox.x, bbox.y))
        }
        FrameItem::Image(_image, axes, _span) => Rect::new(*point, Point::new(axes.x, axes.y)),
        FrameItem::Link(..) => Rect::new(*point, *point),
        FrameItem::Tag(..) => Rect::new(*point, *point),
    };

    let range = frame_item_range(item, sink, context, world);

    let mut item = BoundFrameItem {
        range,
        bounds,
        item: item.clone(),
        point: point.clone(),
    };

    if let Some(point) = parent_point {
        item.point.x += point.x;
        item.point.y += point.y;
        item.bounds.min.x += point.x;
        item.bounds.min.y += point.y;
        item.bounds.max.x += point.x;
        item.bounds.max.y += point.y;
    }

    sink.process_tooltips(&item);

    Box::from_iter(iter::once(item))
}

#[derive(Default)]
struct BoundFrameSink {
    tooltips: Vec<Vec<BoundFrameItem>>,
    tag_stack: Vec<(&'static str, Span)>,
}

// #[comemo::track]
impl BoundFrameSink {
    pub fn process_tooltips(&mut self, block: &BoundFrameItem) {
        if let Some((name, _span)) = self.tag_stack.last() {
            match *name {
                "equation" => {
                    let mut block = block.clone();
                    block.range = block.range.map(|range| (range.start - 1)..(range.end + 1));
                    self.tooltips.last_mut().unwrap().push(block);
                }
                _ => {}
            }
        }
    }

    pub fn push_tag(&mut self, name: &'static str, span: Span) {
        self.tooltips.push(Vec::new());
        self.tag_stack.push((name, span));
    }

    pub fn pop_tag(&mut self) -> Option<(&'static str, Span)> {
        self.tag_stack.pop()
    }
}

/// Determines the source range for a frame item, using tag stack for introspectable tags.
#[typst_macros::time]
fn frame_item_range(
    item: &FrameItem,
    sink: &mut BoundFrameSink,
    context: &SourceContext,
    world: &MnemoWorld,
) -> Option<Range<usize>> {
    let span = match item {
        FrameItem::Group(..) => unreachable!(),
        FrameItem::Text(text) => {
            let first_glyph_span = text.glyphs.first()?.span.0;
            let first_glyph_range = world.range(first_glyph_span)?;

            let last_glyph_span = text.glyphs.last()?.span.0;
            let last_glyph_range = world.range(last_glyph_span)?;

            return Some(first_glyph_range.start..last_glyph_range.end);
        }
        FrameItem::Shape(_shape, span) => *span,
        FrameItem::Image(_image, _axes, span) => *span,
        FrameItem::Link(_destination, _axes) => return None,
        FrameItem::Tag(tag) => {
            match tag {
                Tag::Start(content, flags) => {
                    let name = content.elem().name();
                    let span = content.span();

                    if flags.introspectable {
                        sink.push_tag(name, span);
                    }

                    // crate::log!("[START FLAGS]: {flags:?} {name}");

                    match name {
                        _ => return None,
                    }
                }
                Tag::End(_location, _key, flags) => {
                    if flags.introspectable
                        && let Some((name, span)) = sink.pop_tag()
                    {
                        match name {
                            "equation" => span,
                            _ => return None,
                        }
                        // span
                    } else {
                        return None;
                    }

                    // crate::log!("[END FLAG]: {flags:?}");

                    // let content = document
                    //     .introspector
                    //     .query_unique(&Selector::Location(location.clone()));

                    // if let Ok(content) = content {
                    //     let span = content.span();

                    //     if Some(context.main_id) == span.id() {
                    //         let range = world.range(span);

                    //         return range.map(|range| range.end..range.end);
                    //     } else {
                    //         return None;
                    //     }
                    // } else {
                    //     Span::detached()
                    // }
                }
            }
        }
    };

    if Some(context.main_id) == span.id() {
        world.range(span)
    } else {
        None
    }
}
