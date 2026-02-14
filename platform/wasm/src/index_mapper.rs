/// Maps byte indices between aux (user/editor) and main (compiled) sources.
///
/// Maintains inflection points to efficiently translate positions in either direction.
#[derive(Debug, Default)]
pub struct IndexMapper {
    inflections: Vec<(usize, usize)>,
}

impl IndexMapper {
    /// Add an inflection point mapping an aux (editor) index to a main (compiled) index.
    pub fn add_aux_to_main(&mut self, aux: usize, main: usize) {
        self.inflections.push((aux, main));
    }

    /// Map a main (compiled) index to an aux (editor) index, searching from the right.
    ///
    /// Returns the closest aux index corresponding to the given main index.
    pub fn map_main_to_aux_from_right(&self, main_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(_, mapped_idx)| main_idx >= *mapped_idx);

        match inflection {
            Some((aux_idx, mapped_idx)) => aux_idx + (main_idx - mapped_idx),
            None => 0,
        }
    }

    /// Map an aux (editor) index to a main (compiled) index, searching from the right.
    ///
    /// Returns the closest main index corresponding to the given aux index.
    pub fn map_aux_to_main_from_right(&self, aux_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(mapped_idx, _)| aux_idx >= *mapped_idx);

        match inflection {
            Some((mapped_idx, main_idx)) => main_idx + (aux_idx - mapped_idx),
            None => 0,
        }
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

        mapped_idx.unwrap_or_default()
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

        mapped_idx.unwrap_or_default()
    }
}
