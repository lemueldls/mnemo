#[derive(Debug, Default)]
pub struct IndexMapper {
    inflections: Vec<(usize, usize)>,
}

impl IndexMapper {
    pub fn map_index(&mut self, main: usize, aux: usize) {
        self.inflections.push((main, aux));
    }

    pub fn main_to_aux(&self, aux_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(_, change)| aux_idx >= *change);

        // crate::log(&format!("[OFFSET]: {aux_idx:?}"));
        // crate::log(&format!("[INFLECTION]: {inflection:?}"));
        // crate::log(&format!("[INFLECTIONS]: {:?}", self.inflections));

        match inflection {
            Some((main_idx, change)) => main_idx + (aux_idx - change),
            None => 0,
        }
    }

    pub fn aux_to_main(&self, main_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(change, _)| main_idx >= *change);

        // crate::log(&format!("[INDEX]: {main_idx:?}"));
        // crate::log(&format!("[INFLECTION]: {inflection:?}"));
        // crate::log(&format!("[INFLECTIONS]: {:?}", self.inflections));

        match inflection {
            Some((change, aux_idx)) => aux_idx + (main_idx - change),
            None => 0,
        }
    }
}
