// pub mod chunk;
pub mod items;

use std::{hash::Hash, ops::Range};

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::layout::{Abs, Frame, FrameItem, Point};
use wasm_bindgen::prelude::*;

use crate::wrappers::TypstDiagnostic;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PagedRenderResult {
    pub frames: Vec<PagedRangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Debug, Clone)]
pub struct FrameBlock {
    range: Option<Range<usize>>,
    start_height: Abs,
    end_height: Abs,
    item: FrameItem,
    point: Point,
}

impl Hash for FrameBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // self.range.hash(state);
        // self.start_height.hash(state);
        // self.end_height.hash(state);
        self.item.hash(state);
        // self.point.hash(state);
    }
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PagedRangedFrame {
    pub range: Range<usize>,
    pub render: PagedFrameRender,
}

impl PagedRangedFrame {
    pub fn new(range: Range<usize>, render: PagedFrameRender) -> Self {
        Self { range, render }
    }
}

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PagedFrameRender {
    svg: String,
    hash: u32,
    height: u32,
    #[serde(rename = "offsetHeight")]
    offset_height: f64,
}
