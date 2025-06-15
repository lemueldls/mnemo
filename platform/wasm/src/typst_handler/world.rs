use std::collections::HashMap;

use time::{OffsetDateTime, UtcOffset};
use typst::{
    Feature, Library, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_ide::IdeWorld;

use crate::typst_handler::{fonts::FontLoader, index_mapper::IndexMapper};

#[derive(Default)]
pub struct MnemoWorld {
    pub main: Option<FileId>,
    pub aux: Option<FileId>,
    pub files: HashMap<FileId, FileSlot>,
    pub index_mapper: IndexMapper,
    library: LazyHash<Library>,
    font_loader: FontLoader,
}

impl MnemoWorld {
    pub fn new() -> Self {
        let mut font_loader = FontLoader::new();
        font_loader.load();

        let features = [Feature::Html].into_iter().collect();
        let library = Library::builder().with_features(features).build();

        Self {
            main: None,
            aux: None,
            index_mapper: IndexMapper::default(),
            files: HashMap::new(),
            library: LazyHash::new(library),
            font_loader,
        }
    }

    pub fn main_source(&self) -> &Source {
        self.files
            .get(self.main.as_ref().unwrap())
            .unwrap()
            .source()
            .unwrap()
    }

    pub fn main_source_mut(&mut self) -> &mut Source {
        self.files
            .get_mut(self.main.as_ref().unwrap())
            .unwrap()
            .source_mut()
            .unwrap()
    }

    pub fn aux_source(&self) -> &Source {
        self.files
            .get(self.aux.as_ref().unwrap())
            .unwrap()
            .source()
            .unwrap()
    }

    pub fn aux_source_mut(&mut self) -> &mut Source {
        self.files
            .get_mut(self.aux.as_ref().unwrap())
            .unwrap()
            .source_mut()
            .unwrap()
    }

    pub fn map_main_to_aux(&self, main_idx: usize) -> usize {
        self.index_mapper.main_to_aux(main_idx)
    }

    pub fn map_aux_to_main(&self, aux_idx: usize) -> usize {
        self.index_mapper.aux_to_main(aux_idx)
    }

    pub fn insert_source(&mut self, id: FileId, text: String) {
        let source = Source::new(id, text);

        self.files.insert(id, FileSlot::Source(source));
    }

    pub fn insert_file(&mut self, id: FileId, bytes: Bytes) {
        self.files.insert(id, FileSlot::Bytes(bytes));
    }

    pub fn get_source(&self, id: FileId) -> Option<&Source> {
        self.get_file(id).and_then(|file| file.source())
    }

    pub fn get_file(&self, id: FileId) -> Option<&FileSlot> {
        if !self.files.contains_key(&id) {
            crate::error!("{id:#?} NOT FOUND");
            // crate::log!("{:#?}", self.files);
        }

        self.files.get(&id)
    }

    // pub fn set_source(&mut self, id: &FileId, text: &str) {
    //     self.files.get_mut(id).unwrap().replace(text);
    // }

    pub fn install_font(&mut self, bytes: Vec<u8>) {
        self.font_loader.install(bytes);
    }
}

impl World for MnemoWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_loader.book
    }

    fn main(&self) -> FileId {
        self.main.unwrap()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.get_source(id).cloned().ok_or(FileError::Other(None))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.get_file(id)
            .map(|file| file.bytes())
            .ok_or(FileError::Other(None))
    }

    fn font(&self, index: usize) -> Option<Font> {
        Some(self.font_loader.fonts[index].clone())
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = if let Some(hours) = offset {
            OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(hours as i8, 0, 0).unwrap())
        } else {
            OffsetDateTime::now_utc()
        };

        Some(Datetime::Date(now.date()))
    }
}

impl IdeWorld for MnemoWorld {
    fn upcast(&self) -> &dyn World {
        self
    }
}

pub enum FileSlot {
    Source(Source),
    Bytes(Bytes),
}

impl FileSlot {
    pub fn source(&self) -> Option<&Source> {
        match self {
            FileSlot::Source(source) => Some(source),
            _ => None,
        }
    }

    pub fn source_mut(&mut self) -> Option<&mut Source> {
        match self {
            FileSlot::Source(source) => Some(source),
            _ => None,
        }
    }

    pub fn bytes(&self) -> Bytes {
        match self {
            FileSlot::Source(source) => Bytes::from_string(source.text().to_string()),
            FileSlot::Bytes(file) => file.clone(),
        }
    }
}
