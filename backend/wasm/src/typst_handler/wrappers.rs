use std::ops::Range;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Error;
use tsify::Tsify;
use typst::{
    World, WorldExt, compile,
    diag::{Severity, SourceDiagnostic},
    ecow::{EcoString, EcoVec},
    layout::{Abs, Frame, FrameItem, Page, PagedDocument, Point, Position},
    syntax::{
        FileId, Source, Span, SyntaxError, SyntaxKind, VirtualPath, ast, package::PackageSpec,
    },
    visualize::Color,
};
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum TypstDiagnosticSeverity {
    Error,
    Warning,
}

impl TypstDiagnosticSeverity {
    pub fn from_severity(severity: Severity) -> Self {
        match severity {
            Severity::Error => Self::Error,
            Severity::Warning => Self::Warning,
        }
    }
}

// #[wasm_bindgen]
#[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TypstDiagnostic {
    pub range: Range<usize>,
    pub severity: TypstDiagnosticSeverity,
    pub message: String,
    pub hints: Box<[String]>,
}

impl TypstDiagnostic {
    pub fn drain_from_errors(errors: &mut Vec<SyntaxError>, world: &dyn WorldExt) -> Box<[Self]> {
        errors
            .drain(..)
            .map(|error| {
                TypstDiagnostic {
                    range: world.range(error.span).unwrap(),
                    severity: TypstDiagnosticSeverity::Error,
                    message: error.message.to_string(),
                    hints: error.hints.into_iter().map(|s| s.to_string()).collect(),
                }
            })
            .collect()
    }

    pub fn from_diagnostics(
        diagnostics: &mut Vec<SourceDiagnostic>,
        world: &dyn WorldExt,
    ) -> Box<[Self]> {
        diagnostics
            .drain(..)
            .map(|diagnostic| {
                TypstDiagnostic {
                    range: world.range(diagnostic.span).unwrap(),
                    severity: TypstDiagnosticSeverity::from_severity(diagnostic.severity),
                    message: diagnostic.message.to_string(),
                    hints: diagnostic
                        .hints
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect(),
                }
            })
            .collect()
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TypstError {
    pub range: Range<usize>,
    pub message: String,
    pub hints: Box<[String]>,
}

impl TypstError {
    pub fn drain_from_errors(errors: &mut Vec<SyntaxError>, world: &dyn WorldExt) -> Box<[Self]> {
        errors
            .drain(..)
            .map(|error| {
                TypstError {
                    range: world.range(error.span).unwrap(),
                    message: error.message.to_string(),
                    hints: error.hints.into_iter().map(|s| s.to_string()).collect(),
                }
            })
            .collect()
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum TypstJump {
    File {
        // id: u64,
        position: usize,
    },
    // Url(String),
    // Position(Position),
}

impl From<typst_ide::Jump> for TypstJump {
    fn from(jump: typst_ide::Jump) -> Self {
        match jump {
            typst_ide::Jump::File(id, position) => {
                // let mut state = DefaultHasher::new();
                // id.hash(&mut state);

                Self::File {
                    // id: state.finish(),
                    position,
                }
            }
            typst_ide::Jump::Url(..) => todo!(),
            typst_ide::Jump::Position(..) => todo!(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum TypstCompletionKind {
    Syntax,
    Func,
    Type,
    Param,
    Constant,
    Path,
    Package,
    Label,
    Font,
    Symbol,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct TypstCompletion {
    kind: TypstCompletionKind,
    label: String,
    apply: Option<String>,
    detail: Option<String>,
}

pub struct TypstCompleteResponse {
    offset: usize,
    completions: Vec<TypstCompletion>,
}

impl From<typst_ide::Completion> for TypstCompletion {
    fn from(value: typst_ide::Completion) -> Self {
        Self {
            kind: match value.kind {
                typst_ide::CompletionKind::Syntax => TypstCompletionKind::Syntax,
                typst_ide::CompletionKind::Func => TypstCompletionKind::Func,
                typst_ide::CompletionKind::Type => TypstCompletionKind::Type,
                typst_ide::CompletionKind::Param => TypstCompletionKind::Param,
                typst_ide::CompletionKind::Constant => TypstCompletionKind::Constant,
                typst_ide::CompletionKind::Path => TypstCompletionKind::Path,
                typst_ide::CompletionKind::Package => TypstCompletionKind::Package,
                typst_ide::CompletionKind::Label => TypstCompletionKind::Label,
                typst_ide::CompletionKind::Font => TypstCompletionKind::Font,
                typst_ide::CompletionKind::Symbol(_) => TypstCompletionKind::Symbol,
            },
            label: value.label.to_string(),
            apply: value.apply.map(|s| s.to_string()),
            detail: value.detail.map(|s| s.to_string()),
        }
    }
}
