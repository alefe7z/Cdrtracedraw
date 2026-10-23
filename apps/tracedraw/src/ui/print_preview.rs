//! File > Print Preview: a window of its own over the drawing with the
//! sheets exactly as File > Print sends them. A menu bar (File, View,
//! Settings, Help); a bar with the print style, Print Options, Print, the
//! zoom, full screen, color, invert, mirror and close; a bar for the
//! tool in use (the image's place and size, the imposition or the
//! marks); the tools down the left (pick, imposition layout, marks
//! placement, zoom); the sheet with the printable area dashed; the
//! pages, or a page's plates, as tabs along the bottom and the status
//! bar under them. See `docs/behavior/printing.md`, Print Preview.

use crate::app::App;
use crate::export::{plate_page, plates, plates_used, print_sheets, Plate};
use crate::i18n::{tr, trf};
use crate::theme::Tokens;
use crate::ui::dialogs::{Dialog, PrintState};
use crate::ui::print_dialog::{Kind, Outcome};
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use tracedraw_core::Document;

/// How big the sheet shows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Zoom {
    /// The whole sheet in the window.
    Page,
    Width,
    Height,
    /// Percent of the real size.
    Percent(f32),
}

/// The tools down the left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PvTool {
    Pick,
    Imposition,
    Marks,
    Zoom,
}

/// Zoom levels of the zoom list, in percent.
const ZOOMS: [f32; 8] = [10.0, 25.0, 50.0, 75.0, 100.0, 200.0, 400.0, 800.0];
/// Points per millimetre at 100 %.
const PT_PER_MM: f32 = 96.0 / 25.4;
/// The printer's unprintable edge drawn dashed, mm.
const PRINTABLE_INSET: f64 = 3.5;
const FONT: f32 = 12.5;

/// The open preview: the settings it shows, the page and plate on screen
/// and what was last built from them.
pub struct PrintPreview {
    pub st: PrintState,
    /// Index of the sheet shown.
    pub sheet: usize,
    /// 0 = composite, n = the n-th plate of [`plates`].
    pub plate: usize,
    pub zoom: Zoom,
    pub tool: PvTool,
    /// Plates show in their ink's colour (else grey).
    pub color: bool,
    /// Only the sheet, over the whole window.
    pub fullscreen: bool,
    /// Opened from the Print dialog: closing goes back to it.
    pub from_dialog: bool,
    /// Print Options open, with the settings being changed.
    pub options: Option<PrintState>,
    /// 0 millimetres, 1 inches, 2 points.
    pub units: usize,
    built: Option<(String, Document, Vec<bool>)>,
    texture: Option<(String, egui::TextureHandle)>,
}

impl PrintPreview {
    pub fn new(st: PrintState) -> Self {
        PrintPreview {
            st,
            sheet: 0,
            plate: 0,
            zoom: Zoom::Page,
            tool: PvTool::Pick,
            color: true,
            fullscreen: false,
            from_dialog: false,
            options: None,
            units: 0,
            built: None,
            texture: None,
        }
    }
}

/// What the sheets depend on: every print setting but the preflight list.
fn settings_key(st: &PrintState) -> String {
    let mut st = st.clone();
    st.preflight = None;
    st.copies = 1;
    st.tab = 0;
    st.ex.side_preview = false;
    format!("{st:?}")
}

/// The sheets for the settings, built again only when they changed, and
/// which plates they use.
fn sheets<'a>(app: &App, pv: &'a mut PrintPreview) -> (&'a Document, &'a [bool]) {
    let key = settings_key(&pv.st);
    if pv.built.as_ref().map(|(k, _, _)| k != &key).unwrap_or(true) {
        let doc = print_sheets(app, &pv.st);
        let used = plates_used(&doc);
        pv.built = Some((key, doc, used));
        pv.texture = None;
    }
    let b = pv
        .built
        .get_or_insert_with(|| (String::new(), Document::default(), Vec::new()));
    (&b.1, &b.2)
}

/// The page shown: the sheet, or one plate of it.
pub fn shown_page(
    doc: &Document,
    sheet: usize,
    plate: usize,
    separations: bool,
) -> Option<tracedraw_core::document::Page> {
    let page = doc.pages.get(sheet)?;
    if !separations || plate == 0 {
        return Some(page.clone());
    }
    let p = plates(doc).get(plate - 1)?.0;
    Some(plate_page(page, p))
}

/// The plate's name, translated for the process inks.
fn plate_name(i: usize, name: &str) -> String {
    match i {
        0 => tr("fxp.cyan"),
        1 => tr("fxp.magenta"),
        2 => tr("fxp.yellow"),
        3 => tr("fxp.black"),
        _ => name.to_string(),
    }
}

/// The tabs along the bottom: each sheet, or with separations each plate
/// of each sheet that prints (the used ones, or all with Show empty
/// plates, less those left out): (sheet, plate, label).
pub fn tabs(doc: &Document, used: &[bool], st: &PrintState) -> Vec<(usize, usize, String)> {
    let mut out = Vec::new();
    let names = plates(doc);
    for (si, _) in doc.pages.iter().enumerate() {
        let page = trf("prt.page_n", &[("n", &(si + 1).to_string())]);
        if !st.separations {
            out.push((si, 0, page));
            continue;
        }
        for (pi, (_, name)) in names.iter().enumerate() {
            let shown = st.ex.show_empty_plates || used.get(pi).copied().unwrap_or(false);
            if shown && !st.ex.excluded_plates.contains(name) {
                out.push((si, pi + 1, format!("{page} - {}", plate_name(pi, name))));
            }
        }
    }
    out
}

/// The ink colour a plate shows in.
fn plate_color(doc: &Document, plate: usize) -> Option<[u8; 3]> {
    match plates(doc).get(plate.checked_sub(1)?)?.0 {
        Plate::Process(0) => Some([0x00, 0xAE, 0xEF]),
        Plate::Process(1) => Some([0xEC, 0x00, 0x8C]),
        Plate::Process(2) => Some([0xFF, 0xF2, 0x00]),
        Plate::Process(_) => Some([0x23, 0x1F, 0x20]),
        Plate::Spot(id) => doc
            .inks
            .iter()
            .find(|i| i.id == id)
            .map(|i| i.color(1.0).to_rgb8()),
    }
}

/// A grey plate as its ink on white: full coverage (black) is the ink.
fn tint(img: &mut egui::ColorImage, ink: [u8; 3]) {
    for px in img.pixels.iter_mut() {
        let v = px.r() as f32 / 255.0;
        let mix = |c: u8| (c as f32 + (255.0 - c as f32) * v).round() as u8;
        *px = Color32::from_rgb(mix(ink[0]), mix(ink[1]), mix(ink[2]));
    }
}

pub fn show(app: &mut App, ui: &mut Ui) {
    let Some(mut pv) = app.print_preview.take() else {
        return;
    };
    let mut close = false;
    let mut print = false;
    let ctx = ui.ctx().clone();
    // Keep the shown tab among the tabs there are.
    let list = {
        let st = pv.st.clone();
        let (doc, used) = sheets(app, &mut pv);
        tabs(doc, used, &st)
    };
    let mut at = list
        .iter()
        .position(|(s, p, _)| *s == pv.sheet && *p == pv.plate)
        .unwrap_or(0);
    if let Some((s, p, _)) = list.get(at) {
        pv.sheet = *s;
        pv.plate = *p;
    }

    if !pv.fullscreen {
        menu_bar(app, ui, &mut pv, &mut print, &mut close);
        egui::Panel::top("print_preview_bar")
            .frame(bar_frame())
            .exact_size(32.0)
            .show(ui, |ui| main_bar(app, ui, &mut pv, &mut print, &mut close));
        egui::Panel::top("print_preview_tool_bar")
            .frame(bar_frame())
            .exact_size(30.0)
            .show(ui, |ui| tool_bar(app, ui, &mut pv));
        egui::Panel::bottom("print_preview_status")
            .frame(bar_frame())
            .exact_size(26.0)
            .show(ui, |ui| status_bar(ui, &pv));
        egui::Panel::bottom("print_preview_pages")
            .frame(
                egui::Frame::new()
                    .fill(Tokens::panel())
                    .inner_margin(egui::Margin::symmetric(4, 0)),
            )
            .exact_size(22.0)
            .show(ui, |ui| page_tabs(ui, &list, &mut at));
        egui::Panel::left("print_preview_tools")
            .frame(
                egui::Frame::new()
                    .fill(Tokens::panel())
                    .inner_margin(egui::Margin::symmetric(3, 6)),
            )
            .exact_size(30.0)
            .resizable(false)
            .show(ui, |ui| toolbox(ui, &mut pv));
    }
    if let Some((s, p, _)) = list.get(at) {
        pv.sheet = *s;
        pv.plate = *p;
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(Tokens::desktop()))
        .show(ui, |ui| sheet_view(app, ui, &mut pv));

    // Print Options.
    if let Some(mut st) = pv.options.take() {
        let mut done = false;
        match crate::ui::print_dialog::show(app, &ctx, &mut st, Kind::PreviewOptions, &mut done) {
            Some(Outcome::Ok) => {
                pv.st = st.clone();
                done = true;
            }
            Some(Outcome::Apply) => pv.st = st.clone(),
            _ => {}
        }
        if !done {
            pv.options = Some(st);
        }
    } else {
        crate::ui::print_dialog::save_style_window(app, &ctx, &mut pv.st);
        let (prev, next, esc, ctrl_p) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::PageUp),
                i.key_pressed(egui::Key::PageDown),
                i.key_pressed(egui::Key::Escape),
                i.modifiers.command && i.key_pressed(egui::Key::P),
            )
        });
        let mut at2 = at;
        if prev {
            at2 = at.saturating_sub(1);
        }
        if next && at + 1 < list.len() {
            at2 = at + 1;
        }
        if let Some((s, p, _)) = list.get(at2) {
            pv.sheet = *s;
            pv.plate = *p;
        }
        if esc {
            if pv.fullscreen {
                pv.fullscreen = false;
            } else {
                close = true;
            }
        }
        if ctrl_p {
            print = true;
        }
    }

    if print {
        match crate::export::print(app, &pv.st) {
            Ok(msg) => app.status = msg,
            Err(e) => app.status = format!("{}: {e}", tr("dialog.print_failed")),
        }
        if app.settings.printing.remember_last {
            app.settings.printing.remember(&pv.st);
            app.settings.save();
        }
        app.print_preview = None;
        return;
    }
    if close {
        // Back to the Print dialog it came from.
        if pv.from_dialog {
            app.dialog = Dialog::Print(pv.st);
        }
        return;
    }
    app.print_preview = Some(pv);
}

fn bar_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Tokens::panel())
        .inner_margin(egui::Margin::symmetric(4, 3))
}

// ----- the menu bar -------------------------------------------------------------

fn menu_bar(app: &mut App, ui: &mut Ui, pv: &mut PrintPreview, print: &mut bool, close: &mut bool) {
    use crate::ui::menus::{access, body, item_label, sep, BAR_FONT, MENU_WIDTH};
    egui::Panel::top("print_preview_menu")
        .frame(
            egui::Frame::new()
                .fill(Tokens::menu_bar())
                .inner_margin(egui::Margin::symmetric(4, 2)),
        )
        .show(ui, |ui| {
            egui::MenuBar::new()
                .style(crate::ui::menus::menu_bar_style)
                .config(
                    egui::containers::menu::MenuConfig::new()
                        .style(crate::ui::menus::menu_popup_style),
                )
                .ui(ui, |ui| {
                    let mut keys: Vec<char> = Vec::new();
                    let mut menu = |ui: &mut Ui, key: &str, f: &mut dyn FnMut(&mut Ui)| {
                        let label = tr(key);
                        let a = access::choose(&label, &keys);
                        keys.push(a.map(|a| a.1).unwrap_or(' '));
                        ui.menu_button(
                            access::title(&label, a.map(|a| a.0), BAR_FONT, Tokens::text()),
                            |ui| {
                                ui.set_min_width(MENU_WIDTH);
                                body(ui, |ui| f(ui))
                            },
                        );
                    };
                    menu(ui, "ppv.menu_file", &mut |ui| {
                        if item_label(ui, &tr("ppv.print"), "Ctrl+P", true) {
                            *print = true;
                        }
                        sep(ui);
                        if item_label(ui, &tr("ppv.save_style"), "", true) {
                            ask_style_name(ui.ctx(), &pv.st);
                        }
                        let has = has_style(app, &pv.st);
                        if item_label(ui, &tr("ppv.delete_style"), "", has) {
                            delete_style(app, &mut pv.st);
                        }
                        sep(ui);
                        if item_label(ui, &tr("ppv.print_options"), "", true) {
                            pv.options = Some(pv.st.clone());
                        }
                        sep(ui);
                        if item_label(ui, &tr("ppv.close"), "Esc", true) {
                            *close = true;
                        }
                    });
                    menu(ui, "ppv.menu_view", &mut |ui| {
                        if item_label(ui, &tr("ppv.full_screen"), "", true) {
                            pv.fullscreen = true;
                        }
                        if item_label(ui, &tr("ppv.enable_color"), "", pv.st.separations) {
                            pv.color = !pv.color;
                        }
                        sep(ui);
                        for (z, key) in [
                            (Zoom::Page, "ppv.zoom_page"),
                            (Zoom::Width, "ppv.zoom_width"),
                            (Zoom::Height, "ppv.zoom_height"),
                            (Zoom::Percent(100.0), "ppv.zoom_actual"),
                        ] {
                            if item_label(ui, &tr(key), "", pv.zoom != z) {
                                pv.zoom = z;
                            }
                        }
                    });
                    menu(ui, "ppv.menu_settings", &mut |ui| {
                        if item_label(ui, &tr("ppv.print_options"), "", true) {
                            pv.options = Some(pv.st.clone());
                        }
                        sep(ui);
                        let seps = if pv.st.separations {
                            tr("ppv.show_composite")
                        } else {
                            tr("ppv.show_separations")
                        };
                        if item_label(ui, &seps, "", true) {
                            pv.st.separations = !pv.st.separations;
                        }
                        if item_label(ui, &tr("ppv.invert"), "", true) {
                            pv.st.invert = !pv.st.invert;
                        }
                        if item_label(ui, &tr("ppv.mirror"), "", true) {
                            pv.st.mirror = !pv.st.mirror;
                        }
                    });
                    menu(ui, "ppv.menu_help", &mut |ui| {
                        if item_label(ui, &tr("ppv.help"), "F1", true) {
                            app.open_help("printing/print");
                        }
                    });
                });
        });
}

/// Ask for a print style's name (the Print dialog's Save As window).
fn ask_style_name(ctx: &egui::Context, st: &PrintState) {
    let name = if st.ex.style.is_empty() {
        tr("prt.new_style")
    } else {
        st.ex.style.clone()
    };
    ctx.data_mut(|d| d.insert_temp(egui::Id::new("prt_style_name"), Some(name)));
}

fn has_style(app: &App, st: &PrintState) -> bool {
    app.settings
        .printing
        .styles
        .iter()
        .any(|s| s.name == st.ex.style)
}

fn delete_style(app: &mut App, st: &mut PrintState) {
    let name = std::mem::take(&mut st.ex.style);
    app.settings.printing.styles.retain(|s| s.name != name);
    app.settings.save();
}

// ----- the bars -----------------------------------------------------------------

/// An icon button of `size`; `paint` draws the icon in its rect.
fn icon_button(
    ui: &mut Ui,
    id: &str,
    tip: &str,
    on: bool,
    enabled: bool,
    paint: impl FnOnce(&egui::Painter, Rect, Color32),
) -> bool {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::hover());
    let resp = ui
        .interact(r, egui::Id::new(id), Sense::click())
        .on_hover_text(tip);
    let p = ui.painter();
    if on {
        p.rect_filled(r, 2.0, Tokens::tool_active());
    } else if enabled && (resp.hovered() || resp.is_pointer_button_down_on()) {
        p.rect_filled(r, 2.0, Tokens::tool_hover());
    }
    let ink = if enabled {
        Tokens::text()
    } else {
        Tokens::icon_off()
    };
    paint(p, r.shrink(4.0), ink);
    enabled && resp.clicked()
}

fn bar_sep(ui: &mut Ui) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(9.0, 22.0), Sense::hover());
    ui.painter().vline(
        r.center().x,
        (r.top() + 2.0)..=(r.bottom() - 2.0),
        Stroke::new(1.0, Tokens::border()),
    );
}

fn main_bar(app: &mut App, ui: &mut Ui, pv: &mut PrintPreview, print: &mut bool, close: &mut bool) {
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        let (r, _) = ui.allocate_exact_size(Vec2::new(130.0, 22.0), Sense::hover());
        crate::ui::print_dialog::style_combo_at(app, ui, r, &mut pv.st);
        if icon_button(
            ui,
            "ppv_add_style",
            &tr("ppv.save_style"),
            false,
            true,
            |p, r, c| {
                p.circle_stroke(r.center(), 7.0, Stroke::new(1.3, c));
                p.hline(
                    (r.center().x - 4.0)..=(r.center().x + 4.0),
                    r.center().y,
                    Stroke::new(1.5, c),
                );
                p.vline(
                    r.center().x,
                    (r.center().y - 4.0)..=(r.center().y + 4.0),
                    Stroke::new(1.5, c),
                );
            },
        ) {
            ask_style_name(ui.ctx(), &pv.st);
        }
        let has = has_style(app, &pv.st);
        if icon_button(
            ui,
            "ppv_del_style",
            &tr("ppv.delete_style"),
            false,
            has,
            |p, r, c| {
                p.circle_stroke(r.center(), 7.0, Stroke::new(1.3, c));
                p.hline(
                    (r.center().x - 4.0)..=(r.center().x + 4.0),
                    r.center().y,
                    Stroke::new(1.5, c),
                );
            },
        ) {
            delete_style(app, &mut pv.st);
        }
        bar_sep(ui);
        if icon_button(
            ui,
            "ppv_options",
            &tr("ppv.print_options"),
            false,
            true,
            |p, r, c| gear_icon(p, r.center(), c),
        ) {
            pv.options = Some(pv.st.clone());
        }
        bar_sep(ui);
        if icon_button(ui, "ppv_print", &tr("ppv.print"), false, true, |p, r, c| {
            printer_icon(p, r, c)
        }) {
            *print = true;
        }
        bar_sep(ui);
        let (r, _) = ui.allocate_exact_size(Vec2::new(84.0, 22.0), Sense::hover());
        crate::ui::chrome::combo(ui, "ppv_zoom", r, zoom_name(pv.zoom), |ui| {
            for z in [Zoom::Page, Zoom::Width, Zoom::Height]
                .into_iter()
                .chain(ZOOMS.iter().map(|p| Zoom::Percent(*p)))
            {
                if ui.selectable_label(pv.zoom == z, zoom_name(z)).clicked() {
                    pv.zoom = z;
                }
            }
        });
        if icon_button(
            ui,
            "ppv_full",
            &tr("ppv.full_screen"),
            false,
            true,
            |p, r, c| {
                let b = r.shrink(1.0);
                p.rect_stroke(b, 0.0, Stroke::new(1.3, c), egui::StrokeKind::Inside);
                p.rect_filled(b.shrink(3.0), 0.0, c);
            },
        ) {
            pv.fullscreen = true;
        }
        bar_sep(ui);
        let seps = pv.st.separations;
        if icon_button(
            ui,
            "ppv_color",
            &tr("ppv.enable_color"),
            seps && pv.color,
            seps,
            |p, r, _| color_icon(p, r),
        ) {
            pv.color = !pv.color;
        }
        if icon_button(
            ui,
            "ppv_invert",
            &tr("ppv.invert"),
            pv.st.invert,
            true,
            |p, r, c| {
                let b = Rect::from_center_size(r.center(), Vec2::new(16.0, 12.0));
                p.rect_stroke(b, 0.0, Stroke::new(1.4, c), egui::StrokeKind::Inside);
                p.hline(b.x_range(), b.top() + 2.5, Stroke::new(2.0, c));
            },
        ) {
            pv.st.invert = !pv.st.invert;
        }
        if icon_button(
            ui,
            "ppv_mirror",
            &tr("ppv.mirror"),
            pv.st.mirror,
            true,
            |p, r, c| {
                let e = if pv.st.mirror { "\u{018E}" } else { "E" };
                p.text(
                    r.center(),
                    Align2::CENTER_CENTER,
                    e,
                    FontId::proportional(17.0),
                    c,
                );
            },
        ) {
            pv.st.mirror = !pv.st.mirror;
        }
        bar_sep(ui);
        if icon_button(ui, "ppv_close", &tr("ppv.close"), false, true, |p, r, c| {
            close_icon(p, r, c)
        }) {
            *close = true;
        }
    });
}

fn zoom_name(z: Zoom) -> String {
    match z {
        Zoom::Page => tr("ppv.zoom_page"),
        Zoom::Width => tr("ppv.zoom_width"),
        Zoom::Height => tr("ppv.zoom_height"),
        Zoom::Percent(p) => format!("{p:.0}%"),
    }
}

/// The second bar: what the tool in use sets.
fn tool_bar(app: &mut App, ui: &mut Ui, pv: &mut PrintPreview) {
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        let st = &mut pv.st;
        let kinds = [
            tr("ppv.imp_as_document"),
            tr("print.layout_n_up"),
            tr("print.layout_booklet"),
            tr("print.layout_repeat"),
        ];
        let (r, _) = ui.allocate_exact_size(Vec2::new(120.0, 22.0), Sense::hover());
        crate::ui::chrome::combo(
            ui,
            "ppv_imposition",
            r,
            kinds[st.layout_kind.min(3)].clone(),
            |ui| {
                for (i, k) in kinds.iter().enumerate() {
                    ui.selectable_value(&mut st.layout_kind, i, k.as_str());
                }
            },
        );
        bar_sep(ui);
        match pv.tool {
            PvTool::Imposition => {
                let spin = |ui: &mut Ui,
                            label: &str,
                            v: &mut f64,
                            range: std::ops::RangeInclusive<f64>,
                            dec: usize| {
                    ui.label(label);
                    let (r, _) = ui.allocate_exact_size(Vec2::new(70.0, 20.0), Sense::hover());
                    if let Some(n) =
                        crate::ui::chrome::spin_field(ui, label, r, *v, dec, "", 1.0, range)
                    {
                        *v = n;
                    }
                };
                let mut cols = st.cols as f64;
                let mut rows = st.rows as f64;
                spin(ui, &tr("ppv.columns"), &mut cols, 1.0..=20.0, 0);
                spin(ui, &tr("ppv.rows"), &mut rows, 1.0..=20.0, 0);
                st.cols = cols as u32;
                st.rows = rows as u32;
                spin(ui, &tr("ppv.gutter"), &mut st.gutter, 0.0..=100.0, 1);
                ui.label(tr("print.sheet"));
                let (r, _) = ui.allocate_exact_size(Vec2::new(110.0, 22.0), Sense::hover());
                let name = crate::ui::dialogs::sheet_name;
                crate::ui::chrome::combo(ui, "ppv_sheet", r, name(st.sheet), |ui| {
                    for i in 0..crate::ui::dialogs::SHEETS.len() {
                        ui.selectable_value(&mut st.sheet, i, name(i));
                    }
                });
                ui.add_enabled(
                    st.sheet > 0,
                    egui::Checkbox::new(&mut st.landscape, tr("print.landscape")),
                );
            }
            PvTool::Marks => {
                ui.checkbox(&mut st.crop_marks, tr("ppv.crop_marks"));
                ui.checkbox(&mut st.registration_marks, tr("ppv.reg_marks"));
                ui.checkbox(&mut st.color_bars, tr("ppv.color_bar"));
                ui.checkbox(&mut st.densitometer, tr("ppv.densitometer"));
                ui.checkbox(&mut st.page_info, tr("ppv.file_info"));
                ui.checkbox(&mut st.page_numbers, tr("ppv.page_numbers"));
            }
            PvTool::Pick | PvTool::Zoom => {
                let (x, y, w, h) = content_box(app);
                let f = |v: f64| unit_value(v, pv.units);
                let col = |ui: &mut Ui, a: (&str, String), b: (&str, String)| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        for (l, v) in [a, b] {
                            ui.label(
                                egui::RichText::new(format!("{l} {v}"))
                                    .size(11.5)
                                    .color(Tokens::text_dim()),
                            );
                        }
                    });
                };
                col(ui, ("x:", f(x)), ("y:", f(y)));
                bar_sep(ui);
                col(ui, ("\u{2195}", f(h)), ("\u{2194}", f(w)));
                bar_sep(ui);
                let s = format!("{:.0} %", st.scale);
                col(ui, ("", s.clone()), ("", s));
                lock_icon(ui);
            }
        }
        bar_sep(ui);
        if icon_button(
            ui,
            "ppv_tiles",
            &tr("prt.tile_pages"),
            st.ex.tile_pages,
            true,
            |p, r, c| {
                let b = r.shrink(1.0);
                for (i, q) in [
                    Rect::from_min_max(b.min, b.center()),
                    Rect::from_min_max(
                        Pos2::new(b.center().x, b.top()),
                        Pos2::new(b.right(), b.center().y),
                    ),
                    Rect::from_min_max(
                        Pos2::new(b.left(), b.center().y),
                        Pos2::new(b.center().x, b.bottom()),
                    ),
                    Rect::from_min_max(b.center(), b.max),
                ]
                .into_iter()
                .enumerate()
                {
                    let fill = [
                        Color32::from_rgb(0xE5, 0x39, 0x35),
                        c,
                        c,
                        Color32::from_rgb(0xE5, 0x39, 0x35),
                    ][i];
                    p.rect_stroke(
                        q.shrink(0.5),
                        0.0,
                        Stroke::new(1.2, fill),
                        egui::StrokeKind::Inside,
                    );
                }
            },
        ) {
            st.ex.tile_pages = !st.ex.tile_pages;
        }
        let units = [tr("ppv.mm"), tr("ppv.inches"), tr("ppv.points")];
        let (r, _) = ui.allocate_exact_size(Vec2::new(90.0, 22.0), Sense::hover());
        crate::ui::chrome::combo(ui, "ppv_units", r, units[pv.units.min(2)].clone(), |ui| {
            for (i, u) in units.iter().enumerate() {
                ui.selectable_value(&mut pv.units, i, u.as_str());
            }
        });
    });
}

/// A length in mm in the chosen unit, with its symbol.
fn unit_value(mm: f64, units: usize) -> String {
    let s = match units {
        1 => format!("{:.3} in", mm / 25.4),
        2 => format!("{:.1} pt", mm * 72.0 / 25.4),
        _ => format!("{mm:.3} mm"),
    };
    s.replace('.', ",")
}

/// The current page's objects' bounds, mm from the page's top left.
fn content_box(app: &App) -> (f64, f64, f64, f64) {
    let doc = app.doc();
    let Some(page) = doc
        .pages
        .iter()
        .find(|p| p.id == app.page)
        .or(doc.pages.first())
    else {
        return (0.0, 0.0, 0.0, 0.0);
    };
    let mut bb: Option<tracedraw_core::Rect> = None;
    for layer in &page.layers {
        for s in &layer.shapes {
            let b = s.bounds();
            bb = Some(match bb {
                Some(a) => a.union(b),
                None => b,
            });
        }
    }
    match bb {
        Some(b) => (b.x0, page.size.height - b.y1, b.width(), b.height()),
        None => (0.0, 0.0, 0.0, 0.0),
    }
}

fn lock_icon(ui: &mut Ui) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
    let c = r.center();
    let ink = Tokens::text_dim();
    let body = Rect::from_center_size(c + Vec2::new(0.0, 3.0), Vec2::new(10.0, 8.0));
    ui.painter()
        .rect_stroke(body, 1.0, Stroke::new(1.2, ink), egui::StrokeKind::Inside);
    ui.painter()
        .circle_stroke(c + Vec2::new(0.0, -2.0), 3.2, Stroke::new(1.2, ink));
}

/// The tools down the left.
fn toolbox(ui: &mut Ui, pv: &mut PrintPreview) {
    ui.vertical_centered(|ui| {
        ui.spacing_mut().item_spacing.y = 3.0;
        let tools = [
            (PvTool::Pick, "ppv.tool_pick"),
            (PvTool::Imposition, "ppv.tool_imposition"),
            (PvTool::Marks, "ppv.tool_marks"),
            (PvTool::Zoom, "ppv.tool_zoom"),
        ];
        for (t, key) in tools {
            if icon_button(ui, key, &tr(key), pv.tool == t, true, |p, r, c| match t {
                PvTool::Pick => pick_icon(p, r, c),
                PvTool::Imposition => imposition_icon(p, r, c),
                PvTool::Marks => marks_icon(p, r, c),
                PvTool::Zoom => {
                    p.circle_stroke(r.center() + Vec2::new(-1.5, -1.5), 5.5, Stroke::new(1.5, c));
                    p.line_segment(
                        [r.center() + Vec2::new(2.5, 2.5), r.right_bottom()],
                        Stroke::new(2.0, c),
                    );
                }
            }) {
                pv.tool = t;
            }
        }
    });
}

/// The page tabs along the bottom, with the first, previous, next and
/// last buttons and "n of m" before them.
fn page_tabs(ui: &mut Ui, list: &[(usize, usize, String)], at: &mut usize) {
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        let n = list.len().max(1);
        let nav = |ui: &mut Ui, glyph: &str, enabled: bool| -> bool {
            let (r, resp) = ui.allocate_exact_size(Vec2::new(16.0, 18.0), Sense::click());
            let c = if enabled {
                Tokens::text()
            } else {
                Tokens::icon_off()
            };
            if enabled && resp.hovered() {
                ui.painter().rect_filled(r, 2.0, Tokens::tool_hover());
            }
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                glyph,
                FontId::proportional(10.0),
                c,
            );
            enabled && resp.clicked()
        };
        if nav(ui, "\u{23EE}", *at > 0) {
            *at = 0;
        }
        if nav(ui, "\u{25C0}", *at > 0) {
            *at -= 1;
        }
        ui.label(
            egui::RichText::new(trf(
                "ppv.n_of",
                &[("n", &(*at + 1).to_string()), ("total", &n.to_string())],
            ))
            .size(11.5),
        );
        if nav(ui, "\u{25B6}", *at + 1 < list.len()) {
            *at += 1;
        }
        if nav(ui, "\u{23ED}", *at + 1 < list.len()) {
            *at = list.len().saturating_sub(1);
        }
        ui.add_space(6.0);
        for (i, (_, _, label)) in list.iter().enumerate() {
            let galley = ui.painter().layout_no_wrap(
                label.clone(),
                FontId::proportional(FONT - 0.5),
                Tokens::text(),
            );
            let w = galley.size().x + 22.0;
            let (r, resp) = ui.allocate_exact_size(Vec2::new(w, 19.0), Sense::click());
            let on = i == *at;
            // A slanted tab, the one in front white.
            let pts = vec![
                Pos2::new(r.left(), r.top()),
                Pos2::new(r.right(), r.top()),
                Pos2::new(r.right() - 7.0, r.bottom()),
                Pos2::new(r.left() + 7.0, r.bottom()),
            ];
            let fill = if on {
                Color32::WHITE
            } else if resp.hovered() {
                Tokens::tool_hover()
            } else {
                Tokens::panel()
            };
            ui.painter().add(egui::epaint::PathShape::convex_polygon(
                pts,
                fill,
                Stroke::new(1.0, Tokens::control_border()),
            ));
            ui.painter().galley(
                Pos2::new(
                    r.center().x - galley.size().x / 2.0,
                    r.center().y - galley.size().y / 2.0,
                ),
                galley,
                if on {
                    Color32::BLACK
                } else {
                    Tokens::text_dim()
                },
            );
            if on {
                // Recolour: the galley was laid out in the text colour.
                let g = ui.painter().layout_no_wrap(
                    label.clone(),
                    FontId::proportional(FONT - 0.5),
                    Color32::BLACK,
                );
                ui.painter()
                    .rect_filled(r.shrink2(Vec2::new(8.0, 2.0)), 0.0, Color32::WHITE);
                ui.painter().galley(
                    Pos2::new(
                        r.center().x - g.size().x / 2.0,
                        r.center().y - g.size().y / 2.0,
                    ),
                    g,
                    Color32::BLACK,
                );
            }
            if resp.clicked() {
                *at = i;
            }
        }
    });
}

fn status_bar(ui: &mut Ui, pv: &PrintPreview) {
    ui.horizontal_centered(|ui| {
        ui.label(egui::RichText::new(tr("ppv.selection_none")).size(11.5));
        bar_sep(ui);
        let (r, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
        printer_icon(ui.painter(), r, Tokens::text());
        let name = pv.st.printer.clone().unwrap_or_else(|| tr("prt.viewer"));
        ui.label(egui::RichText::new(trf("ppv.printer", &[("name", &name)])).size(11.5));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let mode = if pv.st.separations {
                tr("ppv.cmyk_seps")
            } else {
                tr("ppv.composite")
            };
            ui.add_space(30.0);
            ui.label(egui::RichText::new(mode).size(11.5));
            let (r, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
            color_icon(ui.painter(), r.shrink(1.0));
        });
    });
}

// ----- the sheet ----------------------------------------------------------------

fn sheet_view(app: &App, ui: &mut Ui, pv: &mut PrintPreview) {
    let area = ui.available_rect_before_wrap();
    let ctx = ui.ctx().clone();
    let (separations, sheet, plate) = (pv.st.separations, pv.sheet, pv.plate);
    let (page, ink) = {
        let (doc, _) = sheets(app, pv);
        let Some(page) = shown_page(doc, sheet, plate, separations) else {
            return;
        };
        let ink = if separations {
            plate_color(doc, plate)
        } else {
            None
        };
        (page, ink)
    };
    let margin = if pv.fullscreen { 8.0 } else { 20.0 };
    let (pw, ph) = (
        page.size.width.max(1e-6) as f32,
        page.size.height.max(1e-6) as f32,
    );
    let fit_w = (area.width() - 2.0 * margin) / pw;
    let fit_h = (area.height() - 2.0 * margin) / ph;
    let scale = match pv.zoom {
        Zoom::Page => fit_w.min(fit_h),
        Zoom::Width => fit_w,
        Zoom::Height => fit_h,
        Zoom::Percent(p) => p / 100.0 * PT_PER_MM,
    }
    .max(0.01);
    let size = Vec2::new(pw * scale, ph * scale);
    let ppp = ctx.pixels_per_point();
    let px_scale = (scale * ppp).min(8000.0 / pw.max(ph));
    let (w, h) = (
        (pw * px_scale).ceil().max(1.0) as u32,
        (ph * px_scale).ceil().max(1.0) as u32,
    );
    let color = pv.color && ink.is_some();
    let key = format!(
        "{}|{sheet}|{plate}|{w}x{h}|{color}",
        pv.built.as_ref().map(|(k, _, _)| k.as_str()).unwrap_or("")
    );
    if pv.texture.as_ref().map(|(k, _)| k != &key).unwrap_or(true) {
        if let Some(mut img) = render_sheet(&page, w, h, px_scale as f64) {
            if let (true, Some(ink)) = (color, ink) {
                tint(&mut img, ink);
            }
            let tex = ctx.load_texture("print_preview", img, egui::TextureOptions::LINEAR);
            pv.texture = Some((key, tex));
        }
    }
    let mut zoom_by: Option<f32> = None;
    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let avail = ui.available_size().max(size + Vec2::splat(2.0 * margin));
            let (r, resp) = ui.allocate_exact_size(avail, Sense::click());
            let sheet_rect = Rect::from_center_size(r.center(), size);
            let p = ui.painter();
            p.rect_filled(
                sheet_rect.translate(Vec2::new(3.0, 3.0)),
                0.0,
                Color32::from_black_alpha(110),
            );
            if let Some((_, tex)) = &pv.texture {
                p.image(
                    tex.id(),
                    sheet_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
            }
            // The printable area.
            let inset = PRINTABLE_INSET as f32 * scale;
            if inset * 2.0 < size.x.min(size.y) {
                dashed_rect(p, sheet_rect.shrink(inset), Color32::from_gray(40));
            }
            p.rect_stroke(
                sheet_rect,
                0.0,
                Stroke::new(1.0, Color32::from_gray(20)),
                egui::StrokeKind::Outside,
            );
            if pv.tool == PvTool::Zoom {
                if resp.clicked() {
                    zoom_by = Some(2.0);
                } else if resp.secondary_clicked() {
                    zoom_by = Some(0.5);
                }
            }
            if resp.hovered() {
                let z = ui.input(|i| i.zoom_delta());
                if (z - 1.0).abs() > 0.01 {
                    zoom_by = Some(z);
                }
            }
        });
    if let Some(f) = zoom_by {
        let now = scale / PT_PER_MM * 100.0;
        pv.zoom = Zoom::Percent((now * f).clamp(5.0, 1600.0).round());
    }
}

fn dashed_rect(p: &egui::Painter, r: Rect, color: Color32) {
    let st = Stroke::new(1.0, color);
    let pts = [
        r.left_top(),
        r.right_top(),
        r.right_bottom(),
        r.left_bottom(),
        r.left_top(),
    ];
    for w in pts.windows(2) {
        p.extend(egui::Shape::dashed_line(&[w[0], w[1]], st, 2.0, 2.0));
    }
}

// ----- icons --------------------------------------------------------------------

fn gear_icon(p: &egui::Painter, c: Pos2, ink: Color32) {
    for k in 0..8 {
        let a = k as f32 * std::f32::consts::TAU / 8.0;
        let d = Vec2::new(a.cos(), a.sin());
        let n = Vec2::new(-d.y, d.x);
        p.add(egui::epaint::PathShape::convex_polygon(
            vec![
                c + d * 4.5 - n * 2.0,
                c + d * 8.0 - n * 1.5,
                c + d * 8.0 + n * 1.5,
                c + d * 4.5 + n * 2.0,
            ],
            ink,
            Stroke::NONE,
        ));
    }
    p.circle_filled(c, 5.8, ink);
    p.circle_filled(c, 2.4, Tokens::panel());
}

fn printer_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let s = Stroke::new(1.3, ink);
    let c = r.center();
    let body = Rect::from_center_size(c + Vec2::new(0.0, 1.0), Vec2::new(16.0, 8.0));
    p.rect_stroke(body, 2.0, s, egui::StrokeKind::Inside);
    let paper = Rect::from_min_max(
        Pos2::new(c.x - 5.0, body.top() - 5.0),
        Pos2::new(c.x + 5.0, body.top()),
    );
    p.rect_stroke(paper, 0.0, s, egui::StrokeKind::Inside);
    let out = Rect::from_min_max(
        Pos2::new(c.x - 5.0, body.bottom() - 2.0),
        Pos2::new(c.x + 5.0, body.bottom() + 3.0),
    );
    p.rect_filled(out, 0.0, Tokens::panel());
    p.rect_stroke(out, 0.0, s, egui::StrokeKind::Inside);
}

/// Two overlapping coloured squares: colour on.
fn color_icon(p: &egui::Painter, r: Rect) {
    let a = Rect::from_min_size(r.min + Vec2::new(1.0, 1.0), Vec2::splat(9.0));
    let b = Rect::from_min_size(r.min + Vec2::new(6.0, 6.0), Vec2::splat(9.0));
    p.rect_filled(a, 1.0, Color32::from_rgb(0xE5, 0x39, 0x35));
    p.rect_filled(b, 1.0, Color32::from_rgb(0x1E, 0x88, 0xE5));
    p.rect_stroke(
        b,
        1.0,
        Stroke::new(1.0, Color32::WHITE),
        egui::StrokeKind::Inside,
    );
}

/// A window with a red cross over its corner: close the preview.
fn close_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let b = Rect::from_min_max(r.min + Vec2::new(0.0, 3.0), r.max - Vec2::new(3.0, 0.0));
    p.rect_stroke(b, 1.0, Stroke::new(1.3, ink), egui::StrokeKind::Inside);
    p.hline(b.x_range(), b.top() + 3.0, Stroke::new(1.3, ink));
    let c = Pos2::new(r.right() - 2.5, r.top() + 2.5);
    let red = Stroke::new(1.6, Color32::from_rgb(0xE5, 0x39, 0x35));
    p.line_segment([c + Vec2::new(-2.5, -2.5), c + Vec2::new(2.5, 2.5)], red);
    p.line_segment([c + Vec2::new(-2.5, 2.5), c + Vec2::new(2.5, -2.5)], red);
}

fn pick_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let o = r.center() + Vec2::new(-3.0, -6.0);
    p.add(egui::epaint::PathShape::convex_polygon(
        vec![
            o,
            o + Vec2::new(0.0, 12.0),
            o + Vec2::new(3.0, 9.0),
            o + Vec2::new(8.0, 9.0),
        ],
        ink,
        Stroke::NONE,
    ));
    for (dx, dy) in [(-6.0, -6.0), (6.0, -6.0), (-6.0, 6.0), (6.0, 6.0)] {
        p.rect_filled(
            Rect::from_center_size(r.center() + Vec2::new(dx, dy), Vec2::splat(2.0)),
            0.0,
            ink,
        );
    }
}

/// Two facing pages: the imposition layout tool.
fn imposition_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let s = Stroke::new(1.2, ink);
    let c = r.center();
    let l = Rect::from_min_max(Pos2::new(c.x - 7.0, c.y - 6.0), Pos2::new(c.x, c.y + 7.0));
    let rr = Rect::from_min_max(Pos2::new(c.x, c.y - 6.0), Pos2::new(c.x + 7.0, c.y + 7.0));
    p.rect_stroke(l, 0.0, s, egui::StrokeKind::Inside);
    p.rect_stroke(rr, 0.0, s, egui::StrokeKind::Inside);
    p.text(
        l.center(),
        Align2::CENTER_CENTER,
        "1",
        FontId::proportional(8.0),
        ink,
    );
    p.text(
        rr.center(),
        Align2::CENTER_CENTER,
        "2",
        FontId::proportional(8.0),
        ink,
    );
}

/// A registration mark with a pointer: the marks placement tool.
fn marks_icon(p: &egui::Painter, r: Rect, ink: Color32) {
    let c = r.center() + Vec2::new(-2.0, -2.0);
    let s = Stroke::new(1.2, ink);
    p.circle_stroke(c, 4.0, s);
    p.hline((c.x - 7.0)..=(c.x + 7.0), c.y, s);
    p.vline(c.x, (c.y - 7.0)..=(c.y + 7.0), s);
    p.rect_filled(
        Rect::from_min_size(c + Vec2::new(-7.0, 5.0), Vec2::new(3.0, 3.0)),
        0.0,
        Color32::from_rgb(0xE5, 0x39, 0x35),
    );
    p.rect_filled(
        Rect::from_min_size(c + Vec2::new(-7.0, 8.0), Vec2::new(3.0, 3.0)),
        0.0,
        Color32::from_rgb(0x1E, 0x88, 0xE5),
    );
}

/// The first sheet for `st`, fitted in `size`, as a texture (the Print
/// dialog's side preview); built again when the settings change.
pub fn thumbnail(
    app: &App,
    ctx: &egui::Context,
    st: &PrintState,
    size: egui::Vec2,
) -> Option<egui::TextureHandle> {
    let key = format!("{}|{:?}", settings_key(st), size);
    let id = egui::Id::new("print_side_preview");
    if let Some((k, tex)) = ctx.data(|d| d.get_temp::<(String, egui::TextureHandle)>(id)) {
        if k == key {
            return Some(tex);
        }
    }
    let doc = print_sheets(app, st);
    let page = doc.pages.first()?;
    let zoom = (size.x as f64 / page.size.width).min(size.y as f64 / page.size.height);
    if !(zoom.is_finite() && zoom > 0.0) {
        return None;
    }
    let w = (page.size.width * zoom).round().max(1.0) as u32;
    let h = (page.size.height * zoom).round().max(1.0) as u32;
    let img = render_sheet(page, w, h, zoom)?;
    let tex = ctx.load_texture("print_side_preview", img, egui::TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, (key, tex.clone())));
    Some(tex)
}

/// The sheet as a bitmap on white paper.
fn render_sheet(
    page: &tracedraw_core::document::Page,
    w: u32,
    h: u32,
    zoom: f64,
) -> Option<egui::ColorImage> {
    let mut doc = Document::new("sheet", page.size);
    doc.pages = vec![page.clone()];
    let opts = tracedraw_render::RenderOptions {
        width: w,
        height: h,
        view: tracedraw_render::ViewTransform {
            zoom,
            origin_x: 0.0,
            origin_y: page.size.height * zoom,
        },
        preview: None,
        wireframe: false,
        ..tracedraw_render::RenderOptions::default()
    };
    let pm = tracedraw_render::render_page(&doc, page.id, &opts)?;
    let mut white = tiny_skia::Pixmap::new(pm.width(), pm.height())?;
    white.fill(tiny_skia::Color::WHITE);
    white.draw_pixmap(
        0,
        0,
        pm.as_ref(),
        &Default::default(),
        tiny_skia::Transform::identity(),
        None,
    );
    Some(egui::ColorImage::from_rgba_premultiplied(
        [white.width() as usize, white.height() as usize],
        white.data(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracedraw_core::{Color, Fill, Shape, ShapeKind};

    fn doc_with_cmyk() -> Document {
        let mut doc = Document::new("t", tracedraw_core::geometry::Size::new(100.0, 100.0));
        let id = doc.ids_mut().shape();
        let mut s = Shape::new(
            id,
            ShapeKind::Rect {
                rect: tracedraw_core::geometry::Rect::new(0.0, 0.0, 10.0, 10.0),
                radius: 0.0,
                corners: None,
            },
        );
        s.fill = Fill::Solid(Color::Cmyk {
            c: 1.0,
            m: 0.5,
            y: 0.0,
            k: 0.0,
        });
        s.stroke = None;
        doc.pages[0].layers[0].shapes.push(s);
        doc
    }

    #[test]
    fn plates_split_the_sheet_by_ink() {
        let doc = doc_with_cmyk();
        let grey = |plate| {
            let p = shown_page(&doc, 0, plate, true).expect("page");
            match p.layers[0].shapes[0].fill {
                Fill::Solid(Color::Gray { v }) => v,
                ref f => panic!("{f:?}"),
            }
        };
        assert!((grey(1) - 0.0).abs() < 1e-6, "cyan plate full");
        assert!((grey(2) - 0.5).abs() < 1e-6, "magenta plate half");
        assert!((grey(3) - 1.0).abs() < 1e-6, "yellow plate empty");
        // Composite, and separations off, show the sheet as it is.
        let p = shown_page(&doc, 0, 0, true).expect("page");
        assert!(matches!(
            p.layers[0].shapes[0].fill,
            Fill::Solid(Color::Cmyk { .. })
        ));
        let p = shown_page(&doc, 0, 2, false).expect("page");
        assert!(matches!(
            p.layers[0].shapes[0].fill,
            Fill::Solid(Color::Cmyk { .. })
        ));
        assert!(shown_page(&doc, 5, 0, false).is_none());
    }

    #[test]
    fn the_settings_key_ignores_copies_and_preflight() {
        let a = PrintState::default();
        let mut b = a.clone();
        b.copies = 4;
        b.tab = 3;
        assert_eq!(settings_key(&a), settings_key(&b));
        b.crop_marks = true;
        assert_ne!(settings_key(&a), settings_key(&b));
    }

    /// With separations the tabs are the plates that print: the used
    /// ones, all with Show empty plates, less those left out.
    #[test]
    fn the_tabs_are_the_pages_or_the_plates_that_print() {
        crate::i18n::set_language("en");
        let doc = doc_with_cmyk();
        let used = plates_used(&doc);
        assert_eq!(used, vec![true, true, false, false]);
        let mut st = PrintState::default();
        let t = tabs(&doc, &used, &st);
        assert_eq!(t, vec![(0, 0, "Page 1".to_string())]);
        st.separations = true;
        let t = tabs(&doc, &used, &st);
        let labels: Vec<&str> = t.iter().map(|x| x.2.as_str()).collect();
        assert_eq!(labels, ["Page 1 - Cyan", "Page 1 - Magenta"]);
        st.ex.show_empty_plates = true;
        assert_eq!(tabs(&doc, &used, &st).len(), 4);
        st.ex.excluded_plates.push("Magenta".into());
        assert_eq!(tabs(&doc, &used, &st).len(), 3);
    }

    #[test]
    fn a_plate_tints_to_its_ink() {
        let mut img = egui::ColorImage::new([2, 1], vec![Color32::BLACK, Color32::WHITE]);
        tint(&mut img, [0x00, 0xAE, 0xEF]);
        assert_eq!(img.pixels[0], Color32::from_rgb(0x00, 0xAE, 0xEF));
        assert_eq!(img.pixels[1], Color32::WHITE);
    }

    /// The preview draws in every tool and mode, and Esc goes back to the
    /// Print dialog it came from.
    #[test]
    fn the_preview_draws_and_closes_back_to_the_dialog() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        for tool in [
            PvTool::Pick,
            PvTool::Imposition,
            PvTool::Marks,
            PvTool::Zoom,
        ] {
            for seps in [false, true] {
                let mut pv = PrintPreview::new(PrintState {
                    separations: seps,
                    ..PrintState::default()
                });
                pv.tool = tool;
                pv.fullscreen = tool == PvTool::Zoom && seps;
                app.print_preview = Some(pv);
                let input = egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 800.0))),
                    ..Default::default()
                };
                let mut out = ctx.run_ui(input, |ui| show(&mut app, ui));
                out.textures_delta.clear();
                assert!(app.print_preview.is_some());
            }
        }
        app.print_preview.as_mut().unwrap().from_dialog = true;
        app.print_preview.as_mut().unwrap().fullscreen = false;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 800.0))),
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            }],
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| show(&mut app, ui));
        out.textures_delta.clear();
        assert!(app.print_preview.is_none());
        assert!(matches!(app.dialog, Dialog::Print(_)));
    }
}
