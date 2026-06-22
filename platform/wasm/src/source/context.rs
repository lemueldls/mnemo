use typst_html::HtmlDocument;
use typst_layout::PagedDocument;
use typst_syntax::{FileId, RootedPath, Source};

use crate::{source::IndexMapper, theme::ThemeColors, world::MnemoWorld};

/// Per-space configuration for rendering (fonts, theme, locale).
#[derive(Debug, Hash)]
pub struct SpaceContext {
    /// Default font for this space.
    pub font: String,
    /// Math font for this space.
    pub math_font: Option<String>,
    /// Code font for this space.
    pub code_font: Option<String>,
    /// Theme colors for this space.
    pub theme: ThemeColors,
    /// Locale for this space.
    pub locale: String,
}

impl SpaceContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            font: String::from("Maple Mono"),
            math_font: Some(String::from("New Computer Modern Math")),
            code_font: Some(String::from("Maple Mono")),
            theme: ThemeColors::default(),
            locale: String::from("en"),
        }
    }
}

impl Default for SpaceContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Context for a single Typst source file, tracking both main and aux sources and their mapping.
#[derive(Debug)]
pub struct SourceContext {
    /// File id of the main (intermediate/compiled) source.
    pub main_id: FileId,
    /// File id of the aux (user/editor) source.
    pub aux_id: FileId,
    /// The space this source belongs to.
    pub space_id: String,
    /// Index mapping between aux and main sources.
    pub index_mapper: IndexMapper,
    /// Cached paged document for this source, if available.
    pub paged_document: Option<PagedDocument>,
    /// Cached HTML document for this source, if available.
    pub html_document: Option<HtmlDocument>,
    /// Page width setting for this source.
    pub width: String,
    /// Page height setting for this source.
    pub height: Option<f64>,
    /// Text size for rendering.
    pub text_size: f64,
}

impl SourceContext {
    #[must_use]
    pub fn new(main_id: FileId, space_id: String) -> Self {
        let aux_id = FileId::new(RootedPath::new(
            main_id.root().clone(),
            main_id.vpath().with_extension("$.typ"),
        ));

        Self {
            main_id,
            aux_id,
            space_id,
            index_mapper: IndexMapper::default(),
            paged_document: None,
            html_document: None,
            width: String::from("auto"),
            height: None,
            text_size: 16.0,
        }
    }

    pub fn main_source<'a>(&self, world: &'a MnemoWorld) -> Option<&'a Source> {
        world.files.get(&self.main_id)?.source()
    }

    pub fn main_source_mut<'a>(&self, world: &'a mut MnemoWorld) -> Option<&'a mut Source> {
        world.files.get_mut(&self.main_id)?.source_mut()
    }

    pub fn aux_source<'a>(&self, world: &'a MnemoWorld) -> Option<&'a Source> {
        world.files.get(&self.aux_id)?.source()
    }

    pub fn aux_source_mut<'a>(&self, world: &'a mut MnemoWorld) -> Option<&'a mut Source> {
        world.files.get_mut(&self.aux_id)?.source_mut()
    }

    #[must_use]
    pub fn map_main_to_aux_from_right(&self, main_idx: usize) -> usize {
        self.index_mapper.map_main_to_aux_from_right(main_idx)
    }

    #[must_use]
    pub fn map_aux_to_main_from_right(&self, aux_idx: usize) -> usize {
        self.index_mapper.map_aux_to_main_from_right(aux_idx)
    }

    #[must_use]
    pub fn map_main_to_aux_from_left(&self, main_idx: usize) -> usize {
        self.index_mapper.map_main_to_aux_from_left(main_idx)
    }

    #[must_use]
    pub fn map_aux_to_main_from_left(&self, aux_idx: usize) -> usize {
        self.index_mapper.map_aux_to_main_from_left(aux_idx)
    }
}
