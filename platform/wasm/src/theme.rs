use std::fmt;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ThemeColors {
    background: Rgb,
    on_background: Rgb,

    outline: Rgb,
    outline_variant: Rgb,

    primary: Rgb,
    on_primary: Rgb,
    primary_container: Rgb,
    on_primary_container: Rgb,

    secondary: Rgb,
    on_secondary: Rgb,
    secondary_container: Rgb,
    on_secondary_container: Rgb,

    tertiary: Rgb,
    on_tertiary: Rgb,
    tertiary_container: Rgb,
    on_tertiary_container: Rgb,

    error: Rgb,
    on_error: Rgb,
    error_container: Rgb,
    on_error_container: Rgb,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            background: Rgb::WHITE,
            on_background: Rgb::BLACK,

            outline: Rgb::BLACK,
            outline_variant: Rgb::BLACK,

            primary: Rgb::BLACK,
            on_primary: Rgb::WHITE,
            primary_container: Rgb::BLACK,
            on_primary_container: Rgb::WHITE,

            secondary: Rgb::BLACK,
            on_secondary: Rgb::WHITE,
            secondary_container: Rgb::BLACK,
            on_secondary_container: Rgb::WHITE,

            tertiary: Rgb::BLACK,
            on_tertiary: Rgb::WHITE,
            tertiary_container: Rgb::BLACK,
            on_tertiary_container: Rgb::WHITE,

            error: Rgb::BLACK,
            on_error: Rgb::WHITE,
            error_container: Rgb::BLACK,
            on_error_container: Rgb::WHITE,
        }
    }
}

#[wasm_bindgen]
impl ThemeColors {
    #[must_use]
    #[allow(clippy::missing_const_for_fn, clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        background: Rgb,
        on_background: Rgb,

        outline: Rgb,
        outline_variant: Rgb,

        primary: Rgb,
        on_primary: Rgb,
        primary_container: Rgb,
        on_primary_container: Rgb,

        secondary: Rgb,
        on_secondary: Rgb,
        secondary_container: Rgb,
        on_secondary_container: Rgb,

        tertiary: Rgb,
        on_tertiary: Rgb,
        tertiary_container: Rgb,
        on_tertiary_container: Rgb,

        error: Rgb,
        on_error: Rgb,
        error_container: Rgb,
        on_error_container: Rgb,
    ) -> Self {
        Self {
            background,
            on_background,

            outline,
            outline_variant,

            primary,
            on_primary,
            primary_container,
            on_primary_container,

            secondary,
            on_secondary,
            secondary_container,
            on_secondary_container,

            tertiary,
            on_tertiary,
            tertiary_container,
            on_tertiary_container,

            error,
            on_error,
            error_container,
            on_error_container,
        }
    }
}

impl fmt::Display for ThemeColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(background:{},on-background:{},outline:{},outline-variant:{},primary:{},on-primary:{},primary-container:{},on-primary-container:{},secondary:{},on-secondary:{},secondary-container:{},on-secondary-container:{},tertiary:{},on-tertiary:{},tertiary-container:{},on-tertiary-container:{},error:{},on-error:{},error-container:{},on-error-container:{})",
            self.background,
            self.on_background,
            self.outline,
            self.outline_variant,
            self.primary,
            self.on_primary,
            self.primary_container,
            self.on_primary_container,
            self.secondary,
            self.on_secondary,
            self.secondary_container,
            self.on_secondary_container,
            self.tertiary,
            self.on_tertiary,
            self.tertiary_container,
            self.on_tertiary_container,
            self.error,
            self.on_error,
            self.error_container,
            self.on_error_container,
        )
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Default, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    pub const BLACK: Self = Self(0, 0, 0);
    pub const WHITE: Self = Self(255, 255, 255);
}

#[wasm_bindgen]
impl Rgb {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    #[must_use]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_js_string(&self) -> String {
        format!("rgb({},{},{})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.0, self.1, self.2)
    }
}
