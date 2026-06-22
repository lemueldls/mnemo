use std::ops::Range;

use typst::{WorldExt, syntax::SyntaxKind};

use crate::{
    bindings::TypstFileId,
    source::{IndexMapper, SourceContext},
    state::TypstState,
    world::MnemoWorld,
};

pub struct SourceSyncResult {
    pub ir: String,
    pub ast_blocks: Vec<AstBlock>,
    pub equation_ranges: Vec<Range<usize>>,
}

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

#[typst_macros::time]
pub fn sync_source_state(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    render_target: RenderTarget,
    state: &mut TypstState,
) -> SourceSyncResult {
    let prelude = state.prelude(id, render_target) + prelude + "\n";
    let context = state.source_context_map.get_mut(id).unwrap();

    sync_source_context(text, prelude, context, &mut state.world)
}

/// Synchronizes the Typst source context, producing an intermediate representation and AST blocks.
///
/// Returns a tuple of the intermediate representation and a vector of AST blocks.
#[typst_macros::time]
pub fn sync_source_context(
    text: &str,
    prelude: String,
    context: &mut SourceContext,
    world: &mut MnemoWorld,
) -> SourceSyncResult {
    let mut ir = prelude;

    context.index_mapper = IndexMapper::default();
    // context.index_mapper.add_aux_to_main(0, ir.len());

    // context
    //     .main_source_mut(&mut world)
    //     .unwrap()
    //     .replace(&ir);
    world.main_id = Some(context.main_id);

    context.aux_source_mut(world).unwrap().replace(text);
    world.aux_id = Some(context.aux_id);

    let aux_source = context.aux_source(world).unwrap();

    let children = aux_source.root().children();
    let text = aux_source.text();

    let mut equation_ranges = Vec::new();

    let mut ast_blocks = Vec::<AstBlock>::new();
    let mut in_block = false;

    let mut last_kind: Option<SyntaxKind> = None;

    for node in children {
        let range = world.range(node.span()).unwrap();

        if node.kind() == SyntaxKind::Equation {
            equation_ranges.push(range.clone());
        }

        if let Some(until_newline) = node.leaf_text().chars().position(|ch| ch == '\n') {
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

                context
                    .index_mapper
                    .push_aux_to_main_unchecked(range.start, ir.len());
                ast_blocks.push(AstBlock {
                    range,
                    is_inline: false,
                });
            }
        }
    }

    if let Some(last_block) = ast_blocks.last_mut()
        && in_block
    {
        wrap_block(&mut ir, text, last_block, last_kind, context);
    }

    // crate::log!("[RANGES]: {block_ranges:?}");

    // crate::log!(
    //     "[SOURCE]:\n{}",
    //     &ir[(state.prelude(id, RenderTarget::Svg) + prelude + "\n").len()..]
    // );

    SourceSyncResult {
        ir,
        ast_blocks,
        equation_ranges,
    }
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
            last_block.is_inline = true;
        }
        _ => {
            *ir += "#block(stroke:0pt,width:100%)[";
            context
                .index_mapper
                .push_aux_to_main_unchecked(last_block.range.start, ir.len());
            *ir += &text[last_block.range.clone()];
            context
                .index_mapper
                .push_aux_to_main_unchecked(last_block.range.end, ir.len());
            *ir += "\n]";

            last_block.is_inline = true;
        }
    }

    *ir += "\n";
    context
        .index_mapper
        .push_aux_to_main_unchecked(last_block.range.end, ir.len());
}
