// pub mod blocks;
pub mod items;
pub mod svg;

use std::{collections::VecDeque, hash::Hash, ops::Range};

use typst::layout::{Abs, FrameItem, PagedDocument, Point};

use crate::{state::SourceContext, wrappers::TypstDiagnostic};

/// Result of paged rendering, containing chunks, diagnostics, and document context.
#[derive(Debug)]
pub struct PagedRender<'a> {
    /// Chunks of blocks for incremental rendering.
    pub chunks: Vec<BlocksChunk>,
    /// Diagnostics and warnings produced during rendering.
    pub diagnostics: Vec<TypstDiagnostic>,
    /// The paged Typst document, if available.
    pub document: Option<PagedDocument>,
    /// Mutable reference to the source context.
    pub context: &'a mut SourceContext,
}

/// A chunk of frame blocks, representing a logical segment of the document.
#[derive(Debug)]
pub struct BlocksChunk {
    /// The frame blocks in this chunk.
    pub blocks: VecDeque<FrameBlock>,
    /// UTF-16 range in the source corresponding to this chunk.
    pub range: Range<usize>,
    /// Height of the chunk in points.
    pub height: f64,
    /// Offset from the top of the page in points.
    pub offset_height: f64,
}

/// A single frame block, representing a renderable item with position and range.
#[derive(Debug, Clone)]
pub struct FrameBlock {
    /// Optional byte range in the source for this block.
    pub range: Option<Range<usize>>,
    /// Start height of the block.
    pub start_height: Abs,
    /// End height of the block.
    pub end_height: Abs,
    /// The frame item to render.
    pub item: FrameItem,
    /// The position of the block on the page.
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
