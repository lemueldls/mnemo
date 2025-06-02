//! Rendering of Typst documents into raster images.

mod image;
mod paint;
mod shape;
mod text;

use std::{num::NonZeroUsize, ops::Range};

use tiny_skia as sk;
use ttf_parser::GlyphId;
use typst_ide::IdeWorld;
use typst_library::{
    WorldExt,
    foundations::Selector,
    introspection::{Introspector, Tag},
    layout::{
        Abs, Axes, Frame, FrameItem, FrameKind, GroupItem, Page, PagedDocument, Point, Position,
        Size, Transform,
    },
    visualize::{Color, Geometry, Paint},
};

/// Export a page into a raster image.
///
/// This renders the page at the given number of pixels per point and returns
/// the resulting `tiny-skia` pixel buffer.
#[typst_macros::time(name = "render")]
pub fn render_world_frame(
    frame: &Frame,
    offset_height: f64,
    pixel_per_pt: f32,
    world: &dyn IdeWorld,
) -> sk::Pixmap {
    let mut size = frame.size();
    size.y -= Abs::pt(offset_height);

    let pxw = (pixel_per_pt * size.x.to_f32()).round().max(1.0) as u32;
    let pxh = (pixel_per_pt * size.y.to_f32()).round().max(1.0) as u32;

    let ts = sk::Transform::from_scale(pixel_per_pt, pixel_per_pt)
        .post_translate(0.0, -offset_height as f32);
    let state = State::new(size, ts, pixel_per_pt);

    let mut canvas = sk::Pixmap::new(pxw, pxh).unwrap();

    for (pos, item) in frame.items() {
        // let point = Point::zero();
        let point = *pos;

        match item {
            FrameItem::Group(group) => {
                render_group(&mut canvas, state, point, group); // TODO: use this

                // let spanned_renders = render(&group.frame, only_after, true_height, pixel_per_pt, introspector, world);
                // let first_item = &spanned_renders.first();
                // let last_item = &spanned_renders.last();

                // match (first_item, last_item) {
                //     (Some(first_render), Some(last_render)) => {
                //         let range = first_render.range.start..last_render.range.end;

                //         if in_tag {
                //             ranged_renders.last_mut().unwrap().range.end = range.end;
                //         } else {
                //             ranged_renders.push(RangedCanvas { range, canvas });
                //         }
                //     }
                //     _ => {}
                // }
            }
            FrameItem::Text(text) => {
                text::render_text(
                    &mut canvas,
                    // state.pre_translate(point), // .with_size(Axes::new(x, text.size))
                    state.pre_translate(point),
                    // .with_size(Axes::new(x, text.size)),
                    // .with_size(Axes::zero()),
                    text,
                );

                let (start_span, _) = text.glyphs.first().unwrap().span;
                let (end_span, _) = text.glyphs.last().unwrap().span;

                let start_range = world.range(start_span);
                let end_range = world.range(end_span);

                match (start_range, end_range) {
                    (Some(start_range), Some(end_range)) => {
                        let range = start_range.start..end_range.end;
                    }
                    _ => {}
                }
            }
            FrameItem::Shape(shape, span) => {
                shape::render_shape(&mut canvas, state.pre_translate(point), shape);

                let range = world.range(*span);
            }
            FrameItem::Image(image, size, span) => {
                image::render_image(&mut canvas, state.pre_translate(point), image, *size);

                let range = world.range(*span);
            }
            FrameItem::Link(..) => {}
            FrameItem::Tag(tag) => {
                // match tag {
                //     Tag::Start(content) => {
                //         in_tag = true;

                //         let range = world.range(content.span()).unwrap();
                //         ranged_renders.push(RangedCanvas {
                //             range,
                //             canvas: sk::Pixmap::new(pxw, pxh).unwrap(),
                //         });
                //     }
                //     Tag::End(location, _) => {
                //         in_tag = false;

                //         let range = world
                //             .range(
                //                 introspector
                //                     .query_first(&Selector::Location(*location))
                //                     .unwrap()
                //                     .span(),
                //             )
                //             .unwrap();
                //         ranged_renders.last_mut().unwrap().range.end = range.end
                //     }
                // }
            }
        };

        // current_block.push
        // span.map(|span| (canvas, span))
    }

    canvas
}

/// Export a document with potentially multiple pages into a single raster image.
pub fn render(
    document: &PagedDocument,
    offset_height: f64,
    pixel_per_pt: f32,
    world: &dyn IdeWorld,
) -> sk::Pixmap {
    let pixmaps: Vec<_> = document
        .pages
        .iter()
        .map(|page| render_world_frame(&page.frame, offset_height, pixel_per_pt, world))
        .collect();

    // let gap = (pixel_per_pt * gap.to_f32()).round() as u32;
    let pxw = pixmaps
        .iter()
        .map(sk::Pixmap::width)
        .max()
        .unwrap_or_default();
    let pxh = pixmaps.iter().map(|pixmap| pixmap.height()).sum::<u32>();
    // + gap * pixmaps.len().saturating_sub(1) as u32;

    let mut canvas = sk::Pixmap::new(pxw, pxh).unwrap();
    // if let Some(fill) = fill {
    //     canvas.fill(paint::to_sk_color(fill));
    // }

    let mut y = 0;
    for pixmap in pixmaps {
        canvas.draw_pixmap(
            0,
            y as i32,
            pixmap.as_ref(),
            &sk::PixmapPaint::default(),
            sk::Transform::identity(),
            None,
        );

        // y += pixmap.height() + gap;
        y += pixmap.height();
    }

    canvas
}

/// Additional metadata carried through the rendering process.
#[derive(Clone, Copy, Default)]
struct State<'a> {
    /// The transform of the current item.
    transform: sk::Transform,
    /// The transform of the first hard frame in the hierarchy.
    container_transform: sk::Transform,
    /// The mask of the current item.
    mask: Option<&'a sk::Mask>,
    /// The pixel per point ratio.
    pixel_per_pt: f32,
    /// The size of the first hard frame in the hierarchy.
    size: Size,
}

impl State<'_> {
    fn new(size: Size, transform: sk::Transform, pixel_per_pt: f32) -> Self {
        Self {
            size,
            transform,
            container_transform: transform,
            pixel_per_pt,
            ..Default::default()
        }
    }

    /// Pre translate the current item's transform.
    fn pre_translate(self, pos: Point) -> Self {
        Self {
            transform: self.transform.pre_translate(pos.x.to_f32(), pos.y.to_f32()),
            ..self
        }
    }

    fn pre_scale(self, scale: Axes<Abs>) -> Self {
        Self {
            transform: self.transform.pre_scale(scale.x.to_f32(), scale.y.to_f32()),
            ..self
        }
    }

    /// Pre concat the current item's transform.
    fn pre_concat(self, transform: sk::Transform) -> Self {
        Self {
            transform: self.transform.pre_concat(transform),
            ..self
        }
    }

    /// Sets the current mask.
    fn with_mask(self, mask: Option<&sk::Mask>) -> State<'_> {
        // Ensure that we're using the parent's mask if we don't have one.
        if mask.is_some() {
            State { mask, ..self }
        } else {
            State { mask: None, ..self }
        }
    }

    /// Sets the size of the first hard frame in the hierarchy.
    fn with_size(self, size: Size) -> Self {
        Self { size, ..self }
    }

    /// Pre concat the container's transform.
    fn pre_concat_container(self, transform: sk::Transform) -> Self {
        Self {
            container_transform: self.container_transform.pre_concat(transform),
            ..self
        }
    }
}

/// Render a frame into the canvas.
fn render_frame(canvas: &mut sk::Pixmap, state: State, frame: &Frame) {
    for (pos, item) in frame.items() {
        match item {
            FrameItem::Group(group) => {
                render_group(canvas, state, *pos, group);
            }
            FrameItem::Text(text) => {
                text::render_text(canvas, state.pre_translate(*pos), text);
            }
            FrameItem::Shape(shape, _) => {
                shape::render_shape(canvas, state.pre_translate(*pos), shape);
            }
            FrameItem::Image(image, size, _) => {
                image::render_image(canvas, state.pre_translate(*pos), image, *size);
            }
            FrameItem::Link(..) => {}
            FrameItem::Tag(_) => {}
        }
    }
}

/// Render a group frame with optional transform and clipping into the canvas.
fn render_group(canvas: &mut sk::Pixmap, state: State, pos: Point, group: &GroupItem) {
    let sk_transform = to_sk_transform(&group.transform);
    let state = match group.frame.kind() {
        FrameKind::Soft => state.pre_translate(pos).pre_concat(sk_transform),
        FrameKind::Hard => {
            state
                .pre_translate(pos)
                .pre_concat(sk_transform)
                .pre_concat_container(
                    state
                        .transform
                        .post_concat(state.container_transform.invert().unwrap()),
                )
                .pre_concat_container(to_sk_transform(&Transform::translate(pos.x, pos.y)))
                .pre_concat_container(sk_transform)
                .with_size(group.frame.size())
        }
    };

    let mut mask = state.mask;
    let storage;
    if let Some(clip_curve) = group.clip.as_ref() {
        if let Some(path) =
            shape::convert_curve(clip_curve).and_then(|path| path.transform(state.transform))
        {
            if let Some(mask) = mask {
                let mut mask = mask.clone();
                mask.intersect_path(
                    &path,
                    sk::FillRule::default(),
                    false,
                    sk::Transform::default(),
                );
                storage = mask;
            } else {
                let pxw = canvas.width();
                let pxh = canvas.height();
                let Some(mut mask) = sk::Mask::new(pxw, pxh) else {
                    // Fails if clipping rect is empty. In that case we just
                    // clip everything by returning.
                    return;
                };

                mask.fill_path(
                    &path,
                    sk::FillRule::default(),
                    false,
                    sk::Transform::default(),
                );
                storage = mask;
            };

            mask = Some(&storage);
        }
    }

    render_frame(canvas, state.with_mask(mask), &group.frame);
}

fn to_sk_transform(transform: &Transform) -> sk::Transform {
    let Transform {
        sx,
        ky,
        kx,
        sy,
        tx,
        ty,
    } = *transform;
    sk::Transform::from_row(
        sx.get() as _,
        ky.get() as _,
        kx.get() as _,
        sy.get() as _,
        tx.to_f32(),
        ty.to_f32(),
    )
}

/// Additional methods for [`Abs`].
trait AbsExt {
    /// Convert to a number of points as f32.
    fn to_f32(self) -> f32;
}

impl AbsExt for Abs {
    fn to_f32(self) -> f32 {
        self.to_pt() as f32
    }
}
