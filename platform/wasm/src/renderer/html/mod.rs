use std::{
    cmp,
    hash::{BuildHasher, Hash, Hasher},
    iter,
    ops::Range,
};

use ecow::eco_vec;
use rustc_hash::{FxBuildHasher, FxHashSet};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{compile, diag::Severity, introspection::Tag, syntax::Span};
// use ecow::{EcoString, eco_format};
// use typst_library::diag::{At, SourceResult, StrResult, bail};
// use typst_library::foundations::Repr;
// use typst_library::introspection::Introspector;
// use typst_syntax::Span;
use typst_html::{HtmlDocument, HtmlElement, HtmlNode, HtmlOptions, tag};

use crate::{
    renderer::{RenderTarget, sync_source_state},
    state::{SourceContext, TypstState},
    world::MnemoWorld,
    wrappers::{TypstDiagnostic, TypstFileId, map_main_span},
};

pub fn render(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> HTMLRenderResult {
    let (ir, ast_blocks) = sync_source_state(id, text, prelude, RenderTarget::Html, state);

    let mut last_document = None;

    let mut diagnostics = Vec::new();
    let mut compiled_warnings = None;

    // let mut erronous_ranges = Vec::new();

    let context = state.source_context_map.get_mut(id).unwrap();

    context
        .main_source_mut(&mut state.world)
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
                        html: element.to_string(),
                        hash: {
                            let mut hasher = FxBuildHasher::default().build_hasher();
                            element.to_string().hash(&mut hasher);
                            hasher.finish() as u32
                        },
                    },
                }]
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

                // crate::log!("[ERROR RANGES]: {error_ranges:?}");

                // let main_source = context.main_source(&self.world);

                let Some(block) = ast_blocks.iter().find(|block| {
                    let aux_range = &block.range;

                    let main_range_start = context.map_aux_to_main_from_right(aux_range.start);
                    let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
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

                let mut end_byte = context.map_aux_to_main_from_right(aux_range.end);
                if block.is_inline {
                    end_byte += 12;
                }

                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &context,
                    &state.world,
                ));

                crate::error!("[ERRORS]: {diagnostics:?}");

                let start_byte = context.map_aux_to_main_from_right(aux_range.start);

                let source = context.main_source_mut(&mut state.world).unwrap();
                source.edit(start_byte..end_byte, &(" ".repeat(end_byte - start_byte)));

                Vec::new()
            }
        }
    }

    crate::debug!("FRAMES: {frames:?}");

    if let Some(warnings) = compiled_warnings {
        diagnostics.extend(TypstDiagnostic::from_diagnostics(
            warnings,
            &context,
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
