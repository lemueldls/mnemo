extern crate alloc;

pub mod fonts;
pub mod index_mapper;
pub mod renderer;
pub mod state;
pub mod world;
pub mod wrappers;

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
        $crate::log(&format!($($e)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($e:tt)*) => {
        $crate::debug(&format!($($e)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($e:tt)*) => {
        $crate::error(&format!($($e)*))
    };
}

#[macro_export]
macro_rules! group {
    ($($e:tt)*) => {
        $crate::group(&format!($($e)*))
    };
}

#[macro_export]
macro_rules! group_end {
    ($($e:tt)*) => {
        $crate::group_end(&format!($($e)*))
    };
}
