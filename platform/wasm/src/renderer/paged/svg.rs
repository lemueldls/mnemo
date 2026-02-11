use std::{collections::VecDeque, hash::BuildHasher, ops::Range};

use comemo::Prehashed;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::layout::{Abs, Frame, FrameKind, Point, Size};
use typst_svg::svg_frame;

use super::FrameBlock;
use crate::{
    renderer::paged::{PagedRender, items::chunk_by_items},
    state::TypstState,
    wrappers::{TypstDiagnostic, TypstFileId},
};

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
    } = chunk_by_items(id, text, prelude, state);

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

    let mut frame = Frame::new(Size::new(width, height), FrameKind::Soft);
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

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SvgRender {
    pub frames: Vec<SvgRangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SvgRangedFrame {
    pub range: Range<usize>,
    pub render: SvgFrameRender,
}

impl SvgRangedFrame {
    pub fn new(range: Range<usize>, render: SvgFrameRender) -> Self {
        Self { range, render }
    }
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SvgFrameRender {
    pub svg: String,
    pub height: f64,
    #[serde(rename = "offsetHeight")]
    pub offset_height: f64,
    pub hash: u32,
}
