use std::{cmp, fmt, iter, ops::Range, str::FromStr};

use hashbrown::{HashMap, HashSet};
use highway::{HighwayHash, HighwayHasher};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tiny_skia::IntRect;
use tsify::Tsify;
use typst::{
    WorldExt, compile,
    diag::Severity,
    ecow::EcoString,
    foundations::Bytes,
    introspection::Tag,
    layout::{Abs, FrameItem, FrameKind, PagedDocument, Point},
    syntax::{FileId, Source, Span, SyntaxKind, VirtualPath, package::PackageSpec},
};
// use typst_html::html;
use typst_pdf::{PdfOptions, pdf};
// use typst_svg::{svg, svg_merged};
use wasm_bindgen::prelude::*;

use super::{
    index_mapper::IndexMapper,
    world::{FileSlot, MnemoWorld},
    wrappers::{TypstCompletion, TypstDiagnostic, TypstFileId, TypstJump},
};
use crate::typst_handler::wrappers::{map_aux_span, map_main_span};

#[wasm_bindgen]
#[derive(Default)]
pub struct TypstState {
    world: MnemoWorld,
    document: Option<PagedDocument>,
    file_contexts: HashMap<TypstFileId, FileContext>,
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

        self.file_contexts
            .insert(id_wrapper.clone(), FileContext::default());

        id_wrapper
    }

    #[wasm_bindgen(js_name = "insertFile")]
    pub fn insert_file(&mut self, id: &TypstFileId, text: String) {
        self.world.insert_source(id.inner(), text);
    }

    #[wasm_bindgen(js_name = "installPackage")]
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

    #[wasm_bindgen(js_name = "installFont")]
    pub fn install_font(&mut self, bytes: Vec<u8>) {
        self.world.install_font(bytes);
    }

    fn prelude(&self, id: &TypstFileId, rendering_mode: RenderingMode) -> String {
        let context = self.file_contexts.get(id).unwrap();

        let page_config = match rendering_mode {
            RenderingMode::Png => {
                format!(
                    r#"
                        #set page(fill:rgb(0,0,0,0),width:{width},height:auto,margin:0pt)

                        #set text(top-edge:"ascender",bottom-edge:"descender")
                        #set par(leading:0em,linebreaks:"simple")

                        #show math.equation.where(block:true):set block(above:0.25em,below:0.25em)
                        #show heading:set block(above:0.25em,below:0.125em)
                        #show heading:set text(top-edge:"bounds",bottom-edge:"bounds")
                        #show list:set block(above:0.25em,below:0em)
                        #show enum:set block(above:0.25em,below:0em)
                    "#,
                    width = context.width,
                )
            }
            RenderingMode::Pdf => {
                format!(
                    r#"
                        #set page(width:{width},height:auto,margin:16pt)
                    "#,
                    width = context.width,
                )
            } // RenderingMode::Html => format!(""),
        };

        format!(
            r#"
                #let theme={theme}
                #set text(fill:theme.on-background,size:16pt,lang:"{locale}")

                #context {{show math.equation:set text(size:text.size*2)}}

                #show math.equation.where(block:true):set text(size:18pt)
                #show math.equation.where(block:true):set par(leading:9pt)

                #set table(stroke:theme.outline)

                #show heading.where(level:1):set text(fill:theme.primary,size:32pt,weight:400)
                #show heading.where(level:2):set text(fill:theme.secondary,size:28pt,weight:400)
                #show heading.where(level:3):set text(fill:theme.tertiary,size:24pt,weight:400)
                #show heading.where(level:4):set text(fill:theme.primary,size:22pt,weight:400)
                #show heading.where(level:5):set text(fill:theme.secondary,size:16pt,weight:500)
                #show heading.where(level:6):set text(fill:theme.tertiary,size:14pt,weight:500)

                {page_config}
            "#,
            theme = context.theme,
            locale = context.locale,
        )
    }

    #[wasm_bindgen]
    pub fn compile(&mut self, id: &TypstFileId, text: String, prelude: &str) -> CompileResult {
        let mut ir = self.prelude(id, RenderingMode::Png) + prelude + "\n";

        let mut index_mapper = IndexMapper::default();
        index_mapper.add_main_to_aux(0, ir.len());

        self.world.main = Some(id.inner());
        self.world.main_source_mut().replace(&ir);

        let pre_ir = ir.clone();

        let aux_id = id.inner().with_extension("$.typ");
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

        let mut ast_blocks = Vec::<AstBlock>::new();
        let mut in_block = false;

        let mut last_kind: Option<SyntaxKind> = None;

        for node in children {
            let range = self.world.range(node.span()).unwrap();

            if let Some(until_newline) = node.text().chars().position(|ch| ch == '\n') {
                in_block = false;

                if let Some(last_block) = ast_blocks.last_mut() {
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
                    ast_blocks.last_mut().unwrap().range.end = range.end;
                } else {
                    in_block = true;

                    index_mapper.add_main_to_aux(range.start, ir.len());
                    ast_blocks.push(AstBlock {
                        range,
                        is_inline: false,
                    });
                }
            }
        }

        if let Some(last_block) = ast_blocks.last_mut() {
            if in_block {
                ir += &text[last_block.range.clone()];
            }
        }

        // crate::log!("[RANGES]: {block_ranges:?}");

        // crate::log!(
        //     "[SOURCE]: {:?}",
        //     &ir[(self.prelude(id, RenderingMode::Png) + prelude + "\n").len()..]
        // );

        self.world.index_mapper = index_mapper;

        let mut last_document = None;

        let mut diagnostics = Vec::new();
        let mut compiled_warnings = None;

        // let mut erronous_ranges = Vec::new();

        let context = self.file_contexts.get(id).unwrap();

        self.world.main_source_mut().replace(&ir);

        let mut ranged_heights = Vec::new();

        while last_document.is_none() {
            let compiled = compile::<PagedDocument>(&self.world);
            compiled_warnings = Some(compiled.warnings);

            // crate::log!("[DOING A THING]");

            ranged_heights = match compiled.output {
                Ok(document) => {
                    let mut frame_blocks = document
                        .pages
                        .iter()
                        .flat_map(|page| {
                            // crate::log!(
                            //     "[ITEMS]: {:?}",
                            //     page.frame
                            //         .items()
                            //         .map(|(_point, item)| {
                            //             match &item {
                            //                 FrameItem::Group(..) => {
                            //                     format!("[GROUP]: {:?}", &item)
                            //                 }
                            //                 FrameItem::Text(..) => {
                            //                     format!("[TEXT]: {:?}", &item)
                            //                 }
                            //                 FrameItem::Shape(..) => {
                            //                     format!("[SHAPE]: {:?}", &item)
                            //                 }
                            //                 FrameItem::Image(..) => {
                            //                     format!("[IMAGE]: {:?}", &item)
                            //                 }
                            //                 FrameItem::Link(..) => {
                            //                     format!("[LINK]: {:?}", &item)
                            //                 }
                            //                 FrameItem::Tag(..) => {
                            //                     format!("[TAG]: {:?}", &item)
                            //                 }
                            //             }
                            //         })
                            //         .collect_vec()
                            // );

                            page.frame.items().flat_map(|frame_item| {
                                fn frame_item_range(
                                    item: &FrameItem,
                                    world: &MnemoWorld,
                                ) -> Option<Range<usize>> {
                                    let span = match item {
                                        // FrameItem::Group(group) => {
                                        //     let items = group.frame.items();

                                        //     let ranges = items.filter_map(|(_point, item)| {
                                        //         frame_item_range(item, &world)
                                        //     });

                                        //     let (starts, ends): (Vec<usize>, Vec<usize>) = ranges
                                        //         .map(|range| (range.start, range.end))
                                        //         .unzip();
                                        //     let start =
                                        //         starts.into_iter().min().unwrap_or_default();
                                        //     let end = ends.into_iter().max().unwrap_or_default();

                                        //     return Some(start..end);
                                        // }
                                        FrameItem::Group(..) => unreachable!(),
                                        FrameItem::Text(text) => {
                                            let first_glyph_span = text.glyphs.first()?.span.0;
                                            let first_glyph_range =
                                                world.range(first_glyph_span)?;

                                            let last_glyph_span = text.glyphs.last()?.span.0;
                                            let last_glyph_range = world.range(last_glyph_span)?;

                                            return Some(
                                                first_glyph_range.start..last_glyph_range.end,
                                            );
                                        }
                                        FrameItem::Shape(_shape, span) => *span,
                                        FrameItem::Image(_image, _axes, span) => *span,
                                        FrameItem::Link(_destination, _axes) => Span::detached(),
                                        FrameItem::Tag(tag) => {
                                            match tag {
                                                Tag::Start(content) => content.span(),
                                                Tag::End(_location, _key) => Span::detached(),
                                            }
                                        }
                                    };

                                    if world.main == span.id() {
                                        world.range(span)
                                    } else {
                                        None
                                    }
                                }

                                fn frame_with_bounds(
                                    frame_item: &(Point, FrameItem),
                                    world: &MnemoWorld,
                                ) -> Box<[FrameBlock]> {
                                    let (point, item) = frame_item;

                                    let (start_height, end_height) = match &item {
                                        FrameItem::Text(text) => {
                                            (cmp::max(point.y - text.size, Abs::zero()), point.y)
                                        }
                                        FrameItem::Group(group) => {
                                            return group
                                                .frame
                                                .items()
                                                .flat_map(|frame_item| {
                                                    frame_with_bounds(frame_item, &world)
                                                        .into_iter()
                                                        .map(|mut frame_block| {
                                                            frame_block.start_height += point.y;
                                                            frame_block.end_height += point.y;

                                                            frame_block
                                                        })
                                                })
                                                .collect::<Box<[_]>>();
                                        }
                                        FrameItem::Shape(shape, _span) => {
                                            (point.y, shape.geometry.bbox_size().y)
                                        }
                                        FrameItem::Image(_image, axes, _span) => (point.y, axes.y),
                                        FrameItem::Link(..) => (point.y, point.y),
                                        FrameItem::Tag(..) => (point.y, point.y),
                                    };

                                    let range = frame_item_range(&item, world);

                                    crate::log!("[F RANGE] {range:?}");

                                    match &item {
                                        FrameItem::Group(..) => {
                                            crate::log!("[F GROUP]: {:?}", &item)
                                        }
                                        FrameItem::Text(..) => {
                                            crate::log!("[F TEXT]: {:?}", &item)
                                        }
                                        FrameItem::Shape(..) => {
                                            crate::log!("[F SHAPE]: {:?}", &item)
                                        }
                                        FrameItem::Image(..) => {
                                            crate::log!("[F IMAGE]: {:?}", &item)
                                        }
                                        FrameItem::Link(..) => {
                                            crate::log!("[F LINK]: {:?}", &item)
                                        }
                                        FrameItem::Tag(..) => {
                                            crate::log!("[F TAG]: {:?}", &item)
                                        }
                                    }

                                    Box::from_iter(iter::once(FrameBlock {
                                        range,
                                        start_height,
                                        end_height,
                                        item: item.clone(),
                                    }))
                                }

                                frame_with_bounds(frame_item, &self.world)
                            })
                        })
                        .peekable();

                    let mut ranged_items = Vec::with_capacity(ast_blocks.len());

                    let mut ast_blocks = ast_blocks.iter().peekable();

                    while let Some(ast_block) = ast_blocks.next() {
                        // let mut index = 0;

                        let aux_source = self.world.aux_source();

                        let aux_range = &ast_block.range;
                        let aux_lines = aux_source.lines();
                        let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start).unwrap();
                        let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end).unwrap();
                        let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                        let main_range_start = self.world.map_aux_to_main(aux_range.start);
                        let main_range_end = self.world.map_aux_to_main(aux_range.end);
                        let main_range = main_range_start..main_range_end;

                        let mut items = Vec::new();
                        let mut deferred_items = Vec::<FrameBlock>::new();

                        let mut block_start_height = None;
                        let mut block_end_height = None;

                        while let Some(frame_block) = frame_blocks.peek() {
                            // index += 1;

                            if let Some(range) = &frame_block.range {
                                if range.end <= main_range_end + 1 {
                                    let frame_block = frame_blocks.next().unwrap();

                                    match block_start_height {
                                        Some(height) if height < frame_block.start_height => {}
                                        _ => block_start_height = Some(frame_block.start_height),
                                    }

                                    match block_end_height {
                                        Some(height) if height > frame_block.end_height => {}
                                        _ => block_end_height = Some(frame_block.end_height),
                                    }

                                    items.push(frame_block);
                                } else {
                                    break;
                                }
                            } else {
                                let frame_block = frame_blocks.next().unwrap();
                                deferred_items.push(frame_block);
                            }
                        }

                        let block_start_height = block_start_height.unwrap_or_default().to_pt();
                        let block_end_height = block_end_height.unwrap_or_default().to_pt();

                        // let items = frame_blocks
                        //     .peeking_take_while(|frame_block| {
                        //         crate::log!("[{index} ITEM_RANGE]: {:?}", &frame_block.range);
                        //         crate::log!("[{index} BLOCK_RANGE]: {:?}", &main_range);

                        //         index += 1;

                        //         if let Some(range) = &frame_block.range {
                        //             range.end <= main_range_end + 1
                        //             // range.start < main_range_end
                        //         } else {
                        //             true
                        //         }
                        //     })
                        //     .collect_vec();

                        for (i, frame_block) in items.iter().enumerate() {
                            crate::group!("{i} [ITEMS]");

                            crate::log!("[BLOCK_RANGE]: {:?}", frame_block.range);
                            crate::log!("[BLOCK_START]: {:?}", frame_block.start_height);
                            crate::log!("[BLOCK_END]: {:?}", frame_block.end_height);

                            match &frame_block.item {
                                FrameItem::Group(..) => {
                                    crate::log!("[GROUP]: {:?}", &frame_block.item)
                                }
                                FrameItem::Text(..) => {
                                    crate::log!("[TEXT]: {:?}", &frame_block.item)
                                }
                                FrameItem::Shape(..) => {
                                    crate::log!("[SHAPE]: {:?}", &frame_block.item)
                                }
                                FrameItem::Image(..) => {
                                    crate::log!("[IMAGE]: {:?}", &frame_block.item)
                                }
                                FrameItem::Link(..) => {
                                    crate::log!("[LINK]: {:?}", &frame_block.item)
                                }
                                FrameItem::Tag(..) => {
                                    crate::log!("[TAG]: {:?}", &frame_block.item)
                                }
                            }

                            crate::group_end!("{i} [ITEMS]");
                        }

                        // crate::log!(
                        //     "[1 ITEMS]: {:?}",
                        //     items
                        //         .iter()
                        //         .map(|frame_block| {
                        //             match &frame_block.item {
                        //                 FrameItem::Group(..) => {
                        //                     format!("[GROUP]: {:?}", &frame_block.item)
                        //                 }
                        //                 FrameItem::Text(..) => {
                        //                     format!("[TEXT]: {:?}", &frame_block.item)
                        //                 }
                        //                 FrameItem::Shape(..) => {
                        //                     format!("[SHAPE]: {:?}", &frame_block.item)
                        //                 }
                        //                 FrameItem::Image(..) => {
                        //                     format!("[IMAGE]: {:?}", &frame_block.item)
                        //                 }
                        //                 FrameItem::Link(..) => {
                        //                     format!("[LINK]: {:?}", &frame_block.item)
                        //                 }
                        //                 FrameItem::Tag(..) => {
                        //                     format!("[TAG]: {:?}", &frame_block.item)
                        //                 }
                        //             }
                        //         })
                        //         .collect_vec()
                        // );

                        // let block_starts = items
                        //     .iter()
                        //     // .filter(|frame_block| frame_block.range.is_some())
                        //     .map(|frame_block| frame_block.start_height);
                        // // crate::log!("[BLOCK_STARTS]: {:?}", block_starts.clone().collect_vec());
                        // let block_start_height = block_starts.min().unwrap_or_default().to_pt();
                        // let block_ends = items
                        //     .iter()
                        //     // .filter(|frame_block| frame_block.range.is_some())
                        //     .map(|frame_block| frame_block.end_height);
                        // // crate::log!("[BLOCK_ENDS]: {:?}", block_ends.clone().collect_vec());
                        // let block_end_height = block_ends.max().unwrap_or_default().to_pt();

                        match context.height {
                            Some(height) if block_start_height >= height => {
                                continue;
                            }
                            _ => {}
                        }

                        // let previous_height = ranged_items.last().unwrap();
                        // let delta = (previous_height - block_end_height - 1.0).max(0.0);

                        let height = block_end_height - block_start_height;

                        if height <= 0_f64 {
                            continue;
                        }

                        ranged_items.push((
                            aux_range_utf16.clone(),
                            block_start_height,
                            block_end_height,
                        ));

                        for frame_block in deferred_items {
                            let previous_end = aux_end_utf16;
                            let next_start = match ast_blocks.peek() {
                                Some(ast_block) => ast_block.range.end,
                                None => {
                                    let main_source = self.world.main_source();
                                    main_source.text().len()
                                }
                            };

                            ranged_items.push((
                                previous_end..next_start,
                                frame_block.start_height.to_pt(),
                                frame_block.end_height.to_pt(),
                            ))
                        }
                    }

                    let document_height = document
                        .pages
                        .iter()
                        .map(|page| page.frame.height())
                        .sum::<Abs>()
                        .to_pt();

                    last_document = Some(document);

                    let mut previous_height = document_height;

                    ranged_items
                        .into_iter()
                        .rev()
                        .filter_map(|(range, block_start_height, block_end_height)| {
                            // crate::log!("PREV HEIGHT: {previous_height}");
                            // crate::log!("ITEM RANGE: {range:?}");
                            // crate::log!("ITEM START HEIGHT: {block_start_height:?}");
                            // crate::log!("ITEM END HEIGHT: {block_end_height:?}");

                            let delta = (previous_height - block_end_height - 1.0).max(0.0);

                            let height = block_end_height - block_start_height;

                            if height <= 0_f64 {
                                return None;
                            }

                            previous_height = block_start_height;

                            // crate::log!("[INDEX]: {i}");
                            // crate::log!("[HEIGHT]: {height:?}");
                            // crate::log!("[DELTA]: {delta:?}");

                            Some((range, height + delta, block_start_height))
                        })
                        .collect_vec()
                        .into_iter()
                        .rev()
                        .collect_vec()
                }
                Err(source_diagnostics) => {
                    let error_ranges = source_diagnostics
                        .iter()
                        .filter_map(|diagnostic| {
                            map_main_span(
                                diagnostic.span,
                                diagnostic.severity == Severity::Error,
                                &diagnostic.trace,
                                &self.world,
                            )
                        })
                        .collect::<HashSet<_>>();

                    crate::log!("[ERROR RANGES]: {error_ranges:?}");

                    let main_source = self.world.main_source();

                    let Some(block) = ast_blocks.iter().find(|block| {
                        let aux_range = &block.range;

                        let main_range_start = self.world.map_aux_to_main(aux_range.start);
                        let main_range_end = self.world.map_aux_to_main(aux_range.end);
                        // let main_range = main_range_start..main_range_end;

                        crate::log!("[BLOCK RANGE]: {main_range_start} - {main_range_end}");

                        error_ranges.iter().any(|error_range| {
                            (main_range_start <= error_range.start
                                && main_range_end >= error_range.start)
                                || (main_range_start <= error_range.end
                                    && main_range_end >= error_range.end)
                        })
                    }) else {
                        break;
                    };

                    // let aux_source = self.world.aux_source();
                    // let aux_lines = aux_source.lines();

                    let aux_range = &block.range;
                    // let aux_start_utf16 = aux_lines.byte_to_utf16(aux_range.start).unwrap();
                    // let aux_end_utf16 = aux_lines.byte_to_utf16(aux_range.end).unwrap();
                    // let aux_range_utf16 = aux_start_utf16..aux_end_utf16;

                    let mut end_byte = self.world.map_aux_to_main(aux_range.end);
                    if block.is_inline {
                        // TODO: proper offsetting (?)
                        end_byte += 10;
                    }

                    diagnostics.extend(TypstDiagnostic::from_diagnostics(
                        source_diagnostics,
                        &self.world,
                    ));

                    // crate::error!("[ERRORS]: {diagnostics:?}");

                    let start_byte = self.world.map_aux_to_main(aux_range.start);

                    let range_delta = end_byte - start_byte;
                    let repeat_range = range_delta - if range_delta > 2 { 2 } else { 1 };

                    let source = self.world.main_source_mut();
                    source.edit(start_byte..end_byte, &(" ".repeat(repeat_range) + "\\ "));

                    Vec::new()
                }
            };
        }

        let frames = if let Some(document) = &last_document {
            let canvas =
                typst_render::render_merged(document, context.pixel_per_pt, Abs::zero(), None);
            let width = canvas.width();

            ranged_heights
                .into_iter()
                .map(|(range, height, offset_height)| {
                    let rect = IntRect::from_xywh(
                        0,
                        (offset_height as f32 * context.pixel_per_pt).ceil() as i32,
                        width,
                        (height as f32 * context.pixel_per_pt).ceil() as u32,
                    )
                    .unwrap();
                    let canvas = canvas.clone_rect(rect).unwrap();
                    let encoding = canvas.encode_png().unwrap();

                    let hash = HighwayHasher::default().hash64(&encoding) as u32;

                    let height = height.ceil() as u32;

                    let render = FrameRender {
                        encoding,
                        hash,
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
        .and_then(|jump| TypstJump::from_mapped(jump, &self.world))
    }

    #[wasm_bindgen]
    pub fn autocomplete(&self, aux_cursor_utf16: usize, explicit: bool) -> Option<Autocomplete> {
        let main_source = self.world.main_source();
        let aux_source = self.world.aux_source();

        let aux_lines = aux_source.lines();
        let aux_cursor = aux_lines.utf16_to_byte(aux_cursor_utf16)?;
        let main_cursor = self.world.map_aux_to_main(aux_cursor);

        let (main_offset, completions) = typst_ide::autocomplete(
            &self.world,
            self.document.as_ref(),
            main_source,
            main_cursor,
            explicit,
        )?;

        let aux_offset = self.world.map_main_to_aux(main_offset);
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
        self.world.main = Some(id.inner());

        let mut ir = self.prelude(id, RenderingMode::Pdf);
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

struct FileContext {
    pub width: String,
    pub height: Option<f64>,
    pub pixel_per_pt: f32,
    pub theme: ThemeColors,
    pub locale: String,
}

impl Default for FileContext {
    fn default() -> Self {
        Self {
            width: String::from("auto"),
            height: None,
            pixel_per_pt: 1_f32,
            theme: ThemeColors::default(),
            locale: String::from("en"),
        }
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

#[derive(Debug, Clone)]
struct AstBlock {
    range: Range<usize>,
    is_inline: bool,
}

#[derive(Debug, Clone)]
struct FrameBlock {
    range: Option<Range<usize>>,
    start_height: Abs,
    end_height: Abs,
    item: FrameItem,
}

#[derive(Debug, Clone)]
struct WidgetBlock {
    range: Range<usize>,
    height: Abs,
    offset_height: Abs,
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

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrameRender {
    #[tsify(type = "Uint8Array")]
    #[serde(with = "serde_bytes")]
    encoding: Vec<u8>,
    hash: u32,
    height: u32,
    #[serde(rename = "offsetHeight")]
    offset_height: f64,
}

enum RenderingMode {
    Png,
    Pdf,
    // Html,
}
