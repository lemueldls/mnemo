// pub mod blocks;
pub mod items;
pub mod svg;

use std::{hash::Hash, ops::Range};

use typst::layout::{Abs, FrameItem, Point};

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
