use std::ops::Range;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{
    World, WorldExt,
    diag::{Severity, SourceDiagnostic, Tracepoint},
    ecow::{EcoVec, eco_format},
    syntax::{FileId, Span, Spanned, SyntaxError},
};
use typst_ide::Tooltip;
use wasm_bindgen::prelude::*;

use crate::typst_handler::{state::FileContext, world::MnemoWorld};

#[wasm_bindgen(js_name = "FileId")]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypstFileId(pub(crate) FileId);

impl TypstFileId {
    pub fn new(id: FileId) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> FileId {
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
    pub fn from_errors(
        errors: EcoVec<SyntaxError>,
        context: &FileContext,
        world: &MnemoWorld,
    ) -> Box<[Self]> {
        errors
            .into_iter()
            .flat_map(|error| {
                map_aux_span(error.span, true, &[], context, world).map(|range| {
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
        context: &FileContext,
        world: &MnemoWorld,
    ) -> Box<[Self]> {
        diagnostics
            .into_iter()
            .flat_map(|mut diagnostic| {
                if diagnostic.message == "failed to load file" {
                    let source = world.source(diagnostic.span.id().unwrap()).unwrap();
                    let text = source
                        .text()
                        .get(world.range(diagnostic.span).unwrap())
                        .unwrap();

                    diagnostic.message = eco_format!("failed to load file: {text}");
                }

                map_aux_span(
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
                            .map(|s| s.to_string())
                            .collect(),
                    }
                })
            })
            .collect()
    }
}

pub fn map_main_span(
    span: Span,
    is_error: bool,
    trace: &[Spanned<Tracepoint>],
    context: &FileContext,
    world: &MnemoWorld,
) -> Option<Range<usize>> {
    let mut main_range = if Some(context.main_id) == span.id() {
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
            } else if Some(context.main_id) == tracepoint.span.id() {
                main_range = world.range(tracepoint.span)
            }
        }
    }

    main_range
}

pub fn map_aux_span(
    span: Span,
    is_error: bool,
    trace: &[Spanned<Tracepoint>],
    context: &FileContext,
    world: &MnemoWorld,
) -> Option<Range<usize>> {
    let aux_source = context.aux_source(&world);

    let main_range = map_main_span(span, is_error, trace, context, world);

    let aux_range = if let Some(main_range) = main_range {
        let aux_start = context.map_main_to_aux(main_range.start);
        let aux_end = context.map_main_to_aux(main_range.end);

        aux_start..aux_end
    } else {
        if !is_error {
            return None;
        }

        0..aux_source.text().len()
    };

    let aux_lines = aux_source.lines();
    let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start)?;
    let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end)?;
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
pub struct TypstHighlight {
    pub tag: TypstTag,
    pub range: Range<usize>,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "kebab-case")]
pub enum TypstTag {
    Comment,
    Punctuation,
    Escape,
    Strong,
    Emph,
    Link,
    Raw,
    Label,
    Ref,
    Heading,
    ListMarker,
    ListTerm,
    MathDelimiter,
    MathOperator,
    Keyword,
    Operator,
    Number,
    String,
    Function,
    Interpolated,
    Error,
}

impl From<typst_syntax::Tag> for TypstTag {
    fn from(tag: typst_syntax::Tag) -> Self {
        match tag {
            typst_syntax::Tag::Comment => TypstTag::Comment,
            typst_syntax::Tag::Punctuation => TypstTag::Punctuation,
            typst_syntax::Tag::Escape => TypstTag::Escape,
            typst_syntax::Tag::Strong => TypstTag::Strong,
            typst_syntax::Tag::Emph => TypstTag::Emph,
            typst_syntax::Tag::Link => TypstTag::Link,
            typst_syntax::Tag::Raw => TypstTag::Raw,
            typst_syntax::Tag::Label => TypstTag::Label,
            typst_syntax::Tag::Ref => TypstTag::Ref,
            typst_syntax::Tag::Heading => TypstTag::Heading,
            typst_syntax::Tag::ListMarker => TypstTag::ListMarker,
            typst_syntax::Tag::ListTerm => TypstTag::ListTerm,
            typst_syntax::Tag::MathDelimiter => TypstTag::MathDelimiter,
            typst_syntax::Tag::MathOperator => TypstTag::MathOperator,
            typst_syntax::Tag::Keyword => TypstTag::Keyword,
            typst_syntax::Tag::Operator => TypstTag::Operator,
            typst_syntax::Tag::Number => TypstTag::Number,
            typst_syntax::Tag::String => TypstTag::String,
            typst_syntax::Tag::Function => TypstTag::Function,
            typst_syntax::Tag::Interpolated => TypstTag::Interpolated,
            typst_syntax::Tag::Error => TypstTag::Error,
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
    pub fn from_mapped(
        jump: typst_ide::Jump,
        context: &FileContext,
        world: &MnemoWorld,
    ) -> Option<Self> {
        match jump {
            typst_ide::Jump::File(id, main_position) => {
                if id != context.main_id {
                    return None;
                }

                let aux_source = context.aux_source(&world);
                let aux_position = context.map_main_to_aux(main_position);
                let aux_position_utf16 = aux_source.lines().byte_to_utf16(aux_position)?;

                Some(Self::File {
                    // id: state.finish(),
                    position: aux_position_utf16,
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

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum TypstTooltip {
    Text(String),
    Code(String),
}

impl From<Tooltip> for TypstTooltip {
    fn from(tooltip: Tooltip) -> Self {
        match tooltip {
            Tooltip::Text(str) => TypstTooltip::Text(str.to_string()),
            Tooltip::Code(str) => TypstTooltip::Code(str.to_string()),
        }
    }
}
