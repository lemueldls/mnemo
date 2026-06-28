use std::{
    io::{Cursor, Read},
    num::NonZeroUsize,
    path::PathBuf,
    str::FromStr,
};

use ecow::EcoVec;
use indoc::formatdoc;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tar::Archive;
use tsify::Tsify;
use typst::{
    compile,
    ecow::EcoString,
    foundations::Bytes,
    introspection::{HtmlPosition, PagedPosition},
    layout::{Abs, Point},
    syntax::{FileId, VirtualPath, package::PackageSpec},
};
use typst_html::{HtmlDocument, HtmlOptions};
use typst_ide::Tooltip;
use typst_layout::PagedDocument;
// use typst_html::html;
// use typst_pdf::{PdfOptions, pdf};
use typst_syntax::{LinkedNode, RootedPath, Side, Source, Tag, VirtualRoot};
use wasm_bindgen::prelude::*;

use crate::{
    bindings::{
        CheckResult, CompileHTMLResult, CompilePagedResult, TypstCompletion, TypstDiagnostic,
        TypstFileId, TypstHighlight, TypstJump,
    },
    renderer::{
        html::{self, RenderHtmlResult},
        paged::svg::render_svgs_by_items,
        recovery::remove_errornous_block,
    },
    source::{RenderTarget, SourceContext, SpaceContext, SynthResult, sync_source_state},
    theme::ThemeColors,
    world::MnemoWorld,
};

/// Global state for Typst rendering and compilation in Mnemo.
///
/// Holds the world, all open source and space contexts, and manages the mapping
/// between user/editor state and Typst's compilation model.
#[wasm_bindgen]
#[derive(Default, Debug)]
pub struct TypstState {
    /// The Typst world, containing all loaded files and fonts.
    pub(crate) world: MnemoWorld,
    /// Mapping from space IDs to their context (fonts, theme, locale).
    pub(crate) space_context_map: FxHashMap<String, SpaceContext>,
    /// Mapping from file IDs to their source context (main/raw sources, index
    /// mapping, etc).
    pub(crate) source_context_map: FxHashMap<TypstFileId, SourceContext>,
}

#[wasm_bindgen]
impl TypstState {
    #[must_use]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(js_name = "setTheme")]
    pub fn set_theme(&mut self, id: &TypstFileId, theme: ThemeColors) {
        self.get_space_context_mut(id).theme = theme;
    }

    #[wasm_bindgen(js_name = "setFont")]
    pub fn set_font(&mut self, id: &TypstFileId, font: String) {
        self.get_space_context_mut(id).font = font;
    }

    #[wasm_bindgen(js_name = "setMathFont")]
    pub fn set_math_font(&mut self, id: &TypstFileId, math_font: Option<String>) {
        self.get_space_context_mut(id).math_font = math_font;
    }

    #[wasm_bindgen(js_name = "setCodeFont")]
    pub fn set_code_font(&mut self, id: &TypstFileId, code_font: Option<String>) {
        self.get_space_context_mut(id).code_font = code_font;
    }

    #[wasm_bindgen(js_name = "setLocale")]
    pub fn set_locale(&mut self, id: &TypstFileId, locale: String) {
        self.get_space_context_mut(id).locale = locale;
    }

    #[wasm_bindgen(js_name = "createSourceId")]
    pub fn create_source_id(&mut self, path: &str, space_id: String) -> TypstFileId {
        let id = FileId::new(RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new(path)
                .expect("Invalid virtual path")
                .with_extension("typ"),
        ));
        let id_wrapper = TypstFileId::new(id);

        let source_ctx = SourceContext::new(id, space_id.clone());
        self.world.insert_source(source_ctx.raw_id, String::new());
        self.source_context_map.insert(id_wrapper, source_ctx);

        let space_ctx = SpaceContext::new();
        self.space_context_map.insert(space_id, space_ctx);

        id_wrapper
    }

    #[wasm_bindgen(js_name = "createFileId")]
    pub fn create_file_id(&mut self, path: &str) -> TypstFileId {
        let id = FileId::new(RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new(path).expect("Invalid virtual path"),
        ));

        TypstFileId::new(id)
    }

    #[wasm_bindgen(js_name = "insertSource")]
    pub fn insert_source(&mut self, id: &TypstFileId, text: String) {
        self.world.insert_source(id.inner(), text);
    }

    #[wasm_bindgen(js_name = "insertFile")]
    pub fn insert_file(&mut self, id: &TypstFileId, bytes: Vec<u8>) {
        self.world.insert_file(id.inner(), Bytes::new(bytes));
    }

    #[wasm_bindgen(js_name = "removeFile")]
    pub fn remove_file(&mut self, id: &TypstFileId) {
        self.source_context_map.remove(id);
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

            let root = match &package_spec {
                Some(spec) => VirtualRoot::Package(spec.clone()),
                None => VirtualRoot::Project,
            };

            let id = FileId::new(RootedPath::new(
                root,
                VirtualPath::new(path.to_str().expect("Invalid virtual path"))
                    .expect("Invalid virtual path"),
            ));

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

    fn process_requests(&self) -> Vec<TypstRequest> {
        let mut requests = Vec::new();

        self.world.requested_sources.retain(|source| {
            requests.push(TypstRequest::Source(PathBuf::from(source.get_with_slash())));
            false
        });

        self.world.requested_files.retain(|file| {
            requests.push(TypstRequest::File(PathBuf::from(file.get_with_slash())));
            false
        });

        self.world.requested_packages.retain(|package| {
            requests.push(TypstRequest::Package {
                namespace: package.namespace.to_string(),
                name: package.name.to_string(),
                version: package.version.to_string(),
            });

            false
        });

        requests
    }

    #[wasm_bindgen(js_name = "compilePaged")]
    pub fn compile_paged(
        &mut self,
        id: &TypstFileId,
        text: &str,
        prelude: &str,
    ) -> CompilePagedResult {
        let result = render_svgs_by_items(id, text, prelude, self);

        CompilePagedResult {
            frames: result.frames,
            tooltips: result.tooltips,
            diagnostics: result.diagnostics,
            requests: self.process_requests(),
        }
    }

    #[wasm_bindgen(js_name = "compileHTML")]
    pub fn compile_html(
        &mut self,
        id: &TypstFileId,
        text: &str,
        prelude: &str,
    ) -> CompileHTMLResult {
        let result = html::render(id, text, prelude, self);

        CompileHTMLResult {
            frames: result.frames,
            diagnostics: result.diagnostics,
            requests: self.process_requests(),
        }
    }

    #[wasm_bindgen(js_name = "checkPaged")]
    pub fn check_paged(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> CheckResult {
        let SynthResult { synth, .. } =
            sync_source_state(id, text, prelude, RenderTarget::Svg, self);

        let context = self.source_context_map.get_mut(id).unwrap();
        context
            .synth_source_mut(&mut self.world)
            .unwrap()
            .replace(&synth);
        context.unstable_synth = synth;

        let compiled = compile::<PagedDocument>(&self.world);
        let compiled_warnings = Some(compiled.warnings);

        let mut diagnostics = Vec::new();

        if let Some(warnings) = compiled_warnings {
            diagnostics.extend(TypstDiagnostic::from_diagnostics(
                warnings,
                context,
                &self.world,
            ));
        }

        match compiled.output {
            Ok(document) => {
                context.paged_document = Some(document);
            }
            Err(source_diagnostics) => {
                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    context,
                    &self.world,
                ));
            }
        }

        CheckResult {
            diagnostics,
            requests: self.process_requests(),
        }
    }

    #[wasm_bindgen(js_name = "checkHTML")]
    pub fn check_html(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> CheckResult {
        let SynthResult { synth, .. } =
            sync_source_state(id, text, prelude, RenderTarget::Html, self);

        let context = self.source_context_map.get_mut(id).unwrap();
        context
            .synth_source_mut(&mut self.world)
            .unwrap()
            .replace(&synth);
        context.unstable_synth = synth;

        let compiled = compile::<HtmlDocument>(&self.world);
        let compiled_warnings = Some(compiled.warnings);

        let mut diagnostics = Vec::new();

        if let Some(warnings) = compiled_warnings {
            diagnostics.extend(TypstDiagnostic::from_diagnostics(
                warnings,
                context,
                &self.world,
            ));
        }

        match compiled.output {
            Ok(document) => {
                context.html_document = Some(document);
            }
            Err(source_diagnostics) => {
                diagnostics.extend(TypstDiagnostic::from_diagnostics(
                    source_diagnostics,
                    context,
                    &self.world,
                ));
            }
        }

        CheckResult {
            diagnostics,
            requests: self.process_requests(),
        }
    }

    #[wasm_bindgen]
    pub fn highlight(&mut self, id: &TypstFileId, text: &str) -> Vec<TypstHighlight> {
        let Some(context) = self.source_context_map.get(id) else {
            return Vec::new();
        };

        let root = typst_syntax::parse(text);
        let Some(raw_source) = context.raw_source_mut(&mut self.world) else {
            return Vec::new();
        };
        raw_source.replace(text);

        let mut queue = vec![LinkedNode::new(&root)];
        let mut highlights = Vec::new();

        let raw_lines = raw_source.lines();

        while let Some(curr) = queue.pop() {
            let tag = typst_syntax::highlight(&curr);
            let range = curr.range();

            let highlight = tag.and_then(|tag| {
                let raw_range_start_utf16 = raw_lines.byte_to_utf16(range.start)?;
                let raw_range_end_utf16 = raw_lines.byte_to_utf16(range.end)?;
                let raw_range_utf16 = raw_range_start_utf16..raw_range_end_utf16;

                let mut css_class = tag.css_class().to_string();

                if tag == Tag::Heading {
                    let node = curr.get();

                    let Some(marker_node) = node.children().next() else {
                        unreachable!()
                    };
                    let level = marker_node.leaf_text().len();

                    css_class += " typ-heading-level-";
                    css_class += level.to_string().as_str();
                }

                Some(TypstHighlight {
                    tag: css_class,
                    range: raw_range_utf16,
                })
            });

            if let Some(highlight) = highlight {
                let idx = highlights
                    .binary_search_by_key(&highlight.range.start, |highlight: &TypstHighlight| {
                        highlight.range.start
                    });

                match idx {
                    Ok(idx) | Err(idx) => highlights.insert(idx, highlight),
                }
            }

            for child in curr.children() {
                queue.push(child);
            }
        }

        highlights
    }

    #[wasm_bindgen(js_name = "jumpPaged")]
    pub fn jump_paged(&mut self, id: &TypstFileId, x: f64, mut y: f64) -> Option<TypstJump> {
        let context = self.source_context_map.get(id)?;
        let document = context.paged_document.as_ref()?;

        let index = document
            .pages()
            .iter()
            .rposition(|page| y >= page.frame.height().to_pt())
            .unwrap_or_default();

        let page_offset = document
            .pages()
            .iter()
            .map(|page| page.frame.height().to_pt())
            .rfind(|height| y >= *height)
            .unwrap_or_default();
        y -= page_offset;

        let position = PagedPosition {
            page: NonZeroUsize::new(index + 1).unwrap(),
            point: Point::new(Abs::pt(x), Abs::pt(y)),
        };

        typst_ide::jump_from_click(&self.world, document, &position)
            .and_then(|jump| TypstJump::from_mapped(jump, context, &self.world))
    }

    #[wasm_bindgen(js_name = "jumpHTML")]
    pub fn jump_html(&mut self, id: &TypstFileId, element: Vec<usize>) -> Option<TypstJump> {
        let context = self.source_context_map.get(id)?;
        let document = context.html_document.as_ref()?;

        typst_ide::jump_from_click(
            &self.world,
            document,
            &HtmlPosition::new(EcoVec::from(element)),
        )
        .and_then(|jump| TypstJump::from_mapped(jump, context, &self.world))
    }

    #[wasm_bindgen]
    pub fn autocomplete(
        &mut self,
        id: &TypstFileId,
        raw_cursor_utf16: usize,
        explicit: bool,
    ) -> Option<Autocomplete> {
        let context = self.source_context_map.get(id)?;

        let raw_source = context.raw_source(&self.world)?;
        // let synth_source = context.synth_source(&self.world)?;
        let synth_source = &Source::new(id.inner(), context.unstable_synth.clone());

        let raw_lines = raw_source.lines();
        let raw_cursor = raw_lines.utf16_to_byte(raw_cursor_utf16)?;
        let synth_cursor = context.map_raw_to_synth_from_left(raw_cursor);

        let text = synth_source.text();
        // let text = context.unstable_synth.as_str();
        crate::log!("{}|{}", &text[..synth_cursor], &text[synth_cursor..]);

        let (synth_offset, completions) = typst_ide::autocomplete(
            &self.world,
            context.paged_document.as_ref(),
            synth_source,
            synth_cursor,
            explicit,
        )?;

        let raw_offset = context.map_synth_to_raw_from_left(synth_offset);
        let raw_offset_utf16 = raw_lines.byte_to_utf16(raw_offset)?;

        Some(Autocomplete {
            offset: raw_offset_utf16,
            completions: completions
                .into_iter()
                .map(TypstCompletion::from)
                .collect::<Box<[_]>>(),
        })
    }

    #[wasm_bindgen]
    pub fn hover(&self, id: &TypstFileId, raw_cursor_utf16: usize, side: i8) -> Option<String> {
        let context = self.source_context_map.get(id).unwrap();

        let synth_source = context.synth_source(&self.world)?;
        let raw_source = context.raw_source(&self.world)?;

        let raw_lines = raw_source.lines();
        let raw_cursor = raw_lines.utf16_to_byte(raw_cursor_utf16)?;
        let synth_cursor = context.map_raw_to_synth_from_right(raw_cursor);

        let side = if side == -1 {
            Side::Before
        } else {
            Side::After
        };

        let tooltip = typst_ide::tooltip(
            &self.world,
            context.paged_document.as_ref(),
            synth_source,
            synth_cursor,
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
        let context = self.source_context_map.get_mut(id).unwrap();

        let width = width
            .map(|width| width.to_string() + "pt")
            .unwrap_or_else(|| String::from("auto"));
        let width_changed = context.width != width;

        context.width = width;
        context.height = height;

        width_changed
    }

    // #[wasm_bindgen(js_name = renderPdf)]
    // pub fn render_pdf(&mut self, id: &TypstFileId) -> RenderPdfResult {
    //     self.world.synth_id = Some(id.inner());

    //     let mut ir = self.prelude(id, RenderTarget::Pdf);

    //     let context = self.source_context_map.get_mut(id).unwrap();
    //     let synth_source = context.synth_source_mut(&mut self.world).unwrap();
    //     let text = synth_source.text().to_string();
    //     ir += &text;

    //     synth_source.replace(&ir);

    //     self.world.insert_source(context.raw_id, text);
    //     self.world.raw_id = Some(context.raw_id);

    //     let compiled = compile(&self.world);
    //     let mut diagnostics =
    //         TypstDiagnostic::from_diagnostics(compiled.warnings, context,
    // &self.world).into_vec();

    //     let bytes = match compiled.output {
    //         Ok(document) => {
    //             match pdf(&document, &PdfOptions::default()) {
    //                 Ok(pdf) => Some(pdf),
    //                 Err(source_diagnostics) => {
    //                     diagnostics.extend(TypstDiagnostic::from_diagnostics(
    //                         source_diagnostics,
    //                         context,
    //                         &self.world,
    //                     ));

    //                     None
    //                 }
    //             }
    //         }
    //         Err(source_diagnostics) => {
    //             diagnostics.extend(TypstDiagnostic::from_diagnostics(
    //                 source_diagnostics,
    //                 context,
    //                 &self.world,
    //             ));

    //             None
    //         }
    //     };

    //     RenderPdfResult { bytes, diagnostics }
    // }

    #[wasm_bindgen(js_name = renderHtml)]
    pub fn render_html(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> RenderHtmlResult {
        let SynthResult { synth, blocks, .. } =
            sync_source_state(id, text, prelude, RenderTarget::Html, self);

        let mut diagnostics = Vec::new();
        let mut compiled_warnings = None;

        let context = self.source_context_map.get_mut(id).unwrap();

        context
            .synth_source_mut(&mut self.world)
            .unwrap()
            .replace(&synth);
        context.unstable_synth = synth;

        let mut document = None;
        let mut convergence = 0_u8;

        while document.is_none() {
            let compiled = compile::<HtmlDocument>(&self.world);
            compiled_warnings = Some(compiled.warnings);

            document = match compiled.output {
                Ok(document) => {
                    let html = typst_html::html(&document, &HtmlOptions::default());

                    match html {
                        Ok(html) => Some(html),
                        Err(source_diagnostics) => {
                            crate::error!("[HTML ERRORS]: {source_diagnostics:?}");

                            diagnostics.extend(TypstDiagnostic::from_diagnostics(
                                source_diagnostics,
                                context,
                                &self.world,
                            ));

                            None
                        }
                    }
                }
                Err(source_diagnostics) => {
                    convergence += 1;
                    if convergence >= 128 {
                        crate::error!("COULD NOT CONVERGE ‼️");

                        break;
                    }

                    diagnostics.extend(TypstDiagnostic::from_diagnostics(
                        source_diagnostics.clone(),
                        context,
                        &self.world,
                    ));

                    crate::error!("[ERRORS]: {diagnostics:?}");

                    let indicies = remove_errornous_block(
                        &blocks,
                        &source_diagnostics,
                        context,
                        &mut self.world,
                    );

                    if indicies.is_empty() {
                        crate::error!("NO ERROR BLOCKS FOUND ‼️");

                        break;
                    }

                    None
                }
            };
        }

        if let Some(warnings) = compiled_warnings {
            diagnostics.extend(TypstDiagnostic::from_diagnostics(
                warnings,
                context,
                &self.world,
            ));
        }

        RenderHtmlResult {
            document,
            diagnostics,
        }
    }
}

impl TypstState {
    pub const fn world(&self) -> &MnemoWorld {
        &self.world
    }

    pub const fn world_mut(&mut self) -> &mut MnemoWorld {
        &mut self.world
    }

    pub fn get_source_context(&self, id: &TypstFileId) -> &SourceContext {
        self.source_context_map.get(id).unwrap()
    }

    pub fn get_source_context_mut(&mut self, id: &TypstFileId) -> &mut SourceContext {
        self.source_context_map.get_mut(id).unwrap()
    }

    pub fn get_space_context(&self, id: &TypstFileId) -> &SpaceContext {
        let space_id = &self.get_source_context(id).space_id;
        self.space_context_map.get(space_id).unwrap()
    }

    pub fn get_space_context_mut(&mut self, id: &TypstFileId) -> &mut SpaceContext {
        let space_id = self.get_source_context(id).space_id.clone();
        self.space_context_map.get_mut(&space_id).unwrap()
    }
}

#[comemo::track]
impl TypstState {
    pub fn prelude(&self, id: &TypstFileId, render_target: RenderTarget) -> String {
        let source_ctx = self.source_context_map.get(id).unwrap();
        let space_ctx = self.space_context_map.get(&source_ctx.space_id).unwrap();

        let page_config = match render_target {
            RenderTarget::Svg => {
                formatdoc!(
                    r#"
                        #set page(fill:rgb(0,0,0,0),width:{width},height:auto,margin:0pt)
                        #set text(top-edge:"ascender",bottom-edge:"descender")
                        #set par(leading:0.125em)
                    "#,
                    width = source_ctx.width,
                )
            }
            RenderTarget::Pdf => {
                formatdoc!(
                    r"
                        #set page(width:{width},height:auto,margin:16pt)
                    ",
                    width = source_ctx.width,
                )
            }
            RenderTarget::Html => formatdoc!(""),
        };

        formatdoc!(
            r#"
                #let theme={theme}
                #set text(fill:theme.on-background,size:{text_size}pt,lang:"{locale}",font:"{font}")

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

                #show math.equation:set text(font:"{math_font}")
                #show math.equation.where(block:true):set text(size:18pt)
                #show math.equation.where(block:true):set par(leading:9pt)

                #show raw:set text(font:"{code_font}")

                #context {{show math.equation:set text(size:text.size*2)}}

                {page_config}
            "#,
            text_size = source_ctx.text_size,
            font = space_ctx.font,
            math_font = space_ctx.math_font.as_ref().unwrap_or(&space_ctx.font),
            code_font = space_ctx.code_font.as_ref().unwrap_or(&space_ctx.font),
            locale = space_ctx.locale,
            theme = space_ctx.theme,
        )
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type", content = "value", rename_all = "kebab-case")]
pub enum TypstRequest {
    Source(PathBuf),
    File(PathBuf),
    Package {
        namespace: String,
        name: String,
        version: String,
    },
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
