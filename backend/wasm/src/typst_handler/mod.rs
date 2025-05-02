mod fonts;
mod index_mapper;
mod world;
mod wrappers;

use core::fmt;
use std::{
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
// use typst_pdf::{PdfOptions, PdfStandard, pdf};
// use typst_html::html;
use typst_render::{render, render_merged};
use wasm_bindgen::prelude::*;
use world::MnemoWorld;
use wrappers::{TypstCompletion, TypstDiagnostic, TypstError, TypstJump};

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

// #[derive(Tsify, Serialize, Deserialize)]
// #[tsify(into_wasm_abi, from_wasm_abi)]
// #[serde(tag = "kind", rename_all = "camelCase", content = "data")]
// pub enum SyncResult {
//     Ok(Box<[RangedRender]>),
//     Error(Box<[String]>),
// }

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
    pub fn install_package(&mut self, spec: &str, files: Vec<PackageFile>) {
        let package_spec = Some(PackageSpec::from_str(spec).unwrap());

        for file in files {
            let id = FileId::new(package_spec.clone(), VirtualPath::new(&file.path));
            let source = Source::new(id, String::from_utf8(file.content).unwrap());

            self.world.files.insert(id, source);
        }
    }

    fn prelude(&self, config_page: bool) -> String {
        let page_config = if config_page {
            format!(
                "#set page(fill:rgb(0,0,0,0),width:{width},height:{height},margin:1pt)",
                width = self.width,
                height = self.height,
            )
        } else {
            format!("")
        };

        format!(
            r#"
                #let theme={theme}
                #set text(fill:theme.on-background,size:{size}pt,tracking:0pt,top-edge:"ascender",bottom-edge:"descender",overhang:false)
                #set align(horizon)
                #set par(leading:0em,linebreaks:"simple")
                {page_config}
                #context {{show math.equation:set text(size:text.size*1.75)}}
                #show math.equation.where(block:true):set text(size:{size}pt*1.125)
                #show math.equation.where(block:true):set par(leading:{size}pt*0.5625)

                #set table(stroke:theme.outline,inset:10pt)

                #show heading.where(level:1):set text(fill:theme.on-primary-container,size:32pt,tracking:0pt,weight:400)
                #show heading.where(level:2):set text(fill:theme.on-secondary-container,size:28pt,tracking:0pt,weight:400)
                #show heading.where(level:3):set text(fill:theme.on-primary-container,size:24pt,tracking:0pt,weight:400)
                #show heading.where(level:4):set text(fill:theme.primary,size:22pt,tracking:0pt,weight:400)
                #show heading.where(level:5):set text(fill:theme.secondary,size:16pt,tracking:0.15pt,weight:500)
                #show heading.where(level:6):set text(fill:theme.tertiary,size:14pt,tracking:0.1pt,weight:500)
            "#,
            theme = self.theme,
            size = self.size + 1.0,
        )
    }

    #[wasm_bindgen]
    pub fn sync(&mut self, id: &FileIdWrapper, text: &str, prelude: &str) -> Box<[RangedRender]> {
        let mut source = self.prelude(true) + prelude;

        self.index_mapper = IndexMapper::default();
        self.index_mapper.add_change(0, source.len());

        let mut blocks = Vec::<Block>::new();
        let mut in_block = false;

        let mut last_start_byte_offset = 0;
        let mut last_end_byte_offset = 0;

        self.world.main = Some(id.inner());
        self.world.main_source_mut().replace(text);

        let children = self.world.main_source().root().children();
        let mut errors = Vec::new();

        for node in children {
            let mut range = self.world.range(node.span()).unwrap();

            let text_slice = &text[..range.end];
            let utf16_count = text_slice.encode_utf16().count();

            let byte_offset = text_slice.len() - utf16_count;

            range.start -= last_end_byte_offset;
            range.end -= byte_offset;

            if let Some(last_block) = blocks.last() {
                self.index_mapper
                    .add_change(last_block.range.start, source.len());
            }

            if let Some(ast::Expr::FuncCall(call)) = node.cast() {
                if let ast::Expr::Ident(ident) = call.callee() {
                    if ident.eq_ignore_ascii_case("pagebreak") {
                        in_block = false;
                        source += "[]";

                        if let Some(block) = blocks.last_mut() {
                            block.range.end = range.end;
                        }

                        continue;
                    }
                }
            }

            if let Some(until_newline) = node.text().encode_utf16().position(|ch| ch == '\n' as u16)
            {
                in_block = false;

                if let Some(last_range) = blocks.last_mut() {
                    last_range.range.end += until_newline;

                    let mut range = last_range.range.clone();
                    range.start += last_start_byte_offset;
                    range.end += last_end_byte_offset;

                    source += &text[range];
                    source += "\n#pagebreak()\n";
                }
            } else if node.erroneous() {
                in_block = false;

                let end_range = range.end;

                blocks.push(Block::new(
                    range,
                    source.encode_utf16().count(),
                    TypstDiagnostic::from_errors(node.errors(), &self.world),
                ));

                source += "\n#pagebreak()\n";
            } else if in_block {
                blocks.last_mut().unwrap().range.end = range.end;
            } else {
                in_block = true;

                blocks.push(Block::new(
                    range,
                    source.encode_utf16().count(),
                    Box::new([]),
                ));
            }

            last_start_byte_offset = last_end_byte_offset;
            last_end_byte_offset = byte_offset;
        }

        if let Some(last_range) = blocks.last_mut() {
            if in_block {
                let mut range = last_range.range.clone();

                range.start += last_start_byte_offset;
                range.end += last_end_byte_offset;

                self.index_mapper.add_change(range.start, source.len());

                source += &text[range];
            }
        }
        // else {
        //     // return serde_wasm_bindgen::to_value::<[(); 0]>(&[]);
        //     // return SyncResult::Ok(Box::new([]));
        //     return SyncResult::Ok(Box::new([
        //         RangedRender {
        //             index: 0,
        //             block: Block::new(0..utf16_count, 0, diagnostics.drain(..).collect::<Box<[_]>>(),),
        //             render: None,
        //         }
        //     ]));
        // }

        crate::log(&format!(
            "[SOURCE]: {}",
            &source[(self.prelude(true) + prelude).len()..]
        ));

        self.world.main_source_mut().replace(&source);

        crate::log(&format!(
            "[LAST_START_BYTE_OFFSET]: {last_start_byte_offset:?}"
        ));
        crate::log(&format!("[LAST_END_BYTE_OFFSET]: {last_end_byte_offset:?}"));

        let compiled = compile::<PagedDocument>(&self.world);
        errors.extend(TypstDiagnostic::from_diagnostics(
            compiled.warnings,
            &self.index_mapper,
            &self.world,
        ));
        match compiled.output {
            Ok(document) => {
                if blocks.is_empty() {
                    self.document = Some(document);

                    Box::new([RangedRender {
                        index: 0,
                        block: Block::new(
                            0..text.encode_utf16().count(),
                            0,
                            errors.into_boxed_slice(),
                        ),
                        render: None,
                    }])
                } else {
                    let pages = &document.pages;

                    let blocks = iter::zip(blocks, pages.iter().cloned())
                        .enumerate()
                        .map(|(index, (block, page))| {
                            let not_empty = page
                                .frame
                                .items()
                                .filter(|(_point, item)| !matches!(item, FrameItem::Tag(..)))
                                .count()
                                > 0;

                            // not_empty.then(|| RangedRender::new(index, block, encode_frame(page, pt)))
                            RangedRender::new(
                                index,
                                block,
                                not_empty.then(|| encode_frame(page, self.pt)),
                            )
                        })
                        .collect::<Box<[_]>>();

                    self.document = Some(document);

                    blocks
                }
            }
            Err(diagnostics) => {
                crate::error(&format!("{errors:?}"));

                crate::log(&format!("[ERRORS]: {errors:?}"));
                crate::log(&format!("[DIAGNOSTICS]: {diagnostics:?}"));

                errors.extend(TypstDiagnostic::from_diagnostics(
                    diagnostics,
                    &self.index_mapper,
                    &self.world,
                ));

                Box::new([RangedRender {
                    index: 0,
                    block: Block::new(0..text.encode_utf16().count(), 0, errors.into_boxed_slice()),
                    render: None,
                }])
            }
        }
    }

    // #[wasm_bindgen(js_name = renderSvg)]
    // pub fn render_svg(&mut self, id: &FileIdWrapper) -> String {
    //     self.world.main = Some(id.inner());

    //     let mut source = self.prelude();
    //     source += self.world.main_source().text();

    //     self.world.main_source_mut().replace(&source);

    //     let compiled = compile(&self.world);
    //     match compiled.output {
    //         Ok(document) => svg_merged(&document, Abs::zero()),
    //         Err(..) => String::new(),
    //     }
    // }

    // #[wasm_bindgen(js_name = renderHtml)]
    // pub fn render_html(&mut self, id: &FileIdWrapper) -> String {
    //     self.world.main = Some(id.inner());

    //     let mut source = self.prelude(false);
    //     source += self.world.main_source().text();

    //     self.world.main_source_mut().replace(&source);

    //     let compiled = compile(&self.world);
    //     // match compiled.output {
    //     //     Ok(document) => html(&document).unwrap(),
    //     //     Err(..) => String::from("error"),
    //     // }
    //     html(&compiled.output.unwrap()).unwrap()
    // }

    #[wasm_bindgen]
    pub fn click(&mut self, index: usize, x: f64, y: f64) -> Option<TypstJump> {
        let document = self.document.as_ref().unwrap();

        typst_ide::jump_from_click(
            &self.world,
            document,
            &document.pages[index].frame,
            Point::new(Abs::raw(x), Abs::raw(y)),
        )
        .map(TypstJump::from)
    }

    #[wasm_bindgen]
    pub fn autocomplete(&self, cursor: usize, explicit: bool) -> Result<JsValue, Error> {
        // let compiled = compile(&self.world);
        // let document = match compiled.output {
        //     Ok(document) => document,
        //     Err(..) => return serde_wasm_bindgen::to_value(&(0, Vec::<TypstCompletion>::new())),
        // };
        let source = self.world.main_source();

        let results = typst_ide::autocomplete(
            &self.world,
            self.document.as_ref(),
            source,
            self.index_mapper.map_index(cursor),
            explicit,
        );

        serde_wasm_bindgen::to_value(&match results {
            Some((offset, completions)) => {
                (
                    self.index_mapper.map_offset(offset),
                    completions
                        .into_iter()
                        .map(TypstCompletion::from)
                        .collect::<Vec<TypstCompletion>>(),
                )
            }
            None => (0_usize, Vec::<TypstCompletion>::new()),
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
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RangedRender {
    pub index: usize,
    pub block: Block,
    pub render: Option<String>,
}

impl RangedRender {
    pub fn new(index: usize, block: Block, render: Option<String>) -> Self {
        Self {
            index,
            block,
            render,
        }
    }
}

// #[wasm_bindgen]
// pub struct RangeUsize {
//     pub start: usize,
//     pub end: usize,
// }

// impl From<Range<usize>> for RangeUsize {
//     fn from(range: Range<usize>) -> Self {
//         Self {
//             start: range.start,
//             end: range.end,
//         }
//     }
// }

#[derive(Tsify, Serialize, Deserialize, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Block {
    pub range: Range<usize>,
    pub offset: usize,
    pub errors: Box<[TypstDiagnostic]>,
}

impl Block {
    pub fn new(range: Range<usize>, offset: usize, errors: Box<[TypstDiagnostic]>) -> Self {
        Self {
            range,
            offset,
            errors,
        }
    }
}

fn encode_frame(frame: Page, pt: f32) -> String {
    let canvas = &render(&frame, pt);

    BASE64.encode(&canvas.encode_png().unwrap())
}
