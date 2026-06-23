use std::cmp;

/// Maps byte offsets between the user's raw text and the synthesized file
/// Typst compiles.
///
/// The synth is built by inserting prelude text and block wrappers around the
/// raw source's content. Every insertion shifts subsequent byte positions
/// forward in the synth relative to where they sit in the raw source.
/// `IndexMapper` records the positions where those shifts happen, so a synth
/// offset can always be translated back to the corresponding raw source offset,
/// and vice versa.
///
/// ## Anchors
///
/// Each anchor is a pair `(raw_byte, synth_byte)` marking a position where the
/// two coordinate systems were aligned at the moment the anchor was recorded.
/// Between two anchors, the mapping is linear: add the difference between the
/// synth and raw source positions at the nearest preceding anchor.
///
/// Anchors are stored in insertion order, which is the order
/// `sync_source_context` encounters nodes while building the synth. They are
/// not guaranteed to be sorted by raw offset unless `insert_sorted` is used.
///
/// ## Lookup direction
///
/// Positions that sit exactly on an anchor boundary are ambiguous: they could
/// mean the last byte before an insertion, or the first byte after it. The two
/// lookup families resolve this differently:
///
/// - `*_from_right` finds the nearest anchor *at or before* the query and
///   extrapolates forward. Use this for diagnostics and hover ranges, where you
///   want the position as seen from outside the inserted text.
///
/// - `*_from_left` walks anchors forward until it finds the pair bracketing the
///   query. Use this for click-to-jump, where you want the position as seen
///   from inside the content.
#[derive(Debug, Default, Clone)]
pub struct IndexMapper {
    anchors: Vec<(usize, usize)>,
}

impl IndexMapper {
    /// Add an anchor point mapping an raw index to a synth index.
    pub fn push_raw_to_synth_unchecked(&mut self, raw_byte: usize, synth_byte: usize) {
        self.anchors.push((raw_byte, synth_byte));
    }

    /// Add an anchor point mapping an raw index to a synth index, keeping the
    /// anchors sorted by raw index.
    pub fn push_raw_to_synth(&mut self, raw: usize, synth: usize) {
        match self.anchors.binary_search_by_key(&raw, |&(a, _)| a) {
            Ok(byte) | Err(byte) => self.anchors.insert(byte, (raw, synth)),
        }
    }

    /// Map a synth index to an raw index, searching from the right.
    ///
    /// Returns the closest raw index corresponding to the given synth index.
    #[must_use]
    pub fn map_synth_to_raw_from_right(&self, synth_byte: usize) -> usize {
        if self.anchors.is_empty() {
            return synth_byte;
        }

        // Binary search for the rightmost anchor where mapped_byte <= synth_byte
        let search = self.anchors.binary_search_by(|&(_, mapped_byte)| {
            if mapped_byte > synth_byte {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Less
            }
        });

        let byte = match search {
            Ok(byte) => byte,
            Err(byte) => {
                if byte == 0 {
                    return synth_byte;
                }

                byte - 1
            }
        };

        let (raw_byte, mapped_byte) = self.anchors[byte];

        raw_byte + (synth_byte - mapped_byte)
    }

    /// Map an raw index to a synth index, searching from the right.
    ///
    /// Returns the closest synth index corresponding to the given raw index.
    #[must_use]
    pub fn map_raw_to_synth_from_right(&self, raw_byte: usize) -> usize {
        if self.anchors.is_empty() {
            return raw_byte;
        }

        // Binary search for the rightmost anchor where mapped_byte <= raw_byte
        let search = self.anchors.binary_search_by(|&(mapped_byte, _)| {
            if mapped_byte > raw_byte {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Less
            }
        });

        let byte = match search {
            Ok(byte) => byte,
            Err(byte) => {
                if byte == 0 {
                    return raw_byte;
                }

                byte - 1
            }
        };

        let (mapped_byte, synth_byte) = self.anchors[byte];

        synth_byte + (raw_byte - mapped_byte)
    }

    /// Map a synth index to an raw index, searching from the left.
    ///
    /// Returns the closest raw index corresponding to the given synth index.
    #[must_use]
    pub fn map_synth_to_raw_from_left(&self, synth_byte: usize) -> usize {
        let mut mapped_byte = None;

        let mut anchors = self.anchors.iter().peekable();
        while let Some((raw_byte, mapped_synth_byte)) = anchors.next() {
            let mapped_raw_byte = raw_byte + (synth_byte - mapped_synth_byte);

            if synth_byte == *mapped_synth_byte {
                mapped_byte = Some(mapped_raw_byte);
                break;
            }

            if let Some((_, next_mapped_synth_byte)) = anchors.peek() {
                if synth_byte < *next_mapped_synth_byte {
                    mapped_byte = Some(mapped_raw_byte);
                    break;
                }
            } else {
                mapped_byte = Some(mapped_raw_byte);
                break;
            }
        }

        mapped_byte.unwrap_or(synth_byte)
    }

    /// Map an raw index to a synth index, searching from the left.
    ///
    /// Returns the closest synth index corresponding to the given raw index.
    #[must_use]
    pub fn map_raw_to_synth_from_left(&self, raw_byte: usize) -> usize {
        let mut mapped_byte = None;

        let mut anchors = self.anchors.iter().peekable();
        while let Some((mapped_raw_byte, synth_byte)) = anchors.next() {
            let mapped_synth_byte = synth_byte + (raw_byte - mapped_raw_byte);

            if raw_byte == *mapped_raw_byte {
                mapped_byte = Some(mapped_synth_byte);
                break;
            }

            if let Some((next_mapped_raw_byte, _)) = anchors.peek() {
                if raw_byte < *next_mapped_raw_byte {
                    mapped_byte = Some(mapped_synth_byte);
                    break;
                }
            } else {
                mapped_byte = Some(mapped_synth_byte);
                break;
            }
        }

        mapped_byte.unwrap_or(raw_byte)
    }

    pub fn bump_synth_from(&mut self, synth_byte: usize, delta: usize) {
        for (_, mapped_synth_byte) in &mut self.anchors {
            if *mapped_synth_byte >= synth_byte {
                *mapped_synth_byte += delta;
            }
        }
    }
}

#[test]
fn test_push_and_map_raw_to_synth_from_right() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(5, 10);
    mapper.push_raw_to_synth_unchecked(10, 20);

    // Exact anchor points
    assert_eq!(mapper.map_raw_to_synth_from_right(0), 0);
    assert_eq!(mapper.map_raw_to_synth_from_right(5), 10);
    assert_eq!(mapper.map_raw_to_synth_from_right(10), 20);

    // Between anchor points
    assert_eq!(mapper.map_raw_to_synth_from_right(7), 12);
    assert_eq!(mapper.map_raw_to_synth_from_right(9), 14);

    // Before first anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(2), 2);
}

#[test]
fn test_push_and_map_synth_to_raw_from_right() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(5, 10);
    mapper.push_raw_to_synth_unchecked(10, 20);

    // Exact anchor points
    assert_eq!(mapper.map_synth_to_raw_from_right(0), 0);
    assert_eq!(mapper.map_synth_to_raw_from_right(10), 5);
    assert_eq!(mapper.map_synth_to_raw_from_right(20), 10);

    // Between anchor points
    assert_eq!(mapper.map_synth_to_raw_from_right(12), 7);
    assert_eq!(mapper.map_synth_to_raw_from_right(19), 14);

    // Before first anchor
    assert_eq!(mapper.map_synth_to_raw_from_right(2), 2);
}

#[test]
fn test_map_raw_to_synth_from_left() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(5, 10);
    mapper.push_raw_to_synth_unchecked(10, 20);

    // Exact anchor points
    assert_eq!(mapper.map_raw_to_synth_from_left(0), 0);
    assert_eq!(mapper.map_raw_to_synth_from_left(5), 10);
    assert_eq!(mapper.map_raw_to_synth_from_left(10), 20);

    // Between anchor points
    assert_eq!(mapper.map_raw_to_synth_from_left(7), 12);
    assert_eq!(mapper.map_raw_to_synth_from_left(9), 14);

    // Before first anchor
    assert_eq!(mapper.map_raw_to_synth_from_left(2), 2);
}

#[test]
fn test_map_synth_to_raw_from_left() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(5, 10);
    mapper.push_raw_to_synth_unchecked(10, 20);

    // Exact anchor points
    assert_eq!(mapper.map_synth_to_raw_from_left(0), 0);
    assert_eq!(mapper.map_synth_to_raw_from_left(10), 5);
    assert_eq!(mapper.map_synth_to_raw_from_left(20), 10);

    // Between anchor points
    assert_eq!(mapper.map_synth_to_raw_from_left(12), 7);
    assert_eq!(mapper.map_synth_to_raw_from_left(19), 14);

    // Before first anchor
    assert_eq!(mapper.map_synth_to_raw_from_left(2), 2);
}

#[test]
fn test_empty_mapper_returns_index() {
    let mapper = IndexMapper::default();

    assert_eq!(mapper.map_raw_to_synth_from_right(5), 5);
    assert_eq!(mapper.map_synth_to_raw_from_right(5), 5);
    assert_eq!(mapper.map_raw_to_synth_from_left(5), 5);
    assert_eq!(mapper.map_synth_to_raw_from_left(5), 5);
}

#[test]
fn test_duplicate_raw_anchors() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(0, 5); // same raw, different main
    mapper.push_raw_to_synth_unchecked(5, 10);

    // Should use the last anchor with raw=0 for right search
    assert_eq!(mapper.map_raw_to_synth_from_right(0), 5);
    assert_eq!(mapper.map_synth_to_raw_from_right(5), 0);

    // After anchors
    assert_eq!(mapper.map_raw_to_synth_from_right(6), 11);
    assert_eq!(mapper.map_synth_to_raw_from_right(12), 7);
}

#[test]
fn test_duplicate_synth_anchors() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(5, 0); // same main, different raw
    mapper.push_raw_to_synth_unchecked(10, 10);

    // Should use the last anchor with main=0 for right search
    assert_eq!(mapper.map_synth_to_raw_from_right(0), 5);
    assert_eq!(mapper.map_raw_to_synth_from_right(5), 0);

    // After anchors
    assert_eq!(mapper.map_synth_to_raw_from_right(12), 12);
    assert_eq!(mapper.map_raw_to_synth_from_right(12), 12);
}

#[test]
fn test_duplicate_raw_and_synth_anchors() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(0, 0); // identical anchor
    mapper.push_raw_to_synth_unchecked(5, 5);

    // Should use the last anchor for right search
    assert_eq!(mapper.map_raw_to_synth_from_right(0), 0);
    assert_eq!(mapper.map_synth_to_raw_from_right(0), 0);
    assert_eq!(mapper.map_raw_to_synth_from_right(5), 5);
    assert_eq!(mapper.map_synth_to_raw_from_right(5), 5);
}

#[test]
fn test_single_anchor() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(3, 7);

    // Before anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(2), 2);
    assert_eq!(mapper.map_synth_to_raw_from_right(6), 6);

    // At anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(3), 7);
    assert_eq!(mapper.map_synth_to_raw_from_right(7), 3);

    // After anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(5), 9);
    assert_eq!(mapper.map_synth_to_raw_from_right(9), 5);
}

#[test]
fn test_non_monotonic_anchors() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(10, 5); // non-monotonic: raw increases, synth decreases

    // Before second anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(5), 5);
    assert_eq!(mapper.map_synth_to_raw_from_right(3), 3);

    // At second anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(10), 5);
    assert_eq!(mapper.map_synth_to_raw_from_right(5), 10);

    // After second anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(12), 7);
    assert_eq!(mapper.map_synth_to_raw_from_right(7), 12);
}

#[test]
fn test_sparse_anchors() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(100, 200);

    // Before second anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(50), 50);
    assert_eq!(mapper.map_synth_to_raw_from_right(150), 150);

    // At second anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(100), 200);
    assert_eq!(mapper.map_synth_to_raw_from_right(200), 100);

    // After second anchor
    assert_eq!(mapper.map_raw_to_synth_from_right(110), 210);
    assert_eq!(mapper.map_synth_to_raw_from_right(210), 110);
}

#[test]
fn test_empty_anchors_edge_cases() {
    let mapper = IndexMapper::default();

    assert_eq!(mapper.map_raw_to_synth_from_right(0), 0);
    assert_eq!(mapper.map_synth_to_raw_from_right(0), 0);
    assert_eq!(mapper.map_raw_to_synth_from_left(0), 0);
    assert_eq!(mapper.map_synth_to_raw_from_left(0), 0);
    assert_eq!(mapper.map_raw_to_synth_from_right(usize::MAX), usize::MAX);
    assert_eq!(mapper.map_synth_to_raw_from_right(usize::MAX), usize::MAX);
    assert_eq!(mapper.map_raw_to_synth_from_left(usize::MAX), usize::MAX);
    assert_eq!(mapper.map_synth_to_raw_from_left(usize::MAX), usize::MAX);
}

#[test]
fn test_multiple_close_anchors() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(0, 0);
    mapper.push_raw_to_synth_unchecked(1, 2);
    mapper.push_raw_to_synth_unchecked(2, 4);

    // Test all mappings between anchors
    assert_eq!(mapper.map_raw_to_synth_from_right(0), 0);
    assert_eq!(mapper.map_raw_to_synth_from_right(1), 2);
    assert_eq!(mapper.map_raw_to_synth_from_right(2), 4);
    assert_eq!(mapper.map_raw_to_synth_from_right(3), 5);
    assert_eq!(mapper.map_synth_to_raw_from_right(0), 0);
    assert_eq!(mapper.map_synth_to_raw_from_right(2), 1);
    assert_eq!(mapper.map_synth_to_raw_from_right(4), 2);
    assert_eq!(mapper.map_synth_to_raw_from_right(5), 3);
}

#[test]
fn test_duplicate_raw_indices_different_synth() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(2, 10);
    mapper.push_raw_to_synth_unchecked(2, 20);
    mapper.push_raw_to_synth_unchecked(4, 30);

    // Should use the last anchor with raw=2 for right search
    assert_eq!(mapper.map_raw_to_synth_from_right(2), 20);
    assert_eq!(mapper.map_synth_to_raw_from_right(20), 2);

    // After anchors
    assert_eq!(mapper.map_raw_to_synth_from_right(4), 30);
    assert_eq!(mapper.map_synth_to_raw_from_right(30), 4);
}

#[test]
fn test_duplicate_synth_indices_different_raw() {
    let mut mapper = IndexMapper::default();
    mapper.push_raw_to_synth_unchecked(1, 5);
    mapper.push_raw_to_synth_unchecked(3, 5);
    mapper.push_raw_to_synth_unchecked(5, 10);

    // Should use the last anchor with main=5 for right search
    assert_eq!(mapper.map_synth_to_raw_from_right(5), 3);
    assert_eq!(mapper.map_raw_to_synth_from_right(3), 5);

    // After anchors
    assert_eq!(mapper.map_synth_to_raw_from_right(10), 5);
    assert_eq!(mapper.map_raw_to_synth_from_right(5), 10);
}
