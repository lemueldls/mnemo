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

/// Per-note rendering context.
///
/// Holds the two source file identities (raw and synth), the current page
/// geometry, the offset map tracking their correspondence, and the last
/// successfully compiled documents for this note.
///
/// One `SourceContext` exists per open note. It is created by
/// [`TypstState::create_source_id`] and stored
/// in `TypstState::source_context_map`.
#[derive(Debug)]
pub struct SourceContext {
    /// File ID for the synth: the synthesized intermediate source
    /// that Typst actually compiles. Built fresh by
    /// `sync_source_context` on each recompile.
    pub synth_id: FileId,

    /// File ID for the raw source: exactly what the user typed.
    /// Never modified by the renderer. All positions returned to the
    /// editor are in raw coordinates.
    pub raw_id: FileId,

    /// Which space (notebook) this note belongs to. Used to look up
    /// the associated [`SpaceContext`] for theme and font settings.
    pub space_id: String,

    /// Tracks the byte-offset correspondence between the raw source
    /// and the synth. Rebuilt on each call to `sync_source_context`.
    pub index_mapper: IndexMapper,

    /// The most recently compiled paged document for this note, if any.
    /// Cached here so hover and jump-to-source queries can avoid
    /// recompiling.
    pub paged_document: Option<PagedDocument>,

    /// The most recently compiled HTML document for this note, if any.
    pub html_document: Option<HtmlDocument>,

    /// Rendered page width, as a Typst dimension string (e.g. `"420pt"`
    /// or `"auto"`). Updated by [`TypstState::resize`].
    pub width: String,

    /// Maximum render height in points, if the note is in a fixed-height
    /// context (e.g. a locked sticky note). `None` for scrolling notes.
    pub height: Option<f64>,

    /// Body text size in points. Drives `#set text(size: ...)` in the
    /// generated prelude.
    pub text_size: f64,
}

impl SourceContext {
    #[must_use]
    pub fn new(synth_id: FileId, space_id: String) -> Self {
        let raw_id = FileId::new(RootedPath::new(
            synth_id.root().clone(),
            synth_id.vpath().with_extension("$.typ"),
        ));

        Self {
            synth_id,
            raw_id,
            space_id,
            index_mapper: IndexMapper::default(),
            paged_document: None,
            html_document: None,
            width: String::from("auto"),
            height: None,
            text_size: 16.0,
        }
    }

    pub fn synth_source<'a>(&self, world: &'a MnemoWorld) -> Option<&'a Source> {
        world.files.get(&self.synth_id)?.source()
    }

    pub fn synth_source_mut<'a>(&self, world: &'a mut MnemoWorld) -> Option<&'a mut Source> {
        world.files.get_mut(&self.synth_id)?.source_mut()
    }

    pub fn raw_source<'a>(&self, world: &'a MnemoWorld) -> Option<&'a Source> {
        world.files.get(&self.raw_id)?.source()
    }

    pub fn raw_source_mut<'a>(&self, world: &'a mut MnemoWorld) -> Option<&'a mut Source> {
        world.files.get_mut(&self.raw_id)?.source_mut()
    }

    #[must_use]
    pub fn map_synth_to_raw_from_right(&self, synth_idx: usize) -> usize {
        self.index_mapper.map_synth_to_raw_from_right(synth_idx)
    }

    #[must_use]
    pub fn map_raw_to_synth_from_right(&self, raw_idx: usize) -> usize {
        self.index_mapper.map_raw_to_synth_from_right(raw_idx)
    }

    #[must_use]
    pub fn map_synth_to_raw_from_left(&self, synth_idx: usize) -> usize {
        self.index_mapper.map_synth_to_raw_from_left(synth_idx)
    }

    #[must_use]
    pub fn map_raw_to_synth_from_left(&self, raw_idx: usize) -> usize {
        self.index_mapper.map_raw_to_synth_from_left(raw_idx)
    }
}
