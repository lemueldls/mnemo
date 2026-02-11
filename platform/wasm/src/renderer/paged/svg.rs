use std::{collections::VecDeque, ops::Range};

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::layout::{Abs, Frame, FrameKind, Point, Size};
use typst_svg::svg_frame;

use super::FrameBlock;
use crate::{
    renderer::paged::items::{PagedRender, chunk_by_items},
    state::TypstState,
    wrappers::{TypstDiagnostic, TypstFileId},
};

pub fn render_svgs_by_items(
    id: &TypstFileId,
    text: &str,
    prelude: &str,
    state: &mut TypstState,
) -> SvgRender {
    let PagedRender {
        ranged_heights,
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

        ranged_heights
            .into_iter()
            .map(|(frame_blocks, range, height, offset_height, hash)| {
                let height = Abs::pt(height);
                let offset_height = Abs::pt(offset_height);

                render_svg(frame_blocks, range, width, height, offset_height, hash)
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
fn render_svg(
    frame_blocks: VecDeque<FrameBlock>,
    range: Range<usize>,
    width: Abs,
    height: Abs,
    offset_height: Abs,
    hash: u32,
) -> SvgRangedFrame {
    let mut frame = Frame::new(Size::new(width, height), FrameKind::Soft);
    frame.push_multiple(frame_blocks.into_iter().map(|block| {
        let point = block.point - Point::new(Abs::zero(), offset_height);

        (point, block.item)
    }));

    let svg = svg_frame(&frame);

    let height = height.to_pt();
    let offset_height = offset_height.to_pt();

    let render = SvgFrameRender {
        svg,
        hash,
        height,
        offset_height,
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
    pub hash: u32,
    pub height: f64,
    #[serde(rename = "offsetHeight")]
    pub offset_height: f64,
}
