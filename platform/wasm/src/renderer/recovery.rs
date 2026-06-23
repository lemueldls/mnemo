//! Error recovery for failed synth compiles.
//!
//! When a compile fails, the recovery pass identifies which part of the synth
//! is responsible, neutralizes it, and returns enough information for the
//! caller to retry. Two strategies are available, applied in order
//! of specificity:
//!
//! ## Block removal
//!
//! [`remove_errornous_block`] operates at the granularity of the blocks
//! produced by `sync_source_context`. It finds the block whose synth range
//! overlaps the diagnostic span and overwrites it with whitespace.
//!
//! Crucially, the replacement is exactly the same byte length as the block it
//! replaces. This keeps every other anchor in the `IndexMapper` valid without
//! needing to recompute them.
//!
//! ## Math marking
//!
//! [`try_mark_errornous`] operates at the granularity of individual math
//! expressions. It is tried first, before block removal, when the diagnostic
//! falls inside an equation range. Rather than blanking the broken expression,
//! it wraps it in a red-text marker so the rest of the equation keeps
//! rendering normally.
//!
//! Because this wrapper changes the synth's byte length, `try_mark_errornous`
//! returns a [`MarkedResult`] that includes the length delta, and the caller
//! is responsible for folding that delta back into the `IndexMapper`.
//!
//! ## Recovery loop
//!
//! The caller runs these passes in a bounded retry loop (see
//! `renderer/paged/items.rs` for the SVG case). The loop terminates when either
//! a compile succeeds or the divergence counter reaches its limit, preventing
//! infinite loops on documents that genuinely cannot be fixed by blanking.

use std::ops::Range;

use ecow::EcoVec;
use rustc_hash::FxHashSet;
use typst::diag::{Severity, SourceDiagnostic};

use crate::{
    bindings::map_synth_span,
    source::{AstBlock, SourceContext},
    world::MnemoWorld,
};

/// Removes the block containing the first error from the source, updating
/// diagnostics and context.
///
/// Returns the indices of the removed blocks, or an empty vector if no error
/// blocks were found.
#[typst_macros::time]
pub fn remove_errornous_block(
    blocks: &[AstBlock],
    source_diagnostics: &EcoVec<SourceDiagnostic>,
    context: &mut SourceContext,
    world: &mut MnemoWorld,
) -> Vec<usize> {
    let error_ranges = source_diagnostics
        .iter()
        .filter_map(|diagnostic| {
            map_synth_span(
                diagnostic.span,
                diagnostic.severity == Severity::Error,
                &diagnostic.trace,
                context,
                world,
            )
        })
        .collect::<FxHashSet<_>>();

    let (indicies, synth_ranges): (Vec<usize>, Vec<Range<usize>>) = blocks
        .iter()
        .enumerate()
        .filter_map(|(idx, block)| {
            let raw_range = &block.range;

            let synth_range_start = context.map_raw_to_synth_from_left(raw_range.start);
            let synth_range_end = context.map_raw_to_synth_from_right(raw_range.end);
            let synth_range = synth_range_start..synth_range_end;

            let in_block = error_ranges.iter().any(|error_range| {
                (synth_range_start <= error_range.start && synth_range_end >= error_range.start)
                    || (synth_range_start <= error_range.end && synth_range_end >= error_range.end)
            });

            in_block.then_some((idx, synth_range))
        })
        .unzip();

    for synth_range in synth_ranges {
        let start_byte = synth_range.start;
        let end_byte = synth_range.end;

        // fill block with whitespace to stablize ranges
        let source = context.synth_source_mut(world).unwrap();
        let byte_length = end_byte - start_byte;
        let whitespace = " ".repeat(byte_length.saturating_sub(1)) + "\n";
        source.edit(start_byte..end_byte, &whitespace);
    }

    indicies
}

/// Tries to mark the specific expressions containing errors and wraps it in a
/// red text expression for visual feedback in the rendered output.
///
/// If marking is unsuccessful (e.g., due to complex expressions or multiple
/// errors), it falls back to removing the entire block containing the error, as
/// implemented in `remove_errornous_block`.
#[typst_macros::time]
pub fn try_mark_errornous(
    source_diagnostics: &EcoVec<SourceDiagnostic>,
    eq_ranges: &[Range<usize>],
    context: &mut SourceContext,
    world: &mut MnemoWorld,
) -> MarkedErrors {
    let pre_text = "#math.italic(text(fill:theme.error)[";
    let post_text = "])";
    let pre_text_len = pre_text.len();
    let post_text_len = post_text.len();
    let total_wrap_len = pre_text_len + post_text_len;

    if eq_ranges.is_empty() {
        return MarkedErrors {
            marks: Vec::new(),
            pre_text_len,
            post_text_len,
            total_wrap_len,
        };
    }

    let eq_ranges = eq_ranges
        .iter()
        .map(|eq_range| {
            let synth_start = context.map_raw_to_synth_from_left(eq_range.start);
            let synth_end = context.map_raw_to_synth_from_right(eq_range.end);

            synth_start..synth_end
        })
        .collect::<Vec<_>>();

    let error_ranges = source_diagnostics
        .iter()
        .filter_map(|diagnostic| {
            map_synth_span(
                diagnostic.span,
                diagnostic.severity == Severity::Error,
                &diagnostic.trace,
                context,
                world,
            )
        })
        .filter(|range| {
            #[allow(clippy::suspicious_operation_groupings)]
            eq_ranges.iter().any(|eq_range| {
                (range.start >= eq_range.start && range.start <= eq_range.end)
                    || (range.end >= eq_range.start && range.end <= eq_range.end)
            })
        })
        .collect::<Vec<_>>();

    let marks = error_ranges
        .into_iter()
        .map(|synth_range| {
            let source = context.synth_source_mut(world).unwrap();
            let original_text = source.text()[synth_range.clone()].to_string();
            // crate::log!("[MARKING]:\n{}", original_text);

            // Wrap the original text in a red text expression
            let marked_text = format!("{pre_text}{original_text}{post_text}");
            source.edit(synth_range.clone(), &marked_text);

            context
                .index_mapper
                .bump_synth_from(synth_range.end, total_wrap_len);

            // crate::log!("[NEW SOURCE]:\n{}", &source.text());

            ErrorMark {
                text: original_text,
                synth_range: synth_range.start..(synth_range.end + total_wrap_len),
            }
        })
        .collect();

    MarkedErrors {
        marks,
        pre_text_len,
        post_text_len,
        total_wrap_len,
    }
}

pub fn map_error_mark_index(marked_errors: &MarkedErrors, context: &mut SourceContext) {
    for mark in &marked_errors.marks {
        // crate::log!("before: {:?}", &context.index_mapper);
        // crate::log!("delta range: {synth_range:?}");

        let raw_start = context
            .index_mapper
            .map_synth_to_raw_from_right(mark.synth_range.start);
        context.index_mapper.push_raw_to_synth(
            raw_start,
            mark.synth_range.start + marked_errors.pre_text_len,
        );
        context
            .index_mapper
            .push_raw_to_synth(raw_start, mark.synth_range.start);

        let raw_end = context
            .index_mapper
            .map_synth_to_raw_from_left(mark.synth_range.end);
        context
            .index_mapper
            .push_raw_to_synth(raw_end, mark.synth_range.end + marked_errors.total_wrap_len);
        context
            .index_mapper
            .push_raw_to_synth(raw_end, mark.synth_range.end + marked_errors.pre_text_len);

        // crate::log!("after: {:?}", &context.index_mapper);
    }
}

#[derive(Debug)]
pub struct MarkedErrors {
    pub marks: Vec<ErrorMark>,
    pub pre_text_len: usize,
    pub post_text_len: usize,
    pub total_wrap_len: usize,
}

#[derive(Debug)]
pub struct ErrorMark {
    pub text: String,
    pub synth_range: Range<usize>,
}
