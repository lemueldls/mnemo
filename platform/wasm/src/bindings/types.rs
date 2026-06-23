use std::ops::Range;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{
    World, WorldExt,
    diag::{Severity, SourceDiagnostic, Tracepoint},
    ecow::{EcoVec, eco_format},
    syntax::{DiagSpan, FileId, Spanned},
};
use wasm_bindgen::prelude::*;

use crate::{source::SourceContext, world::MnemoWorld};

#[wasm_bindgen(js_name = "FileId")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TypstFileId(pub(crate) FileId);

impl TypstFileId {
    #[must_use]
    pub const fn new(id: FileId) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn inner(&self) -> FileId {
        self.0
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TypstDiagnostic {
    pub range: Range<usize>,
    pub severity: TypstDiagnosticSeverity,
    pub message: String,
    pub hints: Box<[String]>,
}

impl TypstDiagnostic {
    pub fn from_diagnostics(
        diagnostics: EcoVec<SourceDiagnostic>,
        context: &SourceContext,
        world: &MnemoWorld,
    ) -> Box<[Self]> {
        diagnostics
            .into_iter()
            .filter_map(|mut diagnostic| {
                if diagnostic.message == "failed to load file" {
                    let source = world.source(diagnostic.span.id().unwrap()).unwrap();
                    let text = source
                        .text()
                        .get(world.range(diagnostic.span).unwrap())
                        .unwrap();

                    diagnostic.message = eco_format!("failed to load file: {text}");
                }

                map_raw_span(
                    diagnostic.span,
                    diagnostic.severity == Severity::Error,
                    &diagnostic.trace,
                    context,
                    world,
                )
                .map(|range| {
                    TypstDiagnostic {
                        range,
                        severity: TypstDiagnosticSeverity::from_severity(diagnostic.severity),
                        message: diagnostic.message.to_string(),
                        hints: diagnostic
                            .hints
                            .into_iter()
                            .map(|s| s.v.to_string())
                            .collect(),
                    }
                })
            })
            .collect()
    }
}

pub fn map_synth_span(
    span: impl Into<DiagSpan>,
    is_error: bool,
    trace: &[Spanned<Tracepoint>],
    context: &SourceContext,
    world: &MnemoWorld,
) -> Option<Range<usize>> {
    let span = span.into();

    let mut synth_range = if Some(context.synth_id) == span.id() {
        world.range(span)
    } else {
        None
    };

    if synth_range.is_none() {
        if !is_error {
            return None;
        }

        for tracepoint in trace {
            if synth_range.is_some() {
                break;
            } else if Some(context.synth_id) == tracepoint.span.id() {
                synth_range = world.range(tracepoint.span);
            }
        }
    }

    synth_range
}

pub fn map_raw_span(
    span: impl Into<DiagSpan>,
    is_error: bool,
    trace: &[Spanned<Tracepoint>],
    context: &SourceContext,
    world: &MnemoWorld,
) -> Option<Range<usize>> {
    let raw_source = context.raw_source(world)?;

    let synth_range = map_synth_span(span, is_error, trace, context, world);

    let raw_range = if let Some(synth_range) = synth_range {
        let raw_start = context.map_synth_to_raw_from_right(synth_range.start);
        let raw_end = context.map_synth_to_raw_from_left(synth_range.end);

        raw_start..raw_end
    } else {
        if !is_error {
            return None;
        }

        0..raw_source.text().len()
    };

    let raw_lines = raw_source.lines();
    let raw_start_utf16 = raw_lines.byte_to_utf16(raw_range.start)?;
    let raw_end_utf16 = raw_lines.byte_to_utf16(raw_range.end)?;
    let raw_range_utf16 = raw_start_utf16..raw_end_utf16;

    Some(raw_range_utf16)
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
    #[must_use]
    pub const fn from_severity(severity: Severity) -> Self {
        match severity {
            Severity::Error => Self::Error,
            Severity::Warning => Self::Warning,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TypstHighlight {
    pub tag: String,
    pub range: Range<usize>,
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
    pub fn from_mapped(
        jump: typst_ide::Jump,
        context: &SourceContext,
        world: &MnemoWorld,
    ) -> Option<Self> {
        match jump {
            typst_ide::Jump::File(id, synth_position) => {
                if id != context.synth_id {
                    return None;
                }

                let raw_source = context.raw_source(world)?;
                let raw_position = context.map_synth_to_raw_from_right(synth_position);
                let raw_position_utf16 = raw_source.lines().byte_to_utf16(raw_position)?;

                Some(Self::File {
                    // id: state.finish(),
                    position: raw_position_utf16,
                })
            }
            typst_ide::Jump::Url(..) => None,
            typst_ide::Jump::Position(..) => None,
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
