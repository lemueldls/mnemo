#[derive(Debug, Default)]
pub struct IndexMapper {
    inflections: Vec<(usize, usize)>,
}

impl IndexMapper {
    pub fn add_change(&mut self, index: usize, offset: usize) {
        self.inflections.push((index, offset));
    }

    pub fn map_offset(&self, offset: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(_, change)| offset >= *change);

        crate::log(&format!("[OFFSET]: {offset:?}"));
        crate::log(&format!("[INFLECTION]: {inflection:?}"));
        crate::log(&format!("[INFLECTIONS]: {:?}", self.inflections));

        match inflection {
            Some((index, change)) => index + (offset - change),
            None => 0,
        }
    }

    pub fn map_index(&self, index: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|(change, _)| index >= *change);

        crate::log(&format!("[INDEX]: {index:?}"));
        crate::log(&format!("[INFLECTION]: {inflection:?}"));
        crate::log(&format!("[INFLECTIONS]: {:?}", self.inflections));

        match inflection {
            Some((change, offset)) => offset + (index - change),
            None => 0,
        }
    }
}
