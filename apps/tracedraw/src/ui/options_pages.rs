//! Pages of Tools > Options added after the owner's third reference video
//! (`docs/reference/ui-reference.md`, "Options"), and the small pieces the
//! pages share: bold headings, the line between sections, notes, tabs,
//! and controls shown greyed where CD Draw has no such feature.

use crate::app::{App, Units, Workspace};
use crate::i18n::tr;
use crate::theme::Tokens;
use crate::ui::options::{grid, row_label};
use egui::Ui;

// ----- shared pieces -------------------------------------------------------

/// A section's bold title.
pub(crate) fn heading(ui: &mut Ui, text: &str) {
    ui.add_space(2.0);
    ui.label(egui::RichText::new(text).font(crate::theme::bold(12.5)));
    ui.add_space(2.0);
}

/// The thin line between two sections, across the page.
pub(crate) fn sep(ui: &mut Ui) {
    ui.add_space(8.0);
    let (r, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().hline(
        r.x_range(),
        r.center().y,
        egui::Stroke::new(1.0, Tokens::control_border()),
    );
    ui.add_space(8.0);
}

/// A note in the dimmer text colour, wrapped to the page.
pub(crate) fn note(ui: &mut Ui, text: &str) {
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .color(Tokens::text_dim())
                .size(12.0),
        )
        .wrap(),
    );
}

/// A check box CD Draw shows but cannot act on.
pub(crate) fn off_check(ui: &mut Ui, on: bool, text: &str) {
    let mut v = on;
    ui.add_enabled(false, egui::Checkbox::new(&mut v, text));
}

/// A choice CD Draw shows but cannot act on.
pub(crate) fn off_radio(ui: &mut Ui, on: bool, text: &str) {
    ui.add_enabled(false, egui::RadioButton::new(on, text));
}

/// A list showing `text`, greyed.
pub(crate) fn off_combo(ui: &mut Ui, id: &str, text: &str, width: f32) {
    ui.add_enabled_ui(false, |ui| {
        egui::ComboBox::from_id_salt(id)
            .selected_text(text)
            .width(width)
            .show_ui(ui, |_| {});
    });
}

/// A push button, greyed.
pub(crate) fn off_button(ui: &mut Ui, text: &str) {
    ui.add_enabled(
        false,
        egui::Button::new(text).min_size(egui::vec2(96.0, 22.0)),
    );
}

/// A slider with its groove showing on the page (the form style draws
/// inputs in the field colour) and the accent colour up to the handle.
pub(crate) fn slider(ui: &mut Ui, s: egui::Slider) -> egui::Response {
    ui.scope(|ui| {
        ui.spacing_mut().slider_width = 140.0;
        let w = &mut ui.visuals_mut().widgets;
        w.inactive.bg_fill = Tokens::slider_rail();
        w.inactive.fg_stroke = egui::Stroke::new(1.0, Tokens::control_border());
        ui.visuals_mut().selection.bg_fill = Tokens::ACCENT;
        ui.add(s.trailing_fill(true))
    })
    .inner
}

/// A number field 64 px wide with a unit after it.
pub(crate) fn number<T: egui::emath::Numeric>(
    ui: &mut Ui,
    value: &mut T,
    range: std::ops::RangeInclusive<T>,
    unit: &str,
) -> bool {
    let changed = ui
        .add_sized([64.0, 22.0], egui::DragValue::new(value).range(range))
        .changed();
    if !unit.is_empty() {
        ui.label(unit);
    }
    changed
}

/// Tabs over a framed panel, the picked tab filled with the accent colour;
/// `add` draws the panel of tab `i`. The tab picked is kept per `id`.
pub(crate) fn tabbed(ui: &mut Ui, id: &str, labels: &[String], add: impl FnOnce(&mut Ui, usize)) {
    let key = egui::Id::new(("options_tabs", id));
    let mut cur: usize = ui
        .data(|d| d.get_temp(key))
        .unwrap_or(0)
        .min(labels.len().saturating_sub(1));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for (i, l) in labels.iter().enumerate() {
            let on = i == cur;
            let text = egui::RichText::new(l).color(if on {
                egui::Color32::WHITE
            } else {
                Tokens::text()
            });
            let b = egui::Button::new(text)
                .fill(if on {
                    Tokens::ACCENT
                } else {
                    Tokens::control_fill()
                })
                .min_size(egui::vec2(0.0, 22.0));
            if ui.add(b).clicked() {
                cur = i;
            }
        }
    });
    ui.data_mut(|d| d.insert_temp(key, cur));
    egui::Frame::new()
        .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add(ui, cur);
        });
}

/// An empty framed box, as a list with nothing in it.
fn empty_box(ui: &mut Ui, size: egui::Vec2) -> egui::Rect {
    let (r, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().rect(
        r,
        0.0,
        Tokens::field(),
        egui::Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    r
}

fn workspace_name(ws: Workspace) -> String {
    tr(&format!("menu.window.workspace_{}", ws.id()))
}

// ----- CD Draw ------------------------------------------------------------

/// CD Draw > Tablet Mode: the switch between the desktop and the tablet
/// workspaces (CD Draw has no tablet mode: shown greyed).
pub fn tablet_mode_page(_app: &mut App, ui: &mut Ui) {
    off_check(ui, false, &tr("opt.tablet_auto_switch"));
    ui.add_space(6.0);
    ui.label(tr("opt.tablet_switch_to"));
    grid("tablet_mode").show(ui, |ui| {
        row_label(ui, &tr("opt.tablet_desktop_mode"));
        off_combo(
            ui,
            "tablet_desktop",
            &workspace_name(Workspace::Default),
            140.0,
        );
        ui.end_row();
        row_label(ui, &tr("opt.tablet_tablet_mode"));
        off_combo(
            ui,
            "tablet_tablet",
            &workspace_name(Workspace::Touch),
            140.0,
        );
        ui.end_row();
    });
}

/// CD Draw > Pen Settings: the pressure strokes and the pen interface
/// (shown greyed: CD Draw reads the pen as the system reports it).
pub fn pen_page(_app: &mut App, ui: &mut Ui) {
    empty_box(ui, egui::vec2(ui.available_width(), 165.0));
    note(ui, &tr("opt.pen_strokes_hint"));
    ui.add_space(4.0);
    ui.horizontal_top(|ui| {
        ui.label(tr("opt.pen_interface"));
        ui.vertical(|ui| {
            off_radio(ui, false, "WinTab");
            off_radio(ui, true, "Windows Ink");
        });
    });
    note(ui, &tr("opt.pen_interface_hint"));
    ui.vertical_centered(|ui| off_button(ui, &tr("options.save")));
}

/// CD Draw > Macros: their security (shown greyed).
pub fn macros_page(_app: &mut App, ui: &mut Ui) {
    heading(ui, &tr("opt.security"));
    off_button(ui, &tr("opt.security_options"));
    off_check(ui, true, &tr("opt.trust_macros"));
    sep(ui);
    off_check(ui, true, &tr("opt.delay_macros"));
}

/// CD Draw > Plug-ins: the folders plug-ins load from (none: greyed).
pub fn plugins_page(_app: &mut App, ui: &mut Ui) {
    heading(ui, &tr("opt.plugin_folders"));
    empty_box(ui, egui::vec2(ui.available_width(), 215.0));
    ui.horizontal(|ui| {
        off_button(ui, &tr("opt.add"));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            off_button(ui, &tr("opt.remove"));
            off_button(ui, &tr("opt.edit"));
        });
    });
}

// ----- Tools --------------------------------------------------------------

/// Tools > Smart Drawing: the delay before a stroke is recognised (greyed:
/// Shape Recognition decides when the button is let go).
pub fn smart_drawing_page(_app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(tr("opt.drawing_assist_delay"));
        ui.add_enabled_ui(false, |ui| {
            let mut v = 1.0f32;
            slider(ui, egui::Slider::new(&mut v, 0.0..=2.0).show_value(false));
            let mut v = 1.0f32;
            ui.add_sized([50.0, 22.0], egui::DragValue::new(&mut v).max_decimals(1));
        });
        ui.label(tr("opt.seconds"));
    });
}

/// Tools > Dimension and Angular Dimension: the style new dimensions take.
pub fn dimension_page(app: &mut App, ui: &mut Ui, angular: bool) {
    let s = &mut app.dimension_style;
    grid(if angular { "angular_dim" } else { "dim" }).show(ui, |ui| {
        if !angular {
            row_label(ui, &tr("opt.dim_style"));
            off_combo(ui, "dim_style", &tr("opt.dim_decimal"), 120.0);
            ui.end_row();
        }
        row_label(ui, &tr("opt.dim_precision"));
        let shown = |p: u8| {
            if p == 0 {
                "0".to_string()
            } else {
                format!("0.{}", "0".repeat(p as usize))
            }
        };
        egui::ComboBox::from_id_salt(("dim_precision", angular))
            .selected_text(shown(s.precision))
            .width(120.0)
            .show_ui(ui, |ui| {
                for p in 0..=6u8 {
                    ui.selectable_value(&mut s.precision, p, shown(p));
                }
            });
        ui.end_row();
        row_label(ui, &tr("opt.dim_units"));
        if angular {
            off_combo(ui, "dim_units_angle", "\u{00B0}", 120.0);
        } else {
            let name = |id: &str| {
                if id.is_empty() {
                    tr("opt.dim_drawing_units")
                } else {
                    Units::from_id(id).short().to_string()
                }
            };
            egui::ComboBox::from_id_salt("dim_units")
                .selected_text(name(&s.units))
                .width(120.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut s.units, String::new(), name(""));
                    for u in Units::ALL {
                        ui.selectable_value(&mut s.units, u.id().to_string(), u.short());
                    }
                });
        }
        ui.end_row();
        ui.label("");
        ui.label("");
        ui.end_row();
        row_label(ui, &tr("opt.dim_prefix"));
        ui.add(egui::TextEdit::singleline(&mut s.prefix).desired_width(120.0));
        ui.end_row();
        row_label(ui, &tr("opt.dim_suffix"));
        ui.add(egui::TextEdit::singleline(&mut s.suffix).desired_width(120.0));
        ui.end_row();
    });
}

/// Tools > Connector: snapping to anchors and the rounded corners.
pub fn connector_page(app: &mut App, ui: &mut Ui) {
    off_check(ui, true, &tr("opt.connector_geometric_anchors"));
    ui.add_space(4.0);
    grid("connector").show(ui, |ui| {
        row_label(ui, &tr("opt.connector_distance"));
        ui.add_enabled_ui(false, |ui| {
            let mut v = 8.0f64;
            ui.add_sized([64.0, 22.0], egui::DragValue::new(&mut v).suffix(" mm"));
        });
        ui.end_row();
        row_label(ui, &tr("opt.connector_radius"));
        ui.horizontal(|ui| {
            number(ui, &mut app.connector_radius, 0.0..=1000.0, "mm");
        });
        ui.end_row();
    });
}

/// Tools > Mesh Fill: the grid new mesh fills take.
pub fn mesh_fill_page(app: &mut App, ui: &mut Ui) {
    grid("mesh_fill").show(ui, |ui| {
        row_label(ui, &tr("opt.columns_count"));
        number(ui, &mut app.mesh_cols, 1..=99, "");
        ui.end_row();
        row_label(ui, &tr("opt.rows_count"));
        number(ui, &mut app.mesh_rows, 1..=99, "");
        ui.end_row();
    });
}

// ----- Global -------------------------------------------------------------

/// Global > Color: the spot colour palettes files open with (greyed: CD
/// Draw keeps the colours a file gives).
pub fn global_color_page(_app: &mut App, ui: &mut Ui) {
    grid("global_color").show(ui, |ui| {
        row_label(ui, &tr("opt.spot_palette"));
        off_combo(ui, "spot_palette", &tr("opt.document_palette"), 220.0);
        ui.end_row();
    });
    sep(ui);
    heading(ui, &tr("opt.palette_mapping"));
    note(ui, &tr("opt.palette_mapping_hint"));
    grid("global_color_map").show(ui, |ui| {
        row_label(ui, &tr("opt.spot_colors"));
        off_combo(ui, "spot_colors", &tr("opt.document_palette"), 220.0);
        ui.end_row();
        ui.label("");
        off_check(ui, true, &tr("opt.show_choose_palette"));
        ui.end_row();
    });
}

// ----- Workspaces ---------------------------------------------------------

fn picked_workspace(ui: &Ui, app: &App) -> Workspace {
    let i: Option<usize> = ui.data(|d| d.get_temp(egui::Id::new("options_ws_pick")));
    i.and_then(|i| Workspace::ALL.get(i).copied())
        .unwrap_or(app.workspace)
}

/// The workspaces in the list at the left of Workspaces, a check at the
/// one in use.
pub fn workspace_list(app: &mut App, ui: &mut Ui, list: egui::Rect) {
    let picked = picked_workspace(ui, app);
    let mut order: Vec<Workspace> = Workspace::ALL.to_vec();
    order.sort_by_key(|w| workspace_name(*w));
    for (i, ws) in order.iter().enumerate() {
        let row = egui::Rect::from_min_size(
            list.min + egui::vec2(2.0, 22.0 + i as f32 * 25.0),
            egui::vec2(list.width() - 4.0, 24.0),
        );
        let name = format!("      {}", workspace_name(*ws));
        let resp =
            crate::ui::options::list_row(ui, row, &name, None, *ws == picked, ("options_ws", i));
        if *ws == app.workspace {
            let c = egui::pos2(row.left() + 14.0, row.center().y);
            ui.painter().line(
                vec![
                    c + egui::vec2(-5.0, 0.0),
                    c + egui::vec2(-1.5, 4.0),
                    c + egui::vec2(6.0, -5.0),
                ],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(0x3C, 0xB4, 0x4A)),
            );
        }
        if resp.clicked() {
            let idx = Workspace::ALL.iter().position(|w| w == ws).unwrap_or(0);
            ui.data_mut(|d| d.insert_temp(egui::Id::new("options_ws_pick"), idx));
        }
        if resp.double_clicked() {
            app.set_workspace(*ws);
        }
    }
}

/// Workspaces: make the picked one current, export, and the description.
pub fn workspace_buttons(app: &mut App, ui: &mut Ui) {
    let picked = picked_workspace(ui, app);
    let w = egui::vec2(100.0, 24.0);
    if ui
        .add_enabled(
            picked != app.workspace,
            egui::Button::new(tr("opt.ws_set_current")).min_size(w),
        )
        .clicked()
    {
        app.set_workspace(picked);
    }
    ui.add_enabled(false, egui::Button::new(tr("opt.ws_duplicate")).min_size(w));
    ui.add_enabled(false, egui::Button::new(tr("opt.ws_delete")).min_size(w));
    let parts = app.settings.workspace_parts;
    if ui
        .add_enabled(
            parts.any(),
            egui::Button::new(tr("opt.ws_export")).min_size(w),
        )
        .clicked()
    {
        app.export_workspace_dialog(parts);
    }
    sep(ui);
    heading(ui, &tr("opt.description"));
    ui.label(tr(&format!("opt.ws_desc_{}", picked.id())));
}

// ----- Document -----------------------------------------------------------

/// Document > Save: what goes into the file (greyed: CD Draw always saves
/// the same way).
pub fn document_save_page(_app: &mut App, ui: &mut Ui) {
    off_check(ui, true, &tr("opt.save_merge_data"));
    off_check(ui, false, &tr("opt.save_cmx"));
    ui.label(egui::RichText::new(tr("opt.file_optimization")).font(crate::theme::bold(12.5)));
    off_check(ui, true, &tr("opt.bitmap_compression"));
    off_check(ui, true, &tr("opt.graphic_compression"));
    ui.label(egui::RichText::new(tr("opt.texture_fills")).font(crate::theme::bold(12.5)));
    off_radio(ui, true, &tr("opt.texture_save"));
    off_radio(ui, false, &tr("opt.texture_rebuild"));
    ui.label(egui::RichText::new(tr("opt.blends_extrudes")).font(crate::theme::bold(12.5)));
    off_radio(ui, false, &tr("opt.blends_save"));
    off_radio(ui, true, &tr("opt.blends_rebuild"));
}
