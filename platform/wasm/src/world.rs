use dashmap::DashSet;
use rustc_hash::FxHashMap;
use time::{OffsetDateTime, UtcOffset};
use typst::{
    Library, LibraryExt, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_ide::IdeWorld;
use typst_syntax::{VirtualPath, package::PackageSpec};

use crate::{fonts::FontLoader, index_mapper::IndexMapper};

pub struct MnemoWorld {
    pub main_id: Option<FileId>,
    pub aux_id: Option<FileId>,
    pub files: FxHashMap<FileId, FileSlot>,
    pub index_mapper: IndexMapper,
    library: LazyHash<Library>,
    font_loader: FontLoader,

    pub requested_sources: DashSet<&'static VirtualPath>,
    pub requested_files: DashSet<&'static VirtualPath>,
    pub requested_packages: DashSet<&'static PackageSpec>,
}

impl Default for MnemoWorld {
    fn default() -> Self {
        Self {
            main_id: None,
            aux_id: None,
            files: FxHashMap::default(),
            index_mapper: IndexMapper::default(),
            library: LazyHash::new(Library::default()),
            font_loader: FontLoader::default(),
            requested_sources: DashSet::default(),
            requested_files: DashSet::default(),
            requested_packages: DashSet::default(),
        }
    }
}

impl MnemoWorld {
    pub fn insert_source(&mut self, id: FileId, text: String) {
        let source = Source::new(id, text);

        self.files.insert(id, FileSlot::Source(source));
    }

    pub fn remove_source(&mut self, id: &FileId) {
        self.files.remove(id);
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
        self.main_id.unwrap()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        match self.get_source(id) {
            Some(source) => Ok(source.clone()),
            None => {
                match id.package() {
                    Some(spec) => self.requested_packages.insert(spec),
                    None => self.requested_sources.insert(id.vpath()),
                };

                Err(FileError::Other(None))
            }
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        match self.get_file(id) {
            Some(file) => Ok(file.bytes()),
            None => {
                match id.package() {
                    Some(spec) => self.requested_packages.insert(spec),
                    None => self.requested_files.insert(id.vpath()),
                };

                Err(FileError::Other(None))
            }
        }
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

#[derive(Debug)]
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
