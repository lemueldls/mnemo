use std::{hash::BuildHasher, ops::Range};

use rustc_hash::{FxBuildHasher, FxHashSet};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{compile, diag::Severity};
use typst_html::{HtmlDocument, HtmlOptions};

use crate::{
    bindings::{TypstDiagnostic, TypstFileId, map_synth_span},
    source::{RenderTarget, SynthResult, sync_source_state},
    state::TypstState,
};

pub fn render(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> HTMLRenderResult {
    let SynthResult {
        synth: ir,
        blocks: blocks,
        ..
    } = sync_source_state(id, text, prelude, RenderTarget::Html, state);

    let mut last_document = None;

    let mut diagnostics = Vec::new();
    let mut compiled_warnings = None;

    // let mut erronous_ranges = Vec::new();

    let context = state.source_context_map.get_mut(id).unwrap();

    context
        .synth_source_mut(&mut state.world)
        .unwrap()
        .replace(&ir);

    let mut frames = Vec::new();

    while last_document.is_none() {
        let compiled = compile::<HtmlDocument>(&state.world);
        compiled_warnings = Some(compiled.warnings);

        // crate::log!("[DOING A THING]");

        frames = match compiled.output {
            Ok(document) => {
                let element = typst_html::html(&document, &HtmlOptions::default())
                    .expect("HTML rendering failed");

                last_document = Some(document);

                vec![HTMLRangedFrame {
                    range: 0..text.len(),
                    render: HTMLFrameRender {
                        html: element.clone(),
                        #[allow(clippy::cast_possible_truncation)]
                        hash: FxBuildHasher.hash_one(&element) as u32,
                    },
                }]
            }
            Err(source_diagnostics) => {
                let error_ranges = source_diagnostics
                    .iter()
                    .filter_map(|diagnostic| {
                        map_synth_span(
                            diagnostic.span,
                            diagnostic.severity == Severity::Error,
                            &diagnostic.trace,
                            context,
                            &state.world,
                        )
                    })
                    .collect::<FxHashSet<_>>();

                // crate::log!("[ERROR RANGES]: {error_ranges:?}");

                // let synth_source = context.synth_source(&self.world);

                let Some(block) = blocks.iter().find(|block| {
                    let raw_range = &block.range;

                    let synth_range_start = context.map_raw_to_synth_from_right(raw_range.start);
                    let synth_range_end = context.map_raw_to_synth_from_right(raw_range.end);
                    // let synth_range = synth_range_start..synth_range_end;

                    // crate::log!("[BLOCK RANGE]: {synth_range_start} - {synth_range_end}");

                    error_ranges.iter().any(|error_range| {
                        (synth_range_start <= error_range.start
                            && synth_range_end >= error_range.start)
                            || (synth_range_start <= error_range.end
                                && synth_range_end >= error_range.end)
                    })
                }) else {
                    break;
                };

                // let raw_source = context.raw_source(&state.world);
                // let raw_lines = raw_source.lines();

                let raw_range = &block.range;
                // let raw_start_utf16 = raw_lines.byte_to_utf16(raw_range.start).unwrap();
                // let raw_end_utf16 = raw_lines.byte_to_utf16(raw_range.end).unwrap();
                // let raw_range_utf16 = raw_start_utf16..raw_end_utf16;

                let mut end_byte = context.map_raw_to_synth_from_right(raw_range.end);
                if block.is_inline {
                    end_byte += 12;
                }

                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    context,
                    &state.world,
                ));

                crate::error!("[ERRORS]: {diagnostics:?}");

                let start_byte = context.map_raw_to_synth_from_right(raw_range.start);

                let source = context.synth_source_mut(&mut state.world).unwrap();
                source.edit(start_byte..end_byte, &(" ".repeat(end_byte - start_byte)));

                Vec::new()
            }
        }
    }

    crate::debug!("FRAMES: {frames:?}");

    if let Some(warnings) = compiled_warnings {
        diagnostics.extend(TypstDiagnostic::from_diagnostics(
            warnings,
            context,
            &state.world,
        ));
    }

    HTMLRenderResult {
        frames,
        diagnostics,
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct HTMLRenderResult {
    pub frames: Vec<HTMLRangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct HTMLRangedFrame {
    pub range: Range<usize>,
    pub render: HTMLFrameRender,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct HTMLFrameRender {
    html: String,
    hash: u32,
}

/// Result of rendering a Typst document to HTML.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderHtmlResult {
    /// The rendered HTML document, if successful.
    pub document: Option<String>,
    /// Diagnostics and warnings produced during rendering.
    pub diagnostics: Vec<TypstDiagnostic>,
}
