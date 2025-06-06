use typst::{
    foundations::Bytes,
    text::{Font, FontBook},
    utils::LazyHash,
};

#[derive(Default)]
pub struct FontLoader {
    pub book: LazyHash<FontBook>,
    pub fonts: Vec<Font>,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            book: LazyHash::new(FontBook::new()),
            fonts: vec![],
        }
    }

    pub fn install<T>(&mut self, bytes: T)
    where T: AsRef<[u8]> + Send + Sync + 'static {
        for font in Font::iter(Bytes::new(bytes)) {
            self.book.push(font.info().clone());
            self.fonts.push(font);
        }
    }

    pub fn load(&mut self) {
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-Regular.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-Italic.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-Bold.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-book/ttf/IosevkaBook-BoldItalic.ttf"
        // ));

        self.install(include_bytes!(
            "../../../../public/fonts/maple/ttf/MapleMono-Regular.ttf"
        ));
        self.install(include_bytes!(
            "../../../../public/fonts/maple/ttf/MapleMono-Italic.ttf"
        ));
        self.install(include_bytes!(
            "../../../../public/fonts/maple/ttf/MapleMono-Bold.ttf"
        ));
        self.install(include_bytes!(
            "../../../../public/fonts/maple/ttf/MapleMono-BoldItalic.ttf"
        ));

        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-regular.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-italic.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-book.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-bookitalic.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-bold.ttf"
        // ));
        // self.install(include_bytes!(
        //     "../../../../public/fonts/iosevka-custom/ttf/iosevka-custom-bolditalic.ttf"
        // ));

        self.install(include_bytes!(
            "../../../../public/fonts/NewCMMath-Book.otf"
        ));
        self.install(include_bytes!(
            "../../../../public/fonts/NewCMMath-Regular.otf"
        ));
    }
}
