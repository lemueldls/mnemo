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
    pub blocks: VecDeque<FrameBlock>,
    pub range: Range<usize>,
    pub height: f64,
    pub offset_height: f64,
}

#[derive(Debug, Clone)]
pub struct FrameBlock {
    pub range: Option<Range<usize>>,
    pub start_height: Abs,
    pub end_height: Abs,
    pub item: FrameItem,
    pub point: Point,
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
