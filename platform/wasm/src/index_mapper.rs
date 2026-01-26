#[derive(Debug, Default)]
pub struct IndexMapper {
    inflections: Vec<(usize, usize)>,
}

impl IndexMapper {
    pub fn add_aux_to_main(&mut self, aux: usize, main: usize) {
        self.inflections.push((aux, main));
    }

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
