//! The Properties docker, laid out after the owner's reference: a row of
//! section buttons that stays put (Outline, Fill, Transparency, then the
//! object's own sections) with two buttons at its right, and under it the
//! sections, each opened and closed from its "^ Title" header. A section
//! button scrolls smoothly to its section, opening it, and stays lit.
//!
//! Outline: the colour, the width with its list and the unit, the line
//! style with "...", the three dash fittings, then (behind a small arrow
//! that hides them) the mitre limit, corners, line ends and position;
//! Arrowheads; Variable Outline; Calligraphy. Fill: the kinds and their
//! settings. Transparency: the merge mode and the kinds.
//!
//! With nothing selected the sections set what new objects get. See
//! docs/behavior/workspaces.md, Properties docker.

use crate::app::App;
use crate::i18n::tr;
use crate::theme::Tokens;
use crate::ui::dockers::{self as dk, SectionIcon};
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Stroke as EStroke, Ui, Vec2};
use tracedraw_core::patch::{FillField, FillMix, StrokeField, StrokeMix};
use tracedraw_core::{Fill, LineCap, LineJoin, OutlinePosition, ShapeKind, Stroke};

const ROW: f32 = 26.0;
/// Where the right-aligned labels of the Outline's lower rows end.
const LABEL_END: f32 = 150.0;

/// A section of the docker.
#[derive(Clone, Copy)]
struct Section {
    id: &'static str,
    key: &'static str,
    icon: Icon,
}

#[derive(Clone, Copy)]
enum Icon {
    Pen,
    Bucket,
    Checker,
    Other(SectionIcon),
}

fn jump_id() -> egui::Id {
    egui::Id::new("props_jump")
}

fn active_id() -> egui::Id {
    egui::Id::new("props_active")
}

/// The docker's page.
pub fn show(app: &mut App, ui: &mut Ui) {
    let cc = app.current_colors;
    ui.data_mut(|d| d.insert_temp(dk::current_colors_id(), cc));
    // A jump from elsewhere (F11 to the fill, the outline from the status
    // bar) goes to that section.
    if let Some(i) = app.properties_open.take() {
        let id = if i == 0 { "fill" } else { "outline" };
        go_to(app, ui.ctx(), id);
    }
    let sections = sections(app);
    top_bar(app, ui, &sections);
    let r = ui.available_rect_before_wrap();
    ui.painter().hline(
        r.x_range(),
        r.top() + 0.5,
        EStroke::new(1.0, Tokens::border()),
    );
    ui.add_space(2.0);
    egui::ScrollArea::vertical()
        .id_salt("properties_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            for s in &sections {
                if header(app, ui, s) {
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: 6,
                            right: 2,
                            top: 0,
                            bottom: 0,
                        })
                        .show(ui, |ui| {
                            // Rows as far apart as the reference's.
                            ui.spacing_mut().item_spacing.y = 8.0;
                            body(app, ui, s.id)
                        });
                    ui.add_space(6.0);
                }
            }
        });
}

/// The sections, in order: what every object has, then its own.
fn sections(app: &App) -> Vec<Section> {
    let mut out = vec![
        Section {
            id: "outline",
            key: "prp.outline",
            icon: Icon::Pen,
        },
        Section {
            id: "fill",
            key: "prp.fill",
            icon: Icon::Bucket,
        },
        Section {
            id: "transparency",
            key: "prp.transparency",
            icon: Icon::Checker,
        },
    ];
    let shapes = app.selected_shapes();
    let Some(first) = shapes.first() else {
        return out;
    };
    if first.qr.is_some() {
        out.insert(
            0,
            Section {
                id: "qr",
                key: "qr.section",
                icon: Icon::Other(SectionIcon::Qr),
            },
        );
    }
    if first.page_corners().is_some() {
        out.push(Section {
            id: "corners",
            key: "kind.rectangle",
            icon: Icon::Other(SectionIcon::Tool(crate::tools::Tool::Rectangle)),
        });
    }
    out.push(Section {
        id: "effects",
        key: "docker.fx",
        icon: Icon::Other(SectionIcon::Effects),
    });
    if matches!(first.kind, ShapeKind::Text { .. }) {
        out.push(Section {
            id: "character",
            key: "docker.character",
            icon: Icon::Other(SectionIcon::Tool(crate::tools::Tool::Text)),
        });
    }
    out.push(Section {
        id: "summary",
        key: "docker.summary",
        icon: Icon::Other(SectionIcon::Summary),
    });
    out
}

fn is_open(app: &App, id: &str) -> bool {
    !app.settings.properties_closed.iter().any(|c| c == id)
}

fn set_open(app: &mut App, id: &str, open: bool) {
    app.settings.properties_closed.retain(|c| c != id);
    if !open {
        app.settings.properties_closed.push(id.to_string());
    }
}

/// Open a closed section, close an open one (kept in the preferences).
#[cfg(test)]
pub(crate) fn toggle_section(app: &mut App, id: &str) {
    let open = is_open(app, id);
    set_open(app, id, !open);
}

/// Open section `id`, light its button and scroll to it.
fn go_to(app: &mut App, ctx: &egui::Context, id: &str) {
    set_open(app, id, true);
    ctx.data_mut(|d| {
        d.insert_temp(jump_id(), id.to_string());
        d.insert_temp(active_id(), id.to_string());
    });
}

// ----- the bar of section buttons ------------------------------------------------

fn top_bar(app: &mut App, ui: &mut Ui, sections: &[Section]) {
    let active: String = ui
        .data(|d| d.get_temp(active_id()))
        .unwrap_or_else(|| "outline".to_string());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.add_space(2.0);
        for s in sections {
            let on = active == s.id;
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), Sense::click());
            paint_button(ui, rect, on, resp.hovered());
            paint_icon(ui.painter(), rect.shrink(5.0), s.icon);
            if resp.on_hover_text(tr(s.key)).clicked() {
                go_to(app, ui.ctx(), s.id);
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            // Only the section picked: the others hide until it is
            // changed.
            let single = app.settings.properties_closed.iter().any(|c| c == "single");
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), Sense::click());
            paint_button(ui, rect, single, resp.hovered());
            single_icon(ui.painter(), rect.shrink(6.0));
            if resp.on_hover_text(tr("prp.single_section")).clicked() {
                set_open(app, "single", single);
            }
            let r = ui.available_rect_before_wrap();
            let (sep, _) = ui.allocate_exact_size(Vec2::new(5.0, 22.0), Sense::hover());
            let _ = r;
            ui.painter().vline(
                sep.center().x,
                sep.y_range(),
                EStroke::new(1.0, Tokens::border()),
            );
            // The selection's outline and fill become what new objects
            // get.
            let has = !app.selection.is_empty();
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), Sense::click());
            paint_button(ui, rect, false, has && resp.hovered());
            set_default_icon(ui.painter(), rect.shrink(6.0), has);
            if resp.on_hover_text(tr("prp.set_default")).clicked() && has {
                if let Some(first) = app.selected_shapes().first().cloned() {
                    app.set_default_properties(tracedraw_core::DefaultKind::Graphic, |p| {
                        p.fill = first.fill.clone();
                        p.stroke = first.stroke.clone();
                    });
                    app.status = tr("prp.default_set");
                }
            }
        });
    });
}

fn paint_button(ui: &Ui, rect: Rect, on: bool, hovered: bool) {
    if on {
        ui.painter()
            .rect_filled(rect, 3.0, Color32::from_rgb(0x2B, 0x4C, 0x7A));
    } else if hovered {
        ui.painter().rect_filled(rect, 3.0, Tokens::tool_hover());
    }
}

fn paint_icon(p: &egui::Painter, r: Rect, icon: Icon) {
    let ink = Tokens::icon();
    match icon {
        Icon::Pen => pen_icon(p, r, ink),
        Icon::Bucket => bucket_icon(p, r, ink),
        Icon::Checker => checker_icon(p, r),
        Icon::Other(s) => s.paint(p, r),
    }
}

/// A pen nib.
fn pen_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let c = r.center();
    let h = r.height() / 2.0;
    let pts = vec![
        c + Vec2::new(0.0, -h),
        c + Vec2::new(h * 0.62, h * 0.05),
        c + Vec2::new(h * 0.3, h * 0.75),
        c + Vec2::new(-h * 0.3, h * 0.75),
        c + Vec2::new(-h * 0.62, h * 0.05),
        c + Vec2::new(0.0, -h),
    ];
    p.add(egui::Shape::line(pts, EStroke::new(1.4, ink)));
    p.line_segment(
        [c + Vec2::new(0.0, -h), c + Vec2::new(0.0, h * 0.15)],
        EStroke::new(1.2, ink),
    );
    p.circle_filled(c + Vec2::new(0.0, h * 0.2), 1.6, ink);
    p.line_segment(
        [c + Vec2::new(-h * 0.35, h), c + Vec2::new(h * 0.35, h)],
        EStroke::new(1.4, ink),
    );
}

/// A paint bucket pouring orange paint.
fn bucket_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let c = r.center() + Vec2::new(-1.5, 0.0);
    let s = r.height() / 2.0;
    let body = vec![
        c + Vec2::new(-s * 0.8, -s * 0.2),
        c + Vec2::new(s * 0.6, -s * 0.2),
        c + Vec2::new(s * 0.4, s * 0.85),
        c + Vec2::new(-s * 0.6, s * 0.85),
    ];
    p.add(egui::epaint::PathShape::convex_polygon(
        body.clone(),
        Color32::TRANSPARENT,
        EStroke::new(1.4, ink),
    ));
    // The paint inside, orange.
    p.add(egui::epaint::PathShape::convex_polygon(
        vec![
            c + Vec2::new(-s * 0.65, s * 0.25),
            c + Vec2::new(s * 0.5, s * 0.25),
            c + Vec2::new(s * 0.38, s * 0.75),
            c + Vec2::new(-s * 0.52, s * 0.75),
        ],
        Color32::from_rgb(0xE8, 0x6A, 0x2C),
        EStroke::NONE,
    ));
    p.circle_stroke(
        c + Vec2::new(-0.1, -s * 0.45),
        s * 0.45,
        EStroke::new(1.2, ink),
    );
    p.circle_filled(
        c + Vec2::new(s * 0.95, s * 0.75),
        s * 0.22,
        Color32::from_rgb(0xE8, 0x6A, 0x2C),
    );
}

/// A checkerboard: transparency.
fn checker_icon(p: &egui::Painter, r: Rect) {
    let n = 5;
    let cell = r.width() / n as f32;
    p.rect_filled(r, 1.0, Color32::from_gray(235));
    for y in 0..n {
        for x in 0..n {
            if (x + y) % 2 == 0 {
                p.rect_filled(
                    Rect::from_min_size(
                        r.min + Vec2::new(x as f32 * cell, y as f32 * cell),
                        Vec2::splat(cell),
                    ),
                    0.0,
                    Color32::from_gray(40),
                );
            }
        }
    }
    p.rect_stroke(
        r,
        1.0,
        EStroke::new(1.0, Tokens::icon()),
        egui::StrokeKind::Outside,
    );
}

/// A tray with an arrow into it.
fn set_default_icon(p: &egui::Painter, r: Rect, enabled: bool) {
    let ink = if enabled {
        Tokens::icon()
    } else {
        Tokens::icon_off()
    };
    let s = EStroke::new(1.3, ink);
    p.line_segment([r.left_top() + Vec2::new(0.0, 3.0), r.left_bottom()], s);
    p.line_segment([r.left_bottom(), r.right_bottom()], s);
    p.line_segment([r.right_bottom(), r.right_top() + Vec2::new(0.0, 3.0)], s);
    p.line_segment(
        [
            r.left_top() + Vec2::new(0.0, 3.0),
            r.right_top() + Vec2::new(0.0, 3.0),
        ],
        s,
    );
    let c = r.center_bottom() + Vec2::new(0.0, -3.0);
    p.line_segment([c + Vec2::new(0.0, -8.0), c], s);
    p.line_segment([c, c + Vec2::new(-3.0, -3.0)], s);
    p.line_segment([c, c + Vec2::new(3.0, -3.0)], s);
}

/// A page with a crossed eye: only the section picked.
fn single_icon(p: &egui::Painter, r: Rect) {
    let s = EStroke::new(1.3, Tokens::icon());
    let page = Rect::from_min_max(r.min + Vec2::new(0.0, 3.0), r.max - Vec2::new(4.0, 0.0));
    p.rect_stroke(page, 0.0, s, egui::StrokeKind::Inside);
    let e = r.right_top() + Vec2::new(-3.0, 3.0);
    p.circle_stroke(e, 3.0, s);
    p.line_segment([e + Vec2::new(-4.0, 4.0), e + Vec2::new(4.0, -4.0)], s);
}

// ----- sections -------------------------------------------------------------------

/// The "^ Title" header of a section; a click opens or closes it. True
/// when open (and shown: with Only the section picked, the others hide).
fn header(app: &mut App, ui: &mut Ui, s: &Section) -> bool {
    let single = app.settings.properties_closed.iter().any(|c| c == "single");
    let active: Option<String> = ui.data(|d| d.get_temp(active_id()));
    if single && active.as_deref().is_some_and(|a| a != s.id) {
        return false;
    }
    let open = is_open(app, s.id);
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), Sense::click());
    let ink = if resp.hovered() {
        Tokens::text()
    } else {
        Tokens::text_dim()
    };
    chevron(
        ui.painter(),
        Pos2::new(rect.left() + 10.0, rect.center().y),
        open,
        ink,
    );
    ui.painter().text(
        Pos2::new(rect.left() + 22.0, rect.center().y),
        Align2::LEFT_CENTER,
        tr(s.key),
        crate::theme::bold(13.0),
        Tokens::text(),
    );
    let jump: Option<String> = ui.data(|d| d.get_temp(jump_id()));
    if jump.as_deref() == Some(s.id) {
        ui.data_mut(|d| d.remove_temp::<String>(jump_id()));
        // The animated scroll of egui's scroll areas.
        ui.scroll_to_rect(rect, Some(egui::Align::TOP));
    }
    if resp.clicked() {
        set_open(app, s.id, !open);
        ui.data_mut(|d| d.insert_temp(active_id(), s.id.to_string()));
        return !open;
    }
    open
}

/// The small "^" (open) or "v" (closed) of a header.
fn chevron(p: &egui::Painter, c: Pos2, up: bool, ink: Color32) {
    let d = if up { 1.0 } else { -1.0 };
    let s = EStroke::new(1.3, ink);
    p.line_segment(
        [c + Vec2::new(-4.0, 2.0 * d), c + Vec2::new(0.0, -2.0 * d)],
        s,
    );
    p.line_segment(
        [c + Vec2::new(0.0, -2.0 * d), c + Vec2::new(4.0, 2.0 * d)],
        s,
    );
}

fn body(app: &mut App, ui: &mut Ui, id: &str) {
    let shapes = app.selected_shapes();
    match id {
        "outline" => {
            outline_section(app, ui);
            pick_variable_tool(app, ui);
        }
        "fill" => fill_section(app, ui),
        "transparency" => transparency_section(app, ui),
        "qr" => {
            if let Some(first) = shapes.first() {
                if let Some(qr) = &first.qr {
                    dk::qr_section(app, ui, first.id, qr);
                }
            }
        }
        "corners" => {
            if let Some(c) = shapes.first().and_then(|s| s.page_corners()) {
                dk::rectangle_section(app, ui, c);
            }
        }
        "effects" => {
            if let Some(first) = shapes.first() {
                crate::ui::effect_dialog::fx_section(app, ui, first.id);
            }
        }
        "character" => {
            if let Some(ShapeKind::Text { spans, .. }) = shapes.first().map(|s| &s.kind) {
                let chosen = app.text_selection_span();
                if let Some(sp) = chosen.as_ref().or(spans.first()) {
                    ui.label(format!("{} {} pt", sp.font_family, sp.size_pt));
                }
                ui.label(
                    egui::RichText::new(tr("docker.edit_with_text_tool"))
                        .color(Tokens::text_dim())
                        .size(11.0),
                );
            }
        }
        _ => {
            let b = app.selection_bounds().unwrap_or_default();
            let o = app.to_ruler(tracedraw_core::geometry::Point::new(b.x0, b.y0));
            ui.label(
                egui::RichText::new(format!(
                    "x {:.2}  y {:.2}  w {:.2}  h {:.2} {}",
                    app.units.world_from_mm(o.x),
                    app.units.world_from_mm(o.y),
                    app.units.world_from_mm(b.width()),
                    app.units.world_from_mm(b.height()),
                    app.units.short()
                ))
                .size(11.5),
            );
            for s in &shapes {
                ui.label(format!(
                    "{}  {}",
                    dk::kind_name(&s.kind),
                    s.name.clone().unwrap_or_default()
                ));
            }
        }
    }
}

/// A thin line across the section between its groups.
fn divider(ui: &mut Ui) {
    ui.add_space(4.0);
    let r = ui.available_rect_before_wrap();
    let (line, _) = ui.allocate_exact_size(Vec2::new(r.width() - 6.0, 1.0), Sense::hover());
    ui.painter().hline(
        line.x_range(),
        line.center().y,
        EStroke::new(1.0, Tokens::control_border().gamma_multiply(0.7)),
    );
    ui.add_space(4.0);
}

/// A bold group title with an optional button at the right.
fn group_title(
    ui: &mut Ui,
    text: &str,
    button: Option<(&str, fn(&egui::Painter, Rect, Color32))>,
) -> bool {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width() - 6.0, 24.0), Sense::hover());
    ui.painter().text(
        Pos2::new(rect.left(), rect.center().y),
        Align2::LEFT_CENTER,
        text,
        crate::theme::bold(12.5),
        Tokens::text(),
    );
    let mut clicked = false;
    if let Some((tip, paint)) = button {
        let b = Rect::from_center_size(
            Pos2::new(rect.right() - 10.0, rect.center().y),
            Vec2::splat(20.0),
        );
        let resp = ui
            .interact(b, ui.id().with(("group_btn", text)), Sense::click())
            .on_hover_text(tip);
        if resp.hovered() {
            ui.painter().rect_filled(b, 3.0, Tokens::tool_hover());
        }
        paint(ui.painter(), b.shrink(3.0), Tokens::icon());
        clicked = resp.clicked();
    }
    clicked
}

// ----- outline --------------------------------------------------------------------

/// What the Outline section edits: the selection's outline (its first
/// object's, with the values that differ as mixed), the characters
/// selected in the text being edited, or with nothing selected the
/// outline of new objects.
fn outline_section(app: &mut App, ui: &mut Ui) {
    let lib = crate::ui::line_style::OutlineLib::of(app);
    if app.selection.is_empty() {
        let mut stroke = app.default_stroke();
        let mut touched = Vec::new();
        if outline_editor(
            ui,
            &mut stroke,
            &lib,
            &StrokeMix::default(),
            &mut touched,
            false,
        ) {
            app.set_default_properties(tracedraw_core::DefaultKind::Graphic, |p| p.stroke = stroke);
        }
        return;
    }
    if app.text_selection_active() {
        let mut stroke = app.text_selection_stroke().unwrap_or(None);
        if outline_editor(
            ui,
            &mut stroke,
            &lib,
            &StrokeMix::default(),
            &mut Vec::new(),
            true,
        ) {
            app.text_apply_outline(stroke);
        }
        return;
    }
    let shapes = app.selected_shapes();
    let Some(first) = shapes.first().cloned() else {
        return;
    };
    let base = app.paint_targets().into_iter().next();
    let old = base
        .as_ref()
        .map_or(first.stroke.clone(), |b| b.stroke.clone());
    let mix = app.selection_stroke_mix();
    let mut stroke = old.clone();
    let mut touched = Vec::new();
    if outline_editor(ui, &mut stroke, &lib, &mix, &mut touched, true) {
        let live = app.live_stroke.is_some() || ui.input(|i| i.pointer.any_down());
        if live {
            app.patch_selection_stroke_live(&old, &stroke, &touched);
        } else {
            app.patch_selection_stroke(&old, &stroke, &touched);
        }
    }
    divider(ui);
    dk::extra_strokes(app, ui, &first);
}

/// The flame of Variable Outline picks its tool.
fn pick_variable_tool(app: &mut App, ui: &Ui) {
    let id = egui::Id::new("props_pick_variable_tool");
    if ui.data_mut(|d| d.remove_temp::<bool>(id)).unwrap_or(false) {
        app.set_tool(crate::tools::Tool::VariableOutline);
    }
}

/// The widths the width list offers, in mm.
const WIDTHS: [f64; 12] = [
    0.1, 0.2, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 2.5, 3.0, 5.0, 10.0,
];

/// The outline's controls; true when something changed (`touched` says
/// what). Without `details` (nothing selected: the outline of new
/// objects) it stops after the line style and the arrow, as the reference
/// does; arrowheads, variable outline and calligraphy belong to objects.
fn outline_editor(
    ui: &mut Ui,
    stroke: &mut Option<Stroke>,
    lib: &crate::ui::line_style::OutlineLib,
    mix: &StrokeMix,
    touched: &mut Vec<StrokeField>,
    details: bool,
) -> bool {
    let mut changed = false;
    let mark = |f: StrokeField, touched: &mut Vec<StrokeField>| {
        touched.push(f);
        true
    };
    // The colour: a list's frame with the swatch; no outline shows a
    // crossed swatch, and a colour picked gives the object one.
    ui.horizontal(|ui| {
        let size = Vec2::new(84.0, 24.0);
        match stroke {
            Some(s) => {
                let mut c = s.color;
                let before = ui.cursor().min;
                if crate::ui::color_picker::color_button_sized(ui, &mut c, size, true) {
                    dk::set_outline_paint(s, Fill::Solid(c));
                    changed |= mark(StrokeField::Paint, touched);
                }
                if mix.has(StrokeField::Paint) {
                    crate::ui::field::mixed_swatch(
                        ui,
                        Rect::from_min_size(before + Vec2::new(4.0, 4.0), Vec2::new(58.0, 16.0)),
                    );
                }
            }
            None => {
                let mut c = lib.black;
                let before = ui.cursor().min;
                if crate::ui::color_picker::color_button_sized(ui, &mut c, size, true) {
                    *stroke = Some(Stroke {
                        color: c,
                        ..Stroke::default()
                    });
                    changed |= mark(StrokeField::Paint, touched);
                    touched.push(StrokeField::Width);
                }
                let sw = Rect::from_min_size(before + Vec2::new(4.0, 4.0), Vec2::new(58.0, 16.0));
                ui.painter().rect_filled(sw, 2.0, Color32::WHITE);
                ui.painter().line_segment(
                    [sw.left_bottom(), sw.right_top()],
                    EStroke::new(1.2, Color32::from_rgb(0xE5, 0x39, 0x35)),
                );
            }
        }
    });
    // The width with its list, and the unit.
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        let u = lib.units;
        let width_mixed = mix.has(StrokeField::Width);
        let mut v = u.from_mm(stroke.as_ref().map_or(0.0, |s| s.width));
        ui.spacing_mut().item_spacing.x = 0.0;
        let resp = ui.add(
            crate::ui::field::NumField::new(&mut v)
                .range(u.from_mm(0.001)..=u.from_mm(1000.0))
                .unit_mm(u.mm())
                .max_decimals(3)
                .suffix(format!(" {}", u.short()))
                .no_spin()
                .local_decimals()
                .width(70.0)
                .mixed(width_mixed || stroke.is_none()),
        );
        let (arrow, _) =
            ui.allocate_exact_size(Vec2::new(18.0, resp.rect.height()), Sense::hover());
        let frame = resp.rect.union(arrow);
        // One box round the number and its arrow, as a list.
        ui.painter().rect_stroke(
            frame,
            0.0,
            EStroke::new(1.0, Tokens::control_border()),
            egui::StrokeKind::Inside,
        );
        ui.add_space(8.0);
        if resp.changed() {
            let w = u.to_mm(v).clamp(0.001, 1000.0);
            set_width(stroke, lib, w);
            changed |= mark(StrokeField::Width, touched);
        }
        let list = ui.interact(arrow, ui.id().with("width_list"), Sense::click());
        down_arrow(ui.painter(), arrow.center(), list.hovered());
        let popup = ui.id().with("width_popup");
        if list.clicked() {
            egui::Popup::toggle_id(ui.ctx(), popup);
        }
        egui::Popup::new(popup, ui.ctx().clone(), frame, ui.layer_id())
            .open_memory(None)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
            .show(|ui| {
                ui.set_min_width(110.0);
                if ui
                    .selectable_label(stroke.is_none(), tr("prp.width_none"))
                    .clicked()
                {
                    *stroke = None;
                    changed = true;
                    touched.push(StrokeField::Paint);
                }
                let hair = stroke
                    .as_ref()
                    .is_some_and(|s| s.width <= Stroke::HAIRLINE + 1e-9);
                if ui
                    .selectable_label(hair, tr("prp.width_hairline"))
                    .clicked()
                {
                    set_width(stroke, lib, Stroke::HAIRLINE);
                    changed = true;
                    touched.push(StrokeField::Width);
                }
                for w in WIDTHS {
                    let on = stroke.as_ref().is_some_and(|s| (s.width - w).abs() < 1e-6);
                    let n = fmt(u.from_mm(w), 3);
                    let n = if crate::i18n::decimal_comma() {
                        n.replace('.', ",")
                    } else {
                        n
                    };
                    let text = format!("{n} {}", u.short());
                    if ui.selectable_label(on, text).clicked() {
                        set_width(stroke, lib, w);
                        changed = true;
                        touched.push(StrokeField::Width);
                    }
                }
            });
        // The unit, by its full name.
        let mut picked = None;
        egui::ComboBox::from_id_salt("outline_units_props")
            .icon(combo_chevron)
            .selected_text(u.label())
            .width(96.0)
            .show_ui(ui, |ui| {
                for o in crate::ui::propbar::OUTLINE_UNITS {
                    if ui.selectable_label(o == u, o.label()).clicked() {
                        picked = Some(o);
                    }
                }
            });
        if let Some(nu) = picked.filter(|n| *n != u) {
            crate::ui::line_style::request_units(ui, nu);
        }
    });
    let Some(s) = stroke.as_mut() else {
        return changed;
    };
    // The line style with "...", Edit Line Style.
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        if line_style_list(ui, s, lib, mix.has(StrokeField::Dash)) {
            changed |= mark(StrokeField::Dash, touched);
        }
        let dots = ui
            .add(egui::Button::new("\u{2022}\u{2022}\u{2022}").frame(false))
            .on_hover_text(tr("docker.edit_line_style"));
        if dots.clicked() {
            crate::ui::line_style::request(ui);
        }
    });
    // How dashes fit: as they come, fitted to corners and ends, or set in
    // Edit Line Style.
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let dashed = !s.dash.is_empty();
        for (i, tip) in ["prp.dash_default", "prp.dash_corners", "prp.dash_custom"]
            .iter()
            .enumerate()
        {
            let on = dashed
                && !mix.has(StrokeField::AlignDashes)
                && ((i == 0 && !s.align_dashes) || (i == 1 && s.align_dashes));
            let (r, resp) = ui.allocate_exact_size(Vec2::new(26.0, 22.0), Sense::click());
            paint_button(ui, r, on, dashed && resp.hovered());
            dash_fit_icon(ui.painter(), r.shrink(4.0), i, dashed);
            if dashed && resp.on_hover_text(tr(tip)).clicked() {
                match i {
                    0 | 1 => {
                        s.align_dashes = i == 1;
                        changed |= mark(StrokeField::AlignDashes, touched);
                    }
                    _ => crate::ui::line_style::request(ui),
                }
            }
        }
    });
    // The arrow that hides the lower rows.
    let more = ui
        .data(|d| d.get_temp::<bool>(egui::Id::new("props_outline_more")))
        .unwrap_or(false);
    let (r, resp) =
        ui.allocate_exact_size(Vec2::new(ui.available_width() - 6.0, 14.0), Sense::click());
    chevron(
        ui.painter(),
        r.center(),
        more,
        if resp.hovered() {
            Tokens::text()
        } else {
            Tokens::text_dim()
        },
    );
    if resp
        .on_hover_text(tr(if more { "prp.less" } else { "prp.more" }))
        .clicked()
    {
        ui.data_mut(|d| d.insert_temp(egui::Id::new("props_outline_more"), !more));
    }
    if more {
        labelled(ui, &tr("prp.miter_limit"), |ui| {
            if ui
                .add(
                    crate::ui::field::NumField::new(&mut s.miter_limit)
                        .range(Stroke::MITER_LIMIT_RANGE)
                        .speed(0.5)
                        .max_decimals(1)
                        .width(80.0)
                        .mixed(mix.has(StrokeField::MiterLimit)),
                )
                .on_hover_text(tr("docker.miter_limit_tip"))
                .changed()
            {
                changed |= mark(StrokeField::MiterLimit, touched);
            }
        });
        labelled(ui, &tr("prp.corners"), |ui| {
            let mixed = mix.has(StrokeField::Join);
            for (i, (j, tip)) in [
                (LineJoin::Miter, "docker.join_miter"),
                (LineJoin::Round, "docker.join_round"),
                (LineJoin::Bevel, "docker.join_bevel"),
            ]
            .into_iter()
            .enumerate()
            {
                if icon_toggle(ui, !mixed && s.join == j, &tr(tip), |p, r, c| {
                    join_icon(p, r, i, c)
                }) {
                    s.join = j;
                    changed |= mark(StrokeField::Join, touched);
                }
            }
        });
        labelled(ui, &tr("prp.line_ends"), |ui| {
            let mixed = mix.has(StrokeField::Cap);
            for (i, (cap, tip)) in [
                (LineCap::Butt, "docker.cap_butt"),
                (LineCap::Round, "docker.cap_round"),
                (LineCap::Square, "docker.cap_square"),
            ]
            .into_iter()
            .enumerate()
            {
                if icon_toggle(ui, !mixed && s.cap == cap, &tr(tip), |p, r, c| {
                    cap_icon(p, r, i, c)
                }) {
                    s.cap = cap;
                    changed |= mark(StrokeField::Cap, touched);
                }
            }
        });
        labelled(ui, &tr("prp.position"), |ui| {
            let mixed = mix.has(StrokeField::Position);
            for (i, (pos, tip)) in [
                (OutlinePosition::Outside, "docker.position_outside"),
                (OutlinePosition::Center, "docker.position_center"),
                (OutlinePosition::Inside, "docker.position_inside"),
            ]
            .into_iter()
            .enumerate()
            {
                if icon_toggle(ui, !mixed && s.position == pos, &tr(tip), |p, r, c| {
                    position_icon(p, r, i, c)
                }) {
                    s.position = pos;
                    changed |= mark(StrokeField::Position, touched);
                }
            }
        });
    }

    if !details {
        return changed;
    }
    // Arrowheads.
    divider(ui);
    group_title(ui, &tr("prp.arrowheads"), None);
    let share_id = egui::Id::new("props_share_arrows");
    let mut share = ui.data(|d| d.get_temp::<bool>(share_id)).unwrap_or(false);
    for (field, left) in [
        (StrokeField::StartArrow, true),
        (StrokeField::EndArrow, false),
    ] {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (r, _) = ui.allocate_exact_size(Vec2::new(16.0, 22.0), Sense::hover());
            arrow_glyph(ui.painter(), r.center(), left, Tokens::icon());
            let a = if left {
                &mut s.start_arrow
            } else {
                &mut s.end_arrow
            };
            if arrow_list(
                ui,
                if left { "start" } else { "end" },
                a,
                &lib.arrows,
                mix.has(field),
            ) {
                changed |= mark(field, touched);
                if share {
                    let v = a.clone();
                    if left {
                        s.end_arrow = v;
                        touched.push(StrokeField::EndArrow);
                    } else {
                        s.start_arrow = v;
                        touched.push(StrokeField::StartArrow);
                    }
                }
            }
            let menu = ui.id().with(("arrow_menu", left));
            let dots = ui
                .add(egui::Button::new("\u{2022}\u{2022}\u{2022}").frame(false))
                .on_hover_text(tr("prp.arrow_options"));
            if dots.clicked() {
                egui::Popup::toggle_id(ui.ctx(), menu);
            }
            egui::Popup::new(menu, ui.ctx().clone(), dots.rect, ui.layer_id())
                .open_memory(None)
                .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
                .show(|ui| {
                    if ui.button(tr("prp.arrow_swap")).clicked() {
                        std::mem::swap(&mut s.start_arrow, &mut s.end_arrow);
                        touched.push(StrokeField::StartArrow);
                        touched.push(StrokeField::EndArrow);
                        changed = true;
                    }
                    if ui.button(tr("prp.arrow_none")).clicked() {
                        if left {
                            s.start_arrow = Default::default();
                            touched.push(StrokeField::StartArrow);
                        } else {
                            s.end_arrow = Default::default();
                            touched.push(StrokeField::EndArrow);
                        }
                        changed = true;
                    }
                });
        });
    }
    ui.horizontal(|ui| {
        ui.add_space(24.0);
        if ui
            .checkbox(&mut share, tr("prp.share_attributes"))
            .changed()
        {
            ui.data_mut(|d| d.insert_temp(share_id, share));
        }
    });

    // Variable outline.
    divider(ui);
    if group_title(
        ui,
        &tr("prp.variable_outline"),
        Some((&tr("tool.variable_outline"), flame_icon)),
    ) {
        ui.data_mut(|d| d.insert_temp(egui::Id::new("props_pick_variable_tool"), true));
    }
    changed |= variable_outline(ui, s, touched);

    // Calligraphy.
    divider(ui);
    if group_title(
        ui,
        &tr("prp.calligraphy"),
        Some((&tr("prp.reset"), reset_icon)),
    ) {
        s.stretch = 1.0;
        s.nib_angle = 0.0;
        touched.push(StrokeField::Stretch);
        touched.push(StrokeField::NibAngle);
        changed = true;
    }
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(16.0, 22.0), Sense::hover());
                stretch_glyph(ui.painter(), r.center(), Tokens::icon());
                let mut pct = (s.stretch * 100.0).round();
                if ui
                    .add(
                        crate::ui::field::NumField::new(&mut pct)
                            .range(1.0..=100.0)
                            .max_decimals(0)
                            .width(190.0)
                            .mixed(mix.has(StrokeField::Stretch)),
                    )
                    .changed()
                {
                    s.stretch = (pct / 100.0).clamp(0.01, 1.0);
                    changed |= mark(StrokeField::Stretch, touched);
                }
            });
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::new(16.0, 22.0), Sense::hover());
                angle_glyph(ui.painter(), r.center(), Tokens::icon());
                if ui
                    .add(
                        crate::ui::field::NumField::new(&mut s.nib_angle)
                            .range(-90.0..=90.0)
                            .max_decimals(1)
                            .width(190.0)
                            .mixed(mix.has(StrokeField::NibAngle)),
                    )
                    .changed()
                {
                    changed |= mark(StrokeField::NibAngle, touched);
                }
            });
        });
        let (r, _) = ui.allocate_exact_size(Vec2::new(50.0, 50.0), Sense::hover());
        nib_preview(ui.painter(), r, s.stretch, s.nib_angle);
    });
    for (field, key) in [
        (StrokeField::BehindFill, "prp.behind_fill"),
        (StrokeField::ScaleWithObject, "prp.scale_with_object"),
    ] {
        let v = if field == StrokeField::BehindFill {
            &mut s.behind_fill
        } else {
            &mut s.scale_with_object
        };
        if ui
            .add(egui::Checkbox::new(v, tr(key)).indeterminate(mix.has(field)))
            .changed()
        {
            changed |= mark(field, touched);
        }
    }
    let mut overprint = false;
    ui.add_enabled(
        false,
        egui::Checkbox::new(&mut overprint, tr("prp.overprint_outline")),
    )
    .on_disabled_hover_text(tr("prp.overprint_outline_tip"));
    changed
}

fn set_width(stroke: &mut Option<Stroke>, lib: &crate::ui::line_style::OutlineLib, w: f64) {
    match stroke {
        Some(s) => s.width = w,
        None => {
            *stroke = Some(Stroke {
                color: lib.black,
                width: w,
                ..Stroke::default()
            })
        }
    }
}

/// A number with up to `d` decimals, a comma for the point.
fn fmt(v: f64, d: usize) -> String {
    let s = format!("{v:.d$}");
    let s = if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    };
    s.replace('.', ",")
}

/// A row whose label ends at [`LABEL_END`], its controls after.
fn labelled(ui: &mut Ui, label: &str, add: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(LABEL_END, ROW), Sense::hover());
        ui.painter().text(
            Pos2::new(r.right(), r.center().y),
            Align2::RIGHT_CENTER,
            label,
            FontId::proportional(12.5),
            Tokens::text(),
        );
        ui.spacing_mut().item_spacing.x = 8.0;
        add(ui);
    });
}

/// A small icon button lit when `on`; true when clicked.
fn icon_toggle(
    ui: &mut Ui,
    on: bool,
    tip: &str,
    paint: impl FnOnce(&egui::Painter, Rect, Color32),
) -> bool {
    let (r, resp) = ui.allocate_exact_size(Vec2::new(26.0, 24.0), Sense::click());
    paint_button(ui, r, on, resp.hovered());
    paint(ui.painter(), r.shrink(4.0), Tokens::icon());
    resp.on_hover_text(tip).clicked()
}

/// The thin "v" at the right of the docker's lists.
fn combo_chevron(ui: &Ui, rect: Rect, visuals: &egui::style::WidgetVisuals, _open: bool) {
    let c = rect.center();
    let s = EStroke::new(1.2, visuals.fg_stroke.color);
    let p = ui.painter();
    p.line_segment([c + Vec2::new(-4.0, -2.0), c + Vec2::new(0.0, 2.0)], s);
    p.line_segment([c + Vec2::new(0.0, 2.0), c + Vec2::new(4.0, -2.0)], s);
}

fn down_arrow(p: &egui::Painter, c: Pos2, hovered: bool) {
    let ink = if hovered {
        Tokens::text()
    } else {
        Tokens::icon()
    };
    let s = EStroke::new(1.3, ink);
    p.line_segment([c + Vec2::new(-4.0, -2.0), c + Vec2::new(0.0, 2.0)], s);
    p.line_segment([c + Vec2::new(0.0, 2.0), c + Vec2::new(4.0, -2.0)], s);
}

/// The line style list: the current dash pattern drawn, the solid line,
/// the usual dashes and the user's own (Edit Line Style).
fn line_style_list(
    ui: &mut Ui,
    s: &mut Stroke,
    lib: &crate::ui::line_style::OutlineLib,
    mixed: bool,
) -> bool {
    let mut changed = false;
    let (frame, resp) = ui.allocate_exact_size(Vec2::new(190.0, 24.0), Sense::click());
    ui.painter().rect(
        frame,
        3.0,
        if resp.hovered() {
            Tokens::tool_hover()
        } else {
            Tokens::control_fill()
        },
        EStroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    if !mixed {
        dash_line(
            ui.painter(),
            Rect::from_min_max(
                frame.min + Vec2::new(6.0, 0.0),
                Pos2::new(frame.right() - 26.0, frame.bottom()),
            ),
            &s.dash,
        );
    }
    down_arrow(
        ui.painter(),
        Pos2::new(frame.right() - 12.0, frame.center().y),
        resp.hovered(),
    );
    let popup = ui.id().with("line_style_popup");
    if resp.clicked() {
        egui::Popup::toggle_id(ui.ctx(), popup);
    }
    let mut styles: Vec<Vec<f64>> = vec![
        vec![],
        vec![4.0, 2.0],
        vec![1.0, 1.0],
        vec![6.0, 2.0, 1.0, 2.0],
        vec![8.0, 3.0],
        vec![2.0, 2.0],
        vec![6.0, 2.0, 1.0, 2.0, 1.0, 2.0],
    ];
    for d in &lib.dashes {
        if !styles.contains(d) {
            styles.push(d.clone());
        }
    }
    egui::Popup::new(popup, ui.ctx().clone(), frame, ui.layer_id())
        .open_memory(None)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
        .show(|ui| {
            for d in &styles {
                let (r, row) =
                    ui.allocate_exact_size(Vec2::new(frame.width(), 20.0), Sense::click());
                if row.hovered() || (!mixed && s.dash == *d) {
                    ui.painter().rect_filled(r, 2.0, Tokens::tool_hover());
                }
                dash_line(ui.painter(), r.shrink2(Vec2::new(6.0, 0.0)), d);
                if row.clicked() {
                    s.dash = d.clone();
                    changed = true;
                }
            }
        });
    changed
}

/// A dash pattern drawn across `r`.
fn dash_line(p: &egui::Painter, r: Rect, dash: &[f64]) {
    let y = r.center().y;
    let st = EStroke::new(2.0, Tokens::text());
    if dash.is_empty() {
        p.hline(r.x_range(), y, st);
        return;
    }
    let unit = 2.0f32;
    let mut x = r.left();
    let mut i = 0;
    while x < r.right() {
        let len = (dash[i % dash.len()] as f32 * unit).max(1.0);
        if i % 2 == 0 {
            p.hline(x..=(x + len).min(r.right()), y, st);
        }
        x += len;
        i += 1;
    }
}

/// The arrowhead list: none, the built-in heads and the user's own.
fn arrow_list(
    ui: &mut Ui,
    id: &str,
    a: &mut tracedraw_core::Arrowhead,
    custom: &[tracedraw_core::Arrowhead],
    mixed: bool,
) -> bool {
    let mut changed = false;
    let text = if mixed {
        String::new()
    } else if a.name().is_empty() || *a == tracedraw_core::Arrowhead::default() {
        "\u{2014}".to_string()
    } else {
        a.name().to_string()
    };
    egui::ComboBox::from_id_salt(("props_arrow", id))
        .icon(combo_chevron)
        .selected_text(text)
        .width(60.0)
        .show_ui(ui, |ui| {
            for k in tracedraw_core::Arrowhead::ALL {
                if ui.selectable_label(!mixed && *a == k, k.name()).clicked() {
                    *a = k;
                    changed = true;
                }
            }
            for (i, k) in custom.iter().enumerate() {
                let name = if k.name().is_empty() {
                    crate::i18n::trf("docker.custom_arrowhead_n", &[("n", &(i + 1).to_string())])
                } else {
                    k.name().to_string()
                };
                if ui.selectable_label(!mixed && a == k, name).clicked() {
                    *a = k.clone();
                    changed = true;
                }
            }
        });
    changed
}

/// Variable outline: a ruler with the width nodes along the outline,
/// and the picked node's place and reach on each side. Without nodes it
/// is greyed, as in the reference; the Variable Outline tool adds them.
fn variable_outline(ui: &mut Ui, s: &mut Stroke, touched: &mut Vec<StrokeField>) -> bool {
    let mut changed = false;
    let has = !s.widths.is_empty();
    let sel_id = egui::Id::new("props_width_node");
    let mut sel: usize = ui.data(|d| d.get_temp(sel_id)).unwrap_or(0);
    if sel >= s.widths.len() {
        sel = 0;
    }
    let (r, resp) =
        ui.allocate_exact_size(Vec2::new(ui.available_width() - 40.0, 30.0), Sense::click());
    let ink = if has {
        Tokens::text()
    } else {
        Tokens::text_dim()
    };
    let line = r.shrink2(Vec2::new(6.0, 0.0));
    ui.painter()
        .hline(line.x_range(), r.center().y, EStroke::new(2.0, ink));
    for k in 0..5 {
        let x = line.left() + line.width() * k as f32 / 4.0;
        ui.painter().vline(
            x,
            (r.center().y - 7.0)..=(r.center().y + 7.0),
            EStroke::new(1.2, ink),
        );
    }
    for (i, n) in s.widths.iter().enumerate() {
        let x = line.left() + line.width() * n.at as f32;
        let c = Pos2::new(x, r.center().y);
        let fill = if i == sel {
            Tokens::ACCENT
        } else {
            Tokens::field()
        };
        ui.painter().circle(c, 4.0, fill, EStroke::new(1.0, ink));
    }
    if has && resp.clicked() {
        if let Some(p) = resp.interact_pointer_pos() {
            let at = ((p.x - line.left()) / line.width()) as f64;
            sel = s
                .widths
                .iter()
                .enumerate()
                .min_by(|a, b| (a.1.at - at).abs().total_cmp(&(b.1.at - at).abs()))
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }
    ui.data_mut(|d| d.insert_temp(sel_id, sel));
    let node = s
        .widths
        .get(sel)
        .copied()
        .unwrap_or(tracedraw_core::WidthNode {
            at: 0.0,
            left: 0.0,
            right: 0.0,
        });
    let mut at = node.at * 100.0;
    let mut left = node.left;
    let mut right = node.right;
    let edit = |ui: &mut Ui,
                label: &str,
                v: &mut f64,
                suffix: &str,
                range: std::ops::RangeInclusive<f64>|
     -> bool {
        let mut hit = false;
        ui.horizontal(|ui| {
            let (lr, _) = ui.allocate_exact_size(Vec2::new(58.0, 22.0), Sense::hover());
            ui.painter().text(
                Pos2::new(lr.right(), lr.center().y),
                Align2::RIGHT_CENTER,
                label,
                FontId::proportional(12.0),
                if has {
                    Tokens::text()
                } else {
                    Tokens::text_dim()
                },
            );
            hit = ui
                .add_enabled(
                    has,
                    crate::ui::field::NumField::new(v)
                        .range(range)
                        .max_decimals(1)
                        .suffix(suffix)
                        .width(70.0),
                )
                .changed();
        });
        hit
    };
    let mut moved = edit(ui, &tr("prp.var_position"), &mut at, " %", 0.0..=100.0);
    moved |= edit(ui, &tr("prp.var_side1"), &mut left, " mm", 0.0..=1000.0);
    let side2 = edit(ui, &tr("prp.var_side2"), &mut right, " mm", 0.0..=1000.0);
    moved |= side2;
    let mut total = left + right;
    if edit(ui, &tr("prp.var_total"), &mut total, " mm", 0.0..=2000.0) {
        let half = total / 2.0;
        left = half;
        right = half;
        moved = true;
    }
    if moved && has {
        if let Some(n) = s.widths.get_mut(sel) {
            n.at = (at / 100.0).clamp(0.0, 1.0);
            n.left = left.max(0.0);
            n.right = right.max(0.0);
        }
        touched.push(StrokeField::Widths);
        changed = true;
    }
    let mut scale = false;
    ui.add_enabled(
        false,
        egui::Checkbox::new(&mut scale, tr("prp.scale_width_brush")),
    );
    changed
}

// ----- fill -----------------------------------------------------------------------

fn fill_section(app: &mut App, ui: &mut Ui) {
    if app.selection.is_empty() {
        let mut fill = app.default_fill();
        if dk::fill_editor_mixed(ui, &mut fill, &FillMix::default(), &mut Vec::new()) {
            app.set_default_properties(tracedraw_core::DefaultKind::Graphic, |p| {
                p.fill = fill.clone()
            });
        }
        if matches!(fill, Fill::None) {
            note(ui, &tr("prp.default_no_fill"));
        }
        return;
    }
    if app.text_selection_active() {
        let mut fill = app.text_selection_fill().unwrap_or(Fill::None);
        if dk::fill_editor_mixed(ui, &mut fill, &FillMix::default(), &mut Vec::new()) {
            app.text_apply_fill(fill);
        }
        return;
    }
    let shapes = app.selected_shapes();
    let Some(first) = shapes.first().cloned() else {
        return;
    };
    let base = app.paint_targets().into_iter().next();
    let old = base.as_ref().map_or(first.fill.clone(), |b| b.fill.clone());
    let mix = app.selection_fill_mix();
    let mut fill = old.clone();
    let mut touched: Vec<FillField> = Vec::new();
    if dk::fill_editor_mixed(ui, &mut fill, &mix, &mut touched) {
        let live = app.live_fill.is_some() || ui.input(|i| i.pointer.any_down());
        if live && !matches!(fill, Fill::Fountain(_)) {
            app.apply_fill_live(fill.clone());
        } else {
            app.patch_selection_fill(&old, &fill, &touched);
        }
    }
    if matches!(fill, Fill::None) && !mix.kind {
        note(ui, &tr("prp.no_fill"));
    }
    divider(ui);
    dk::extra_fills(app, ui, &first);
}

fn note(ui: &mut Ui, text: &str) {
    divider(ui);
    ui.label(egui::RichText::new(text).size(12.5).color(Tokens::text()));
}

// ----- transparency ---------------------------------------------------------------

/// Transparency: the merge mode, the kinds as pictures (none, uniform,
/// fountain, pattern, texture), then the kind's own settings and what it
/// applies to. With nothing selected it sets what transparency the tool
/// starts with.
fn transparency_section(app: &mut App, ui: &mut Ui) {
    use crate::mixed::{transparency_mixed, TransparencyField as F};
    let ts = app.selection_transparency();
    let mixed = transparency_mixed(&ts);
    let has_selection = !ts.is_empty();
    let (mut set, current) = match ts.first() {
        Some(t) => {
            let mut set = app.transparency_default.clone();
            if t.kind >= 1 {
                set.kind = (t.kind - 1) as u8;
            }
            if t.kind == 1 {
                set.amount = t.amount.round();
            }
            if let Some(m) = &t.mask {
                set.mask = m.clone();
                set.merge = t.merge;
                set.target = t.target;
            }
            (set, (!mixed.contains(&F::Kind)).then_some(t.kind))
        }
        None => (app.transparency_default.clone(), Some(0)),
    };
    let mut field = None;
    // The merge mode across the section.
    let merge_shown = !mixed.contains(&F::Merge);
    let w = ui.available_width() - 8.0;
    egui::ComboBox::from_id_salt("props_transparency_merge")
        .icon(combo_chevron)
        .selected_text(if merge_shown {
            set.merge.name().to_string()
        } else {
            String::new()
        })
        .width(w)
        .show_ui(ui, |ui| {
            for m in tracedraw_core::live::MergeMode::ALL {
                if ui
                    .selectable_label(merge_shown && set.merge == m, m.name())
                    .clicked()
                {
                    set.merge = m;
                    field = Some(F::Merge);
                }
            }
        });
    divider(ui);
    let kinds = [
        "transparency.none",
        "transparency.uniform",
        "transparency.fountain",
        "transparency.pattern",
        "transparency.texture",
    ];
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for (i, key) in kinds.iter().enumerate() {
            let on = current == Some(i);
            let (r, resp) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), Sense::click());
            paint_button(ui, r, on, resp.hovered());
            transparency_kind_icon(ui.painter(), r.shrink(6.0), i);
            if resp.on_hover_text(tr(key)).clicked() && !on {
                if i == 0 {
                    if has_selection {
                        app.clear_transparency();
                    }
                } else {
                    set.set_kind((i - 1) as u8);
                    field = Some(F::Kind);
                }
            }
        }
    });
    let shown_kind = current.filter(|k| *k > 0);
    if let Some(k) = shown_kind {
        ui.add_space(4.0);
        if k == 1 {
            let mut amount = set.amount;
            let slider = egui::Slider::new(&mut amount, 0.0..=100.0).suffix(" %");
            if ui.add(crate::ui::Rail(slider)).changed() {
                set.amount = amount;
                field = Some(F::Amount);
            }
        } else if k == 2 {
            if let Fill::Fountain(f) = &mut set.mask {
                let mut angle = f.angle;
                ui.horizontal(|ui| {
                    ui.label(tr("prp.angle"));
                    if ui
                        .add(
                            crate::ui::field::NumField::new(&mut angle)
                                .suffix("°")
                                .speed(1.0)
                                .mixed(mixed.contains(&F::MaskAngle)),
                        )
                        .changed()
                    {
                        f.angle = angle;
                        field = Some(F::MaskAngle);
                    }
                });
            }
        }
        let targets = [
            tr("transparency.target_fill"),
            tr("transparency.target_outline"),
            tr("transparency.target_all"),
        ];
        let target_shown = !mixed.contains(&F::Target);
        ui.horizontal(|ui| {
            ui.label(tr("prp.applies_to"));
            egui::ComboBox::from_id_salt("props_transparency_target")
                .icon(combo_chevron)
                .selected_text(if target_shown {
                    targets[(set.target as usize).min(2)].clone()
                } else {
                    String::new()
                })
                .width(110.0)
                .show_ui(ui, |ui| {
                    for (i, t) in targets.iter().enumerate() {
                        if ui
                            .selectable_label(target_shown && set.target as usize == i, t)
                            .clicked()
                        {
                            set.target = i as u8;
                            field = Some(F::Target);
                        }
                    }
                });
        });
    }
    if let Some(f) = field {
        app.transparency_default = set.clone();
        if has_selection {
            app.patch_transparency(&set, &[f]);
        }
    }
}

// ----- small pictures -------------------------------------------------------------

/// The transparency kinds: none (a checker with a red dot), uniform,
/// fountain, pattern and texture over a checker.
fn transparency_kind_icon(p: &egui::Painter, r: Rect, kind: usize) {
    // A checker behind, and a card in front showing the kind.
    let back = Rect::from_min_size(r.min, r.size() * 0.72);
    checker_icon(p, back);
    if kind == 0 {
        p.circle_filled(
            r.right_top() + Vec2::new(-2.0, 2.0),
            3.2,
            Color32::from_rgb(0xE5, 0x39, 0x35),
        );
        return;
    }
    let card = Rect::from_min_max(r.min + r.size() * 0.32, r.max);
    p.rect_filled(card, 1.0, Color32::from_gray(30));
    let inner = card.shrink(1.5);
    match kind {
        1 => {
            p.rect_filled(inner, 0.0, Color32::from_gray(200));
        }
        2 => {
            let n = 6;
            for k in 0..n {
                let x0 = inner.left() + inner.width() * k as f32 / n as f32;
                let g = (60.0 + 180.0 * k as f32 / n as f32) as u8;
                p.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(x0, inner.top()),
                        Vec2::new(inner.width() / n as f32 + 0.5, inner.height()),
                    ),
                    0.0,
                    Color32::from_gray(g),
                );
            }
        }
        3 => {
            p.rect_filled(inner, 0.0, Color32::from_gray(200));
            for k in 1..3 {
                let x = inner.left() + inner.width() * k as f32 / 3.0;
                p.vline(
                    x,
                    inner.y_range(),
                    EStroke::new(1.0, Color32::from_gray(40)),
                );
                let y = inner.top() + inner.height() * k as f32 / 3.0;
                p.hline(
                    inner.x_range(),
                    y,
                    EStroke::new(1.0, Color32::from_gray(40)),
                );
            }
        }
        _ => {
            p.rect_filled(inner, 0.0, Color32::from_gray(200));
            for (dx, dy) in [
                (0.25, 0.3),
                (0.7, 0.25),
                (0.45, 0.65),
                (0.8, 0.8),
                (0.15, 0.85),
            ] {
                p.circle_filled(
                    inner.min + Vec2::new(inner.width() * dx, inner.height() * dy),
                    1.2,
                    Color32::from_gray(40),
                );
            }
        }
    }
}

/// The dash fittings: an outline corner drawn as it comes, fitted, set
/// by hand.
fn dash_fit_icon(p: &egui::Painter, r: Rect, i: usize, enabled: bool) {
    let ink = if enabled {
        Tokens::icon()
    } else {
        Tokens::icon_off()
    };
    let s = EStroke::new(3.0, ink);
    let o = r.left_top() + Vec2::new(2.0, 2.0);
    let (h, v): (Vec<(f32, f32)>, Vec<(f32, f32)>) = match i {
        0 => (vec![(0.0, 12.0)], vec![(0.0, 9.0)]),
        1 => (vec![(0.0, 5.0), (8.0, 13.0)], vec![(0.0, 5.0), (8.0, 10.0)]),
        _ => (
            vec![(0.0, 3.0), (6.0, 9.0), (12.0, 14.0)],
            vec![(0.0, 3.0), (6.0, 9.0)],
        ),
    };
    for (a, b) in h {
        p.hline((o.x + a)..=(o.x + b), o.y, s);
    }
    for (a, b) in v {
        p.vline(o.x, (o.y + a)..=(o.y + b), s);
    }
}

/// The corners: mitred, round, bevelled.
fn join_icon(p: &egui::Painter, r: Rect, i: usize, ink: Color32) {
    let body = Color32::from_gray(140);
    let o = r.left_top() + Vec2::new(2.0, 3.0);
    let w = r.width() - 2.0;
    let t = 7.0;
    match i {
        0 => {
            p.rect_filled(Rect::from_min_size(o, Vec2::new(w, t)), 0.0, body);
            p.rect_filled(
                Rect::from_min_size(o, Vec2::new(t, r.height() - 3.0)),
                0.0,
                body,
            );
        }
        1 => {
            p.rect_filled(
                Rect::from_min_size(o + Vec2::new(t / 2.0, 0.0), Vec2::new(w - t / 2.0, t)),
                0.0,
                body,
            );
            p.rect_filled(
                Rect::from_min_size(
                    o + Vec2::new(0.0, t / 2.0),
                    Vec2::new(t, r.height() - 3.0 - t / 2.0),
                ),
                0.0,
                body,
            );
            p.circle_filled(o + Vec2::new(t / 2.0, t / 2.0), t / 2.0, body);
        }
        _ => {
            p.add(egui::epaint::PathShape::convex_polygon(
                vec![
                    o + Vec2::new(t / 2.0, 0.0),
                    o + Vec2::new(w, 0.0),
                    o + Vec2::new(w, t),
                    o + Vec2::new(t, t),
                    o + Vec2::new(t, r.height() - 3.0),
                    o + Vec2::new(0.0, r.height() - 3.0),
                    o + Vec2::new(0.0, t / 2.0),
                ],
                body,
                EStroke::NONE,
            ));
        }
    }
    // The path through the middle of the outline.
    let m = o + Vec2::new(t / 2.0, t / 2.0);
    p.line_segment([m, m + Vec2::new(w - t / 2.0, 0.0)], EStroke::new(1.0, ink));
    p.line_segment(
        [m, m + Vec2::new(0.0, r.height() - 3.0 - t / 2.0)],
        EStroke::new(1.0, ink),
    );
    p.circle_stroke(m, 1.8, EStroke::new(1.0, ink));
}

/// The line ends: flat at the end, round, square past the end.
fn cap_icon(p: &egui::Painter, r: Rect, i: usize, ink: Color32) {
    let body = Color32::from_gray(140);
    let y = r.center().y;
    let end = r.left() + 6.0;
    let t = 9.0;
    match i {
        0 => {
            p.rect_filled(
                Rect::from_min_max(
                    Pos2::new(end, y - t / 2.0),
                    Pos2::new(r.right(), y + t / 2.0),
                ),
                0.0,
                body,
            );
        }
        1 => {
            p.rect_filled(
                Rect::from_min_max(
                    Pos2::new(end, y - t / 2.0),
                    Pos2::new(r.right(), y + t / 2.0),
                ),
                0.0,
                body,
            );
            p.circle_filled(Pos2::new(end, y), t / 2.0, body);
        }
        _ => {
            p.rect_filled(
                Rect::from_min_max(
                    Pos2::new(end - t / 2.0, y - t / 2.0),
                    Pos2::new(r.right(), y + t / 2.0),
                ),
                0.0,
                body,
            );
        }
    }
    p.line_segment(
        [Pos2::new(end, y), Pos2::new(r.right(), y)],
        EStroke::new(1.0, ink),
    );
    p.circle_stroke(Pos2::new(end, y), 1.8, EStroke::new(1.0, ink));
}

/// The outline position: outside, centred on, inside the path.
fn position_icon(p: &egui::Painter, r: Rect, i: usize, ink: Color32) {
    let body = Color32::from_gray(140);
    let path_y = r.top() + 6.0;
    let t = 7.0;
    let (y0, y1) = match i {
        0 => (path_y - t, path_y),
        1 => (path_y - t / 2.0, path_y + t / 2.0),
        _ => (path_y, path_y + t),
    };
    p.rect_filled(
        Rect::from_min_max(Pos2::new(r.left(), y0), Pos2::new(r.right(), y1)),
        0.0,
        body,
    );
    let x = r.center().x + 3.0;
    p.line_segment(
        [Pos2::new(r.left(), path_y), Pos2::new(x, path_y)],
        EStroke::new(1.0, ink),
    );
    p.line_segment(
        [Pos2::new(x, path_y), Pos2::new(x, r.bottom())],
        EStroke::new(1.0, ink),
    );
    p.circle_stroke(Pos2::new(x, path_y), 1.8, EStroke::new(1.0, ink));
}

fn arrow_glyph(p: &egui::Painter, c: Pos2, left: bool, ink: Color32) {
    let d = if left { -1.0 } else { 1.0 };
    let s = EStroke::new(1.6, ink);
    p.line_segment(
        [c + Vec2::new(-6.0 * d, 0.0), c + Vec2::new(6.0 * d, 0.0)],
        s,
    );
    p.line_segment(
        [c + Vec2::new(6.0 * d, 0.0), c + Vec2::new(2.0 * d, -4.0)],
        s,
    );
    p.line_segment(
        [c + Vec2::new(6.0 * d, 0.0), c + Vec2::new(2.0 * d, 4.0)],
        s,
    );
}

fn stretch_glyph(p: &egui::Painter, c: Pos2, ink: Color32) {
    let s = EStroke::new(1.3, ink);
    p.vline(c.x, (c.y - 6.0)..=(c.y + 6.0), s);
    for d in [-1.0f32, 1.0] {
        let a = c + Vec2::new(7.0 * d, 0.0);
        p.line_segment([c + Vec2::new(2.0 * d, 0.0), a], s);
        p.line_segment([a, a + Vec2::new(-3.0 * d, -3.0)], s);
        p.line_segment([a, a + Vec2::new(-3.0 * d, 3.0)], s);
    }
}

fn angle_glyph(p: &egui::Painter, c: Pos2, ink: Color32) {
    let s = EStroke::new(1.3, ink);
    let o = c + Vec2::new(-5.0, 5.0);
    p.line_segment([o, o + Vec2::new(11.0, 0.0)], s);
    p.line_segment([o, o + Vec2::new(8.0, -9.0)], s);
    p.circle_stroke(o + Vec2::new(8.0, -9.0), 1.5, s);
}

/// The nib: a black square with the pen's ellipse in white at its
/// stretch and angle.
fn nib_preview(p: &egui::Painter, r: Rect, stretch: f64, angle: f64) {
    p.rect_filled(r, 0.0, Color32::WHITE);
    p.rect_filled(r.shrink(6.0), 0.0, Color32::from_gray(30));
    let c = r.center();
    let rx = 12.0f32;
    let ry = (12.0 * stretch as f32).max(0.8);
    let a = (angle as f32).to_radians();
    let pts: Vec<Pos2> = (0..32)
        .map(|k| {
            let t = k as f32 / 32.0 * std::f32::consts::TAU;
            let (x, y) = (rx * t.cos(), ry * t.sin());
            c + Vec2::new(x * a.cos() - y * a.sin(), -(x * a.sin() + y * a.cos()))
        })
        .collect();
    p.add(egui::epaint::PathShape::convex_polygon(
        pts,
        Color32::WHITE,
        EStroke::NONE,
    ));
}

fn flame_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let c = r.center();
    let s = EStroke::new(1.4, ink);
    p.add(egui::Shape::line(
        vec![
            c + Vec2::new(-4.0, 6.0),
            c + Vec2::new(-1.0, 1.0),
            c + Vec2::new(1.0, 2.0),
            c + Vec2::new(4.0, -6.0),
        ],
        s,
    ));
    p.circle_filled(c + Vec2::new(-4.0, 6.0), 1.8, ink);
}

fn reset_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let c = r.center();
    let pts: Vec<Pos2> = (0..20)
        .map(|k| {
            let t = (40.0 + k as f32 * 14.0).to_radians();
            c + Vec2::new(5.5 * t.cos(), -5.5 * t.sin())
        })
        .collect();
    let tip = *pts.first().unwrap_or(&c);
    p.add(egui::Shape::line(pts, EStroke::new(1.4, ink)));
    p.line_segment([tip, tip + Vec2::new(-3.5, -1.0)], EStroke::new(1.4, ink));
    p.line_segment([tip, tip + Vec2::new(0.5, -3.5)], EStroke::new(1.4, ink));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(app: &mut App, ctx: &egui::Context, events: Vec<egui::Event>) {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 900.0))),
            events,
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| show(app, ui));
        out.textures_delta.clear();
    }

    /// The docker draws with nothing selected and with a rectangle, every
    /// section open and closed.
    #[test]
    fn the_docker_draws_with_and_without_a_selection() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        app.settings.properties_closed.clear();
        run(&mut app, &ctx, vec![]);
        app.settings.autocorrect.enabled = false;
        let id = app
            .new_shape(ShapeKind::Rect {
                rect: tracedraw_core::geometry::Rect::new(0.0, 0.0, 30.0, 20.0),
                radius: 0.0,
                corners: None,
            })
            .expect("rect");
        app.select(vec![id]);
        run(&mut app, &ctx, vec![]);
        assert!(sections(&app).iter().any(|s| s.id == "corners"));
        for id in ["outline", "fill", "transparency"] {
            set_open(&mut app, id, false);
        }
        run(&mut app, &ctx, vec![]);
        set_open(&mut app, "single", false);
        go_to(&mut app, &ctx, "fill");
        run(&mut app, &ctx, vec![]);
        assert!(is_open(&app, "fill"));
    }

    #[test]
    fn a_section_button_opens_and_lights_its_section() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        set_open(&mut app, "transparency", false);
        go_to(&mut app, &ctx, "transparency");
        assert!(is_open(&app, "transparency"));
        let active: Option<String> = ctx.data(|d| d.get_temp(active_id()));
        assert_eq!(active.as_deref(), Some("transparency"));
        run(&mut app, &ctx, vec![]);
        // The jump is used up by the header it scrolled to.
        let jump: Option<String> = ctx.data(|d| d.get_temp(jump_id()));
        assert!(jump.is_none());
    }

    #[test]
    fn numbers_show_with_a_comma_and_no_trailing_zeros() {
        assert_eq!(fmt(0.2, 3), "0,2");
        assert_eq!(fmt(10.0, 3), "10");
        assert_eq!(fmt(0.25, 3), "0,25");
    }
}
