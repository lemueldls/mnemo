use std::{collections::VecDeque, hash::BuildHasher, ops::Range};

use comemo::Prehashed;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::layout::{Abs, Frame, Point, Size};
use typst_svg::svg_frame;

use super::FrameBlock;
use crate::{
    renderer::{
        RenderTarget,
        paged::{PagedRender, items::chunk_by_items},
    },
    state::TypstState,
    wrappers::{TypstDiagnostic, TypstFileId},
};

/// Renders SVG frames for each chunked item in a Typst document.
#[typst_macros::time]
pub fn render_svgs_by_items(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> SvgRender {
    let PagedRender {
        chunks,
        diagnostics,
        document,
        context,
    } = chunk_by_items(id, text, prelude, RenderTarget::Svg, state);

    let frames = if let Some(document) = &document {
        let width = document
            .pages
            .iter()
            .map(|page| page.frame.width())
            .max()
            .unwrap_or_default();

        chunks
            .into_iter()
            .map(|chunk| {
                let blocks = Prehashed::new(chunk.blocks);

                let height = Abs::pt(chunk.height);
                let offset_height = Abs::pt(chunk.offset_height);

                render_svg(blocks, chunk.range, width, height, offset_height)
            })
            .collect()
    } else {
        Vec::new()
    };

    context.paged_document = document;

    SvgRender {
        frames,
        diagnostics,
    }
}

/// Renders a single SVG frame from a set of frame blocks and metadata.
#[comemo::memoize]
#[typst_macros::time]
fn render_svg(
    blocks: Prehashed<VecDeque<FrameBlock>>,
    range: Range<usize>,
    width: Abs,
    height: Abs,
    offset_height: Abs,
) -> SvgRangedFrame {
    let hash = FxBuildHasher.hash_one(&blocks) as u32;

    let mut frame = Frame::soft(Size::new(width, height));
    frame.push_multiple(blocks.into_inner().into_iter().map(|block| {
        let point = block.point - Point::new(Abs::zero(), offset_height);

        (point, block.item)
    }));

    let svg = svg_frame(&frame);

    let height = height.to_pt();
    let offset_height = offset_height.to_pt();

    let render = SvgFrameRender {
        svg,
        height,
        offset_height,
        hash,
    };

    SvgRangedFrame { range, render }
}

/// Result of SVG rendering, containing SVG frames and diagnostics.
#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SvgRender {
    /// Rendered SVG frames for each chunk.
    pub frames: Vec<SvgRangedFrame>,
    /// Diagnostics and warnings produced during rendering.
    pub diagnostics: Vec<TypstDiagnostic>,
}

/// An SVG frame with its corresponding source range.
#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SvgRangedFrame {
    /// UTF-16 range in the source for this frame.
    pub range: Range<usize>,
    /// The SVG render data.
    pub render: SvgFrameRender,
}

impl SvgRangedFrame {
    pub fn new(range: Range<usize>, render: SvgFrameRender) -> Self {
        Self { range, render }
    }
}

/// Rendered SVG data for a frame, including metadata.
#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SvgFrameRender {
    /// SVG markup as a string.
    pub svg: String,
    /// Height of the frame in points.
    pub height: f64,
    /// Offset from the top of the page in points.
    #[serde(rename = "offsetHeight")]
    pub offset_height: f64,
    /// Hash of the frame blocks for change detection.
    pub hash: u32,
}
