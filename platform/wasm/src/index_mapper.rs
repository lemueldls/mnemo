use std::cmp;

/// Maps byte indices between aux (user/editor) and main (compiled) sources.
///
/// Maintains inflection points to efficiently translate positions in either direction.
#[derive(Debug, Default)]
pub struct IndexMapper {
    inflections: Vec<(usize, usize)>,
}

impl IndexMapper {
    /// Add an inflection point mapping an aux (editor) index to a main (compiled) index.
    pub fn push_aux_to_main(&mut self, aux: usize, main: usize) {
        self.inflections.push((aux, main));
    }

    /// Map a main (compiled) index to an aux (editor) index, searching from the right.
    ///
    /// Returns the closest aux index corresponding to the given main index.
    pub fn map_main_to_aux_from_right(&self, main_idx: usize) -> usize {
        if self.inflections.is_empty() {
            return main_idx;
        }

        // Binary search for the rightmost inflection where mapped_idx <= main_idx
        let search = self.inflections.binary_search_by(|&(_, mapped_idx)| {
            if mapped_idx > main_idx {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Less
            }
        });

        let idx = match search {
            Ok(i) => i,
            Err(i) => {
                if i == 0 {
                    return main_idx;
                } else {
                    i - 1
                }
            }
        };

        let (aux_idx, mapped_idx) = self.inflections[idx];

        aux_idx + (main_idx - mapped_idx)
    }

    /// Map an aux (editor) index to a main (compiled) index, searching from the right.
    ///
    /// Returns the closest main index corresponding to the given aux index.
    pub fn map_aux_to_main_from_right(&self, aux_idx: usize) -> usize {
        if self.inflections.is_empty() {
            return aux_idx;
        }

        // Binary search for the rightmost inflection where mapped_idx <= aux_idx
        let search = self.inflections.binary_search_by(|&(mapped_idx, _)| {
            if mapped_idx > aux_idx {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Less
            }
        });

        let idx = match search {
            Ok(i) => i,
            Err(i) => {
                if i == 0 {
                    return aux_idx;
                } else {
                    i - 1
                }
            }
        };

        let (mapped_idx, main_idx) = self.inflections[idx];

        main_idx + (aux_idx - mapped_idx)
    }

    /// Map a main (compiled) index to an aux (editor) index, searching from the left.
    ///
    /// Returns the closest aux index corresponding to the given main index.
    pub fn map_main_to_aux_from_left(&self, main_idx: usize) -> usize {
        let mut mapped_idx = None;

        let mut inflections = self.inflections.iter().peekable();
        while let Some((aux_idx, mapped_main_idx)) = inflections.next() {
            let mapped_aux_idx = aux_idx + (main_idx - mapped_main_idx);

            if main_idx == *mapped_main_idx {
                mapped_idx = Some(mapped_aux_idx);
                break;
            }

            if let Some((_, next_mapped_main_idx)) = inflections.peek() {
                if main_idx < *next_mapped_main_idx {
                    mapped_idx = Some(mapped_aux_idx);
                    break;
                }
            } else {
                mapped_idx = Some(mapped_aux_idx);
                break;
            }
        }

        mapped_idx.unwrap_or(main_idx)
    }

    /// Map an aux (editor) index to a main (compiled) index, searching from the left.
    ///
    /// Returns the closest main index corresponding to the given aux index.
    pub fn map_aux_to_main_from_left(&self, aux_idx: usize) -> usize {
        let mut mapped_idx = None;

        let mut inflections = self.inflections.iter().peekable();
        while let Some((mapped_aux_idx, main_idx)) = inflections.next() {
            let mapped_main_idx = main_idx + (aux_idx - mapped_aux_idx);

            if aux_idx == *mapped_aux_idx {
                mapped_idx = Some(mapped_main_idx);
                break;
            }

            if let Some((next_mapped_aux_idx, _)) = inflections.peek() {
                if aux_idx < *next_mapped_aux_idx {
                    mapped_idx = Some(mapped_main_idx);
                    break;
                }
            } else {
                mapped_idx = Some(mapped_main_idx);
                break;
            }
        }

        mapped_idx.unwrap_or(aux_idx)
    }
}

#[test]
fn test_push_and_map_aux_to_main_from_right() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(5, 10);
    mapper.push_aux_to_main(10, 20);

    // Exact inflection points
    assert_eq!(mapper.map_aux_to_main_from_right(0), 0);
    assert_eq!(mapper.map_aux_to_main_from_right(5), 10);
    assert_eq!(mapper.map_aux_to_main_from_right(10), 20);

    // Between inflection points
    assert_eq!(mapper.map_aux_to_main_from_right(7), 12);
    assert_eq!(mapper.map_aux_to_main_from_right(9), 14);

    // Before first inflection
    assert_eq!(mapper.map_aux_to_main_from_right(2), 2);
}

#[test]
fn test_push_and_map_main_to_aux_from_right() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(5, 10);
    mapper.push_aux_to_main(10, 20);

    // Exact inflection points
    assert_eq!(mapper.map_main_to_aux_from_right(0), 0);
    assert_eq!(mapper.map_main_to_aux_from_right(10), 5);
    assert_eq!(mapper.map_main_to_aux_from_right(20), 10);

    // Between inflection points
    assert_eq!(mapper.map_main_to_aux_from_right(12), 7);
    assert_eq!(mapper.map_main_to_aux_from_right(19), 14);

    // Before first inflection
    assert_eq!(mapper.map_main_to_aux_from_right(2), 2);
}

#[test]
fn test_map_aux_to_main_from_left() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(5, 10);
    mapper.push_aux_to_main(10, 20);

    // Exact inflection points
    assert_eq!(mapper.map_aux_to_main_from_left(0), 0);
    assert_eq!(mapper.map_aux_to_main_from_left(5), 10);
    assert_eq!(mapper.map_aux_to_main_from_left(10), 20);

    // Between inflection points
    assert_eq!(mapper.map_aux_to_main_from_left(7), 12);
    assert_eq!(mapper.map_aux_to_main_from_left(9), 14);

    // Before first inflection
    assert_eq!(mapper.map_aux_to_main_from_left(2), 2);
}

#[test]
fn test_map_main_to_aux_from_left() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(5, 10);
    mapper.push_aux_to_main(10, 20);

    // Exact inflection points
    assert_eq!(mapper.map_main_to_aux_from_left(0), 0);
    assert_eq!(mapper.map_main_to_aux_from_left(10), 5);
    assert_eq!(mapper.map_main_to_aux_from_left(20), 10);

    // Between inflection points
    assert_eq!(mapper.map_main_to_aux_from_left(12), 7);
    assert_eq!(mapper.map_main_to_aux_from_left(19), 14);

    // Before first inflection
    assert_eq!(mapper.map_main_to_aux_from_left(2), 2);
}

#[test]
fn test_empty_mapper_returns_index() {
    let mapper = IndexMapper::default();

    assert_eq!(mapper.map_aux_to_main_from_right(5), 5);
    assert_eq!(mapper.map_main_to_aux_from_right(5), 5);
    assert_eq!(mapper.map_aux_to_main_from_left(5), 5);
    assert_eq!(mapper.map_main_to_aux_from_left(5), 5);
}

#[test]
fn test_duplicate_aux_inflections() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(0, 5); // same aux, different main
    mapper.push_aux_to_main(5, 10);

    // Should use the last inflection with aux=0 for right search
    assert_eq!(mapper.map_aux_to_main_from_right(0), 5);
    assert_eq!(mapper.map_main_to_aux_from_right(5), 0);

    // After inflections
    assert_eq!(mapper.map_aux_to_main_from_right(6), 11);
    assert_eq!(mapper.map_main_to_aux_from_right(12), 7);
}

#[test]
fn test_duplicate_main_inflections() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(5, 0); // same main, different aux
    mapper.push_aux_to_main(10, 10);

    // Should use the last inflection with main=0 for right search
    assert_eq!(mapper.map_main_to_aux_from_right(0), 5);
    assert_eq!(mapper.map_aux_to_main_from_right(5), 0);

    // After inflections
    assert_eq!(mapper.map_main_to_aux_from_right(12), 12);
    assert_eq!(mapper.map_aux_to_main_from_right(12), 12);
}

#[test]
fn test_duplicate_aux_and_main_inflections() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(0, 0); // identical inflection
    mapper.push_aux_to_main(5, 5);

    // Should use the last inflection for right search
    assert_eq!(mapper.map_aux_to_main_from_right(0), 0);
    assert_eq!(mapper.map_main_to_aux_from_right(0), 0);
    assert_eq!(mapper.map_aux_to_main_from_right(5), 5);
    assert_eq!(mapper.map_main_to_aux_from_right(5), 5);
}

#[test]
fn test_single_inflection() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(3, 7);

    // Before inflection
    assert_eq!(mapper.map_aux_to_main_from_right(2), 2);
    assert_eq!(mapper.map_main_to_aux_from_right(6), 6);

    // At inflection
    assert_eq!(mapper.map_aux_to_main_from_right(3), 7);
    assert_eq!(mapper.map_main_to_aux_from_right(7), 3);

    // After inflection
    assert_eq!(mapper.map_aux_to_main_from_right(5), 9);
    assert_eq!(mapper.map_main_to_aux_from_right(9), 5);
}

#[test]
fn test_non_monotonic_inflections() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(10, 5); // non-monotonic: aux increases, main decreases

    // Before second inflection
    assert_eq!(mapper.map_aux_to_main_from_right(5), 5);
    assert_eq!(mapper.map_main_to_aux_from_right(3), 3);

    // At second inflection
    assert_eq!(mapper.map_aux_to_main_from_right(10), 5);
    assert_eq!(mapper.map_main_to_aux_from_right(5), 10);

    // After second inflection
    assert_eq!(mapper.map_aux_to_main_from_right(12), 7);
    assert_eq!(mapper.map_main_to_aux_from_right(7), 12);
}

#[test]
fn test_sparse_inflections() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(100, 200);

    // Before second inflection
    assert_eq!(mapper.map_aux_to_main_from_right(50), 50);
    assert_eq!(mapper.map_main_to_aux_from_right(150), 150);

    // At second inflection
    assert_eq!(mapper.map_aux_to_main_from_right(100), 200);
    assert_eq!(mapper.map_main_to_aux_from_right(200), 100);

    // After second inflection
    assert_eq!(mapper.map_aux_to_main_from_right(110), 210);
    assert_eq!(mapper.map_main_to_aux_from_right(210), 110);
}

#[test]
fn test_empty_inflections_edge_cases() {
    let mapper = IndexMapper::default();

    assert_eq!(mapper.map_aux_to_main_from_right(0), 0);
    assert_eq!(mapper.map_main_to_aux_from_right(0), 0);
    assert_eq!(mapper.map_aux_to_main_from_left(0), 0);
    assert_eq!(mapper.map_main_to_aux_from_left(0), 0);
    assert_eq!(mapper.map_aux_to_main_from_right(usize::MAX), usize::MAX);
    assert_eq!(mapper.map_main_to_aux_from_right(usize::MAX), usize::MAX);
    assert_eq!(mapper.map_aux_to_main_from_left(usize::MAX), usize::MAX);
    assert_eq!(mapper.map_main_to_aux_from_left(usize::MAX), usize::MAX);
}

#[test]
fn test_multiple_close_inflections() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(0, 0);
    mapper.push_aux_to_main(1, 2);
    mapper.push_aux_to_main(2, 4);

    // Test all mappings between inflections
    assert_eq!(mapper.map_aux_to_main_from_right(0), 0);
    assert_eq!(mapper.map_aux_to_main_from_right(1), 2);
    assert_eq!(mapper.map_aux_to_main_from_right(2), 4);
    assert_eq!(mapper.map_aux_to_main_from_right(3), 5);
    assert_eq!(mapper.map_main_to_aux_from_right(0), 0);
    assert_eq!(mapper.map_main_to_aux_from_right(2), 1);
    assert_eq!(mapper.map_main_to_aux_from_right(4), 2);
    assert_eq!(mapper.map_main_to_aux_from_right(5), 3);
}

#[test]
fn test_duplicate_aux_indices_different_main() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(2, 10);
    mapper.push_aux_to_main(2, 20);
    mapper.push_aux_to_main(4, 30);

    // Should use the last inflection with aux=2 for right search
    assert_eq!(mapper.map_aux_to_main_from_right(2), 20);
    assert_eq!(mapper.map_main_to_aux_from_right(20), 2);

    // After inflections
    assert_eq!(mapper.map_aux_to_main_from_right(4), 30);
    assert_eq!(mapper.map_main_to_aux_from_right(30), 4);
}

#[test]
fn test_duplicate_main_indices_different_aux() {
    let mut mapper = IndexMapper::default();
    mapper.push_aux_to_main(1, 5);
    mapper.push_aux_to_main(3, 5);
    mapper.push_aux_to_main(5, 10);

    // Should use the last inflection with main=5 for right search
    assert_eq!(mapper.map_main_to_aux_from_right(5), 3);
    assert_eq!(mapper.map_aux_to_main_from_right(3), 5);

    // After inflections
    assert_eq!(mapper.map_main_to_aux_from_right(10), 5);
    assert_eq!(mapper.map_aux_to_main_from_right(5), 10);
}
