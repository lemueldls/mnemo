extern crate alloc;

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
