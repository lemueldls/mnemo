#[derive(Debug, Default)]
pub struct IndexMapper {
    inflections: Vec<(usize, usize)>,
}

impl IndexMapper {
    pub fn add_main_to_aux(&mut self, main: usize, aux: usize) {
        self.inflections.push((main, aux));
    }

    pub fn main_to_aux(&self, main_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(_, change)| main_idx >= *change);

        // crate::log!("[MAIN_INDEX]: {main_idx:?}");
        // crate::log!("[INFLECTION]: {inflection:?}");
        // crate::log!("[INFLECTIONS]: {:?}", self.inflections);

        match inflection {
            Some((aux_idx, change)) => aux_idx + (main_idx - change),
            None => 0,
        }
    }

    pub fn aux_to_main(&self, aux_idx: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(change, _)| aux_idx >= *change);

        // crate::log!("[AUX_INDEX]: {aux_idx:?}");
        // crate::log!("[INFLECTION]: {inflection:?}");
        // crate::log!("[INFLECTIONS]: {:?}", self.inflections);

        match inflection {
            Some((change, main_idx)) => main_idx + (aux_idx - change),
            None => 0,
        }
    }
}
