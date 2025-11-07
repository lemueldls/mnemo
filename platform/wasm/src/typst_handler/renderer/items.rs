use std::{cmp, iter, ops::Range};

use hashbrown::HashSet;
use highway::{HighwayHash, HighwayHasher};
use itertools::Itertools;
use tiny_skia::IntRect;
use typst::{
    WorldExt, compile,
    diag::Severity,
    introspection::Tag,
    layout::{Abs, FrameItem, PagedDocument, Point},
    syntax::Span,
};

// use typst_html::html;
// use typst_svg::{svg, svg_merged};
use crate::typst_handler::{
    renderer::{CompileResult, FrameBlock, FrameRender, RangedFrame, sync_file_context},
    state::{FileContext, TypstState},
    world::MnemoWorld,
    wrappers::{TypstDiagnostic, TypstFileId, map_main_span},
};

pub fn render_by_items(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> CompileResult {
    let (ir, ast_blocks) = sync_file_context(id, text, prelude, state);

    let mut last_document = None;

    let mut diagnostics = Vec::new();
    let mut compiled_warnings = None;

    // let mut erronous_ranges = Vec::new();

    let context = state.file_contexts.get_mut(id).unwrap();

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
                        // crate::log!(
                        //     "[ITEMS]: {:?}",
                        //     page.frame
                        //         .items()
                        //         .map(|(_point, item)| {
                        //             match &item {
                        //                 FrameItem::Group(..) => {
                        //                     format!("[GROUP]: {:?}", &item)
                        //                 }
                        //                 FrameItem::Text(..) => {
                        //                     format!("[TEXT]: {:?}", &item)
                        //                 }
                        //                 FrameItem::Shape(..) => {
                        //                     format!("[SHAPE]: {:?}", &item)
                        //                 }
                        //                 FrameItem::Image(..) => {
                        //                     format!("[IMAGE]: {:?}", &item)
                        //                 }
                        //                 FrameItem::Link(..) => {
                        //                     format!("[LINK]: {:?}", &item)
                        //                 }
                        //                 FrameItem::Tag(..) => {
                        //                     format!("[TAG]: {:?}", &item)
                        //                 }
                        //             }
                        //         })
                        //         .collect_vec()
                        // );

                        page.frame.items().flat_map(|frame_item| {
                            fn frame_item_range(
                                item: &FrameItem,
                                context: &FileContext,
                                world: &MnemoWorld,
                            ) -> Option<Range<usize>> {
                                let span = match item {
                                    // FrameItem::Group(group) => {
                                    //     let items = group.frame.items();

                                    //     let ranges = items.filter_map(|(_point, item)| {
                                    //         frame_item_range(item, &world)
                                    //     });

                                    //     let (starts, ends): (Vec<usize>, Vec<usize>) = ranges
                                    //         .map(|range| (range.start, range.end))
                                    //         .unzip();
                                    //     let start =
                                    //         starts.into_iter().min().unwrap_or_default();
                                    //     let end = ends.into_iter().max().unwrap_or_default();

                                    //     return Some(start..end);
                                    // }
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
                                        match tag {
                                            Tag::Start(content, _flags) => content.span(),
                                            Tag::End(_location, _key, _flags) => Span::detached(),
                                        }
                                    }
                                };

                                if Some(context.main_id) == span.id() {
                                    world.range(span)
                                } else {
                                    None
                                }
                            }

                            fn frame_with_bounds(
                                frame_item: &(Point, FrameItem),
                                context: &FileContext,
                                world: &MnemoWorld,
                            ) -> Box<[FrameBlock]> {
                                let (point, item) = frame_item;

                                let (start_height, end_height) = match &item {
                                    FrameItem::Text(text) => {
                                        (cmp::max(point.y - text.size, Abs::zero()), point.y)
                                    }
                                    FrameItem::Group(group) => {
                                        return group
                                            .frame
                                            .items()
                                            .flat_map(|frame_item| {
                                                frame_with_bounds(frame_item, &context, &world)
                                                    .into_iter()
                                                    .map(|mut frame_block| {
                                                        frame_block.start_height += point.y;
                                                        frame_block.end_height += point.y;

                                                        frame_block
                                                    })
                                            })
                                            .collect::<Box<[_]>>();
                                    }
                                    FrameItem::Shape(shape, _span) => {
                                        (point.y, shape.geometry.bbox_size().y)
                                    }
                                    FrameItem::Image(_image, axes, _span) => (point.y, axes.y),
                                    FrameItem::Link(..) => (point.y, point.y),
                                    FrameItem::Tag(..) => (point.y, point.y),
                                };

                                let range = frame_item_range(&item, &context, world);

                                // crate::log!("[F RANGE] {range:?}");

                                // match &item {
                                //     FrameItem::Group(..) => {
                                //         crate::log!("[F GROUP]: {:?}", &item)
                                //     }
                                //     FrameItem::Text(..) => {
                                //         crate::log!("[F TEXT]: {:?}", &item)
                                //     }
                                //     FrameItem::Shape(..) => {
                                //         crate::log!("[F SHAPE]: {:?}", &item)
                                //     }
                                //     FrameItem::Image(..) => {
                                //         crate::log!("[F IMAGE]: {:?}", &item)
                                //     }
                                //     FrameItem::Link(..) => {
                                //         crate::log!("[F LINK]: {:?}", &item)
                                //     }
                                //     FrameItem::Tag(..) => {
                                //         crate::log!("[F TAG]: {:?}", &item)
                                //     }
                                // }

                                Box::from_iter(iter::once(FrameBlock {
                                    range,
                                    start_height,
                                    end_height,
                                    item: item.clone(),
                                }))
                            }

                            frame_with_bounds(frame_item, &context, &state.world)
                        })
                    })
                    .peekable();

                let mut ranged_items = Vec::with_capacity(ast_blocks.len());

                let mut ast_blocks = ast_blocks.iter().peekable();

                while let Some(ast_block) = ast_blocks.next() {
                    // let mut index = 0;

                    let aux_source = context.aux_source(&state.world).unwrap();

                    let aux_range = &ast_block.range;
                    let aux_lines = aux_source.lines();
                    let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start).unwrap();
                    let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end).unwrap();
                    let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                    let main_range_start = context.map_aux_to_main(aux_range.start);
                    let main_range_end = context.map_aux_to_main(aux_range.end);
                    let main_range = main_range_start..main_range_end;

                    let mut items = Vec::new();
                    // let mut deferred_items = Vec::<FrameBlock>::new();

                    let mut block_start_height = None;
                    let mut block_end_height = None;

                    while let Some(frame_block) = frame_blocks.peek() {
                        // index += 1;

                        if let Some(range) = &frame_block.range {
                            if range.end <= main_range_end + 1 {
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
                            } else {
                                break;
                            }
                        } else {
                            frame_blocks.next().unwrap();
                            // let frame_block = frame_blocks.next().unwrap();
                            // deferred_items.push(frame_block);
                        }
                    }

                    let block_start_height = block_start_height.unwrap_or_default().to_pt();
                    let block_end_height = block_end_height.unwrap_or_default().to_pt();

                    // let items = frame_blocks
                    //     .peeking_take_while(|frame_block| {
                    //         crate::log!("[{index} ITEM_RANGE]: {:?}", &frame_block.range);
                    //         crate::log!("[{index} BLOCK_RANGE]: {:?}", &main_range);

                    //         index += 1;

                    //         if let Some(range) = &frame_block.range {
                    //             range.end <= main_range_end + 1
                    //             // range.start < main_range_end
                    //         } else {
                    //             true
                    //         }
                    //     })
                    //     .collect_vec();

                    // for (i, frame_block) in items.iter().enumerate() {
                    //     crate::group!("{i} [ITEMS]");

                    //     crate::log!("[BLOCK_RANGE]: {:?}", frame_block.range);
                    //     crate::log!("[BLOCK_START]: {:?}", frame_block.start_height);
                    //     crate::log!("[BLOCK_END]: {:?}", frame_block.end_height);

                    //     match &frame_block.item {
                    //         FrameItem::Group(..) => {
                    //             crate::log!("[GROUP]: {:?}", &frame_block.item)
                    //         }
                    //         FrameItem::Text(..) => {
                    //             crate::log!("[TEXT]: {:?}", &frame_block.item)
                    //         }
                    //         FrameItem::Shape(..) => {
                    //             crate::log!("[SHAPE]: {:?}", &frame_block.item)
                    //         }
                    //         FrameItem::Image(..) => {
                    //             crate::log!("[IMAGE]: {:?}", &frame_block.item)
                    //         }
                    //         FrameItem::Link(..) => {
                    //             crate::log!("[LINK]: {:?}", &frame_block.item)
                    //         }
                    //         FrameItem::Tag(..) => {
                    //             crate::log!("[TAG]: {:?}", &frame_block.item)
                    //         }
                    //     }

                    //     crate::group_end!("{i} [ITEMS]");
                    // }

                    // crate::log!(
                    //     "[1 ITEMS]: {:?}",
                    //     items
                    //         .iter()
                    //         .map(|frame_block| {
                    //             match &frame_block.item {
                    //                 FrameItem::Group(..) => {
                    //                     format!("[GROUP]: {:?}", &frame_block.item)
                    //                 }
                    //                 FrameItem::Text(..) => {
                    //                     format!("[TEXT]: {:?}", &frame_block.item)
                    //                 }
                    //                 FrameItem::Shape(..) => {
                    //                     format!("[SHAPE]: {:?}", &frame_block.item)
                    //                 }
                    //                 FrameItem::Image(..) => {
                    //                     format!("[IMAGE]: {:?}", &frame_block.item)
                    //                 }
                    //                 FrameItem::Link(..) => {
                    //                     format!("[LINK]: {:?}", &frame_block.item)
                    //                 }
                    //                 FrameItem::Tag(..) => {
                    //                     format!("[TAG]: {:?}", &frame_block.item)
                    //                 }
                    //             }
                    //         })
                    //         .collect_vec()
                    // );

                    // let block_starts = items
                    //     .iter()
                    //     // .filter(|frame_block| frame_block.range.is_some())
                    //     .map(|frame_block| frame_block.start_height);
                    // // crate::log!("[BLOCK_STARTS]: {:?}", block_starts.clone().collect_vec());
                    // let block_start_height = block_starts.min().unwrap_or_default().to_pt();
                    // let block_ends = items
                    //     .iter()
                    //     // .filter(|frame_block| frame_block.range.is_some())
                    //     .map(|frame_block| frame_block.end_height);
                    // // crate::log!("[BLOCK_ENDS]: {:?}", block_ends.clone().collect_vec());
                    // let block_end_height = block_ends.max().unwrap_or_default().to_pt();

                    match context.height {
                        Some(height) if block_start_height >= height => {
                            continue;
                        }
                        _ => {}
                    }

                    // let previous_height = ranged_items.last().unwrap();
                    // let delta = (previous_height - block_end_height - 1.0).max(0.0);

                    let height = block_end_height - block_start_height;

                    if height <= 0_f64 {
                        continue;
                    }

                    ranged_items.push((
                        aux_range_utf16.clone(),
                        block_start_height,
                        block_end_height,
                    ));

                    // for frame_block in deferred_items {
                    //     let previous_end = aux_end_utf16;
                    //     let next_start = match ast_blocks.peek() {
                    //         Some(ast_block) => ast_block.range.end,
                    //         None => {
                    //             let main_source = state.world.main_source();
                    //             main_source.text().len()
                    //         }
                    //     };

                    //     ranged_items.push((
                    //         previous_end..next_start,
                    //         frame_block.start_height.to_pt(),
                    //         frame_block.end_height.to_pt(),
                    //     ))
                    // }
                }

                let document_height = document
                    .pages
                    .iter()
                    .map(|page| page.frame.height())
                    .sum::<Abs>()
                    .to_pt();

                last_document = Some(document);

                let mut previous_height = document_height;

                ranged_items
                    .into_iter()
                    .rev()
                    .filter_map(|(range, block_start_height, block_end_height)| {
                        // crate::log!("PREV HEIGHT: {previous_height}");
                        // crate::log!("ITEM RANGE: {range:?}");
                        // crate::log!("ITEM START HEIGHT: {block_start_height:?}");
                        // crate::log!("ITEM END HEIGHT: {block_end_height:?}");

                        let delta = (previous_height - block_end_height - 1.0).max(0.0);

                        let height = block_end_height - block_start_height;

                        if height <= 0_f64 {
                            return None;
                        }

                        previous_height = block_start_height;

                        // crate::log!("[INDEX]: {i}");
                        // crate::log!("[HEIGHT]: {height:?}");
                        // crate::log!("[DELTA]: {delta:?}");

                        Some((range, height + delta, block_start_height))
                    })
                    .collect_vec()
                    .into_iter()
                    .rev()
                    .collect_vec()
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
                    .collect::<HashSet<_>>();

                // crate::log!("[ERROR RANGES]: {error_ranges:?}");

                // let main_source = context.main_source(&self.world);

                let Some(block) = ast_blocks.iter().find(|block| {
                    let aux_range = &block.range;

                    let main_range_start = context.map_aux_to_main(aux_range.start);
                    let main_range_end = context.map_aux_to_main(aux_range.end);
                    // let main_range = main_range_start..main_range_end;

                    // crate::log!("[BLOCK RANGE]: {main_range_start} - {main_range_end}");

                    error_ranges.iter().any(|error_range| {
                        (main_range_start <= error_range.start
                            && main_range_end >= error_range.start)
                            || (main_range_start <= error_range.end
                                && main_range_end >= error_range.end)
                    })
                }) else {
                    break;
                };

                // let aux_source = context.aux_source(&state.world);
                // let aux_lines = aux_source.lines();

                let aux_range = &block.range;
                // let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start).unwrap();
                // let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end).unwrap();
                // let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                let mut end_byte = context.map_aux_to_main(aux_range.end);
                if block.is_inline {
                    // TODO: proper offsetting (?)
                    end_byte += 10;
                }

                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &context,
                    &state.world,
                ));

                // crate::error!("[ERRORS]: {diagnostics:?}");
                panic!("[ERRORS]: {diagnostics:?}");

                let start_byte = context.map_aux_to_main(aux_range.start);

                let range_delta = end_byte - start_byte;
                let repeat_range = range_delta - if range_delta > 2 { 2 } else { 1 };

                let source = context.main_source_mut(&mut state.world).unwrap();
                source.edit(start_byte..end_byte, &(" ".repeat(repeat_range) + "\\ "));

                Vec::new()
            }
        };
    }

    let frames = if let Some(document) = &last_document {
        let canvas = typst_render::render_merged(document, context.pixel_per_pt, Abs::zero(), None);
        let width = canvas.width();

        ranged_heights
            .into_iter()
            .map(|(range, height, offset_height)| {
                let rect = IntRect::from_xywh(
                    0,
                    (offset_height as f32 * context.pixel_per_pt).ceil() as i32,
                    width,
                    (height as f32 * context.pixel_per_pt).ceil() as u32,
                )
                .unwrap();
                let canvas = canvas.clone_rect(rect).unwrap();
                let encoding = canvas.encode_png().unwrap();

                let hash = HighwayHasher::default().hash64(&encoding) as u32;

                let height = height.ceil() as u32;

                let render = FrameRender {
                    encoding,
                    hash,
                    height,
                    offset_height,
                };

                RangedFrame { range, render }
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

    context.document = last_document;
    context
        .main_source_mut(&mut state.world)
        .unwrap()
        .replace(&ir);

    CompileResult {
        frames,
        diagnostics,
    }
}
