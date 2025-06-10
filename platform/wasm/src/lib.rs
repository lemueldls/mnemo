extern crate alloc;

mod typst_handler;
mod utils;

// #[cfg(feature = "lol_alloc")]
// #[cfg(target_arch = "wasm32")]
// use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// #[cfg(feature = "lol_alloc")]
// #[cfg(target_arch = "wasm32")]
// #[global_allocator]
// static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
//     unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

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
