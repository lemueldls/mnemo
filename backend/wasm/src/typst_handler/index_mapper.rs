
#[derive(Debug, Default)]
pub struct IndexMapper {
    changes: Vec<(usize, usize)>
}

impl IndexMapper {
    pub fn add_change(&mut self, index: usize, offset: usize) {
        self.changes.push((index, offset));
    }

    pub fn map_offset(&self, offset: usize) -> usize {
        crate::log(&format!("[CHANGES]: {:?}", &self.changes));

        let inflection = self.changes.iter().rfind(|(_, change)| offset > *change);
        let (index, change) = inflection.unwrap();

        index + (offset - change)
    }
}
