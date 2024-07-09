use std::{
    collections::HashMap,
    // fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::{Datelike, Local};
use comemo::Prehashed;
use typst::{
    compile,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{package::PackageSpec, FileId, Source, VirtualPath},
    text::{Font, FontBook},
    utils::LazyHash,
    Library, World,
};

use super::fonts::{FontSearcher, FontSlot};

pub struct MnemoWorld {
    main: Option<FileId>,
    files: HashMap<FileId, Source>,
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

        let mut files = HashMap::new();

        let packages = mnemo_wasm_macros::packages!();
        for (package, pkg_files) in packages {
            let package_spec = Some(PackageSpec::from_str(package).unwrap());

            for (path, content) in &*pkg_files {
                let id = FileId::new(package_spec.clone(), VirtualPath::new(path));
                let source = Source::new(id, content.to_string());

                files.insert(id, source);
            }
        }

        Self {
            main: None,
            files,
            library: LazyHash::new(typst::Library::builder().build()),
            book: LazyHash::new(searcher.book),
            fonts: searcher.fonts,
        }
    }

    pub fn main_path(&self) -> &FileId {
        self.main.as_ref().unwrap()
    }

    pub fn main_source(&self) -> &Source {
        self.files.get(self.main.as_ref().unwrap()).unwrap()
    }

    pub fn main_source_mut(&mut self) -> &mut Source {
        self.files.get_mut(self.main.as_ref().unwrap()).unwrap()
    }

    pub fn set_main(&mut self, id: FileId, text: String) {
        let source = Source::new(id, text);

        self.files.insert(id, source);
        self.main = Some(id);
    }

    pub fn get_source(&self, id: FileId) -> Option<&Source> {
        if !self.files.contains_key(&id) {
            crate::log(&format!("{id:#?} NOT FOUND"));
            crate::log(&format!("{:#?}", self.files));
        }

        self.files.get(&id)
    }

    pub fn set_source(&mut self, id: &FileId, text: &str) {
        self.files.get_mut(id).unwrap().replace(text);
    }

    pub fn compile(&self) {
        let compiled = compile(self);
        for warning in compiled.warnings {
            crate::warn(&warning.message);
        }
        let document = compiled.output.unwrap();

        // for page in document.pages {
        //     for (point, item) in page.items() {
        //         match item {
        //             FrameItem::Group(_) => todo!(),
        //             FrameItem::Text(_) => todo!(),
        //             FrameItem::Shape(..) => todo!(),
        //             FrameItem::Image(..) => todo!(),
        //             FrameItem::Meta(meta, size) => match meta {
        //                 Meta::Link(_) => todo!(),
        //                 Meta::Elem(content) => {
        //                     // for field in content.fields() {
        //                     //     dbg!(field);
        //                     // }
        //                 }
        //                 Meta::PageNumbering(page_numbering) => {
        //                     dbg!(page_numbering);
        //                 }
        //                 Meta::Hide => todo!(),
        //             },
        //         }
        //     }
        // }
    }
}

impl World for MnemoWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> Source {
        self.files.get(self.main.as_ref().unwrap()).unwrap().clone()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.get_source(id).cloned().ok_or(FileError::Other(None))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.get_source(id)
            .map(|source| Bytes::from(source.text().as_bytes()))
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
        let now = Local::now();

        Datetime::from_ymd(
            now.year(),
            now.month().try_into().ok()?,
            now.day().try_into().ok()?,
        )
    }
}
