//! The compact, checkable catalog opened by a docker column's plus button.
//! All artwork is drawn with the project's original vector painters.

use crate::app::{App, DockerPlace, DockerTab};
use crate::i18n::tr;
use crate::theme::Tokens;
use egui::{Color32, FontId, Rect, Sense, Stroke, Ui};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    Panel(DockerTab),
}

use DockerTab::*;
use Entry::Panel as P;

// Dedicated ordering: the Window menu keeps its own existing organization.
const GROUPS: &[&[Entry]] = &[
    &[
        P(Properties),
        P(Objects),
        P(Pages),
        P(ObjectData),
        P(ObjectStyles),
        P(Symbols),
        P(FindReplace),
        P(Export),
    ],
    &[P(Transformations), P(Coordinates)],
    &[P(LiveGuides), P(Guidelines)],
    &[P(AlignDistribute), P(FitPath), P(StepAndRepeat)],
    &[P(Shaping), P(Corners), P(JoinCurves)],
    &[P(Text), P(FontSampler), P(Glyphs)],
    &[
        P(BrushStrokes),
        P(Bevel),
        P(BitmapMask),
        P(Blend),
        P(Contour),
        P(Envelope),
        P(Extrude),
        P(Lens),
        P(PictureMosaic),
        P(VectorMosaic),
    ],
    &[P(Color), P(Palettes), P(ColorStyles), P(ColorProof)],
    &[P(Assets), P(Shelf)],
    &[P(Links), P(Fonts)],
    &[P(Views), P(Undo), P(Scripts)],
    &[P(Hints)],
];

pub(super) fn label(tab: DockerTab) -> String {
    tr(match tab {
        Transformations => "docker_catalog.transform",
        Shaping => "docker_catalog.shape",
        BrushStrokes => "docker_catalog.artistic_media",
        Bevel => "docker_catalog.bevel",
        Shelf => "docker_catalog.tray",
        Links => "docker_catalog.links",
        Undo => "docker_catalog.history",
        Hints => "docker_catalog.learn",
        FontSampler => "docker_catalog.font_sampler",
        PictureMosaic => "docker_catalog.picture_mosaic",
        VectorMosaic => "docker_catalog.vector_mosaic",
        _ => tab.key(),
    })
}

/// Toggling closes a panel wherever it lives, or opens it in this column.
/// A panel in a collapsed column is still checked because it is still open.
fn toggle(app: &mut App, tab: DockerTab, place: DockerPlace) {
    if app.docker_open(tab) {
        app.close_docker(tab);
    } else {
        app.move_docker(tab, place, None);
    }
}

pub(super) fn popup_style(style: &mut egui::Style) {
    crate::ui::menus::menu_popup_style(style);
    if crate::theme::theme() == crate::theme::Theme::Dark {
        style.visuals.window_fill = Color32::from_rgb(38, 38, 38);
        style.visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(82, 82, 82));
    }
}

fn label_color() -> Color32 {
    if crate::theme::theme() == crate::theme::Theme::Dark {
        Color32::from_rgb(202, 206, 212)
    } else {
        Tokens::text()
    }
}

pub(super) fn show(app: &mut App, ui: &mut Ui, place: DockerPlace) {
    ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
    let width = GROUPS
        .iter()
        .flat_map(|g| g.iter())
        .map(|entry| {
            let text = match entry {
                P(tab) => label(*tab),
            };
            ui.painter()
                .layout_no_wrap(text, FontId::proportional(12.0), Tokens::text())
                .size()
                .x
        })
        .fold(252.0_f32, f32::max)
        + 48.0;
    // The full catalog remains reachable on small screens and at large UI scales.
    let height = (ui.ctx().content_rect().height() - 32.0).max(80.0);
    egui::ScrollArea::vertical()
        .max_height(height)
        .show(ui, |ui| {
            for (index, group) in GROUPS.iter().enumerate() {
                if index > 0 {
                    separator(ui, width);
                }
                for entry in *group {
                    row(app, ui, *entry, place, width);
                }
            }
        });
}

fn separator(ui: &mut Ui, width: f32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 15.0), Sense::hover());
    let color = if crate::theme::theme() == crate::theme::Theme::HighContrast {
        Tokens::text()
    } else {
        Color32::from_rgb(1, 125, 188)
    };
    ui.painter().hline(
        rect.x_range().shrink(3.0),
        rect.center().y,
        Stroke::new(1.0, color),
    );
}

fn row(app: &mut App, ui: &mut Ui, entry: Entry, place: DockerPlace, width: f32) {
    let P(tab) = entry;
    let text = label(tab);
    let on = app.docker_open(tab);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 17.0), Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, Tokens::menu_hover());
        ui.painter().rect_stroke(
            rect,
            0.0,
            Stroke::new(1.0, Tokens::menu_hover_line()),
            egui::StrokeKind::Inside,
        );
    }
    let check = Rect::from_center_size(
        egui::pos2(rect.left() + 11.0, rect.center().y),
        egui::vec2(12.0, 12.0),
    );
    let fill = if on && crate::theme::theme().is_dark() {
        Color32::from_rgb(64, 64, 64)
    } else {
        Tokens::field()
    };
    ui.painter().rect_filled(check, 3.0, fill);
    ui.painter().rect_stroke(
        check,
        3.0,
        Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    if on {
        let c = check.center();
        ui.painter().add(egui::epaint::PathShape::line(
            vec![
                c + egui::vec2(-3.0, 0.0),
                c + egui::vec2(-0.5, 3.0),
                c + egui::vec2(3.5, -3.5),
            ],
            Stroke::new(1.4, Tokens::text()),
        ));
    }
    let icon_rect = Rect::from_center_size(
        egui::pos2(rect.left() + 29.0, rect.center().y),
        egui::vec2(16.0, 16.0),
    );
    draw_icon(ui.painter(), icon_rect, tab);
    ui.painter().text(
        egui::pos2(rect.left() + 43.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        FontId::proportional(12.0),
        label_color(),
    );
    if response.clicked() {
        toggle(app, tab, place);
    }
}

/// The same pictogram appears in the catalog and on the vertical tab.
pub(super) fn draw_icon(painter: &egui::Painter, rect: Rect, tab: DockerTab) {
    super::reference_icons::draw(painter, rect, tab);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_covers_each_panel_once_including_extra_panels() {
        let mut tabs: Vec<_> = GROUPS
            .iter()
            .flat_map(|g| g.iter())
            .map(|entry| {
                let P(tab) = entry;
                *tab
            })
            .collect();
        tabs.extend([Comments, Effects, Macros]);
        assert_eq!(tabs.len(), DockerTab::ALL.len());
        for tab in DockerTab::ALL {
            assert_eq!(tabs.iter().filter(|t| **t == tab).count(), 1, "{tab:?}");
        }
    }

    #[test]
    fn catalog_toggle_opens_at_the_requested_side_and_closes_anywhere() {
        let mut app = App::headless();
        app.close_docker(Bevel);
        toggle(&mut app, Bevel, DockerPlace::Left);
        assert!(app.docker_open(Bevel));
        assert_eq!(app.docker_place(Bevel), Some(DockerPlace::Left));
        app.set_docker_column(DockerPlace::Left, false, None);
        assert!(app.docker_open(Bevel));
        toggle(&mut app, Bevel, DockerPlace::Right);
        assert!(!app.docker_open(Bevel));
        toggle(&mut app, Bevel, DockerPlace::Right);
        assert_eq!(app.docker_place(Bevel), Some(DockerPlace::Right));
        assert_eq!(app.docker_tab, Bevel);
        app.move_docker(Bevel, DockerPlace::Floating, None);
        assert_eq!(app.docker_place(Bevel), Some(DockerPlace::Floating));
        toggle(&mut app, Bevel, DockerPlace::Right);
        assert!(!app.docker_open(Bevel));
    }

    #[test]
    fn a_popup_stays_open_while_multiple_panel_rows_are_clicked() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        app.close_docker(Text);
        app.close_docker(Bevel);
        let id = egui::Id::new("catalog_multi_toggle_test");
        egui::Popup::open_id(&ctx, id);
        let run = |app: &mut App, events: Vec<egui::Event>| {
            let mut positions = Vec::new();
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            let mut output = ctx.run_ui(input, |ui| {
                let anchor = Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(20.0, 20.0));
                // As the Add Panel button shows it (dockers.rs).
                egui::Popup::new(id, ctx.clone(), anchor, ui.layer_id())
                    .kind(egui::PopupKind::Menu)
                    .open_memory(None)
                    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                    .show(|ui| {
                        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                        for tab in [Text, Bevel] {
                            positions.push(ui.cursor().min + egui::vec2(80.0, 8.5));
                            row(app, ui, P(tab), DockerPlace::Right, 296.0);
                        }
                    });
            });
            output.textures_delta.clear();
            positions
        };
        run(&mut app, Vec::new());
        let positions = run(&mut app, Vec::new());
        assert_eq!(positions.len(), 2);
        for (index, tab) in [Text, Bevel].into_iter().enumerate() {
            let at = positions[index];
            run(&mut app, vec![egui::Event::PointerMoved(at)]);
            for pressed in [true, false] {
                run(
                    &mut app,
                    vec![egui::Event::PointerButton {
                        pos: at,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    }],
                );
            }
            assert!(app.docker_open(tab), "{tab:?}");
            assert!(egui::Popup::is_id_open(&ctx, id));
        }
        assert!(app.docker_open(Text));
        assert!(app.docker_open(Bevel));
    }
}
