//! Dedicated panel pictograms, redrawn on a 16-point vector grid.

use crate::app::DockerTab;
use crate::theme::{self, Tokens};
use egui::{Color32, Painter, Pos2, Rect, Stroke};

struct Icon<'a> {
    p: &'a Painter,
    r: Rect,
    c: Color32,
}

impl Icon<'_> {
    fn at(&self, x: f32, y: f32) -> Pos2 {
        self.r.min + egui::vec2(x, y) * (self.r.width() / 16.0)
    }
    fn stroke(&self) -> Stroke {
        Stroke::new(self.r.width() / 16.0, self.c)
    }
    fn path(&self, points: &[(f32, f32)]) {
        self.p.add(egui::epaint::PathShape::line(
            points.iter().map(|&(x, y)| self.at(x, y)).collect(),
            self.stroke(),
        ));
    }
    fn closed(&self, points: &[(f32, f32)], fill: Color32) {
        self.p.add(egui::epaint::PathShape::convex_polygon(
            points.iter().map(|&(x, y)| self.at(x, y)).collect(),
            fill,
            self.stroke(),
        ));
    }
    fn line(&self, a: (f32, f32), b: (f32, f32)) {
        self.path(&[a, b]);
    }
    fn rect(&self, x: f32, y: f32, w: f32, h: f32) {
        self.p.rect_stroke(
            Rect::from_min_max(self.at(x, y), self.at(x + w, y + h)),
            0.0,
            self.stroke(),
            egui::StrokeKind::Inside,
        );
    }
    fn block(&self, x: f32, y: f32, w: f32, h: f32, color: Color32) {
        self.p.rect_filled(
            Rect::from_min_max(self.at(x, y), self.at(x + w, y + h)),
            0.0,
            color,
        );
    }
    fn circle(&self, x: f32, y: f32, r: f32, fill: Color32) {
        self.p.circle(
            self.at(x, y),
            r * self.r.width() / 16.0,
            fill,
            self.stroke(),
        );
    }
    fn arc(&self, x: f32, y: f32, r: f32, start: f32, end: f32) {
        let points: Vec<_> = (0..=20)
            .map(|n| {
                let a = start + (end - start) * n as f32 / 20.0;
                (x + r * a.cos(), y + r * a.sin())
            })
            .collect();
        self.path(&points);
    }
    fn text(&self, value: &str) {
        self.p.text(
            self.r.center(),
            egui::Align2::CENTER_CENTER,
            value,
            egui::FontId::proportional(self.r.height()),
            self.c,
        );
    }
    fn arrow(&self, from: (f32, f32), to: (f32, f32)) {
        self.line(from, to);
        let d = egui::vec2(to.0 - from.0, to.1 - from.1).normalized();
        let tip = self.at(to.0, to.1);
        let k = self.r.width() / 16.0;
        let end = tip - d * 3.0 * k;
        let n = egui::vec2(-d.y, d.x) * 2.0 * k;
        self.p.add(egui::epaint::PathShape::convex_polygon(
            vec![tip, end + n, end - n],
            self.c,
            Stroke::NONE,
        ));
    }
    fn diamond(&self, y: f32) {
        self.closed(
            &[(1.5, y), (8.0, y - 3.0), (14.5, y), (8.0, y + 3.0)],
            Color32::TRANSPARENT,
        );
    }
    fn cube(&self) {
        self.closed(
            &[(2.0, 5.0), (9.0, 1.0), (14.0, 4.0), (7.0, 8.0)],
            Color32::TRANSPARENT,
        );
        self.path(&[
            (2.0, 5.0),
            (2.0, 12.0),
            (7.0, 15.0),
            (14.0, 11.0),
            (14.0, 4.0),
        ]);
        self.line((7.0, 8.0), (7.0, 15.0));
    }
}

pub(super) fn draw(p: &Painter, r: Rect, tab: DockerTab) {
    use std::f32::consts::PI;
    use DockerTab::*;
    let c = if theme::theme().is_dark() && theme::theme() != theme::Theme::HighContrast {
        Color32::from_rgb(186, 190, 194)
    } else {
        Tokens::icon()
    };
    let i = Icon { p, r, c };
    let clear = Color32::TRANSPARENT;
    let blue = if theme::theme() == theme::Theme::HighContrast {
        c
    } else {
        Color32::from_rgb(70, 176, 211)
    };
    let red = if theme::theme() == theme::Theme::HighContrast {
        c
    } else {
        Color32::from_rgb(210, 58, 55)
    };
    match tab {
        Properties => {
            i.closed(&[(2.0, 1.0), (8.0, 7.0), (5.0, 10.0), (0.5, 5.0)], clear);
            i.line((3.0, 3.0), (7.0, 7.0));
            for (x, y) in [(9.0, 9.0), (11.0, 12.0), (8.0, 15.0)] {
                i.line((x, y), (15.0, y));
            }
            i.circle(5.0, 11.0, 1.0, blue);
        }
        Objects => {
            i.diamond(10.0);
            i.diamond(5.0);
        }
        Pages => {
            i.rect(1.0, 5.0, 9.0, 10.0);
            i.rect(6.0, 1.0, 9.0, 10.0);
        }
        ObjectData => {
            i.rect(1.0, 2.0, 14.0, 12.0);
            for y in [5.0, 8.0, 11.0] {
                i.line((1.0, y), (15.0, y));
            }
            for x in [5.0, 10.0] {
                i.line((x, 5.0), (x, 14.0));
            }
        }
        ObjectStyles => {
            i.diamond(8.0);
            i.line((3.0, 5.0), (8.0, 13.0));
            i.path(&[(7.0, 2.0), (13.0, 8.0), (9.0, 13.0)]);
        }
        Symbols => {
            i.circle(8.0, 4.0, 3.0, c);
            i.circle(4.0, 11.0, 3.0, clear);
            i.circle(12.0, 11.0, 3.0, clear);
        }
        FindReplace => {
            i.circle(4.0, 10.0, 3.5, clear);
            i.circle(12.0, 10.0, 3.5, clear);
            i.path(&[(1.0, 8.0), (3.0, 3.0), (5.0, 3.0), (7.0, 8.0)]);
            i.path(&[(9.0, 8.0), (11.0, 3.0), (13.0, 3.0), (15.0, 8.0)]);
            i.line((7.0, 9.0), (9.0, 9.0));
        }
        Export => {
            i.path(&[(2.0, 8.0), (2.0, 14.0), (14.0, 14.0), (14.0, 8.0)]);
            i.arrow((8.0, 11.0), (8.0, 1.0));
        }
        Transformations => {
            i.line((2.0, 8.0), (10.0, 8.0));
            i.line((6.0, 4.0), (6.0, 12.0));
            i.arc(10.0, 7.0, 5.0, -PI / 2.0, PI / 2.0);
            i.arrow((13.0, 10.0), (10.0, 12.0));
        }
        Coordinates => i.text("xy"),
        LiveGuides => {
            i.rect(1.0, 3.0, 5.0, 5.0);
            i.rect(10.0, 8.0, 5.0, 5.0);
            for x in [0.0, 3.0, 6.0, 9.0, 12.0, 15.0] {
                i.block(x, 9.0, 1.0, 1.0, blue);
            }
        }
        Guidelines => {
            for x in [2.0, 5.0, 8.0, 11.0, 14.0] {
                i.block(x, 1.0, 1.0, 1.0, c);
                i.block(x, 14.0, 1.0, 1.0, c);
                i.block(1.0, x, 1.0, 1.0, c);
                i.block(14.0, x, 1.0, 1.0, c);
            }
        }
        AlignDistribute => {
            i.line((2.0, 1.0), (2.0, 15.0));
            for (y, w) in [(2.0, 9.0), (6.0, 12.0), (10.0, 6.0)] {
                i.block(4.0, y, w, 2.0, c);
            }
        }
        FitPath => {
            i.arc(8.0, 12.0, 6.0, PI, 2.0 * PI);
            for (x, y) in [(2.0, 10.0), (6.0, 6.5), (12.0, 7.5)] {
                i.circle(x, y, 1.7, blue);
            }
        }
        StepAndRepeat => {
            i.rect(1.0, 9.0, 6.0, 6.0);
            i.rect(5.0, 5.0, 6.0, 6.0);
            i.rect(9.0, 1.0, 6.0, 6.0);
        }
        Shaping => {
            i.rect(1.0, 2.0, 9.0, 9.0);
            i.circle(10.0, 10.0, 5.0, clear);
        }
        Corners => {
            i.path(&[(2.0, 15.0), (2.0, 7.0), (3.0, 3.0), (7.0, 1.0), (15.0, 1.0)]);
            i.block(1.0, 1.0, 3.0, 3.0, blue);
        }
        JoinCurves => {
            i.path(&[
                (2.0, 14.0),
                (2.0, 7.0),
                (4.0, 3.0),
                (8.0, 2.0),
                (12.0, 4.0),
                (14.0, 8.0),
                (14.0, 14.0),
            ]);
            i.block(1.0, 12.0, 3.0, 3.0, c);
            i.block(12.0, 12.0, 3.0, 3.0, c);
        }
        Text => i.text("A"),
        FontSampler => {
            i.p.text(
                i.at(6.0, 8.0),
                egui::Align2::CENTER_CENTER,
                "A",
                egui::FontId::proportional(r.height() * 0.85),
                c,
            );
            i.circle(9.5, 8.0, 5.0, clear);
            i.line((13.0, 12.0), (15.5, 15.5));
        }
        Glyphs => i.text("Ω"),
        BrushStrokes => {
            i.path(&[
                (1.0, 14.0),
                (3.0, 9.0),
                (6.0, 11.0),
                (10.0, 4.0),
                (14.0, 2.0),
            ]);
            i.line((5.0, 14.0), (14.0, 5.0));
        }
        Bevel => {
            i.cube();
            i.path(&[
                (4.0, 6.0),
                (9.0, 3.0),
                (12.0, 5.0),
                (12.0, 10.0),
                (8.0, 13.0),
            ]);
        }
        BitmapMask => {
            i.rect(1.0, 1.0, 14.0, 14.0);
            i.circle(8.0, 8.0, 5.0, red);
        }
        Blend => {
            for n in 0..3 {
                let n = n as f32;
                i.rect(1.0 + n * 4.0, 1.0 + n * 4.0, 6.0, 6.0);
            }
        }
        Contour => {
            for n in 0..3 {
                let n = n as f32;
                i.rect(1.0 + n * 3.0, 1.0 + n * 3.0, 14.0 - n * 6.0, 14.0 - n * 6.0);
            }
        }
        Envelope => {
            i.path(&[
                (2.0, 2.0),
                (8.0, 4.0),
                (14.0, 2.0),
                (12.0, 8.0),
                (14.0, 14.0),
                (8.0, 12.0),
                (2.0, 14.0),
                (4.0, 8.0),
                (2.0, 2.0),
            ]);
            for (x, y) in [(1.0, 1.0), (12.0, 1.0), (1.0, 12.0), (12.0, 12.0)] {
                i.rect(x, y, 3.0, 3.0);
            }
        }
        Extrude => {
            i.cube();
            i.rect(1.0, 6.0, 7.0, 9.0);
        }
        Lens => {
            i.rect(1.0, 2.0, 9.0, 11.0);
            i.circle(10.0, 9.0, 5.0, clear);
        }
        PictureMosaic | VectorMosaic | Palettes | ColorStyles => {
            let dots = tab == VectorMosaic;
            for y in 0..3 {
                for x in 0..3 {
                    let col = if theme::theme() == theme::Theme::HighContrast {
                        c
                    } else {
                        [red, blue, Color32::from_rgb(221, 196, 86)][(x + y) % 3]
                    };
                    let px = 2.0 + x as f32 * 5.0;
                    let py = 2.0 + y as f32 * 5.0;
                    if dots {
                        i.circle(px + 1.5, py + 1.5, 1.3, col);
                    } else {
                        i.block(px, py, 3.0, 3.0, col);
                    }
                }
            }
            if tab == Palettes {
                i.rect(0.0, 0.0, 16.0, 16.0);
            }
            if tab == ColorStyles {
                i.line((1.0, 15.0), (15.0, 1.0));
            }
        }
        Color => {
            i.circle(8.0, 8.0, 6.0, clear);
            i.line((3.0, 13.0), (13.0, 3.0));
        }
        ColorProof => {
            i.rect(1.0, 1.0, 14.0, 11.0);
            i.line((5.0, 15.0), (11.0, 15.0));
            i.line((8.0, 12.0), (8.0, 15.0));
            i.block(4.0, 4.0, 4.0, 4.0, blue);
            i.block(8.0, 4.0, 4.0, 4.0, red);
        }
        Assets => {
            i.rect(1.0, 3.0, 11.0, 9.0);
            i.path(&[(5.0, 1.0), (15.0, 1.0), (15.0, 10.0)]);
            i.block(1.0, 9.0, 11.0, 3.0, blue);
        }
        Shelf => {
            i.rect(1.0, 3.0, 14.0, 10.0);
            for x in [3.0, 7.0, 11.0] {
                i.rect(x, 6.0, 2.0, 4.0);
            }
        }
        Links => {
            i.circle(8.0, 8.0, 6.0, clear);
            i.arc(8.0, 8.0, 3.0, -PI / 2.0, PI / 2.0);
            i.arc(8.0, 8.0, 3.0, PI / 2.0, PI * 1.5);
            i.line((2.0, 6.0), (14.0, 6.0));
            i.line((2.0, 10.0), (14.0, 10.0));
        }
        Fonts => {
            i.text("A");
            i.line((2.0, 15.0), (14.0, 15.0));
        }
        Views => {
            i.circle(6.5, 6.5, 5.5, clear);
            i.line((10.0, 10.0), (15.0, 15.0));
        }
        Undo => {
            i.arc(8.0, 8.0, 6.0, -PI / 2.0, PI);
            i.arrow((2.0, 5.0), (2.0, 9.0));
        }
        Scripts | Macros => {
            i.rect(1.0, 2.0, 14.0, 12.0);
            i.line((1.0, 6.0), (15.0, 6.0));
            for x in [4.0, 8.0, 12.0] {
                i.block(x, 9.0, 1.0, 1.0, c);
                i.block(x, 12.0, 1.0, 1.0, c);
            }
            if tab == Macros {
                i.arrow((3.0, 8.0), (11.0, 11.0));
            }
        }
        Hints => {
            i.closed(&[(0.5, 5.0), (8.0, 1.0), (15.5, 5.0), (8.0, 9.0)], c);
            i.path(&[
                (4.0, 8.0),
                (4.0, 12.0),
                (8.0, 14.0),
                (12.0, 12.0),
                (12.0, 8.0),
            ]);
            i.line((15.0, 6.0), (15.0, 12.0));
        }
        Comments => {
            i.rect(1.0, 2.0, 14.0, 10.0);
            i.path(&[(4.0, 12.0), (4.0, 15.0), (8.0, 12.0)]);
        }
        Effects => {
            i.line((2.0, 14.0), (12.0, 4.0));
            i.line((12.0, 0.5), (12.0, 7.5));
            i.line((8.5, 4.0), (15.5, 4.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_panel_has_vector_artwork() {
        let ctx = crate::theme::ui_context();
        for tab in DockerTab::ALL {
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                draw(
                    ui.painter(),
                    Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(16.0, 16.0)),
                    tab,
                );
            });
            output.textures_delta.clear();
            assert!(!output.shapes.is_empty(), "{tab:?}");
        }
    }
}
