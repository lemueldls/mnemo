mod fonts;
mod index_mapper;
mod world;
mod wrappers;

use core::fmt;
use std::{
    borrow::Cow,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    iter,
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Mutex,
};

use data_encoding::BASE64;
use index_mapper::IndexMapper;
// use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Error;
use tsify::Tsify;
use typst::{
    World, WorldExt, compile,
    diag::SourceDiagnostic,
    ecow::{EcoString, EcoVec},
    layout::{Abs, Frame, FrameItem, Page, PagedDocument, Point, Position},
    syntax::{
        FileId, Source, Span, SyntaxError, SyntaxKind, VirtualPath, ast, package::PackageSpec,
    },
    visualize::Color,
};
// use typst_svg::{svg, svg_merged};
use typst_pdf::{PdfOptions, PdfStandard, pdf};
// use typst_html::html;
use typst_render::{render, render_merged};
use wasm_bindgen::prelude::*;
use world::MnemoWorld;
use wrappers::{TypstCompletion, TypstDiagnostic, TypstJump};

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
#[derive(Serialize, Deserialize, Clone, Copy)]
// #[derive(Tsify, Serialize, Deserialize)]
// #[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ThemeColors {
    primary: Rgb,
    secondary: Rgb,
    tertiary: Rgb,
    outline: Rgb,
    on_primary_container: Rgb,
    on_secondary_container: Rgb,
    on_tertiary_container: Rgb,
    on_background: Rgb,
}

#[wasm_bindgen]
impl ThemeColors {
    #[wasm_bindgen(constructor)]
    pub fn new(
        primary: Rgb,
        secondary: Rgb,
        tertiary: Rgb,
        outline: Rgb,
        on_primary_container: Rgb,
        on_secondary_container: Rgb,
        on_tertiary_container: Rgb,
        on_background: Rgb,
    ) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
            outline,
            on_primary_container,
            on_secondary_container,
            on_tertiary_container,
            on_background,
        }
    }
}

impl fmt::Display for ThemeColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(primary:{},secondary:{},tertiary:{},outline:{},on-primary-container:{},on-secondary-container:{},on-tertiary-container:{},on-background:{})",
            self.primary,
            self.secondary,
            self.tertiary,
            self.outline,
            self.on_primary_container,
            self.on_secondary_container,
            self.on_tertiary_container,
            self.on_background
        )
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
// #[derive(Tsify, Serialize, Deserialize)]
// #[tsify(into_wasm_abi, from_wasm_abi)]
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

#[wasm_bindgen]
// #[derive(Tsify, Serialize, Deserialize)]
// #[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TypstState {
    world: MnemoWorld,
    document: Option<PagedDocument>,
    index_mapper: IndexMapper,
    width: String,
    height: String,
    // last_working_edit: String,
    pt: f32,
    size: f32,
    theme: ThemeColors,
}

impl Default for TypstState {
    fn default() -> Self {
        Self {
            world: MnemoWorld::new(),
            document: None,
            index_mapper: IndexMapper::default(),
            width: String::from("auto"),
            height: String::from("auto"),
            pt: 0_f32,
            size: 0_f32,
            theme: ThemeColors::new(
                Rgb(0, 0, 0),
                Rgb(0, 0, 0),
                Rgb(0, 0, 0),
                Rgb(0, 0, 0),
                Rgb(0, 0, 0),
                Rgb(0, 0, 0),
                Rgb(0, 0, 0),
                Rgb(0, 0, 0),
            ),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompileResult {
    renders: Vec<RangedRender>,
    diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderPdfResult {
    pub bytes: Option<Vec<u8>>,
    pub diagnostics: Vec<TypstDiagnostic>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(js_name = "FileId")]
pub struct FileIdWrapper(FileId);

impl FileIdWrapper {
    fn new(id: FileId) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> FileId {
        self.0
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
    pub fn insert_file(&mut self, path: String, text: String) -> FileIdWrapper {
        let id = FileId::new(None, VirtualPath::new(&path));
        self.world.insert_file(id, text);

        FileIdWrapper::new(id)
    }

    #[wasm_bindgen(js_name = installPackage)]
    pub fn install_package(
        &mut self,
        spec: &str,
        files: Vec<PackageFile>,
    ) -> Result<(), TypstError> {
        let package_spec = Some(PackageSpec::from_str(spec).map_err(|err| TypstError(err))?);

        for file in files {
            let id = FileId::new(package_spec.clone(), VirtualPath::new(&file.path));
            let source = Source::new(id, String::from_utf8(file.content).unwrap());

            self.world.files.insert(id, source);
        }

        Ok(())
    }

    fn prelude(&self, config_page: bool) -> String {
        let page_config = if config_page {
            format!(
                "#set page(fill:rgb(0,0,0,0),width:{width},height:{height},margin:0pt)",
                width = self.width,
                height = self.height,
            )
        } else {
            format!("")
        };

        format!(
            r#"
                #let theme={theme}
                #set text(fill:theme.on-background,size:{size}pt,top-edge:"ascender",bottom-edge:"descender")
                #set align(horizon)
                #set par(leading:0em,linebreaks:"simple")
                {page_config}
                #context {{show math.equation:set text(size:text.size*1.75)}}
                #show math.equation.where(block:true):set block(above:0em,below:0em)
                #show math.equation.where(block:true):set text(size:{size}pt*1.125)
                #show math.equation.where(block:true):set par(leading:{size}pt*0.5625)

                #set table(stroke:theme.outline,inset:10pt)

                #show heading:set block(above:0em,below:0em)
                #show heading.where(level:1):set text(fill:theme.on-primary-container,size:32pt,weight:400)
                #show heading.where(level:2):set text(fill:theme.on-secondary-container,size:28pt,weight:400)
                #show heading.where(level:3):set text(fill:theme.on-primary-container,size:24pt,weight:400)
                #show heading.where(level:4):set text(fill:theme.primary,size:22pt,weight:400)
                #show heading.where(level:5):set text(fill:theme.secondary,size:16pt,weight:500)
                #show heading.where(level:6):set text(fill:theme.tertiary,size:14pt,weight:500)

                #show list:set block(above:0em,below:0em)
                #show enum:set block(above:0em,below:0em)
            "#,
            theme = self.theme,
            size = self.size,
        )
    }

    #[wasm_bindgen]
    pub fn compile(&mut self, id: &FileIdWrapper, text: String, prelude: &str) -> CompileResult {
        let mut ir = self.prelude(true) + prelude;

        self.index_mapper = IndexMapper::default();
        self.index_mapper.map_index(0, ir.len());

        let mut block_ranges = Vec::<RangedBlock>::new();
        let mut in_block = false;

        let aux_id = id.inner().join("$");
        self.world.insert_file(aux_id, text);
        self.world.aux = Some(aux_id);

        let children = self.world.aux_source().root().children();
        let text = self.world.aux_source().text();

        let mut last_kind: Option<SyntaxKind> = None;

        for node in children {
            let range = self.world.range(node.span()).unwrap();

            if let Some(until_newline) = node.text().chars().position(|ch| ch == '\n') {
                in_block = false;

                if let Some(last_block) = block_ranges.last_mut() {
                    last_block.range.end += until_newline;

                    ir += &text[last_block.range.clone()];

                    match last_kind {
                        Some(kind) if kind.is_stmt() => {}
                        _ => {
                            ir += "\n#box() \\";
                            last_block.is_expr = true
                        }
                    }

                    // crate::log(&format!("[LAST_KIND]: {last_kind:?}"));

                    ir += "\n";
                }
            } else {
                last_kind = Some(node.kind());

                if in_block {
                    block_ranges.last_mut().unwrap().range.end = range.end;
                } else {
                    in_block = true;

                    self.index_mapper.map_index(range.start, ir.len());

                    block_ranges.push(RangedBlock {
                        range,
                        is_expr: false,
                    });
                }
            }
        }

        if let Some(last_block) = block_ranges.last_mut() {
            if in_block {
                ir += &text[last_block.range.clone()];
            }
        }

        // crate::log(&format!("[RANGES]: {block_ranges:?}"));

        // crate::log(&format!(
        //     "[SOURCE]: {:?}",
        //     &ir[(self.prelude(true) + prelude).len()..]
        // ));

        self.world.main = Some(id.inner());

        let mut temp_ir = Cow::from(&ir);

        // TODO: exclude possible prelude height?
        let mut offset_height = 0_f64;
        let mut diagnostics = Vec::new();

        let renders = block_ranges
            .into_iter()
            .filter_map(|block| {
                let aux_range = block.range;

                let aux_source = self.world.aux_source();
                let start_byte_diff =
                    aux_range.start - aux_source.byte_to_utf16(aux_range.start).unwrap();
                let end_byte_diff =
                    aux_range.end - aux_source.byte_to_utf16(aux_range.start).unwrap();

                let mut end_byte = self.index_mapper.aux_to_main(aux_range.end);
                if block.is_expr {
                    // TODO: proper offsetting
                    end_byte += 9;
                }

                let source = self.world.main_source_mut();
                source.replace(&temp_ir.get(..end_byte)?);

                // crate::log(&format!("[SOURCE]: {:?}", partial_ir.get(start_byte..end_byte)));

                let start_utf16 = aux_range.start - start_byte_diff;
                let end_utf16 = aux_range.end - end_byte_diff;
                let range_utf16 = start_utf16..end_utf16;

                // crate::log(&format!("[RANGE_UTF8]: {:?}", aux_range));
                // crate::log(&format!("[RANGE_UTF16]: {range_utf16:?}"));

                let compiled = compile::<PagedDocument>(&self.world);
                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    compiled.warnings,
                    &self.index_mapper,
                    &self.world,
                ));

                match compiled.output {
                    Ok(document) => {
                        // TODO: handle changes in page margins

                        let page_height = document
                            .pages
                            .iter()
                            .map(|page| page.frame.height())
                            .sum::<Abs>()
                            .to_pt();
                        let height = page_height - offset_height;

                        // crate::log(&format!("[PAGE_HEIGHT]: {page_height}"));
                        // crate::log(&format!("[OFFSET_HEIGHT]: {offset_height}"));
                        // crate::log(&format!("[HEIGHT]: {height}"));

                        if height <= 0_f64 {
                            return None;
                        }

                        let canvas = mnemo_render::render(&document, offset_height, self.pt);
                        let render = BASE64
                            .encode(&canvas.encode_png().unwrap())
                            .into_boxed_str();

                        offset_height = page_height;

                        self.document = Some(document);

                        Some(RangedRender::new(range_utf16, RenderedFrame {
                            render,
                            height,
                        }))
                    }
                    Err(source_diagnostics) => {
                        diagnostics.extend(TypstDiagnostic::from_diagnostics(
                            source_diagnostics,
                            &self.index_mapper,
                            &self.world,
                        ));

                        // crate::error(&format!("[ERRORS]: {diagnostics:?}"));

                        let start_range = self.index_mapper.aux_to_main(aux_range.start);

                        temp_ir.to_mut().replace_range(
                            start_range..end_byte,
                            &(" ".repeat(end_byte - start_range - 1) + "\\"),
                        );

                        None
                    }
                }
            })
            .collect();

        self.world.main_source_mut().replace(&ir);

        CompileResult {
            renders,
            diagnostics,
        }
    }

    #[wasm_bindgen(js_name = renderPdf)]
    pub fn render_pdf(&mut self, id: &FileIdWrapper) -> RenderPdfResult {
        self.world.main = Some(id.inner());

        let mut ir = self.prelude(false);
        let main_source = self.world.main_source_mut();
        ir += main_source.text();
        main_source.replace(&ir);

        let compiled = compile(&self.world);
        let mut diagnostics =
            TypstDiagnostic::from_diagnostics(compiled.warnings, &self.index_mapper, &self.world)
                .into_vec();

        let bytes = match compiled.output {
            Ok(document) => {
                match pdf(&document, &PdfOptions::default()) {
                    Ok(pdf) => Some(pdf),
                    Err(source_diagnostics) => {
                        diagnostics.extend(TypstDiagnostic::from_diagnostics(
                            source_diagnostics,
                            &self.index_mapper,
                            &self.world,
                        ));

                        None
                    }
                }
            }
            Err(source_diagnostics) => {
                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    &self.index_mapper,
                    &self.world,
                ));

                None
            }
        };

        RenderPdfResult { bytes, diagnostics }
    }

    #[wasm_bindgen]
    pub fn click(&mut self, index: usize, x: f64, y: f64) -> Option<TypstJump> {
        let document = self.document.as_ref().unwrap();

        typst_ide::jump_from_click(
            &self.world,
            document,
            &document.pages[index].frame,
            Point::new(Abs::raw(x), Abs::raw(y)),
        )
        .map(|jump| TypstJump::from_mapped(jump, &self.index_mapper))
    }

    #[wasm_bindgen]
    pub fn autocomplete(&self, cursor: usize, explicit: bool) -> Autocomplete {
        let main_source = self.world.main_source();
        let aux_source = self.world.aux_source();

        let byte_diff = aux_source.utf16_to_byte(cursor).unwrap() - cursor;
        let cursor = self.index_mapper.aux_to_main(cursor + byte_diff);

        let results = typst_ide::autocomplete(
            &self.world,
            self.document.as_ref(),
            main_source,
            cursor,
            explicit,
        );

        match results {
            Some((offset, completions)) => {
                Autocomplete {
                    offset: self.index_mapper.main_to_aux(offset) - byte_diff,
                    completions: completions
                        .into_iter()
                        .map(TypstCompletion::from)
                        .collect::<Box<[_]>>(),
                }
            }
            None => {
                Autocomplete {
                    offset: 0,
                    completions: Box::new([]),
                }
            }
        }
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
}

#[derive(Debug)]
struct RangedBlock {
    range: Range<usize>,
    is_expr: bool,
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
pub struct RangedRender {
    pub range: Range<usize>,
    pub render: RenderedFrame,
}

impl RangedRender {
    pub fn new(range: Range<usize>, render: RenderedFrame) -> Self {
        Self { range, render }
    }
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RenderedFrame {
    render: Box<str>,
    height: f64,
}
