use std::{cmp, hash::BuildHasher, iter, ops::Range};

use rustc_hash::{FxBuildHasher, FxHashSet};
use typst::{
    WorldExt, compile,
    diag::Severity,
    layout::{Abs, Frame, FrameItem, FrameKind, PagedDocument, Point, Size},
    syntax::Span,
};
// use typst_html::html;
use typst_svg::svg_html_frame;

use crate::{
    renderer::{
        RenderTarget,
        paged::{FrameBlock, PagedFrameRender, PagedRangedFrame, PagedRenderResult},
        sync_source_context,
    },
    state::{SourceContext, TypstState},
    world::MnemoWorld,
    wrappers::{TypstDiagnostic, TypstFileId, map_main_span},
};

pub fn render_by_items(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> PagedRenderResult {
    let (ir, ast_blocks) = sync_source_context(id, text, prelude, RenderTarget::Png, state);

    let mut last_document = None;

    let mut diagnostics = Vec::new();
    let mut compiled_warnings = None;

    // let mut erronous_ranges = Vec::new();

    let context = state.source_context_map.get_mut(id).unwrap();

    context
        .main_source_mut(&mut state.world)
        .unwrap()
        .replace(&ir);

    let mut ranged_heights = Vec::new();

    while last_document.is_none() {
        let compiled = compile::<PagedDocument>(&state.world);
        compiled_warnings = Some(compiled.warnings);

        // crate::log!("[DOING A THING]");

        ranged_heights = match compiled.output {
            Ok(document) => {
                let mut frame_blocks = document
                    .pages
                    .iter()
                    .flat_map(|page| {
                        page.frame.items().flat_map(|frame_item| {
                            frame_with_bounds(frame_item, context, &state.world)
                        })
                    })
                    .peekable();

                let mut ranged_items = Vec::with_capacity(ast_blocks.len());

                let mut ast_blocks = ast_blocks.iter().peekable();

                let mut deferred_items = Vec::<FrameBlock>::new();

                while let Some(ast_block) = ast_blocks.next() {
                    let aux_source = context.aux_source(&state.world).unwrap();

                    let aux_range = &ast_block.range;
                    let aux_lines = aux_source.lines();
                    let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start).unwrap();
                    let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end).unwrap();
                    let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                    // let main_range_start = context.map_aux_to_main(aux_range.start);
                    let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
                    // let main_range = main_range_start..main_range_end;

                    let mut items = deferred_items.drain(..).collect::<Vec<_>>();

                    let mut block_start_height = None;
                    let mut block_end_height = None;

                    while let Some(frame_block) = frame_blocks.peek() {
                        if let Some(range) = &frame_block.range {
                            if range.end <= main_range_end {
                                let frame_block = frame_blocks.next().unwrap();

                                match block_start_height {
                                    Some(height) if height < frame_block.start_height => {}
                                    _ => block_start_height = Some(frame_block.start_height),
                                }

                                match block_end_height {
                                    Some(height) if height > frame_block.end_height => {}
                                    _ => block_end_height = Some(frame_block.end_height),
                                }

                                items.push(frame_block);
                                items.extend(deferred_items.drain(..));
                            } else {
                                break;
                            }
                        } else {
                            let frame_block = frame_blocks.next().unwrap();
                            deferred_items.push(frame_block);
                        }
                    }

                    let block_start_height = block_start_height.unwrap_or_default().to_pt();
                    let block_end_height = block_end_height.unwrap_or_default().to_pt();

                    match context.height {
                        Some(height) if block_start_height >= height => {
                            continue;
                        }
                        _ => {}
                    }

                    let height = block_end_height - block_start_height;

                    if height <= 0_f64 {
                        continue;
                    }

                    let hash = FxBuildHasher.hash_one(&items) as u32;

                    ranged_items.push((
                        items,
                        aux_range_utf16.clone(),
                        height,
                        block_start_height,
                        hash,
                    ));
                }

                if !deferred_items.is_empty()
                    && let Some((last_items, ..)) = ranged_items.last_mut()
                {
                    last_items.extend(deferred_items.drain(..));
                }

                last_document = Some(document);

                ranged_items
            }
            Err(source_diagnostics) => {
                let error_ranges = source_diagnostics
                    .iter()
                    .filter_map(|diagnostic| {
                        map_main_span(
                            diagnostic.span,
                            diagnostic.severity == Severity::Error,
                            &diagnostic.trace,
                            &context,
                            &state.world,
                        )
                    })
                    .collect::<FxHashSet<_>>();

                let Some(main_range) = ast_blocks.iter().find_map(|block| {
                    let aux_range = &block.range;

                    let main_range_start = context.map_aux_to_main_left(aux_range.start);
                    let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
                    let main_range = main_range_start..main_range_end;

                    let in_block = error_ranges.iter().any(|error_range| {
                        (main_range_start <= error_range.start
                            && main_range_end >= error_range.start)
                            || (main_range_start <= error_range.end
                                && main_range_end >= error_range.end)
                    });

                    in_block.then_some(main_range)
                }) else {
                    break;
                };

                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &context,
                    &state.world,
                ));

                crate::error!("[ERRORS]: {diagnostics:?}");

                let start_byte = main_range.start;
                let end_byte = main_range.end;

                // let range_delta = end_byte - start_byte;
                // let repeat_range = range_delta - if range_delta > 2 { 2 } else { 1 };

                let source = context.main_source_mut(&mut state.world).unwrap();
                // crate::log!("[REPLACING]:\n{}", &source.text()[start_byte..end_byte]);
                source.edit(start_byte..end_byte, &(" ".repeat(end_byte - start_byte)));
                // crate::log!("[NEW SOURCE]:\n{}", &source.text());

                // break;

                Vec::new()
            }
        };
    }

    let frames = if let Some(document) = &last_document {
        let width = document
            .pages
            .iter()
            .map(|page| page.frame.width())
            .max()
            .unwrap_or_default();

        // let height = document
        //     .pages
        //     .iter()
        //     .map(|page| page.frame.height())
        //     .sum::<Abs>();

        ranged_heights
            .into_iter()
            .map(|(frame_blocks, range, height, offset_height, hash)| {
                let mut frame = Frame::new(Size::new(width, Abs::pt(height)), FrameKind::Soft);

                frame.push_multiple(frame_blocks.into_iter().map(|block| {
                    let point = block.point - Point::new(Abs::zero(), Abs::pt(offset_height));

                    (point, block.item)
                }));

                // let hash = FxBuildHasher.hash_one(&frame) as u32;
                let svg = svg_html_frame(&frame, Abs::pt(16.0), None, &[], &document.introspector);

                let height = height.ceil() as u32;

                let render = PagedFrameRender {
                    svg,
                    hash,
                    height,
                    offset_height,
                };

                PagedRangedFrame { range, render }
            })
            .collect()
    } else {
        Vec::new()
    };

    if let Some(warnings) = compiled_warnings {
        diagnostics.extend(TypstDiagnostic::from_diagnostics(
            warnings,
            &context,
            &state.world,
        ));
    }

    context.paged_document = last_document;
    context
        .main_source_mut(&mut state.world)
        .unwrap()
        .replace(&ir);

    PagedRenderResult {
        frames,
        diagnostics,
    }
}

fn frame_with_bounds(
    frame_item: &(Point, FrameItem),
    context: &SourceContext,
    world: &MnemoWorld,
) -> Box<[FrameBlock]> {
    let (point, item) = frame_item;

    let (start_height, end_height) = match &item {
        FrameItem::Text(text) => (cmp::max(point.y - text.size, Abs::zero()), point.y),
        FrameItem::Group(group) => {
            // crate::log!("[F GROUP]: {:?}", group);
            if !group.transform.is_identity() {
                crate::log!("[G TRANSFORM]: {:?}", group.transform);
            }

            // TOOD: Handle transform
            return group
                .frame
                .items()
                .flat_map(|frame_item| {
                    frame_with_bounds(frame_item, context, world)
                        .into_iter()
                        .map(|mut frame_block| {
                            frame_block.start_height += point.y;
                            frame_block.end_height += point.y;
                            frame_block.point.x += point.x;
                            frame_block.point.y += point.y;

                            frame_block
                        })
                })
                .collect::<Box<[_]>>();
        }
        FrameItem::Shape(shape, _span) => (point.y, shape.geometry.bbox_size().y),
        FrameItem::Image(_image, axes, _span) => (point.y, axes.y),
        FrameItem::Link(..) => (point.y, point.y),
        FrameItem::Tag(..) => (point.y, point.y),
    };

    let range = frame_item_range(item, context, world);

    Box::from_iter(iter::once(FrameBlock {
        range,
        start_height,
        end_height,
        item: item.clone(),
        point: point.clone(),
    }))
}

fn frame_item_range(
    item: &FrameItem,
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
        FrameItem::Link(_destination, _axes) => Span::detached(),
        FrameItem::Tag(tag) => {
            // match tag {
            //     Tag::Start(content, _flags) => content.span(),
            //     Tag::End(_location, _key, _flags) => Span::detached(),
            // }
            Span::detached()
        }
    };

    if Some(context.main_id) == span.id() {
        world.range(span)
    } else {
        None
    }
}
