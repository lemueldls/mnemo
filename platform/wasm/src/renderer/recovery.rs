use std::ops::Range;

use ecow::EcoVec;
use rustc_hash::FxHashSet;
use typst::diag::{Severity, SourceDiagnostic};

use crate::{
    bindings::map_main_span,
    source::{AstBlock, SourceContext},
    world::MnemoWorld,
};

/// Removes the block containing the first error from the source, updating diagnostics and context.
///
/// Returns the indices of the removed blocks, or an empty vector if no error blocks were found.
#[typst_macros::time]
pub fn remove_errornous_block(
    ast_blocks: &[AstBlock],
    source_diagnostics: &EcoVec<SourceDiagnostic>,
    context: &mut SourceContext,
    world: &mut MnemoWorld,
) -> Vec<usize> {
    let error_ranges = source_diagnostics
        .iter()
        .filter_map(|diagnostic| {
            map_main_span(
                diagnostic.span,
                diagnostic.severity == Severity::Error,
                &diagnostic.trace,
                context,
                world,
            )
        })
        .collect::<FxHashSet<_>>();

    let (indicies, main_ranges): (Vec<usize>, Vec<Range<usize>>) = ast_blocks
        .iter()
        .enumerate()
        .filter_map(|(idx, block)| {
            let aux_range = &block.range;

            let main_range_start = context.map_aux_to_main_from_left(aux_range.start);
            let main_range_end = context.map_aux_to_main_from_right(aux_range.end);
            let main_range = main_range_start..main_range_end;

            let in_block = error_ranges.iter().any(|error_range| {
                (main_range_start <= error_range.start && main_range_end >= error_range.start)
                    || (main_range_start <= error_range.end && main_range_end >= error_range.end)
            });

            in_block.then_some((idx, main_range))
        })
        .unzip();

    for main_range in main_ranges {
        let start_byte = main_range.start;
        let end_byte = main_range.end;

        // fill block with whitespace to stablize ranges
        let source = context.main_source_mut(world).unwrap();
        let byte_length = end_byte - start_byte;
        let whitespace = " ".repeat(byte_length.saturating_sub(1)) + "\n";
        source.edit(start_byte..end_byte, &whitespace);
    }

    indicies
}

/// Tries to mark the specific expressions containing errors and wraps it in a red text expression for visual feedback in the rendered output.
///
/// If marking is unsuccessful (e.g., due to complex expressions or multiple errors), it falls back to removing the entire block containing the error, as implemented in `remove_errornous_block`.
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
            let main_start = context.map_aux_to_main_from_left(eq_range.start);
            let main_end = context.map_aux_to_main_from_right(eq_range.end);

            main_start..main_end
        })
        .collect::<Vec<_>>();

    let error_ranges = source_diagnostics
        .iter()
        .filter_map(|diagnostic| {
            map_main_span(
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
        .map(|main_range| {
            let source = context.main_source_mut(world).unwrap();
            let original_text = source.text()[main_range.clone()].to_string();
            // crate::log!("[MARKING]:\n{}", original_text);

            // Wrap the original text in a red text expression
            let marked_text = format!("{pre_text}{original_text}{post_text}");
            source.edit(main_range.clone(), &marked_text);

            context
                .index_mapper
                .bump_main_from(main_range.end, total_wrap_len);

            // crate::log!("[NEW SOURCE]:\n{}", &source.text());

            ErrorMark {
                text: original_text,
                main_range: main_range.start..(main_range.end + total_wrap_len),
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
        // crate::log!("delta range: {main_range:?}");

        let aux_start = context
            .index_mapper
            .map_main_to_aux_from_right(mark.main_range.start);
        context.index_mapper.push_aux_to_main(
            aux_start,
            mark.main_range.start + marked_errors.pre_text_len,
        );
        context
            .index_mapper
            .push_aux_to_main(aux_start, mark.main_range.start);

        let aux_end = context
            .index_mapper
            .map_main_to_aux_from_left(mark.main_range.end);
        context
            .index_mapper
            .push_aux_to_main(aux_end, mark.main_range.end + marked_errors.total_wrap_len);
        context
            .index_mapper
            .push_aux_to_main(aux_end, mark.main_range.end + marked_errors.pre_text_len);

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
    pub main_range: Range<usize>,
}
