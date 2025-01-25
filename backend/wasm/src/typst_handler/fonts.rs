use std::{
    // fs::File,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use memmap2::Mmap;
use typst::{
    foundations::Bytes,
    text::{Font, FontBook, FontInfo},
};
// use walkdir::WalkDir;

/// Holds details about the location of a font and lazily the font itself.
pub struct FontSlot {
    pub path: PathBuf,
    pub index: u32,
    pub font: OnceLock<Option<Font>>,
}

pub struct FontSearcher {
    pub book: FontBook,
    pub fonts: Vec<FontSlot>,
}

impl FontSearcher {
    /// Create a new, empty system searcher.
    pub fn new() -> Self {
        Self {
            book: FontBook::new(),
            fonts: vec![],
        }
    }

    /// Search everything that is available.
    pub fn search(&mut self, font_paths: &[PathBuf]) {
        // #[cfg(feature = "embed-fonts")]
        self.search_embedded();

        // self.search_system();

        // for path in font_paths {
        //     self.search_dir(path);
        // }
    }

    /// Add fonts that are embedded in the binary.
    // #[cfg(feature = "embed-fonts")]
    fn search_embedded(&mut self) {
        let mut search = |bytes: &'static [u8]| {
            for (i, font) in Font::iter(Bytes::from_static(bytes)).enumerate() {
                self.book.push(font.info().clone());
                self.fonts.push(FontSlot {
                    path: PathBuf::new(),
                    index: i as u32,
                    font: OnceLock::from(Some(font)),
                });
            }
        };

        // Embed default fonts.
        search(include_bytes!(
            "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-Regular.ttf"
        ));
        search(include_bytes!(
            "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-Italic.ttf"
        ));
        search(include_bytes!(
            "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-Bold.ttf"
        ));
        search(include_bytes!(
            "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-BoldItalic.ttf"
        ));

        // search(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-regular.ttf"
        // ));
        // search(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-italic.ttf"
        // ));
        // search(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-book.ttf"
        // ));
        // search(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-bookitalic.ttf"
        // ));
        // search(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-bold.ttf"
        // ));
        // search(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-bolditalic.ttf"
        // ));

        // search(include_bytes!("../../../../../public/fonts/LinLibertine_R.ttf"));
        // search(include_bytes!("../../../../../public/fonts/LinLibertine_RB.ttf"));
        // search(include_bytes!("../../../../../public/fonts/LinLibertine_RBI.ttf"));
        // search(include_bytes!("../../../../../public/fonts/LinLibertine_RI.ttf"));

        search(include_bytes!(
            "../../../../public/fonts/NewCMMath-Book.otf"
        ));
        // search(include_bytes!(
        //     "../../../../public/fonts/NewCMMath-Regular.otf"
        // ));

        // search(include_bytes!("../../../../../public/fonts/DejaVuSansMono.ttf"
        // )); search(include_bytes!(
        //     "../../../../../public/fonts/DejaVuSansMono-Bold.ttf"
        // ));
        // search(include_bytes!(
        //     "../../../../../public/fonts/DejaVuSansMono-Oblique.ttf"
        // ));
        // search(include_bytes!(
        //     "../../../../../public/fonts/DejaVuSansMono-BoldOblique.ttf"
        // ));
    }

    // /// Search for fonts in the linux system font directories.
    // #[cfg(all(unix, not(target_os = "macos")))]
    // fn search_system(&mut self) {
    //     self.search_dir("/usr/share/fonts");
    //     self.search_dir("/usr/local/share/fonts");

    //     if let Some(dir) = dirs::font_dir() {
    //         self.search_dir(dir);
    //     }
    // }

    // /// Search for fonts in the macOS system font directories.
    // #[cfg(target_os = "macos")]
    // fn search_system(&mut self) {
    //     self.search_dir("/Library/Fonts");
    //     self.search_dir("/Network/Library/Fonts");
    //     self.search_dir("/System/Library/Fonts");

    //     if let Some(dir) = dirs::font_dir() {
    //         self.search_dir(dir);
    //     }
    // }

    // /// Search for fonts in the Windows system font directories.
    // #[cfg(windows)]
    // fn search_system(&mut self) {
    //     let windir = std::env::var("WINDIR").unwrap_or_else(|_|
    // "C:\\Windows".to_string());

    //     self.search_dir(Path::new(&windir).join("Fonts"));

    //     if let Some(roaming) = dirs::config_dir() {
    //         self.search_dir(roaming.join("Microsoft\\Windows\\Fonts"));
    //     }

    //     if let Some(local) = dirs::cache_dir() {
    //         self.search_dir(local.join("Microsoft\\Windows\\Fonts"));
    //     }
    // }

    // /// Search for all fonts in a directory recursively.
    // fn search_dir(&mut self, path: impl AsRef<Path>) {
    //     for entry in WalkDir::new(path)
    //         .follow_links(true)
    //         .sort_by(|a, b| a.file_name().cmp(b.file_name()))
    //         .into_iter()
    //         .filter_map(|e| e.ok())
    //     {
    //         let path = entry.path();
    //         if matches!(
    //             path.extension().and_then(|s| s.to_str()),
    //             Some("ttf" | "otf" | "TTF" | "OTF" | "ttc" | "otc" | "TTC" |
    // "OTC"),
    //         ) { self.search_file(path);
    //         }
    //     }
    // }

    // /// Index the fonts in the file at the given path.
    // fn search_file(&mut self, path: impl AsRef<Path>) {
    //     let path = path.as_ref();
    //     if let Ok(file) = File::open(path) {
    //         if let Ok(mmap) = unsafe { Mmap::map(&file) } {
    //             for (i, info) in FontInfo::iter(&mmap).enumerate() {
    //                 self.book.push(info);
    //                 self.fonts.push(FontSlot {
    //                     path: path.into(),
    //                     index: i as u32,
    //                     font: OnceCell::new(),
    //                 });
    //             }
    //         }
    //     }
    // }
}
