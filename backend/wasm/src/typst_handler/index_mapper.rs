#[derive(Debug, Default)]
pub struct IndexMapper {
    changes: Vec<(usize, usize)>,
}

impl IndexMapper {
    pub fn add_change(&mut self, index: usize, offset: usize) {
        self.changes.push((index, offset));
    }

    pub fn map_offset(&self, offset: usize) -> usize {
        let inflection = self.changes.iter().rfind(|(_, change)| offset >= *change);
        let (index, change) = inflection.unwrap_or(&(0, 0));

        index + (offset - change)
    }

    pub fn map_index(&self, index: usize) -> usize {
        let inflection = self.changes.iter().rfind(|(change, _)| index >= *change);
        let (change, offset) = inflection.unwrap_or(&(0, 0));

        offset + (index - change)
    }
}
