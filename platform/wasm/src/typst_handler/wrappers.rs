use std::ops::Range;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{
    World, WorldExt,
    diag::{Severity, SourceDiagnostic, Tracepoint},
    ecow::{EcoVec, eco_format},
    syntax::{Span, Spanned, SyntaxError},
};
use wasm_bindgen::prelude::*;

use crate::typst_handler::world::MnemoWorld;

#[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TypstDiagnostic {
    pub range: Range<usize>,
    pub severity: TypstDiagnosticSeverity,
    pub message: String,
    pub hints: Box<[String]>,
}

impl TypstDiagnostic {
    pub fn from_errors(errors: EcoVec<SyntaxError>, world: &MnemoWorld) -> Box<[Self]> {
        errors
            .into_iter()
            .flat_map(|error| {
                map_span(error.span, true, EcoVec::new(), world).map(|range| {
                    TypstDiagnostic {
                        range,
                        severity: TypstDiagnosticSeverity::Error,
                        message: error.message.to_string(),
                        hints: error.hints.into_iter().map(|s| s.to_string()).collect(),
                    }
                })
            })
            .collect()
    }

    pub fn from_diagnostics(
        diagnostics: EcoVec<SourceDiagnostic>,
        world: &MnemoWorld,
    ) -> Box<[Self]> {
        diagnostics
            .into_iter()
            .flat_map(|mut diagnostic| {
                if diagnostic.message == "failed to load file" {
                    let source = world.source(diagnostic.span.id().unwrap()).unwrap();
                    let text = source.get(world.range(diagnostic.span).unwrap()).unwrap();

                    diagnostic.message = eco_format!("failed to load file: {text}");
                }

                map_span(diagnostic.span, diagnostic.severity == Severity::Error, diagnostic.trace, world).map(|range| {
                    TypstDiagnostic {
                        range,
                        severity: TypstDiagnosticSeverity::from_severity(diagnostic.severity),
                        message: diagnostic.message.to_string(),
                        hints: diagnostic
                            .hints
                            .into_iter()
                            .map(|s| s.to_string())
                            .collect(),
                    }
                })
            })
            .collect()
    }
}

fn map_span(
    span: Span,
    is_error: bool,
    trace: EcoVec<Spanned<Tracepoint>>,
    world: &MnemoWorld,
) -> Option<Range<usize>> {
    let aux_source = world.aux_source();

    let mut main_range = if world.main == span.id() {
        world.range(span)
    } else {
        None
    };

    if main_range.is_none() {
        if !is_error {
            return None;
        }

        for tracepoint in trace {
            if main_range.is_some() {
                break;
            } else if world.main == tracepoint.span.id() {
                main_range = world.range(tracepoint.span)
            }
        }
    }

    let aux_range = if let Some(main_range) = main_range {
        let aux_start = world.map_main_to_aux(main_range.start);
        let aux_end = world.map_main_to_aux(main_range.end);

        aux_start..aux_end
    } else {
        0..aux_source.text().len()
    };

    let aux_start_utf16 = aux_source.byte_to_utf16(aux_range.start)?;
    let aux_end_utf16 = aux_source.byte_to_utf16(aux_range.end)?;
    let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

    Some(aux_range_utf16)
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "kebab-case")]
pub enum TypstDiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

impl TypstDiagnosticSeverity {
    pub fn from_severity(severity: Severity) -> Self {
        match severity {
            Severity::Error => Self::Error,
            Severity::Warning => Self::Warning,
        }
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

impl TypstJump {
    pub fn from_mapped(jump: typst_ide::Jump, world: &MnemoWorld) -> Self {
        match jump {
            typst_ide::Jump::File(_id, main_position) => {
                let aux_source = world.aux_source();
                let aux_position = world.map_main_to_aux(main_position);
                let aux_position_utf16 = aux_source.byte_to_utf16(aux_position).unwrap();

                Self::File {
                    // id: state.finish(),
                    position: aux_position_utf16,
                }
            }
            typst_ide::Jump::Url(..) => todo!(),
            typst_ide::Jump::Position(..) => todo!(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "kebab-case")]
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
    #[serde(rename = "type")]
    kind: TypstCompletionKind,
    label: String,
    apply: Option<String>,
    detail: Option<String>,
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
