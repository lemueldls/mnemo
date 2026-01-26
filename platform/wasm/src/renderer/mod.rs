pub mod html;
pub mod paged;
// pub mod svg;

use std::{collections::VecDeque, iter, ops::Range};

use itertools::{Either, Itertools, MinMaxResult};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{WorldExt, syntax::SyntaxKind};
use typst_syntax::{Span, SyntaxNode};
// use typst_html::html;
// use typst_svg::{svg, svg_merged};
use wasm_bindgen::prelude::*;

use crate::{
    index_mapper::IndexMapper,
    state::TypstState,
    wrappers::{TypstDiagnostic, TypstFileId},
};

pub fn sync_source_context(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    render_target: RenderTarget,
    state: &mut TypstState,
) -> (String, Vec<AstBlock>) {
    let mut ir = state.prelude(id, render_target) + prelude + "\n";

    let context = state.source_context_map.get_mut(id).unwrap();

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

                match last_kind {
                    Some(
                        SyntaxKind::LetBinding
                        | SyntaxKind::SetRule
                        | SyntaxKind::ShowRule
                        | SyntaxKind::ModuleImport
                        | SyntaxKind::ModuleInclude
                        | SyntaxKind::Contextual
                        | SyntaxKind::ListItem
                        | SyntaxKind::EnumItem
                        | SyntaxKind::TermItem
                        | SyntaxKind::Linebreak
                        | SyntaxKind::Semicolon
                        | SyntaxKind::LineComment
                        | SyntaxKind::BlockComment,
                    ) => {
                        ir += &text[last_block.range.clone()];
                    }
                    _ => {
                        ir += "#block(stroke:red,width:100%)[";
                        ir += &text[last_block.range.clone()];
                        ir += "]";

                        last_block.is_inline = true
                    }
                }

                // crate::log!("[LAST_KIND]: {last_kind:?}");

                ir += "\n";
                context
                    .index_mapper
                    .add_main_to_aux(last_block.range.end, ir.len());
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
            context
                .index_mapper
                .add_main_to_aux(last_block.range.end, ir.len());
        }
    }

    // crate::log!("[RANGES]: {block_ranges:?}");

    crate::log!(
        "[SOURCE]:\n{}",
        &ir[(state.prelude(id, RenderTarget::Png) + prelude + "\n").len()..]
    );

    (ir, ast_blocks)
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
    pub range: Range<usize>,
    pub is_inline: bool,
}

pub enum RenderTarget {
    Png,
    Pdf,
    Html,
}
