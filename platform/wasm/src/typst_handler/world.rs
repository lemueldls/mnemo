use std::{
    collections::HashMap,
    // fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use comemo::Prehashed;
use serde::{Deserialize, Serialize};
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, UtcOffset};
use tsify::Tsify;
use typst::{
    Feature, Features, Library, World, compile,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    layout::PagedDocument,
    syntax::{FileId, Source, VirtualPath, package::PackageSpec},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_ide::IdeWorld;

use super::{
    PackageFile,
    fonts::{FontSearcher, FontSlot},
};

#[derive(Default)]
pub struct MnemoWorld {
    pub main: Option<FileId>,
    pub files: HashMap<FileId, Source>,
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<FontSlot>,
}

impl MnemoWorld {
    pub fn new() -> Self {
        // let source = Source::new(
        //     FileId::new(None, VirtualPath::new(main)),
        //     fs::read_to_string(path).unwrap(),
        // );

        // let mut files = HashMap::new();
        // files.insert(main.to_string(), source);

        let mut searcher = FontSearcher::new();
        searcher.search(&[]);

        let features = [Feature::Html].into_iter().collect();
        let library = typst::Library::builder().with_features(features).build();

        Self {
            main: None,
            files: HashMap::new(),
            library: LazyHash::new(library),
            book: LazyHash::new(searcher.book),
            fonts: searcher.fonts,
        }
    }

    // pub fn set_main(&mut self, id: FileId, text: String) {
    //     let source = Source::new(id, text);
    //     self.main = Some(id);
    // }

    // pub fn main_path(&self) -> &FileId {
    //     self.main.as_ref().unwrap()
    // }

    // pub fn main_path_mut(&mut self) -> &mut Option<FileId> {
    //     &mut self.main
    // }

    pub fn main_source(&self) -> &Source {
        self.files.get(self.main.as_ref().unwrap()).unwrap()
    }

    pub fn main_source_mut(&mut self) -> &mut Source {
        self.files.get_mut(self.main.as_ref().unwrap()).unwrap()
    }

    pub fn insert_file(&mut self, id: FileId, text: String) {
        let source = Source::new(id, text);

        self.files.insert(id, source);
    }

    pub fn get_source(&self, id: FileId) -> Option<&Source> {
        if !self.files.contains_key(&id) {
            crate::log(&format!("{id:#?} NOT FOUND"));
            crate::log(&format!("{:#?}", self.files));
        }

        self.files.get(&id)
    }

    // pub fn set_source(&mut self, id: &FileId, text: &str) {
    //     self.files.get_mut(id).unwrap().replace(text);
    // }
}

impl World for MnemoWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main.unwrap()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.get_source(id).cloned().ok_or(FileError::Other(None))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.get_source(id)
            .map(|source| Bytes::from_string(source.text().to_string()))
            .ok_or(FileError::Other(None))
    }

    fn font(&self, index: usize) -> Option<Font> {
        let slot = &self.fonts[index];

        // dbg!(&slot.font);

        slot.font
            .get_or_init(|| {
                // let data = fs::read(&slot.path).map(Bytes::from).ok()?;

                // Font::new(data, slot.index)
                todo!()
            })
            .clone()
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
