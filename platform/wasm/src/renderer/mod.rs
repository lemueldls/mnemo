pub mod html;
pub mod paged;
// pub mod svg;

use std::{
    collections::VecDeque,
    iter,
    ops::{ControlFlow, Range},
};

use comemo::Tracked;
use ecow::EcoVec;
use itertools::{Either, Itertools, MinMaxResult};
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{
    WorldExt,
    diag::{Severity, SourceDiagnostic},
    syntax::SyntaxKind,
};
use typst_syntax::{Span, SyntaxNode};
// use typst_html::html;
// use typst_svg::{svg, svg_merged};
use wasm_bindgen::prelude::*;

use crate::{
    index_mapper::IndexMapper,
    state::{SourceContext, TypstState},
    world::MnemoWorld,
    wrappers::{TypstDiagnostic, TypstFileId, map_main_span},
};

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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum RenderTarget {
    Svg,
    Pdf,
    Html,
}

#[typst_macros::time]
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
    // context.index_mapper.add_aux_to_main(0, ir.len());

    // context
    //     .main_source_mut(&mut state.world)
    //     .unwrap()
    //     .replace(&ir);
    state.world.main_id = Some(context.main_id);

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
                wrap_block(&mut ir, text, last_block, last_kind, context);
            }
        } else {
            last_kind = Some(node.kind());

            if in_block {
                ast_blocks.last_mut().unwrap().range.end = range.end;
            } else {
                in_block = true;

                context.index_mapper.add_aux_to_main(range.start, ir.len());
                ast_blocks.push(AstBlock {
                    range,
                    is_inline: false,
                });
            }
        }
    }

    if let Some(last_block) = ast_blocks.last_mut() {
        if in_block {
            wrap_block(&mut ir, text, last_block, last_kind, context);
        }
    }

    // crate::log!("[RANGES]: {block_ranges:?}");

    // crate::log!(
    //     "[SOURCE]:\n{}",
    //     &ir[(state.prelude(id, RenderTarget::Svg) + prelude + "\n").len()..]
    // );

    (ir, ast_blocks)
}

#[typst_macros::time]
fn wrap_block(
    ir: &mut String,
    text: &str,
    last_block: &mut AstBlock,
    last_kind: Option<SyntaxKind>,
    context: &mut SourceContext,
) {
    match last_kind {
        Some(
            SyntaxKind::LetBinding
            | SyntaxKind::SetRule
            | SyntaxKind::ShowRule
            | SyntaxKind::ModuleImport
            | SyntaxKind::ModuleInclude
            | SyntaxKind::Contextual
            | SyntaxKind::Linebreak
            | SyntaxKind::Semicolon
            | SyntaxKind::LineComment
            | SyntaxKind::BlockComment,
        ) => {
            *ir += &text[last_block.range.clone()];
        }
        Some(SyntaxKind::ListItem | SyntaxKind::EnumItem | SyntaxKind::TermItem) => {
            *ir += &text[last_block.range.clone()];
            last_block.is_inline = true
        }
        _ => {
            *ir += "#block(stroke:0pt,width:100%)[";
            context
                .index_mapper
                .add_aux_to_main(last_block.range.start, ir.len());
            *ir += &text[last_block.range.clone()];
            context
                .index_mapper
                .add_aux_to_main(last_block.range.end, ir.len());
            *ir += "]";

            last_block.is_inline = true
        }
    }

    *ir += "\n";
    context
        .index_mapper
        .add_aux_to_main(last_block.range.end, ir.len());
}

#[typst_macros::time]
pub fn remove_errornous_block(
    ast_blocks: &[AstBlock],
    source_diagnostics: EcoVec<SourceDiagnostic>,
    diagnostics: &mut Vec<TypstDiagnostic>,
    context: &mut SourceContext,
    world: &mut MnemoWorld,
) -> ControlFlow<(), usize> {
    let error_ranges = source_diagnostics
        .iter()
        .filter_map(|diagnostic| {
            map_main_span(
                diagnostic.span,
                diagnostic.severity == Severity::Error,
                &diagnostic.trace,
                context,
                world,
            )
        })
        .collect::<FxHashSet<_>>();

    diagnostics.extend(TypstDiagnostic::from_diagnostics(
        source_diagnostics,
        context,
        world,
    ));

    crate::error!("[ERRORS]: {diagnostics:?}");

    let Some((idx, main_range)) = ast_blocks.iter().enumerate().find_map(|(idx, block)| {
        let aux_range = &block.range;

        let main_range_start = context.map_aux_to_main_from_left(aux_range.start);
        let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
        let main_range = main_range_start..main_range_end;

        let in_block = error_ranges.iter().any(|error_range| {
            (main_range_start <= error_range.start && main_range_end >= error_range.start)
                || (main_range_start <= error_range.end && main_range_end >= error_range.end)
        });

        in_block.then_some((idx, main_range))
    }) else {
        return ControlFlow::Break(());
    };

    let start_byte = main_range.start;
    let end_byte = main_range.end;

    let source = context.main_source_mut(world).unwrap();
    // crate::log!("[REPLACING]:\n{}", &source.text()[start_byte..end_byte]);
    let byte_length = end_byte - start_byte;
    let whitespace = " ".repeat(byte_length.saturating_sub(1)) + "\n";
    source.edit(start_byte..end_byte, &whitespace);
    // crate::log!("[NEW SOURCE]:\n{}", &source.text());

    ControlFlow::Continue(idx)
}
