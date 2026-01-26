use std::hash::BuildHasher;

use rustc_hash::FxBuildHasher;
use tiny_skia::IntRect;
use typst::{
    WorldExt, compile,
    layout::{Abs, PagedDocument},
};

// use typst_svg::{svg, svg_merged};
use crate::{
    renderer::{
        paged::{FrameRender, RangedFrame, RenderResult},
        sync_source_context,
    },
    state::TypstState,
    wrappers::{TypstDiagnostic, TypstFileId},
};

pub fn render_by_chunk(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> RenderResult {
    let (ir, ast_blocks) = sync_source_context(id, text, prelude, state);

    let mut last_document = None;

    let mut offset_height = 0_f64;
    let mut diagnostics = Vec::new();
    let mut compiled_warnings = None;

    let context = state.source_context_map.get_mut(id).unwrap();

    let ranged_heights = ast_blocks
        .into_iter()
        .filter_map(|block| {
            match context.height {
                Some(height) if offset_height >= height => return None,
                _ => {}
            }

            let aux_source = context.aux_source(&state.world)?;

            let aux_range = block.range;
            let aux_lines = aux_source.lines();
            let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start)?;
            let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end)?;
            let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

            let mut end_byte = context.map_aux_to_main(aux_range.end);
            if block.is_inline {
                // TODO: proper offsetting (?)
                end_byte += 29;
            }

            let source = context.main_source_mut(&mut state.world)?;
            let source_len = source.text().len();
            source.edit(source_len..source_len, ir.get(source_len..end_byte)?);

            // crate::log!("[RANGE_UTF8]: {aux_range:?}");
            // crate::log!("[RANGE_UTF16]: {range_utf16:?}");

            let compiled = compile::<PagedDocument>(&state.world);
            compiled_warnings = Some(compiled.warnings);

            match compiled.output {
                Ok(document) => {
                    // TODO: handle changes in page margins

                    let document_height = document
                        .pages
                        .iter()
                        .map(|page| page.frame.height())
                        .sum::<Abs>()
                        .to_pt();

                    let height = document_height - offset_height - 1.0;

                    if height <= 0_f64 {
                        return None;
                    }

                    let ranged_height = Some((aux_range_utf16, height, offset_height));

                    offset_height = document_height - 1.0;
                    last_document = Some(document);

                    ranged_height
                }
                Err(source_diagnostics) => {
                    diagnostics.extend(TypstDiagnostic::from_diagnostics(
                        source_diagnostics,
                        &context,
                        &state.world,
                    ));

                    // crate::error!("[ERRORS]: {diagnostics:?}");

                    let start_byte = context.map_aux_to_main(aux_range.start);

                    let range_delta = end_byte - start_byte;

                    let source = context.main_source_mut(&mut state.world)?;
                    source.edit(start_byte..end_byte, &(" ".repeat(range_delta)));

                    None
                }
            }
        })
        .collect::<Vec<_>>();

    let frames = if let Some(document) = &last_document {
        let canvas = typst_render::render_merged(document, context.pixel_per_pt, Abs::zero(), None);
        let width = canvas.width();

        ranged_heights
            .into_iter()
            .filter_map(|(range, height, offset_height)| {
                let rect = IntRect::from_xywh(
                    0,
                    (offset_height as f32 * context.pixel_per_pt).ceil() as i32,
                    width,
                    (height as f32 * context.pixel_per_pt).ceil() as u32,
                )?;
                let canvas = canvas.clone_rect(rect)?;
                let encoding = canvas.encode_png().ok()?;

                let hash = FxBuildHasher.hash_one(&encoding) as u32;

                let height = height.ceil() as u32;

                let render = FrameRender {
                    encoding,
                    hash,
                    height,
                    offset_height,
                };

                Some(RangedFrame { range, render })
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
    if let Some(main_source) = context.main_source_mut(&mut state.world) {
        main_source.replace(&ir);
    }

    RenderResult {
        frames,
        diagnostics,
    }
}
