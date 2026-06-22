use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::{
    bindings::TypstDiagnostic,
    renderer::{html::HTMLRangedFrame, paged::svg::SvgRangedFrame},
    state::TypstRequest,
};

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompilePagedResult {
    pub frames: Vec<SvgRangedFrame>,
    pub tooltips: Vec<SvgRangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
    pub requests: Vec<TypstRequest>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompileHTMLResult {
    pub frames: Vec<HTMLRangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
    pub requests: Vec<TypstRequest>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CheckResult {
    pub diagnostics: Vec<TypstDiagnostic>,
    pub requests: Vec<TypstRequest>,
}
