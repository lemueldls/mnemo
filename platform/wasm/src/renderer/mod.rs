//! Document renderer for Typst in Mnemo.
//!
//! This module provides the core logic for rendering Typst documents, including chunking, diagnostics, and output for multiple targets (SVG, HTML, PDF).
//!
//! # Main and Aux Sources
//!
//! The renderer operates on two parallel representations of the document:
//!
//! - **Aux source**: The origin text a user writes in an editor. This is the user's direct input and is used for mapping diagnostics, highlights, and incremental updates.
//! - **Main source**: The intermediate file used for compilation. This is a transformed version of the aux source, prepared for Typst's incremental compilation and error reporting. The main source is the authoritative version used for diagnostics and output.
//!
//! The mapping between aux and main sources is maintained throughout the rendering process (see [`IndexMapper`]), allowing for robust error localization and efficient incremental updates. Most rendering functions synchronize both sources before producing output or diagnostics.
//!
//! Unless extending the renderer or integrating with Typst's incremental compilation, you will rarely need to interact with both sources directly. Most APIs abstract over this duality.

pub mod html;
pub mod paged;

use std::ops::{ControlFlow, Range};

use ecow::EcoVec;
use rustc_hash::FxHashSet;
use typst::{
    WorldExt,
    diag::{Severity, SourceDiagnostic},
    syntax::SyntaxKind,
};

use crate::{
    index_mapper::IndexMapper,
    state::{SourceContext, TypstState},
    world::MnemoWorld,
    wrappers::{TypstDiagnostic, TypstFileId, map_main_span},
};

/// Represents a block in the Typst AST, with its byte range and inline status.
#[derive(Debug, Clone)]
pub struct AstBlock {
    /// Byte range of the block in the source.
    pub range: Range<usize>,
    /// Whether the block is inline (not a standalone block).
    pub is_inline: bool,
}

/// Output target for rendering.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum RenderTarget {
    /// Render as SVG.
    Svg,
    /// Render as PDF.
    Pdf,
    /// Render as HTML.
    Html,
}

/// Synchronizes the Typst source context, producing an intermediate representation and AST blocks.
///
/// Returns a tuple of the intermediate representation and a vector of AST blocks.
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

                context.index_mapper.push_aux_to_main(range.start, ir.len());
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

/// Wraps a block of Typst source for rendering, updating the intermediate representation and block metadata.
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
        Some(
            SyntaxKind::ListItem | SyntaxKind::EnumItem | SyntaxKind::TermItem | SyntaxKind::Label,
        ) => {
            *ir += &text[last_block.range.clone()];
            last_block.is_inline = true
        }
        _ => {
            *ir += "#block(stroke:0pt,width:100%)[";
            context
                .index_mapper
                .push_aux_to_main(last_block.range.start, ir.len());
            *ir += &text[last_block.range.clone()];
            context
                .index_mapper
                .push_aux_to_main(last_block.range.end, ir.len());
            *ir += "\n]";

            last_block.is_inline = true
        }
    }

    *ir += "\n";
    context
        .index_mapper
        .push_aux_to_main(last_block.range.end, ir.len());
}

/// Removes the block containing the first error from the source, updating diagnostics and context.
///
/// Returns the indices of the removed blocks, or an empty vector if no error blocks were found.
#[typst_macros::time]
pub fn remove_errornous_block(
    ast_blocks: &[AstBlock],
    source_diagnostics: EcoVec<SourceDiagnostic>,
    diagnostics: &mut Vec<TypstDiagnostic>,
    context: &mut SourceContext,
    world: &mut MnemoWorld,
) -> Vec<usize> {
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

    let (indicies, main_ranges): (Vec<usize>, Vec<Range<usize>>) = ast_blocks
        .iter()
        .enumerate()
        .filter_map(|(idx, block)| {
            let aux_range = &block.range;

            let main_range_start = context.map_aux_to_main_from_left(aux_range.start);
            let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
            let main_range = main_range_start..main_range_end;

            let in_block = error_ranges.iter().any(|error_range| {
                (main_range_start <= error_range.start && main_range_end >= error_range.start)
                    || (main_range_start <= error_range.end && main_range_end >= error_range.end)
            });

            in_block.then_some((idx, main_range))
        })
        .unzip();

    for main_range in main_ranges {
        let start_byte = main_range.start;
        let end_byte = main_range.end;

        let source = context.main_source_mut(world).unwrap();
        crate::log!("[REPLACING]:\n{}", &source.text()[start_byte..end_byte]);
        let byte_length = end_byte - start_byte;
        let whitespace = " ".repeat(byte_length.saturating_sub(1)) + "\n";
        source.edit(start_byte..end_byte, &whitespace);
        // crate::log!("[NEW SOURCE]:\n{}", &source.text());
    }

    indicies
}

/// Tries to mark the specific expressions containing errors and wraps it in a red text expression (using #text(fill:theme.error)[expr]) for visual feedback in the rendered output.
/// If marking is unsuccessful (e.g., due to complex expressions or multiple errors), it falls back to removing the entire block containing the error, as implemented in `remove_errornous_block`.
#[typst_macros::time]
pub fn try_mark_errornous(
    ast_blocks: &[AstBlock],
    source_diagnostics: EcoVec<SourceDiagnostic>,
    context: &mut SourceContext,
    world: &mut MnemoWorld,
) {
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

    let mut marked = false;

    for block in ast_blocks {
        let aux_range = &block.range;

        let main_range_start = context.map_aux_to_main_from_left(aux_range.start);
        let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
        let main_range = main_range_start..main_range_end;

        let in_block = error_ranges.iter().any(|error_range| {
            (main_range_start <= error_range.start && main_range_end >= error_range.start)
                || (main_range_start <= error_range.end && main_range_end >= error_range.end)
        });

        if in_block {
            let source = context.main_source_mut(world).unwrap();
            let original_text = &source.text()[main_range.clone()];
            crate::log!("[MARKING]:\n{}", original_text);

            // Wrap the original text in a red text expression
            let marked_text = format!("#text(fill:theme.error)[{}]", original_text);
            source.edit(main_range.clone(), &marked_text);

            crate::log!("[NEW SOURCE]:\n{}", &source.text());

            marked = true;
            break;
        }
    }

    if !marked {
        crate::log!("Failed to mark specific expression, falling back to block removal.");
    }
}
