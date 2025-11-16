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
}
