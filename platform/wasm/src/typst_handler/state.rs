use std::{
    fmt,
    fs::File,
    io::{Cursor, Read},
    str::FromStr,
};

use hashbrown::HashMap;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use tar::Archive;
use tsify::Tsify;
use typst::{
    compile,
    ecow::EcoString,
    foundations::Bytes,
    layout::{Abs, PagedDocument, Point},
    syntax::{FileId, Source, VirtualPath, package::PackageSpec},
};
use typst_ide::Tooltip;
// use typst_html::html;
use typst_pdf::{PdfOptions, pdf};
use typst_syntax::{LinkedNode, Side};
// use typst_svg::{svg, svg_merged};
use wasm_bindgen::prelude::*;

use super::{
    index_mapper::IndexMapper,
    world::MnemoWorld,
    wrappers::{TypstCompletion, TypstDiagnostic, TypstFileId, TypstJump},
};
use crate::typst_handler::{
    renderer::{CompileResult, RenderPdfResult, RenderingMode, render_by_chunk, sync_file_context},
    wrappers::TypstHighlight,
};

#[wasm_bindgen]
#[derive(Default)]
pub struct TypstState {
    pub(crate) world: MnemoWorld,
    pub(crate) file_contexts: HashMap<TypstFileId, FileContext>,
}

#[wasm_bindgen]
impl TypstState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(js_name = "setPixelPerPt")]
    pub fn set_pixel_per_pt(&mut self, id: &TypstFileId, size: f32) {
        self.file_contexts.get_mut(id).unwrap().pixel_per_pt = size;
    }

    #[wasm_bindgen(js_name = "setTheme")]
    pub fn set_theme(&mut self, id: &TypstFileId, theme: ThemeColors) {
        self.file_contexts.get_mut(id).unwrap().theme = theme;
    }

    #[wasm_bindgen(js_name = "setLocale")]
    pub fn set_locale(&mut self, id: &TypstFileId, locale: String) {
        self.file_contexts.get_mut(id).unwrap().locale = locale;
    }

    #[wasm_bindgen(js_name = "createFileId")]
    pub fn create_file_id(&mut self, path: String) -> TypstFileId {
        let id = FileId::new(None, VirtualPath::new(&path).with_extension("typ"));
        let id_wrapper = TypstFileId::new(id);

        let context = FileContext::new(id);
        self.world.insert_source(context.aux_id, String::new());
        self.file_contexts.insert(id_wrapper.clone(), context);

        id_wrapper
    }

    #[wasm_bindgen(js_name = "insertFile")]
    pub fn insert_file(&mut self, id: &TypstFileId, text: String) {
        self.world.insert_source(id.inner(), text);
    }

    #[wasm_bindgen(js_name = "removeFile")]
    pub fn remove_file(&mut self, id: &TypstFileId) {
        self.file_contexts.remove(id);
        self.world.remove_source(&id.inner());
    }

    #[wasm_bindgen(js_name = "installPackage")]
    pub fn install_package(&mut self, spec: &str, data: Vec<u8>) -> Result<(), TypstError> {
        let package_spec = Some(PackageSpec::from_str(spec).map_err(TypstError)?);

        let data = Cursor::new(data);
        let data = flate2::read::GzDecoder::new(data);
        let mut archive = Archive::new(data);

        for entry in archive.entries().unwrap() {
            let mut file = entry.unwrap();
            let path = file.path().unwrap();

            let id = FileId::new(package_spec.clone(), VirtualPath::new(&path));

            let mut content = Vec::new();
            file.read_to_end(&mut content).unwrap();

            match String::from_utf8(content.clone()) {
                Ok(content) => self.world.insert_source(id, content),
                Err(..) => self.world.insert_file(id, Bytes::new(content)),
            }
        }

        Ok(())
    }

    #[wasm_bindgen(js_name = "installFont")]
    pub fn install_font(&mut self, bytes: Vec<u8>) {
        self.world.install_font(bytes);
    }

    pub(crate) fn prelude(&self, id: &TypstFileId, rendering_mode: RenderingMode) -> String {
        let context = self.file_contexts.get(id).unwrap();

        let page_config = match rendering_mode {
            RenderingMode::Png => {
                formatdoc!(
                    r#"
                        #set page(fill:rgb(0,0,0,0),width:{width},height:auto,margin:0pt)

                        #set text(top-edge:"ascender",bottom-edge:"descender")
                        #set par(leading:0.08em,linebreaks:"simple")

                        #set list(spacing:0.125em)
                        #set enum(spacing:0.125em)

                        #show math.equation.where(block:true):set block(above:0.25em,below:0.25em)
                        #show heading:set block(above:0.25em,below:0.125em)
                        #show heading:set text(top-edge:"bounds",bottom-edge:"bounds")
                        #show list:set block(above:0.25em,below:0.125em)
                        #show enum:set block(above:0.25em,below:0.125em)
                    "#,
                    width = context.width,
                )
            }
            RenderingMode::Pdf => {
                formatdoc!(
                    r#"
                        #set page(width:{width},height:auto,margin:16pt)
                    "#,
                    width = context.width,
                )
            } // RenderingMode::Html => formatdoc!(""),
        };

        formatdoc!(
            r#"
                #let theme={theme}
                #set text(fill:theme.on-background,size:16pt,lang:"{locale}")

                #show heading.where(level:1):set text(fill:theme.primary,size:32pt,weight:400)
                #show heading.where(level:2):set text(fill:theme.secondary,size:28pt,weight:400)
                #show heading.where(level:3):set text(fill:theme.tertiary,size:24pt,weight:400)
                #show heading.where(level:4):set text(fill:theme.primary,size:22pt,weight:400)
                #show heading.where(level:5):set text(fill:theme.secondary,size:16pt,weight:500)
                #show heading.where(level:6):set text(fill:theme.tertiary,size:14pt,weight:500)

                #show link:set text(fill:theme.primary)
                #show link:underline

                #set line(stroke:theme.outline)
                #set table(stroke:theme.outline)
                #set circle(stroke:theme.outline)
                #set ellipse(stroke:theme.outline)
                #set line(stroke:theme.outline)
                #set curve(stroke:theme.outline)
                #set polygon(stroke:theme.outline)
                #set rect(stroke:theme.outline)
                #set square(stroke:theme.outline)

                #show math.equation.where(block:true):set text(size:18pt)
                #show math.equation.where(block:true):set par(leading:9pt)

                #context {{show math.equation:set text(size:text.size*2)}}

                {page_config}
            "#,
            theme = context.theme,
            locale = context.locale,
        )
    }

    #[wasm_bindgen]
    pub fn compile(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> CompileResult {
        render_by_chunk(id, text, prelude, self)
    }

    #[wasm_bindgen]
    pub fn check(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> Vec<TypstDiagnostic> {
        let (ir, _) = sync_file_context(id, text, prelude, self);

        let context = self.file_contexts.get_mut(id).unwrap();
        context
            .main_source_mut(&mut self.world)
            .unwrap()
            .replace(&ir);

        let compiled = compile::<PagedDocument>(&self.world);
        let compiled_warnings = Some(compiled.warnings);

        let mut diagnostics = Vec::new();

        if let Some(warnings) = compiled_warnings {
            diagnostics.extend(TypstDiagnostic::from_diagnostics(
                warnings,
                &context,
                &self.world,
            ));
        }

        match compiled.output {
            Ok(document) => {
                context.document = Some(document);
            }
            Err(source_diagnostics) => {
                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &context,
                    &self.world,
                ));
            }
        }

        diagnostics
    }

    #[wasm_bindgen]
    pub fn highlight(&mut self, id: &TypstFileId, text: &str) -> Vec<TypstHighlight> {
        let Some(context) = self.file_contexts.get(id) else {
            return Vec::new();
        };

        let root = typst_syntax::parse(text);
        let Some(aux_source) = context.aux_source_mut(&mut self.world) else {
            return Vec::new();
        };
        aux_source.replace(text);

        let mut queue = vec![LinkedNode::new(&root)];
        let mut highlights = Vec::new();

        let aux_lines = aux_source.lines();

        while queue.len() > 0 {
            let curr = queue.pop().unwrap();
            let tag = typst_syntax::highlight(&curr);
            let range = curr.range();

            let highlight = tag.and_then(|tag| {
                let aux_range_start_utf16 = aux_lines.byte_to_utf16(range.start)?;
                let aux_range_end_utf16 = aux_lines.byte_to_utf16(range.end)?;
                let aux_range_utf16 = aux_range_start_utf16..aux_range_end_utf16;

                Some(TypstHighlight {
                    tag: tag.css_class().to_string(),
                    range: aux_range_utf16,
                })
            });

            if let Some(highlight) = highlight {
                highlights.push(highlight);
            }

            for child in curr.children() {
                queue.push(child);
            }
        }

        highlights.sort_by_key(|highlight| highlight.range.start);

        highlights
    }

    #[wasm_bindgen]
    pub fn click(&mut self, id: &TypstFileId, x: f64, mut y: f64) -> Option<TypstJump> {
        let context = self.file_contexts.get(id)?;
        let document = context.document.as_ref()?;

        let index = document
            .pages
            .iter()
            .rposition(|page| y >= page.frame.height().to_pt())
            .unwrap_or_default();
        let page = &document.pages[index];

        let page_offset = document
            .pages
            .iter()
            .map(|page| page.frame.height().to_pt())
            .rfind(|height| y >= *height)
            .unwrap_or_default();
        y -= page_offset;

        typst_ide::jump_from_click(
            &self.world,
            document,
            &page.frame,
            Point::new(Abs::pt(x), Abs::pt(y)),
        )
        .and_then(|jump| TypstJump::from_mapped(jump, context, &self.world))
    }

    #[wasm_bindgen]
    pub fn autocomplete(
        &self,
        id: &TypstFileId,
        aux_cursor_utf16: usize,
        explicit: bool,
    ) -> Option<Autocomplete> {
        let context = self.file_contexts.get(id)?;

        let main_source = context.main_source(&self.world)?;
        let aux_source = context.aux_source(&self.world)?;

        let aux_lines = aux_source.lines();
        let aux_cursor = aux_lines.utf16_to_byte(aux_cursor_utf16)?;
        let main_cursor = context.map_aux_to_main(aux_cursor);

        let (main_offset, completions) = typst_ide::autocomplete(
            &self.world,
            context.document.as_ref(),
            main_source,
            main_cursor,
            explicit,
        )?;

        let aux_offset = context.map_main_to_aux(main_offset);
        let aux_offset_utf16 = aux_lines.byte_to_utf16(aux_offset)?;

        Some(Autocomplete {
            offset: aux_offset_utf16,
            completions: completions
                .into_iter()
                .map(TypstCompletion::from)
                .collect::<Box<[_]>>(),
        })
    }

    #[wasm_bindgen]
    pub fn hover(&self, id: &TypstFileId, aux_cursor_utf16: usize, side: i8) -> Option<String> {
        let context = self.file_contexts.get(id).unwrap();

        let main_source = context.main_source(&self.world)?;
        let aux_source = context.aux_source(&self.world)?;

        let aux_lines = aux_source.lines();
        let aux_cursor = aux_lines.utf16_to_byte(aux_cursor_utf16)?;
        let main_cursor = context.map_aux_to_main(aux_cursor);

        let side = if side == -1 {
            Side::Before
        } else {
            Side::After
        };

        let tooltip = typst_ide::tooltip(
            &self.world,
            context.document.as_ref(),
            main_source,
            main_cursor,
            side,
        );

        tooltip.map(|tooltip| {
            match tooltip {
                Tooltip::Text(text) => text.to_string(),
                Tooltip::Code(text) => typst_syntax::highlight_html(&typst_syntax::parse(&text)),
            }
        })
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, id: &TypstFileId, width: Option<f64>, height: Option<f64>) -> bool {
        let context = self.file_contexts.get_mut(id).unwrap();

        let width = width
            .map(|width| width.to_string() + "pt")
            .unwrap_or_else(|| String::from("auto"));
        let width_changed = context.width != width;

        context.width = width;
        context.height = height;

        width_changed
    }

    #[wasm_bindgen(js_name = renderPdf)]
    pub fn render_pdf(&mut self, id: &TypstFileId) -> RenderPdfResult {
        self.world.main_id = Some(id.inner());

        let mut ir = self.prelude(id, RenderingMode::Pdf);

        let context = self.file_contexts.get_mut(id).unwrap();
        let main_source = context.main_source_mut(&mut self.world).unwrap();
        let text = main_source.text().to_string();
        ir += &text;

        main_source.replace(&ir);

        self.world.insert_source(context.aux_id, text);
        self.world.aux_id = Some(context.aux_id);

        let compiled = compile(&self.world);
        let mut diagnostics =
            TypstDiagnostic::from_diagnostics(compiled.warnings, &context, &self.world).into_vec();

        let bytes = match compiled.output {
            Ok(document) => {
                match pdf(&document, &PdfOptions::default()) {
                    Ok(pdf) => Some(pdf),
                    Err(source_diagnostics) => {
                        diagnostics.extend(TypstDiagnostic::from_diagnostics(
                            source_diagnostics,
                            &context,
                            &self.world,
                        ));

                        None
                    }
                }
            }
            Err(source_diagnostics) => {
                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &context,
                    &self.world,
                ));

                None
            }
        };

        RenderPdfResult { bytes, diagnostics }
    }
}

pub struct FileContext {
    pub main_id: FileId,
    pub aux_id: FileId,
    pub index_mapper: IndexMapper,
    pub document: Option<PagedDocument>,

    pub width: String,
    pub height: Option<f64>,
    pub pixel_per_pt: f32,
    pub theme: ThemeColors,
    pub locale: String,
}

impl FileContext {
    pub fn new(main_id: FileId) -> Self {
        let aux_id: FileId = main_id.with_extension("$.typ");

        Self {
            main_id,
            aux_id,
            index_mapper: IndexMapper::default(),
            document: None,
            width: String::from("auto"),
            height: None,
            pixel_per_pt: 1_f32,
            theme: ThemeColors::default(),
            locale: String::from("en"),
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

    pub fn map_main_to_aux(&self, main_idx: usize) -> usize {
        self.index_mapper.main_to_aux(main_idx)
    }

    pub fn map_aux_to_main(&self, aux_idx: usize) -> usize {
        self.index_mapper.aux_to_main(aux_idx)
    }
}

#[wasm_bindgen]
pub struct PackageFile {
    path: String,
    content: Vec<u8>,
}

#[wasm_bindgen]
impl PackageFile {
    #[wasm_bindgen(constructor)]
    pub fn new(path: String, content: Vec<u8>) -> Self {
        Self { path, content }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ThemeColors {
    background: Rgb,
    on_background: Rgb,

    outline: Rgb,
    outline_variant: Rgb,

    primary: Rgb,
    on_primary: Rgb,
    primary_container: Rgb,
    on_primary_container: Rgb,

    secondary: Rgb,
    on_secondary: Rgb,
    secondary_container: Rgb,
    on_secondary_container: Rgb,

    tertiary: Rgb,
    on_tertiary: Rgb,
    tertiary_container: Rgb,
    on_tertiary_container: Rgb,

    error: Rgb,
    on_error: Rgb,
    error_container: Rgb,
    on_error_container: Rgb,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            background: Rgb::WHITE,
            on_background: Rgb::BLACK,

            outline: Rgb::BLACK,
            outline_variant: Rgb::BLACK,

            primary: Rgb::BLACK,
            on_primary: Rgb::WHITE,
            primary_container: Rgb::BLACK,
            on_primary_container: Rgb::WHITE,

            secondary: Rgb::BLACK,
            on_secondary: Rgb::WHITE,
            secondary_container: Rgb::BLACK,
            on_secondary_container: Rgb::WHITE,

            tertiary: Rgb::BLACK,
            on_tertiary: Rgb::WHITE,
            tertiary_container: Rgb::BLACK,
            on_tertiary_container: Rgb::WHITE,

            error: Rgb::BLACK,
            on_error: Rgb::WHITE,
            error_container: Rgb::BLACK,
            on_error_container: Rgb::WHITE,
        }
    }
}

#[wasm_bindgen]
impl ThemeColors {
    #[wasm_bindgen(constructor)]
    pub fn new(
        background: Rgb,
        on_background: Rgb,

        outline: Rgb,
        outline_variant: Rgb,

        primary: Rgb,
        on_primary: Rgb,
        primary_container: Rgb,
        on_primary_container: Rgb,

        secondary: Rgb,
        on_secondary: Rgb,
        secondary_container: Rgb,
        on_secondary_container: Rgb,

        tertiary: Rgb,
        on_tertiary: Rgb,
        tertiary_container: Rgb,
        on_tertiary_container: Rgb,

        error: Rgb,
        on_error: Rgb,
        error_container: Rgb,
        on_error_container: Rgb,
    ) -> Self {
        Self {
            background,
            on_background,

            outline,
            outline_variant,

            primary,
            on_primary,
            primary_container,
            on_primary_container,

            secondary,
            on_secondary,
            secondary_container,
            on_secondary_container,

            tertiary,
            on_tertiary,
            tertiary_container,
            on_tertiary_container,

            error,
            on_error,
            error_container,
            on_error_container,
        }
    }
}

impl fmt::Display for ThemeColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(background:{},on-background:{},outline:{},outline-variant:{},primary:{},on-primary:{},primary-container:{},on-primary-container:{},secondary:{},on-secondary:{},secondary-container:{},on-secondary-container:{},tertiary:{},on-tertiary:{},tertiary-container:{},on-tertiary-container:{},error:{},on-error:{},error-container:{},on-error-container:{})",
            self.background,
            self.on_background,
            self.outline,
            self.outline_variant,
            self.primary,
            self.on_primary,
            self.primary_container,
            self.on_primary_container,
            self.secondary,
            self.on_secondary,
            self.secondary_container,
            self.on_secondary_container,
            self.tertiary,
            self.on_tertiary,
            self.tertiary_container,
            self.on_tertiary_container,
            self.error,
            self.on_error,
            self.error_container,
            self.on_error_container,
        )
    }
}

#[wasm_bindgen]
#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    pub const BLACK: Self = Self(0, 0, 0);
    pub const WHITE: Self = Self(255, 255, 255);
}

#[wasm_bindgen]
impl Rgb {
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("rgb({},{},{})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.0, self.1, self.2)
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TypstError(EcoString);

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Autocomplete {
    pub offset: usize,
    pub completions: Box<[TypstCompletion]>,
}
