//! Source management: the raw/synth two-file model.
//!
//! Every note in Mnemo is backed by two Typst source files:
//!
//! - **Raw** (`raw_id`): Exactly what the user typed. Nothing is added or
//!   removed. All editor-facing positions (diagnostics, cursor locations, hover
//!   ranges) are expressed in raw byte offsets.
//!
//! - **Synth** (`synth_id`): A synthesized intermediate file produced by
//!   [`synth::sync_source_context`] on every recompile. It contains:
//!   1. A generated prelude (page geometry, color theme, text defaults)
//!   2. The raw source's content, with block-level wrappers added around each
//!      run of content-producing top-level nodes
//!
//! The [`IndexMapper`] tracks where each synth byte corresponds to in the
//! raw source, so compiler output can be translated back to coordinates the
//! editor understands.
//!
//! ## Invariant
//!
//! The synth is always rebuilt from scratch before each compile. It is never
//! edited directly. All structural edits (error recovery whitespace, math error
//! markers) are applied to the synth's `Source` object in place during the
//! compile loop, then reverted or rebuilt before the next call.

mod context;
mod index;
mod synth;

pub use context::{SourceContext, SpaceContext};
pub use index::IndexMapper;
pub use synth::{AstBlock, RenderTarget, SynthResult, sync_source_context, sync_source_state};
