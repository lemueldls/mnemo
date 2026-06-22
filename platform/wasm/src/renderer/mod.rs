//! Document renderer for Typst in Mnemo.
//!
//! This module provides the core logic for rendering Typst documents, including chunking, diagnostics, and output for multiple targets (SVG, HTML, PDF).
//!
//! # Main and Aux Sources
//!
//! The renderer operates on two parallel representations of the document:
//!
//! - **Aux source**: The origin text a user writes in an editor. This is the user's direct input and is used for mapping diagnostics, highlights, and incremental updates.
//! - **Main source**: The intermediate file used for compilation. This is a transformed version of the aux source, prepared for Typst's incremental compilation and error reporting. The main source is the authoritative version used for diagnostics and output.
//!
//! The mapping between aux and main sources is maintained throughout the rendering process (see [`IndexMapper`]), allowing for robust error localization and efficient incremental updates. Most rendering functions synchronize both sources before producing output or diagnostics.
//!
//! Unless extending the renderer or integrating with Typst's incremental compilation, you will rarely need to interact with both sources directly. Most APIs abstract over this duality.

pub mod html;
pub mod paged;
pub mod recovery;
