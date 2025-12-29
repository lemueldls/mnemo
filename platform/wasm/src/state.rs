use std::{
    fmt,
    io::{Cursor, Read},
    path::PathBuf,
    str::FromStr,
};

use ecow::EcoVec;
use indoc::formatdoc;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use tar::Archive;
use tsify::Tsify;
use typst::{
    compile,
    diag::Severity,
    ecow::EcoString,
    foundations::Bytes,
    introspection::HtmlPosition,
    syntax::{FileId, Source, VirtualPath, package::PackageSpec},
};
use typst_html::HtmlDocument;
use typst_ide::Tooltip;
// use typst_html::html;
use typst_pdf::{PdfOptions, pdf};
use typst_syntax::{LinkedNode, Side, Tag};
// use typst_svg::{svg, svg_merged};
use wasm_bindgen::prelude::*;

use crate::{
    index_mapper::IndexMapper,
    renderer::{RenderHtmlResult, RenderPdfResult, html, sync_source_context},
    world::MnemoWorld,
    wrappers::{
        TypstCompletion, TypstDiagnostic, TypstFileId, TypstHighlight, TypstJump, map_main_span,
    },
};

#[wasm_bindgen]
#[derive(Default)]
pub struct TypstState {
    pub(crate) world: MnemoWorld,
    pub(crate) space_context_map: FxHashMap<String, SpaceContext>,
    pub(crate) source_context_map: FxHashMap<TypstFileId, SourceContext>,
}

#[wasm_bindgen]
impl TypstState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn get_source_context(&self, id: &TypstFileId) -> &SourceContext {
        self.source_context_map.get(id).unwrap()
    }

    pub(crate) fn get_source_context_mut(&mut self, id: &TypstFileId) -> &mut SourceContext {
        self.source_context_map.get_mut(id).unwrap()
    }

    pub(crate) fn get_space_context(&self, id: &TypstFileId) -> &SpaceContext {
        let space_id = &self.get_source_context(id).space_id;
        let space_ctx = self.space_context_map.get(space_id).unwrap();

        space_ctx
    }

    pub(crate) fn get_space_context_mut(&mut self, id: &TypstFileId) -> &mut SpaceContext {
        let space_id = self.get_source_context(id).space_id.clone();
        let space_ctx = self.space_context_map.get_mut(&space_id).unwrap();

        space_ctx
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

    #[wasm_bindgen(js_name = "setLocale")]
    pub fn set_locale(&mut self, id: &TypstFileId, locale: String) {
        self.get_space_context_mut(id).locale = locale;
    }

    #[wasm_bindgen(js_name = "createSourceId")]
    pub fn create_source_id(&mut self, path: String, space_id: String) -> TypstFileId {
        let id = FileId::new(None, VirtualPath::new(&path).with_extension("typ"));
        let id_wrapper = TypstFileId::new(id);

        let source_ctx = SourceContext::new(id, space_id.clone());
        self.world.insert_source(source_ctx.aux_id, String::new());
        self.source_context_map
            .insert(id_wrapper.clone(), source_ctx);

        let space_ctx = SpaceContext::new();
        self.space_context_map.insert(space_id, space_ctx);

        id_wrapper
    }

    #[wasm_bindgen(js_name = "createFileId")]
    pub fn create_file_id(&mut self, path: String) -> TypstFileId {
        let id = FileId::new(None, VirtualPath::new(&path));
        let id_wrapper = TypstFileId::new(id);

        id_wrapper
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

    fn process_requests(&self) -> Vec<TypstRequest> {
        let mut requests = Vec::new();

        for source in self.world.requested_sources.iter() {
            requests.push(TypstRequest::Source(source.as_rooted_path().to_path_buf()));
        }
        self.world.requested_sources.clear();

        for file in self.world.requested_files.iter() {
            requests.push(TypstRequest::File(file.as_rooted_path().to_path_buf()));
        }
        self.world.requested_files.clear();

        for package in self.world.requested_packages.iter() {
            requests.push(TypstRequest::Package {
                namespace: package.namespace.to_string(),
                name: package.name.to_string(),
                version: package.version.to_string(),
            });
        }
        self.world.requested_packages.clear();

        requests
    }

    pub(crate) fn prelude(&self, id: &TypstFileId) -> String {
        let source_ctx = self.get_source_context(id);
        let space_ctx = self.space_context_map.get(&source_ctx.space_id).unwrap();

        space_ctx.prelude()
    }

    #[wasm_bindgen]
    pub fn compile(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> CompileResult {
        let result = html::render(id, text, prelude, self);

        CompileResult {
            frames: result.frames,
            diagnostics: result.diagnostics,
            requests: self.process_requests(),
        }
    }

    #[wasm_bindgen]
    pub fn check(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> CheckResult {
        let (ir, _) = sync_source_context(id, text, prelude, self);

        let context = self.source_context_map.get_mut(id).unwrap();
        context
            .main_source_mut(&mut self.world)
            .unwrap()
            .replace(&ir);

        let compiled = compile::<HtmlDocument>(&self.world);
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

                let mut css_class = tag.css_class().to_string();

                match tag {
                    Tag::Heading => {
                        let node = curr.get();

                        let Some(marker_node) = node.children().next() else {
                            unreachable!()
                        };
                        let level = marker_node.text().len();

                        css_class += " typ-heading-level-";
                        css_class += level.to_string().as_str();
                    }
                    _ => {}
                }

                Some(TypstHighlight {
                    tag: css_class,
                    range: aux_range_utf16,
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

    #[wasm_bindgen]
    pub fn click(&mut self, id: &TypstFileId, element: Vec<usize>) -> Option<TypstJump> {
        let context = self.source_context_map.get(id)?;
        let document = context.document.as_ref()?;

        typst_ide::jump_from_click(
            &self.world,
            document,
            &HtmlPosition::new(EcoVec::from(element)),
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
        let context = self.source_context_map.get(id)?;

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
        let context = self.source_context_map.get(id).unwrap();

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
        let context = self.source_context_map.get_mut(id).unwrap();

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

        let mut ir = self.prelude(id);

        let context = self.source_context_map.get_mut(id).unwrap();
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

    #[wasm_bindgen(js_name = renderHtml)]
    pub fn render_html(&mut self, id: &TypstFileId, text: &str, prelude: &str) -> RenderHtmlResult {
        let (ir, ast_blocks) = sync_source_context(id, text, prelude, self);

        let mut diagnostics = Vec::new();
        let mut compiled_warnings = None;

        let context = self.source_context_map.get_mut(id).unwrap();

        context
            .main_source_mut(&mut self.world)
            .unwrap()
            .replace(&ir);

        let mut document = None;

        while document.is_none() {
            let compiled = compile::<HtmlDocument>(&self.world);
            compiled_warnings = Some(compiled.warnings);

            document = match compiled.output {
                Ok(document) => {
                    let html = typst_html::html(&document);

                    match html {
                        Ok(html) => Some(html),
                        Err(source_diagnostics) => {
                            crate::error!("[HTML ERRORS]: {source_diagnostics:?}");

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
                    let error_ranges = source_diagnostics
                        .iter()
                        .filter_map(|diagnostic| {
                            map_main_span(
                                diagnostic.span,
                                diagnostic.severity == Severity::Error,
                                &diagnostic.trace,
                                &context,
                                &self.world,
                            )
                        })
                        .collect::<FxHashSet<_>>();

                    let Some(block) = ast_blocks.iter().find(|block| {
                        let aux_range = &block.range;

                        let main_range_start = context.map_aux_to_main(aux_range.start);
                        let main_range_end = context.map_aux_to_main(aux_range.end);
                        // let main_range = main_range_start..main_range_end;

                        // crate::log!("[BLOCK RANGE]: {main_range_start} - {main_range_end}");

                        error_ranges.iter().any(|error_range| {
                            (main_range_start <= error_range.start
                                && main_range_end >= error_range.start)
                                || (main_range_start <= error_range.end
                                    && main_range_end >= error_range.end)
                        })
                    }) else {
                        break;
                    };

                    let aux_range = &block.range;

                    let mut end_byte = context.map_aux_to_main(aux_range.end);
                    if block.is_inline {
                        end_byte += 12;
                    }

                    diagnostics.extend(TypstDiagnostic::from_diagnostics(
                        source_diagnostics,
                        &context,
                        &self.world,
                    ));

                    crate::error!("[ERRORS]: {diagnostics:?}");

                    let start_byte = context.map_aux_to_main(aux_range.start);

                    let source = context.main_source_mut(&mut self.world).unwrap();
                    source.edit(start_byte..end_byte, &(" ".repeat(end_byte - start_byte)));

                    None
                }
            };
        }

        if let Some(warnings) = compiled_warnings {
            diagnostics.extend(TypstDiagnostic::from_diagnostics(
                warnings,
                &context,
                &self.world,
            ));
        }

        RenderHtmlResult {
            document,
            diagnostics,
        }
    }
}

#[derive(Hash)]
pub struct SpaceContext {
    pub theme: ThemeColors,
    pub locale: String,
    pub font: String,
    pub math_font: Option<String>,
}

impl SpaceContext {
    pub fn new() -> Self {
        Self {
            theme: ThemeColors::default(),
            locale: String::from("en"),
            font: String::from("Maple Mono"),
            math_font: Some(String::from("New Computer Modern Math")),
        }
    }

    #[comemo::memoize]
    pub fn prelude(&self) -> String {
        formatdoc!(
            r#"
                #let theme={theme}
                #set text(fill:theme.on-background,size:16pt,lang:"{locale}",font:"{font}")

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

                #context {{show math.equation:set text(size:text.size*2)}}
            "#,
            theme = self.theme,
            locale = self.locale,
            font = self.font,
            math_font = self.math_font.as_ref().unwrap_or(&self.font),
        )
    }
}

pub struct SourceContext {
    pub main_id: FileId,
    pub aux_id: FileId,
    pub space_id: String,
    pub index_mapper: IndexMapper,
    pub document: Option<HtmlDocument>,
    pub width: String,
    pub height: Option<f64>,
}

impl SourceContext {
    pub fn new(main_id: FileId, space_id: String) -> Self {
        let aux_id: FileId = main_id.with_extension("$.typ");

        Self {
            main_id,
            aux_id,
            space_id,
            index_mapper: IndexMapper::default(),
            document: None,
            width: String::from("auto"),
            height: None,
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

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompileResult {
    pub frames: Vec<html::RangedFrame>,
    pub diagnostics: Vec<TypstDiagnostic>,
    pub requests: Vec<TypstRequest>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CheckResult {
    pub diagnostics: Vec<TypstDiagnostic>,
    pub requests: Vec<TypstRequest>,
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

#[wasm_bindgen]
#[derive(Clone, Copy, Hash, Serialize, Deserialize)]
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
#[derive(Default, Clone, Copy, Hash, Serialize, Deserialize)]
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
