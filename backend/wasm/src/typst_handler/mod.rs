mod fonts;
mod world;

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
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Error;
use tsify::Tsify;
use typst::{
    compile,
    layout::{Abs, Frame, FrameItem, Page, Point, Position},
    model::Document,
    syntax::{ast, package::PackageSpec, FileId, Source, Span, SyntaxKind, VirtualPath},
    visualize::Color,
    World, WorldExt,
};
use typst_render::{render, render_merged};
use typst_svg::svg;
use wasm_bindgen::prelude::*;
use world::MnemoWorld;

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
#[derive(Clone, Copy)]
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
            self.primary, self.secondary, self.tertiary, self.outline, self.on_primary_container, self.on_secondary_container, self.on_tertiary_container, self.on_background
        )
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Rgb(u8, u8, u8);

#[wasm_bindgen]
impl Rgb {
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.0, self.1, self.2)
    }
}

#[wasm_bindgen]
pub struct TypstState {
    world: MnemoWorld,
    document: Option<Document>,
    width: String,
    height: String,
    pub pt: f32,
    pub size: f32,
    pub theme: ThemeColors,
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
#[serde(tag = "kind", rename_all = "camelCase", content = "data")]
pub enum SyncResult {
    Ok(Box<[RangedRender]>),
    Error(Box<[String]>),
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

    #[wasm_bindgen]
    pub fn sync(&mut self, id: &FileIdWrapper, text: &str) -> SyncResult {
        let mut source = format!(
            r#"
                #let theme={theme}
                #set align(horizon)
                #set text(fill:theme.on-background,size:{size}pt,tracking:0pt,top-edge:"ascender",bottom-edge:"descender",overhang:false)
                #set par(leading:0em,linebreaks:"simple")
                #set page(fill:rgb(0,0,0,0),width:{width},height:{height},margin:1pt)
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
            width = self.width,
            height = self.height,
        );

        let mut blocks = Vec::<Block>::new();
        let mut in_block = false;

        let mut last_start_byte_offset = 0;
        let mut last_end_byte_offset = 0;

        self.world.main = Some(id.inner());
        self.world.main_source_mut().replace(text);

        let children = self.world.main_source().root().children();
        for node in children {
            let mut range = self.world.range(node.span()).unwrap();

            let text_slice = &text[..range.end];
            let utf16_count = text_slice.encode_utf16().count();

            let byte_offset = text_slice.len() - utf16_count;

            range.start -= last_end_byte_offset;
            range.end -= byte_offset;

            if let Some(ast::Expr::FuncCall(call)) = node.cast() {
                if let ast::Expr::Ident(ident) = call.callee() {
                    if ident.eq_ignore_ascii_case("pagebreak") {
                        in_block = false;
                        source += "[]";

                        blocks.last_mut().unwrap().range.end = range.end;

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
            } else if in_block {
                blocks.last_mut().unwrap().range.end = range.end;
            } else {
                in_block = true;
                blocks.push(Block::new(range, source.encode_utf16().count()));
            }

            match node.kind() {
                _ => {}
            }

            last_start_byte_offset = last_end_byte_offset;
            last_end_byte_offset = byte_offset;
        }

        if let Some(last_range) = blocks.last_mut() {
            if in_block {
                let mut range = last_range.range.clone();

                range.start += last_start_byte_offset;
                range.end += last_end_byte_offset;

                source += &text[range];
            }
        } else {
            // return serde_wasm_bindgen::to_value::<[(); 0]>(&[]);
            return SyncResult::Ok(Box::new([]));
        }

        self.world.main_source_mut().replace(&source);

        let compiled = compile(&self.world);
        match compiled.output {
            Ok(document) => {
                let pages = &document.pages;

                let pt = self.pt;

                // crate::log(&format!(
                //     "{:#?}",
                //     iter::zip(blocks.clone(), pages.iter().cloned()).collect::<Vec<_>>()
                // ));

                let blocks = iter::zip(blocks, pages.iter().cloned())
                    .enumerate()
                    .filter_map(|(index, (block, page))| {
                        let not_empty = page
                            .frame
                            .items()
                            .filter(|(_point, item)| !matches!(item, FrameItem::Tag(..)))
                            .count()
                            > 0;

                        not_empty.then(|| RangedRender::new(index, block, encode_frame(page, pt)))
                    })
                    .collect::<Box<[_]>>();

                self.document = Some(document);

                SyncResult::Ok(blocks)
            }
            Err(errors) => {
                crate::error(&format!("{errors:?}"));

                // serde_wasm_bindgen::to_value::<[(); 0]>(&[])
                SyncResult::Error(errors.into_iter().map(|e| e.message.to_string()).collect())
            }
        }
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
        .map(TypstJump::from)
    }

    #[wasm_bindgen]
    pub fn autocomplete(&self, cursor: usize, explicit: bool) -> Result<JsValue, Error> {
        let compiled = compile(&self.world);
        let document = match compiled.output {
            Ok(document) => document,
            Err(..) => return serde_wasm_bindgen::to_value(&(0, Vec::<TypstCompletion>::new())),
        };
        let source = self.world.main_source();

        let results =
            typst_ide::autocomplete(&self.world, Some(&document), source, cursor, explicit);

        serde_wasm_bindgen::to_value(&match results {
            Some((offset, completions)) => (
                offset,
                completions
                    .into_iter()
                    .map(TypstCompletion::from)
                    .collect::<Vec<TypstCompletion>>(),
            ),
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
    pub render: String,
}

impl RangedRender {
    pub fn new(index: usize, block: Block, render: String) -> Self {
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
}

impl Block {
    pub fn new(range: Range<usize>, offset: usize) -> Self {
        Self { range, offset }
    }
}

fn encode_frame(frame: Page, pt: f32) -> String {
    let canvas = &render(&frame, pt);

    BASE64.encode(&canvas.encode_png().unwrap())
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum TypstJump {
    Source {
        // id: u64,
        position: usize,
    },
    // Url(String),
    // Position(Position),
}

impl From<typst_ide::Jump> for TypstJump {
    fn from(jump: typst_ide::Jump) -> Self {
        match jump {
            typst_ide::Jump::Source(id, position) => {
                // let mut state = DefaultHasher::new();
                // id.hash(&mut state);

                Self::Source {
                    // id: state.finish(),
                    position,
                }
            }
            typst_ide::Jump::Url(..) => todo!(),
            typst_ide::Jump::Position(..) => todo!(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub enum TypstCompletionKind {
    Syntax,
    Function,
    Parameter,
    Constant,
    Symbol,
    Type,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct TypstCompletion {
    kind: TypstCompletionKind,
    label: String,
    apply: Option<String>,
    detail: Option<String>,
}

pub struct TypstCompleteResponse {
    offset: usize,
    completions: Vec<TypstCompletion>,
}

impl From<typst_ide::Completion> for TypstCompletion {
    fn from(value: typst_ide::Completion) -> Self {
        Self {
            kind: match value.kind {
                typst_ide::CompletionKind::Syntax => TypstCompletionKind::Syntax,
                typst_ide::CompletionKind::Func => TypstCompletionKind::Function,
                typst_ide::CompletionKind::Param => TypstCompletionKind::Parameter,
                typst_ide::CompletionKind::Constant => TypstCompletionKind::Constant,
                typst_ide::CompletionKind::Symbol(..) => TypstCompletionKind::Symbol,
                typst_ide::CompletionKind::Type => TypstCompletionKind::Type,
            },
            label: value.label.to_string(),
            apply: value.apply.map(|s| s.to_string()),
            detail: value.detail.map(|s| s.to_string()),
        }
    }
}
