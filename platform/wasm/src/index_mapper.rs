#[derive(Debug, Default)]
pub struct IndexMapper {
    inflections: Vec<(usize, usize)>,
}

impl IndexMapper {
    pub fn add_aux_to_main(&mut self, aux: usize, main: usize) {
        self.inflections.push((aux, main));
    }

    pub fn main_to_aux(&self, main_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(_, mapped_idx)| main_idx >= *mapped_idx);

        match inflection {
            Some((aux_idx, mapped_idx)) => aux_idx + (main_idx - mapped_idx),
            None => 0,
        }
    }

    pub fn aux_to_main(&self, aux_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(mapped_idx, _)| aux_idx >= *mapped_idx);

        match inflection {
            Some((mapped_idx, main_idx)) => main_idx + (aux_idx - mapped_idx),
            None => 0,
        }
    }

    pub fn rmain_to_aux(&self, main_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .take_while(|(_, mapped_idx)| main_idx >= *mapped_idx)
            .last();

        match inflection {
            Some((aux_idx, mapped_idx)) => aux_idx + (main_idx - mapped_idx),
            None => 0,
        }
    }

    pub fn raux_to_main(&self, aux_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .take_while(|(mapped_idx, _)| aux_idx >= *mapped_idx)
            .last();

        match inflection {
            Some((mapped_idx, main_idx)) => main_idx + (aux_idx - mapped_idx),
            None => 0,
        }
    }
}
