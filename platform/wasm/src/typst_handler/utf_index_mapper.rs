#[derive(Debug)]
pub struct Inflection {
    pub utf8_index: usize,
    pub utf16_index: usize,
}

/// Maps character indices between UTF-8 byte offsets and UTF-16 code unit offsets.
///
/// This optimized version stores only "inflection points" where the
/// UTF-8/UTF-16 character length profile changes. It then interpolates
/// indices within these uniform segments.
///
/// It can be configured with a "prelude" offset. If a prelude is defined,
/// the mapping methods operate relative to the end of this prelude.
#[derive(Default, Debug)]
pub struct UtfIndexMapper {
    inflections: Vec<Inflection>,
}

impl UtfIndexMapper {
    pub fn new(text: &str, utf8_offset: usize) -> Self {
        let mut inflections = Vec::from_iter([Inflection {
            utf8_index: utf8_offset,
            utf16_index: 0,
        }]);

        for (index, ch) in text.char_indices() {
            let len_utf8 = ch.len_utf8();
            let len_utf16 = ch.len_utf16();

            if len_utf8 > len_utf16 {
                inflections.push(Inflection {
                    utf8_index: index + len_utf8 + utf8_offset,
                    utf16_index: index + len_utf16,
                })
            }
        }

        UtfIndexMapper { inflections }
    }

    pub fn utf8_to_utf16(&self, utf8_index: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|inflection| utf8_index >= inflection.utf8_index);

        match inflection {
            Some(inflection) => inflection.utf16_index + (utf8_index - inflection.utf8_index),
            None => 0,
        }
    }

    pub fn utf16_to_utf8(&self, utf16_index: usize) -> usize {
        let inflection = self
            .inflections
            .iter()
            .rfind(|inflection| utf16_index >= inflection.utf16_index);

        match inflection {
            Some(inflection) => inflection.utf8_index + (utf16_index - inflection.utf16_index),
            None => 0,
        }
    }
}
