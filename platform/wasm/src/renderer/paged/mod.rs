// pub mod blocks;
pub mod items;
pub mod svg;

use std::{collections::VecDeque, hash::Hash, ops::Range};

use typst::layout::{Abs, FrameItem, PagedDocument, Point};

use crate::{state::SourceContext, wrappers::TypstDiagnostic};

#[derive(Debug)]
pub struct PagedRender<'a> {
    pub chunks: Vec<BlocksChunk>,
    pub diagnostics: Vec<TypstDiagnostic>,
    pub document: Option<PagedDocument>,
    pub context: &'a mut SourceContext,
}

#[derive(Debug)]
pub struct BlocksChunk {
    blocks: VecDeque<FrameBlock>,
    range: Range<usize>,
    height: f64,
    offset_height: f64,
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
        self.point.hash(state);
    }
}
