pub mod attr;
pub mod charsets;
pub mod tag;
pub mod writer;

use std::{
    cmp,
    fmt::Write,
    hash::{BuildHasher, Hash, Hasher},
    iter,
    ops::{Deref, Range},
};

use ecow::{EcoString, EcoVec, eco_format, eco_vec};
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashSet};
use serde::{Deserialize, Serialize};
use tiny_skia::IntRect;
use tsify::Tsify;
use typst::{
    __bail as bail, World, WorldExt, compile,
    diag::{At, Severity, SourceResult, StrResult},
    foundations::{Element, NativeRuleMap, Repr, Selector, Target},
    introspection::{Introspector, Location, Tag},
    layout::BlockElem,
    syntax::Span,
};
// use ecow::{EcoString, eco_format};
// use typst_library::diag::{At, SourceResult, StrResult, bail};
// use typst_library::foundations::Repr;
// use typst_library::introspection::Introspector;
// use typst_syntax::Span;
use typst_html::{HtmlDocument, HtmlElement, HtmlFrame, HtmlNode, HtmlSliceExt, HtmlTag};
use writer::{Writer, write_node};

use crate::{
    renderer::sync_source_context,
    state::{SourceContext, TypstRequest, TypstState},
    world::MnemoWorld,
    wrappers::{TypstDiagnostic, TypstFileId, map_main_span},
};

pub fn render(id: &TypstFileId, text: &str, prelude: &str, state: &mut TypstState) -> RenderResult {
    let (ir, ast_blocks) = sync_source_context(id, text, prelude, state);

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
                let HtmlNode::Element(body) = document
                    .root
                    .children
                    .iter()
                    .find(|node| matches!(node, HtmlNode::Element(el) if el.tag == tag::body))
                    .unwrap()
                    .clone()
                else {
                    unreachable!()
                };

                let mut blocks = Vec::with_capacity(ast_blocks.len());

                let mut ast_blocks = ast_blocks.iter().peekable();

                let mut children = body
                    .children
                    .into_iter()
                    .flat_map(flatten_node)
                    .filter_map(|node| {
                        let location = match &node {
                            HtmlNode::Tag(tag) => {
                                match tag {
                                    Tag::Start(content, ..) => content.location(),
                                    Tag::End(location, ..) => Some(location.clone()),
                                }
                            }
                            HtmlNode::Text(..) => None,
                            HtmlNode::Element(element) => element.parent,
                            HtmlNode::Frame(..) => None,
                        };

                        let position = location.and_then(|location| {
                            document.introspector.position(location).as_html()
                        });

                        crate::debug!("position: {position:?}");

                        let range = flat_node_range(&node, context, &state.world)?;

                        Some((node, range, position))
                    })
                    // .sorted_by_key(|(_, range)| range.start)
                    .peekable();

                while let Some(ast_block) = ast_blocks.next() {
                    let mut w = Writer::new(&document.introspector, false);

                    let mut hasher = FxBuildHasher::default().build_hasher();

                    let aux_source = context.aux_source(&state.world).unwrap();

                    let aux_range = &ast_block.range;
                    let aux_lines = aux_source.lines();
                    let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start).unwrap();
                    let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end).unwrap();
                    let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                    let main_range_start = context.map_aux_to_main(aux_range.start);
                    let main_range_end = context.map_aux_to_main(aux_range.end);
                    let main_range = main_range_start..main_range_end;

                    while let Some((node, range, position)) = children.peek() {
                        crate::debug!("comparing ast {main_range:?} with node {range:?}");
                        crate::debug!("node {node:?}");

                        if range.end <= main_range_end {
                            let (node, ..) = children.next().unwrap();

                            write_node(&mut w, &node, body.pre_span).unwrap();
                            node.hash(&mut hasher);
                        } else {
                            break;
                        }
                    }

                    if !w.buf.is_empty() {
                        blocks.push(RangedFrame {
                            render: FrameRender {
                                html: w.buf,
                                hash: hasher.finish() as u32,
                            },
                            range: aux_range_utf16.clone(),
                        })
                    }
                }

                last_document = Some(document);

                blocks
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
                    end_byte += 12;
                }

                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &context,
                    &state.world,
                ));

                crate::error!("[ERRORS]: {diagnostics:?}");

                let start_byte = context.map_aux_to_main(aux_range.start);

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

    RenderResult {
        frames,
        diagnostics,
    }
}

fn flatten_node(node: HtmlNode) -> Box<[HtmlNode]> {
    match node {
        HtmlNode::Element(element) => {
            match element.tag {
                tag::p => Box::from_iter(element.children),
                tag::ul => {
                    let children = element.children;
                    Box::from_iter(children.into_iter().map(|node| {
                        HtmlNode::Element(HtmlElement {
                            tag: tag::ul,
                            attrs: element.attrs.clone(),
                            children: eco_vec![node],
                            parent: element.parent,
                            pre_span: element.pre_span,
                            span: Span::detached(),
                        })
                    }))
                }
                tag::ol => {
                    let mut start = 1;

                    let children = element.children;
                    Box::from_iter(children.into_iter().map(|mut node| {
                        let HtmlNode::Element(HtmlElement { attrs, .. }) = &mut node else {
                            unreachable!()
                        };

                        if let Some(value) = attrs.get(attr::value) {
                            start = value.parse::<u16>().unwrap() + 1;
                        } else {
                            attrs.push(attr::value, start.to_string());
                            start += 1;
                        }

                        HtmlNode::Element(HtmlElement {
                            tag: tag::ol,
                            attrs: element.attrs.clone(),
                            children: eco_vec![node],
                            parent: element.parent,
                            pre_span: element.pre_span,
                            span: Span::detached(),
                        })
                    }))
                }
                tag::dl => {
                    let children = element.children;
                    Box::from_iter(children.into_iter().map(|node| {
                        HtmlNode::Element(HtmlElement {
                            tag: tag::dl,
                            attrs: element.attrs.clone(),
                            children: eco_vec![node],
                            parent: element.parent,
                            pre_span: element.pre_span,
                            span: Span::detached(),
                        })
                    }))
                }
                _ => Box::from_iter(iter::once(HtmlNode::Element(element))),
            }
        }
        _ => Box::from_iter(iter::once(node)),
    }
}

fn flat_node_range(
    node: &HtmlNode,
    context: &SourceContext,
    world: &MnemoWorld,
) -> Option<Range<usize>> {
    match node {
        HtmlNode::Tag(_) => None,
        HtmlNode::Text(_, span) => map_main_span(*span, false, &[], context, world),
        HtmlNode::Element(element) => {
            let range = map_main_span(element.span, false, &[], context, world);

            element
                .children
                .iter()
                .map(|node| flat_node_range(node, context, world))
                .fold(range, |a, b| {
                    match (a, b) {
                        (Some(a), Some(b)) => {
                            let start = cmp::min(a.start, b.start);
                            let end = cmp::max(a.end, b.end);

                            Some(start..end)
                        }
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(b),
                        (None, None) => None,
                    }
                })
        }
        HtmlNode::Frame(frame) => map_main_span(frame.span, false, &[], context, world),
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderResult {
    pub frames: Vec<RangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RangedFrame {
    pub range: Range<usize>,
    pub render: FrameRender,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrameRender {
    html: String,
    hash: u32,
}

fn with_dom_indices(nodes: EcoVec<HtmlNode>) -> impl Iterator<Item = (HtmlNode, usize)> {
    let mut cursor = 0;
    let mut was_text = false;

    nodes.into_iter().map(move |child| {
        let mut i = cursor;

        match child {
            HtmlNode::Tag(_) => {}
            HtmlNode::Text(..) => was_text = true,
            _ => {
                cursor += usize::from(was_text);
                i = cursor;
                cursor += 1;
                was_text = false;
            }
        }

        (child, i)
    })
}
