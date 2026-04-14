// pub mod blocks;
pub mod items;
pub mod svg;

use std::{collections::VecDeque, hash::Hash, ops::Range};

use typst::layout::{FrameItem, PagedDocument, Point, Rect};

use crate::{state::SourceContext, wrappers::TypstDiagnostic};

/// Result of paged rendering, containing chunks, diagnostics, and document context.
#[derive(Debug)]
pub struct PagedRender {
    /// Chunks of blocks for incremental rendering.
    pub chunks: Vec<FrameItemsChunk>,
    /// Tooltips for the rendered content.
    pub tooltips: Vec<FrameItemsChunk>,
    /// Diagnostics and warnings produced during rendering.
    pub diagnostics: Vec<TypstDiagnostic>,
    /// The paged Typst document, if available.
    pub document: Option<PagedDocument>,
}

/// A chunk of frame items, representing a logical segment of the document.
#[derive(Debug)]
pub struct FrameItemsChunk {
    /// The bound frame items in this chunk.
    pub items: VecDeque<BoundFrameItem>,
    /// UTF-16 range in the source corresponding to this chunk.
    pub range: Range<usize>,
    /// Width of the chunk in points.
    pub width: f64,
    /// Height of the chunk in points.
    pub height: f64,
    /// Offset from the left of the page in points.
    pub x_offset: f64,
    /// Offset from the top of the page in points.
    pub y_offset: f64,
}

/// A single frame item with bounds and range.
#[derive(Debug, Clone)]
pub struct BoundFrameItem {
    /// Optional byte range in the source for this block.
    pub range: Option<Range<usize>>,
    /// Bounding box of the block.
    pub bounds: Rect,
    /// The frame item to render.
    pub item: FrameItem,
    /// The position of the block on the page.
    pub point: Point,
}

impl Hash for BoundFrameItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // self.range.hash(state);
        // self.start_height.hash(state);
        // self.end_height.hash(state);
        self.item.hash(state);
        self.point.hash(state);
    }
}
