//! The Typst rendering and IDE engine for Mnemo, compiled to WebAssembly.
//!
//! ## Architecture
//!
//! The central type is [`state::TypstState`], which owns a
//! [`world::MnemoWorld`] and a collection of per-note
//! [`source::context::SourceContext`] values. JavaScript creates a single
//! `TypstState`, registers fonts into it, then
//! calls [`TypstState::compile_paged`] or [`TypstState::compile_html`] on
//! individual notes as the user edits them.
//!
//! ## The Two-File Model
//!
//! Each note is backed by two Typst source files internally. See the
//! [`source`] module documentation for a full explanation. In brief:
//!
//! - The `raw` source is exactly what the user typed. All positions returned to
//!   the editor (diagnostics, cursor jumps, hover ranges) are expressed in raw
//!   coordinates.
//!
//! - The `synth` source is a synthesized file built from the raw source plus a
//!   generated prelude. This is the file Typst actually compiles. Positions in
//!   compiler output are converted back to raw coordinates before being
//!   returned.
//!
//! ## Error Recovery
//!
//! Compile failures are handled by [`renderer::recovery`]. When the synth
//! fails to compile, the recovery pass identifies the offending block,
//! overwrites it with length-preserving whitespace, and retries. Math errors
//! get finer treatment: the broken sub-expression is wrapped in a red-text
//! marker rather than blanked, so the rest of the equation keeps rendering.

pub mod bindings;
pub mod fonts;
pub mod renderer;
pub mod source;
pub mod state;
pub mod theme;
pub mod world;

mod utils;

use wasm_bindgen::prelude::*;

// #[cfg(debug_assertions)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn group(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = groupEnd)]
    fn group_end(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn time(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = timeEnd)]
    fn time_end(s: &str);
}

// #[cfg(debug_assertions)]
#[wasm_bindgen(start)]
fn start() {
    utils::set_panic_hook();
}

#[macro_export]
macro_rules! log {
    ($($e:tt)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::log(&format!($($e)*));
        #[cfg(not(target_arch="wasm32"))]
        eprintln!($($e)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($e:tt)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::debug(&format!($($e)*));
        #[cfg(not(target_arch="wasm32"))]
        eprintln!($($e)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($e:tt)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::error(&format!($($e)*));
        #[cfg(not(target_arch="wasm32"))]
        eprintln!($($e)*);
    };
}

#[macro_export]
macro_rules! group {
    ($($e:tt)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::group(&format!($($e)*));
        #[cfg(not(target_arch="wasm32"))]
        eprintln!($($e)*);
    };
}

#[macro_export]
macro_rules! group_end {
    ($($e:tt)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::group_end(&format!($($e)*));
        #[cfg(not(target_arch="wasm32"))]
        eprintln!($($e)*);
    };
}
