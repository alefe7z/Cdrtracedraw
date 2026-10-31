//! The Properties docker's three section icons, drawn from the owner's
//! SVG files in `assets/icons/properties/` (outline, fill,
//! transparency): the same paths, widths and colours on the files' 100
//! by 100 grid, rasterised once per size and kept as textures. The
//! files' dark backgrounds are the docker itself; the outline file's blue
//! square is the lit button, drawn behind whichever icon is on.

use egui::{Color32, Rect, Ui, Vec2};
use tiny_skia::{FillRule, LineCap, LineJoin, Paint, PathBuilder, Pixmap, Stroke, Transform};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Glyph {
    Outline,
    Fill,
    Transparency,
}

/// The lit button's square on the files' grid (the outline file's blue
/// background): 12 to 88, corners of 14.
const LIT: (f32, f32, f32) = (12.0, 76.0, 14.0);
/// The lit button's blue.
pub const LIT_BLUE: Color32 = Color32::from_rgb(0x1C, 0x46, 0x70);
/// A monochrome icon on the lit blue, in either theme.
pub const LIT_ICON: Color32 = Color32::from_rgb(0xE8, 0xEC, 0xF0);
/// The files' background, showing through the holes of the checker.
const BACK: [u8; 3] = [0x23, 0x23, 0x23];

/// Paint `glyph` on the button `rect` (the lit square's size), lit or not.
pub fn paint(ui: &Ui, rect: Rect, glyph: Glyph, lit: bool) {
    // One unit of the files' grid: the lit square fills the button.
    let unit = rect.width() / LIT.1;
    let grid = Rect::from_min_size(
        rect.min - Vec2::splat(LIT.0 * unit),
        Vec2::splat(100.0 * unit),
    );
    if lit {
        ui.painter().rect_filled(rect, LIT.2 * unit, LIT_BLUE);
    }
    let ppp = ui.ctx().pixels_per_point();
    let px = (grid.width() * ppp).round().max(8.0) as u32;
    let id = egui::Id::new(("prop_icon", glyph, px, lit));
    let tex = ui.ctx().data(|d| d.get_temp::<egui::TextureHandle>(id));
    let tex = match tex {
        Some(t) => t,
        None => {
            let Some(image) = render(glyph, px, lit) else {
                return;
            };
            let t = ui.ctx().load_texture(
                format!("prop_icon_{glyph:?}_{px}_{lit}"),
                image,
                egui::TextureOptions::LINEAR,
            );
            ui.ctx().data_mut(|d| d.insert_temp(id, t.clone()));
            t
        }
    };
    ui.painter().image(
        tex.id(),
        grid,
        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}

fn paint_of(rgb: [u8; 3]) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color_rgba8(rgb[0], rgb[1], rgb[2], 255);
    p.anti_alias = true;
    p
}

fn stroke(width: f32, cap: LineCap, join: LineJoin) -> Stroke {
    Stroke {
        width,
        line_cap: cap,
        line_join: join,
        ..Stroke::default()
    }
}

fn line(x0: f32, y0: f32, x1: f32, y1: f32) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(x0, y0);
    pb.line_to(x1, y1);
    pb.finish()
}

/// A rectangle with round corners of radius `r`.
fn round_rect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Option<tiny_skia::Path> {
    let r = r.min(w / 2.0).min(h / 2.0);
    let k = 0.552_284_8 * r;
    let mut pb = PathBuilder::new();
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.cubic_to(x + w - r + k, y, x + w, y + r - k, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.cubic_to(x + w, y + h - r + k, x + w - r + k, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.cubic_to(x + r - k, y + h, x, y + h - r + k, x, y + h - r);
    pb.line_to(x, y + r);
    pb.cubic_to(x, y + r - k, x + r - k, y, x + r, y);
    pb.close();
    pb.finish()
}

/// The glyph on the files' grid, `px` pixels square, transparent around.
fn render(glyph: Glyph, px: u32, lit: bool) -> Option<egui::ColorImage> {
    let mut pm = Pixmap::new(px, px)?;
    let t = Transform::from_scale(px as f32 / 100.0, px as f32 / 100.0);
    match glyph {
        // outline.svg
        Glyph::Outline => {
            let ink = paint_of([0xDC, 0xE2, 0xE8]);
            let mut pb = PathBuilder::new();
            pb.move_to(50.0, 26.0);
            pb.line_to(34.0, 62.0);
            pb.line_to(42.0, 72.0);
            pb.line_to(58.0, 72.0);
            pb.line_to(66.0, 62.0);
            pb.close();
            let nib = pb.finish()?;
            pm.stroke_path(
                &nib,
                &ink,
                &stroke(4.5, LineCap::Round, LineJoin::Round),
                t,
                None,
            );
            let slit = line(50.0, 28.0, 50.0, 51.0)?;
            pm.stroke_path(
                &slit,
                &ink,
                &stroke(3.5, LineCap::Round, LineJoin::Miter),
                t,
                None,
            );
            // The breather hole is filled with what is behind it.
            let hole = PathBuilder::from_circle(50.0, 55.0, 4.5)?;
            let behind = if lit {
                [LIT_BLUE.r(), LIT_BLUE.g(), LIT_BLUE.b()]
            } else {
                BACK
            };
            pm.fill_path(&hole, &paint_of(behind), FillRule::Winding, t, None);
            pm.stroke_path(
                &hole,
                &ink,
                &stroke(3.5, LineCap::Butt, LineJoin::Miter),
                t,
                None,
            );
            for (x0, x1, y) in [(37.0, 63.0, 72.0), (41.0, 59.0, 78.0)] {
                let l = line(x0, y, x1, y)?;
                pm.stroke_path(
                    &l,
                    &ink,
                    &stroke(4.5, LineCap::Round, LineJoin::Miter),
                    t,
                    None,
                );
            }
        }
        // fill.svg
        Glyph::Fill => {
            let orange = paint_of([0xF0, 0x5A, 0x00]);
            let ink = paint_of([0xD4, 0xD4, 0xD4]);
            let mut pb = PathBuilder::new();
            pb.move_to(33.0, 56.0);
            pb.line_to(59.0, 56.0);
            pb.line_to(46.0, 69.0);
            pb.close();
            pm.fill_path(&pb.finish()?, &orange, FillRule::Winding, t, None);
            let mut pb = PathBuilder::new();
            pb.move_to(46.0, 34.0);
            pb.line_to(62.0, 50.0);
            pb.line_to(48.0, 64.0);
            pb.cubic_to(45.0, 67.0, 41.0, 67.0, 38.0, 64.0);
            pb.line_to(30.0, 56.0);
            pb.cubic_to(27.0, 53.0, 27.0, 49.0, 30.0, 46.0);
            pb.close();
            pm.stroke_path(
                &pb.finish()?,
                &ink,
                &stroke(5.0, LineCap::Butt, LineJoin::Round),
                t,
                None,
            );
            let handle = line(41.0, 29.0, 67.0, 55.0)?;
            pm.stroke_path(
                &handle,
                &ink,
                &stroke(6.0, LineCap::Round, LineJoin::Miter),
                t,
                None,
            );
            let mut pb = PathBuilder::new();
            pb.move_to(67.0, 62.0);
            pb.cubic_to(67.0, 62.0, 74.0, 71.0, 74.0, 76.0);
            pb.cubic_to(74.0, 80.0, 71.0, 83.0, 67.0, 83.0);
            pb.cubic_to(63.0, 83.0, 60.0, 80.0, 60.0, 76.0);
            pb.cubic_to(60.0, 71.0, 67.0, 62.0, 67.0, 62.0);
            pb.close();
            pm.fill_path(&pb.finish()?, &orange, FillRule::Winding, t, None);
        }
        // transparency.svg
        Glyph::Transparency => {
            let ink = paint_of([0xD4, 0xD4, 0xD4]);
            pm.fill_path(
                &round_rect(24.0, 24.0, 52.0, 52.0, 6.0)?,
                &ink,
                FillRule::Winding,
                t,
                None,
            );
            let back = paint_of(BACK);
            for (x, y, w, h, r) in [
                (39.0, 28.0, 9.0, 9.0, 1.0),
                (57.0, 28.0, 9.0, 9.0, 1.0),
                (30.0, 39.0, 9.0, 9.0, 1.0),
                (48.0, 39.0, 9.0, 9.0, 1.0),
                (66.0, 39.0, 6.0, 9.0, 1.0),
                (39.0, 50.0, 9.0, 9.0, 1.0),
                (57.0, 50.0, 9.0, 9.0, 1.0),
                (30.0, 61.0, 9.0, 9.0, 1.0),
                (48.0, 61.0, 9.0, 9.0, 1.0),
                (66.0, 61.0, 6.0, 9.0, 1.0),
                (39.0, 70.0, 9.0, 3.0, 0.5),
                (57.0, 70.0, 9.0, 3.0, 0.5),
            ] {
                pm.fill_path(
                    &round_rect(x, y, w, h, r)?,
                    &back,
                    FillRule::Winding,
                    t,
                    None,
                );
            }
        }
    }
    let pixels: Vec<Color32> = pm
        .pixels()
        .iter()
        .map(|p| Color32::from_rgba_premultiplied(p.red(), p.green(), p.blue(), p.alpha()))
        .collect();
    Some(egui::ColorImage {
        size: [px as usize, px as usize],
        source_size: egui::vec2(px as f32, px as f32),
        pixels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An icon on the lit blue keeps a strong contrast in either theme
    /// (the dark theme's icon colour would vanish on it in the light one).
    #[test]
    fn icons_on_the_lit_blue_are_light() {
        let lum =
            |c: Color32| 0.2126 * c.r() as f32 + 0.7152 * c.g() as f32 + 0.0722 * c.b() as f32;
        assert!(lum(LIT_ICON) - lum(LIT_BLUE) > 150.0);
    }

    /// Each icon renders, with ink where the files put it: the outline's
    /// nib tip, the fill's orange drop, the checker's light frame.
    #[test]
    fn the_icons_render_from_the_files() {
        for (g, x, y, want) in [
            (Glyph::Outline, 50, 72, [0xDC, 0xE2, 0xE8]),
            (Glyph::Fill, 67, 78, [0xF0, 0x5A, 0x00]),
            (Glyph::Transparency, 27, 50, [0xD4, 0xD4, 0xD4]),
        ] {
            let img = render(g, 100, false).expect("rendered");
            let p = img.pixels[y * 100 + x];
            assert_eq!([p.r(), p.g(), p.b()], want, "{g:?}");
            assert_eq!(img.pixels[0].a(), 0, "{g:?} has no background");
        }
    }
}
