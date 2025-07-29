mod fonts;
mod index_mapper;
mod world;
mod wrappers;

use core::fmt;
use std::{borrow::Cow, ops::Range, str::FromStr};

use data_encoding::BASE64;
use index_mapper::IndexMapper;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use typst::{
    WorldExt, compile,
    ecow::EcoString,
    foundations::Bytes,
    layout::{Abs, PagedDocument, Point},
    syntax::{FileId, Source, SyntaxKind, VirtualPath, package::PackageSpec},
};
// use typst_html::html;
use typst_pdf::{PdfOptions, pdf};
// use typst_svg::{svg, svg_merged};
use wasm_bindgen::prelude::*;
use world::{FileSlot, MnemoWorld};
pub use wrappers::TypstFileId;
use wrappers::{TypstCompletion, TypstDiagnostic, TypstJump};

#[wasm_bindgen]
pub struct TypstState {
    world: MnemoWorld,
    document: Option<PagedDocument>,
    width: String,
    height: String,
    pt: f32,
    size: f32,
    theme: ThemeColors,
}

impl Default for TypstState {
    fn default() -> Self {
        Self {
            world: MnemoWorld::new(),
            document: None,
            width: String::from("auto"),
            height: String::from("auto"),
            pt: 0_f32,
            size: 0_f32,
            theme: ThemeColors::default(),
        }
    }
}

#[wasm_bindgen]
impl TypstState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pt(&self) -> f32 {
        self.pt
    }

    #[wasm_bindgen(js_name = "setPt")]
    pub fn set_pt(&mut self, pt: f32) {
        self.pt = pt;
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    #[wasm_bindgen(js_name = "setSize")]
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn theme(&self) -> ThemeColors {
        self.theme
    }

    #[wasm_bindgen(js_name = "setTheme")]
    pub fn set_theme(&mut self, theme: ThemeColors) {
        self.theme = theme;
    }

    #[wasm_bindgen(js_name = insertFile)]
    pub fn insert_file(&mut self, path: String, text: String) -> TypstFileId {
        let id = FileId::new(None, VirtualPath::new(&path).with_extension("typ"));
        self.world.insert_source(id, text);

        TypstFileId::new(id)
    }

    #[wasm_bindgen(js_name = installPackage)]
    pub fn install_package(
        &mut self,
        spec: &str,
        files: Vec<PackageFile>,
    ) -> Result<(), TypstError> {
        let package_spec = Some(PackageSpec::from_str(spec).map_err(TypstError)?);

        for file in files {
            let id = FileId::new(package_spec.clone(), VirtualPath::new(&file.path));

            match String::from_utf8(file.content.clone()) {
                Ok(content) => self.world.insert_source(id, content),
                Err(..) => self.world.insert_file(id, Bytes::new(file.content)),
            }
        }

        Ok(())
    }

    #[wasm_bindgen(js_name = installFont)]
    pub fn install_font(&mut self, bytes: Vec<u8>) {
        self.world.install_font(bytes);
    }

    fn prelude(&self, rendering_mode: RenderingMode) -> String {
        let page_config = match rendering_mode {
            RenderingMode::Png => {
                format!(
                    r#"
                        #set page(fill:rgb(0,0,0,0),width:{width},height:{height},margin:0pt)

                        #set text(top-edge:"ascender",bottom-edge:"descender")
                        #set par(leading:0em,linebreaks:"simple")

                        #show math.equation.where(block:true):set block(above:0em,below:0em)
                        #show heading:set block(above:0em,below:0em)
                        #show heading:set text(top-edge:"bounds",bottom-edge:"bounds")
                        #show list:set block(above:0em,below:0em)
                        #show enum:set block(above:0em,below:0em)
                    "#,
                    width = self.width,
                    height = self.height,
                )
            }
            RenderingMode::Pdf => {
                format!(
                    r#"
                        #set page(width:{width},height:{height},margin:16pt)
                    "#,
                    width = self.width,
                    height = self.height,
                )
            } // RenderingMode::Html => format!(""),
        };

        format!(
            r#"
                #let theme={theme}
                #set text(fill:theme.on-background,size:{size}pt)

                #context {{show math.equation:set text(size:text.size*2)}}

                #show math.equation.where(block:true):set text(size:{size}pt*1.125)
                #show math.equation.where(block:true):set par(leading:{size}pt*0.5625)

                #set table(stroke:theme.outline)

                #show heading.where(level:1):set text(fill:theme.primary,size:32pt,weight:400)
                #show heading.where(level:2):set text(fill:theme.secondary,size:28pt,weight:400)
                #show heading.where(level:3):set text(fill:theme.tertiary,size:24pt,weight:400)
                #show heading.where(level:4):set text(fill:theme.primary,size:22pt,weight:400)
                #show heading.where(level:5):set text(fill:theme.secondary,size:16pt,weight:500)
                #show heading.where(level:6):set text(fill:theme.tertiary,size:14pt,weight:500)

                {page_config}
            "#,
            theme = self.theme,
            size = self.size,
        )
    }

    #[wasm_bindgen]
    pub fn compile(&mut self, id: &TypstFileId, text: String, prelude: &str) -> CompileResult {
        let mut ir = self.prelude(RenderingMode::Png) + prelude + "\n\n";

        let mut index_mapper = IndexMapper::default();
        index_mapper.add_main_to_aux(0, ir.len());

        let aux_id = id.inner().with_extension("aux.typ");
        self.world
            .files
            .entry(aux_id)
            .and_modify(|file| {
                file.source_mut().unwrap().replace(&text);
            })
            .or_insert_with(|| FileSlot::Source(Source::new(aux_id, text)));
        self.world.aux = Some(aux_id);

        let aux_source = self.world.aux_source();

        let children = aux_source.root().children();
        let text = aux_source.text();

        let mut block_ranges = Vec::<RangedBlock>::new();
        let mut in_block = false;

        let mut last_kind: Option<SyntaxKind> = None;

        for node in children {
            let range = self.world.range(node.span()).unwrap();

            if let Some(until_newline) = node.text().chars().position(|ch| ch == '\n') {
                in_block = false;

                if let Some(last_block) = block_ranges.last_mut() {
                    last_block.range.end += until_newline;

                    ir += &text[last_block.range.clone()];

                    match last_kind {
                        Some(
                            SyntaxKind::LetBinding
                            | SyntaxKind::SetRule
                            | SyntaxKind::ShowRule
                            | SyntaxKind::ModuleImport
                            | SyntaxKind::ModuleInclude
                            | SyntaxKind::Contextual
                            | SyntaxKind::ListItem
                            | SyntaxKind::EnumItem
                            | SyntaxKind::Linebreak,
                        ) => {}
                        _ => {
                            ir += "\n#box() \\";
                            last_block.is_inline = true
                        }
                    }

                    // crate::log!("[LAST_KIND]: {last_kind:?}");

                    ir += "\n";
                }
            } else {
                last_kind = Some(node.kind());

                if in_block {
                    block_ranges.last_mut().unwrap().range.end = range.end;
                } else {
                    in_block = true;

                    index_mapper.add_main_to_aux(range.start, ir.len());
                    block_ranges.push(RangedBlock {
                        range,
                        is_inline: false,
                    });
                }
            }
        }

        if let Some(last_block) = block_ranges.last_mut() {
            if in_block {
                ir += &text[last_block.range.clone()];
            }
        }

        // crate::log!("[RANGES]: {block_ranges:?}");

        // crate::log!(
        //     "[SOURCE]: {:?}",
        //     &ir[(self.prelude(RenderingMode::Png) + prelude).len()..]
        // );

        self.world.index_mapper = index_mapper;
        self.world.main = Some(id.inner());

        let mut last_document = None;
        let mut partial_ir = Cow::from(&ir);

        let mut offset_height = 0_f64;
        let mut diagnostics = Vec::new();
        let mut compiled_warnings = None;

        let ranged_heights = block_ranges
            .into_iter()
            .filter_map(|block| {
                let aux_source = self.world.aux_source();

                let aux_range = block.range;
                let aux_start_utf16 = aux_source.byte_to_utf16(aux_range.start).unwrap();
                let aux_end_utf16 = aux_source.byte_to_utf16(aux_range.end).unwrap();
                let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                let mut end_byte = self.world.map_aux_to_main(aux_range.end);
                if block.is_inline {
                    // TODO: proper offsetting
                    end_byte += 10;
                }

                let source = self.world.main_source_mut();
                source.replace(partial_ir.get(..end_byte)?);

                // crate::log!("[RANGE_UTF8]: {aux_range:?}");
                // crate::log!("[RANGE_UTF16]: {range_utf16:?}");

                let compiled = compile::<PagedDocument>(&self.world);
                compiled_warnings = Some(compiled.warnings);

                match compiled.output {
                    Ok(document) => {
                        // TODO: handle changes in page margins

                        let document_height = document
                            .pages
                            .iter()
                            .map(|page| page.frame.height())
                            .sum::<Abs>()
                            .to_pt();
                        let height = document_height - offset_height;

                        if height <= 0_f64 {
                            return None;
                        }

                        let ranged_height = Some((aux_range_utf16, height, offset_height));

                        offset_height = document_height;
                        last_document = Some(document);

                        ranged_height
                    }
                    Err(source_diagnostics) => {
                        diagnostics.extend(TypstDiagnostic::from_diagnostics(
                            source_diagnostics,
                            &self.world,
                        ));

                        // crate::error!("[ERRORS]: {diagnostics:?}");

                        let start_range = self.world.map_aux_to_main(aux_range.start);

                        partial_ir.to_mut().replace_range(
                            start_range..end_byte,
                            &(" ".repeat(end_byte - start_range - 1) + "\\"),
                        );

                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        let frames = if let Some(document) = &last_document {
            ranged_heights
                .into_iter()
                .map(|(range, height, offset_height)| {
                    let canvas = mnemo_render::render(document, height, offset_height, self.pt);
                    let encoding = BASE64.encode(&canvas.encode_png().unwrap());

                    let height = height.ceil() as u32;

                    let render = FrameRender {
                        encoding,
                        height,
                        offset_height,
                    };

                    RangedFrame { range, render }
                })
                .collect()
        } else {
            Vec::new()
        };

        if let Some(warnings) = compiled_warnings {
            diagnostics.extend(TypstDiagnostic::from_diagnostics(warnings, &self.world));
        }

        self.document = last_document;
        self.world.main_source_mut().replace(&ir);

        CompileResult {
            frames,
            diagnostics,
        }
    }

    #[wasm_bindgen]
    pub fn click(&mut self, x: f64, mut y: f64) -> Option<TypstJump> {
        let document = self.document.as_ref()?;

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
        .map(|jump| TypstJump::from_mapped(jump, &self.world))
    }

    #[wasm_bindgen]
    pub fn autocomplete(&self, aux_cursor_utf16: usize, explicit: bool) -> Option<Autocomplete> {
        let main_source = self.world.main_source();
        let aux_source = self.world.aux_source();

        let aux_cursor = aux_source.utf16_to_byte(aux_cursor_utf16)?;
        let main_cursor = self.world.map_aux_to_main(aux_cursor);

        let (main_offset, completions) = typst_ide::autocomplete(
            &self.world,
            self.document.as_ref(),
            main_source,
            main_cursor,
            explicit,
        )?;

        let aux_offset = self.world.map_main_to_aux(main_offset);
        let aux_offset_utf16 = aux_source.byte_to_utf16(aux_offset)?;

        Some(Autocomplete {
            offset: aux_offset_utf16,
            completions: completions
                .into_iter()
                .map(TypstCompletion::from)
                .collect::<Box<[_]>>(),
        })
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: Option<f64>, height: Option<f64>) {
        self.width = width
            .map(|width| width.to_string() + "pt")
            .unwrap_or_else(|| String::from("auto"));

        self.height = height
            .map(|height| height.to_string() + "pt")
            .unwrap_or_else(|| String::from("auto"));
    }

    #[wasm_bindgen(js_name = renderPdf)]
    pub fn render_pdf(&mut self, id: &TypstFileId) -> RenderPdfResult {
        self.world.main = Some(id.inner());

        let mut ir = self.prelude(RenderingMode::Pdf);
        let main_source = self.world.main_source_mut();
        let text = main_source.text().to_string();
        ir += &text;
        main_source.replace(&ir);

        let aux_id = id.inner().with_extension("aux.typ");
        self.world.insert_source(aux_id, text);
        self.world.aux = Some(aux_id);

        let compiled = compile(&self.world);
        let mut diagnostics =
            TypstDiagnostic::from_diagnostics(compiled.warnings, &self.world).into_vec();

        let bytes = match compiled.output {
            Ok(document) => {
                match pdf(&document, &PdfOptions::default()) {
                    Ok(pdf) => Some(pdf),
                    Err(source_diagnostics) => {
                        diagnostics.extend(TypstDiagnostic::from_diagnostics(
                            source_diagnostics,
                            &self.world,
                        ));

                        None
                    }
                }
            }
            Err(source_diagnostics) => {
                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &self.world,
                ));

                None
            }
        };

        RenderPdfResult { bytes, diagnostics }
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
#[derive(Default, Clone, Copy, Serialize, Deserialize)]
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
pub struct CompileResult {
    frames: Vec<RangedFrame>,
    diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderPdfResult {
    pub bytes: Option<Vec<u8>>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderHtmlResult {
    pub document: Option<String>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Debug)]
struct RangedBlock {
    range: Range<usize>,
    is_inline: bool,
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

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RangedFrame {
    pub range: Range<usize>,
    pub render: FrameRender,
}

impl RangedFrame {
    pub fn new(range: Range<usize>, render: FrameRender) -> Self {
        Self { range, render }
    }
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrameRender {
    encoding: String,
    height: u32,
    #[serde(rename = "offsetHeight")]
    offset_height: f64,
}

enum RenderingMode {
    Png,
    Pdf,
    // Html,
}
