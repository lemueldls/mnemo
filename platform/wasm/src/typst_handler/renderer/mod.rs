mod chunk;
mod items;

use std::ops::Range;

pub use chunk::render_by_chunk;
pub use items::render_by_items;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{
    WorldExt,
    layout::{Abs, FrameItem},
    syntax::SyntaxKind,
};
// use typst_html::html;
// use typst_svg::{svg, svg_merged};
use wasm_bindgen::prelude::*;

use super::{
    index_mapper::IndexMapper,
    wrappers::{TypstDiagnostic, TypstFileId},
};
use crate::typst_handler::state::TypstState;

pub fn sync_file_context(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> (String, Vec<AstBlock>) {
    let mut ir = state.prelude(id, RenderingMode::Png) + prelude + "\n";

    let context = state.file_contexts.get_mut(id).unwrap();

    context.index_mapper = IndexMapper::default();
    context.index_mapper.add_main_to_aux(0, ir.len());

    state.world.main_id = Some(context.main_id);
    context
        .main_source_mut(&mut state.world)
        .unwrap()
        .replace(&ir);

    context
        .aux_source_mut(&mut state.world)
        .unwrap()
        .replace(&text);
    state.world.aux_id = Some(context.aux_id);

    let aux_source = context.aux_source(&state.world).unwrap();

    let children = aux_source.root().children();
    let text = aux_source.text();

    let mut ast_blocks = Vec::<AstBlock>::new();
    let mut in_block = false;

    let mut last_kind: Option<SyntaxKind> = None;

    for node in children {
        let range = state.world.range(node.span()).unwrap();

        if let Some(until_newline) = node.text().chars().position(|ch| ch == '\n') {
            in_block = false;

            if let Some(last_block) = ast_blocks.last_mut() {
                last_block.range.end += until_newline;

                ir += &text[last_block.range.clone()];

                match last_kind {
                    Some(
                        SyntaxKind::LetBinding
                        | SyntaxKind::SetRule
                        | SyntaxKind::ShowRule
                        | SyntaxKind::ModuleImport
                        | SyntaxKind::ModuleInclude
                        | SyntaxKind::Contextual
                        // | SyntaxKind::ListItem
                        // | SyntaxKind::EnumItem
                        | SyntaxKind::Linebreak
                        | SyntaxKind::Semicolon
                        | SyntaxKind::LineComment
                        | SyntaxKind::BlockComment,
                    ) => {}
                    _ => {
                        ir += "\n#block(above:0pt,below:0pt)";
                        last_block.is_inline = true
                    }
                }

                // crate::log!("[LAST_KIND]: {last_kind:?}");

                ir += "\n";
            }
        } else {
            last_kind = Some(node.kind());

            if in_block {
                ast_blocks.last_mut().unwrap().range.end = range.end;
            } else {
                in_block = true;

                context.index_mapper.add_main_to_aux(range.start, ir.len());
                ast_blocks.push(AstBlock {
                    range,
                    is_inline: false,
                });
            }
        }
    }

    if let Some(last_block) = ast_blocks.last_mut() {
        if in_block {
            ir += &text[last_block.range.clone()];
        }
    }

    // crate::log!("[RANGES]: {block_ranges:?}");

    // crate::log!(
    //     "[SOURCE]: {:?}",
    //     &ir[(state.prelude(id, RenderingMode::Png) + prelude + "\n").len()..]
    // );

    (ir, ast_blocks)
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderResult {
    pub frames: Vec<RangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderPdfResult {
    pub bytes: Option<Vec<u8>>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderHtmlResult {
    pub document: Option<String>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Debug, Clone)]
pub struct AstBlock {
    range: Range<usize>,
    is_inline: bool,
}

#[derive(Debug, Clone)]
pub struct FrameBlock {
    range: Option<Range<usize>>,
    start_height: Abs,
    end_height: Abs,
    item: FrameItem,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RangedFrame {
    pub range: Range<usize>,
    pub render: FrameRender,
}

impl RangedFrame {
    pub fn new(range: Range<usize>, render: FrameRender) -> Self {
        Self { range, render }
    }
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrameRender {
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    encoding: Vec<u8>,
    hash: u32,
    height: u32,
    #[serde(rename = "offsetHeight")]
    offset_height: f64,
}

pub enum RenderingMode {
    Png,
    Pdf,
    // Html,
}
