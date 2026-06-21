use dashmap::DashSet;
use rustc_hash::FxHashMap;
use time::{OffsetDateTime, UtcOffset};
use typst::{
    Feature, Library, LibraryExt, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime, Duration},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_ide::IdeWorld;
use typst_syntax::{VirtualPath, VirtualRoot, package::PackageSpec};

use crate::{fonts::FontLoader, index_mapper::IndexMapper};

/// Implementation of Typst's `World` for Mnemo, managing all loaded files, fonts, and compilation state.
#[derive(Debug)]
pub struct MnemoWorld {
    /// The main (compiled/intermediate) source file id.
    pub main_id: Option<FileId>,
    /// The aux (user/editor/origin) source file id.
    pub aux_id: Option<FileId>,
    /// All loaded files (sources and binaries) by id.
    pub files: FxHashMap<FileId, FileSlot>,
    /// Index mapping between aux and main sources.
    pub index_mapper: IndexMapper,
    /// The Typst standard library for this world.
    library: LazyHash<Library>,
    /// Font loader and font book.
    font_loader: FontLoader,

    /// Sources requested by Typst but not yet loaded.
    pub requested_sources: DashSet<VirtualPath>,
    /// Files requested by Typst but not yet loaded.
    pub requested_files: DashSet<VirtualPath>,
    /// Packages requested by Typst but not yet loaded.
    pub requested_packages: DashSet<PackageSpec>,
}

impl Default for MnemoWorld {
    fn default() -> Self {
        let features = [Feature::Html, Feature::A11yExtras].into_iter().collect();
        let library = Library::builder().with_features(features).build();

        Self {
            main_id: None,
            aux_id: None,
            files: FxHashMap::default(),
            index_mapper: IndexMapper::default(),
            library: LazyHash::new(library),
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
        // if !self.files.contains_key(&id) {
        //     crate::error!("{id:#?} NOT FOUND");
        // }

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
        if let Some(source) = self.get_source(id) {
            Ok(source.clone())
        } else {
            match id.root() {
                VirtualRoot::Project => self.requested_sources.insert(id.vpath().clone()),
                VirtualRoot::Package(spec) => self.requested_packages.insert(spec.clone()),
            };

            Err(FileError::Other(None))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(file) = self.get_file(id) {
            Ok(file.bytes())
        } else {
            match id.root() {
                VirtualRoot::Project => self.requested_files.insert(id.vpath().clone()),
                VirtualRoot::Package(spec) => self.requested_packages.insert(spec.clone()),
            };

            Err(FileError::Other(None))
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        Some(self.font_loader.fonts[index].clone())
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        let now = if let Some(duration) = offset {
            #[allow(clippy::cast_possible_truncation)]
            OffsetDateTime::now_utc()
                .to_offset(UtcOffset::from_hms(duration.hours() as i8, 0, 0).unwrap())
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

/// Slot for a loaded file in the world, either a source or binary.
#[derive(Debug)]
pub enum FileSlot {
    /// A Typst source file.
    Source(Source),
    /// A binary file (e.g., image, font, etc).
    Bytes(Bytes),
}

impl FileSlot {
    #[must_use]
    pub const fn source(&self) -> Option<&Source> {
        match self {
            FileSlot::Source(source) => Some(source),
            FileSlot::Bytes(..) => None,
        }
    }

    pub const fn source_mut(&mut self) -> Option<&mut Source> {
        match self {
            FileSlot::Source(source) => Some(source),
            FileSlot::Bytes(..) => None,
        }
    }

    #[must_use]
    pub fn bytes(&self) -> Bytes {
        match self {
            FileSlot::Source(source) => Bytes::from_string(source.text().to_string()),
            FileSlot::Bytes(file) => file.clone(),
        }
    }
}
