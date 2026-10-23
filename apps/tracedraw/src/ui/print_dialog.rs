//! File > Print and the print preview's Print Options: one dialog of
//! seven tabs (General, Color, Separations, Layout, Prepress, PostScript
//! and the preflight issues) laid out at fixed places, with the help
//! button, Print Preview and the side preview at the bottom left and
//! Print, Cancel and Apply (OK, Cancel and Apply in the preview's
//! options) at the bottom right. See docs/behavior/printing.md.

use crate::app::App;
use crate::i18n::{tr, trf};
use crate::theme::Tokens;
use crate::ui::chrome;
use crate::ui::dialogs::{Dialog, PrintState};
use egui::{Align2, Color32, Context, FontId, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use serde::{Deserialize, Serialize};

/// The window: title bar, tabs, the tab's panel and the button row.
pub const SIZE: Vec2 = Vec2::new(636.0, 555.0);
/// Width the side preview adds.
const SIDE: f32 = 330.0;
/// The tab panel in body coordinates.
const PANEL: Rect = Rect {
    min: Pos2::new(12.0, 39.0),
    max: Pos2::new(607.0, 461.0),
};
/// The tabs: left edge and width in body coordinates.
const TABS: [(f32, f32); 7] = [
    (12.0, 61.0),
    (77.0, 57.0),
    (140.0, 83.0),
    (229.0, 61.0),
    (296.0, 101.0),
    (402.0, 77.0),
    (485.0, 121.0),
];
const FONT: f32 = 12.5;

/// The options of the tabs that are not part of the older print state:
/// kept with the preferences and in print styles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrintExtra {
    // General.
    pub use_ppd: bool,
    pub to_file: bool,
    /// 0 a single file, 1 pages to separate files, 2 plates to separate
    /// files.
    pub file_mode: usize,
    /// 0 even and odd pages, 1 even pages, 2 odd pages.
    pub parity: usize,
    /// The print style applied last ("" for none).
    pub style: String,
    // Color.
    /// 0 document color, 1 proof color.
    pub color_settings: usize,
    /// 0 CMYK, 1 RGB, 2 grayscale.
    pub output_colors: usize,
    pub preserve_cmyk: bool,
    pub preserve_black: bool,
    /// 0 perceptual, 1 relative colorimetric, 2 saturation, 3 absolute.
    pub intent: usize,
    // Separations.
    /// 0 preserve, 1 simulate, 2 ignore.
    pub overprints: usize,
    /// 0 the default screen, else an index into [`SCREENS`].
    pub screen: usize,
    /// 0 none, 1 all objects.
    pub trap: usize,
    pub trap_width: f64,
    pub trap_text: f64,
    pub color_separations: bool,
    pub show_empty_plates: bool,
    /// Plates left out of the separations, by name.
    pub excluded_plates: Vec<String>,
    // Layout.
    /// 0 as in the document, 1 fit to page, 2 reposition.
    pub position: usize,
    pub printer_marks: bool,
    /// 0 centre, 1 top left, 2 top right, 3 bottom left, 4 bottom right.
    pub reposition_to: usize,
    pub tiles_x: u32,
    pub tiles_y: u32,
    pub tile_pages: bool,
    pub tile_marks: bool,
    pub tile_overlap: f64,
    pub tile_overlap_pct: f64,
    // Prepress.
    pub reg_style: usize,
    pub info_inside: bool,
    pub crop_exterior: bool,
    /// The densities shown on the densitometer scale, in percent.
    pub densities: Vec<u8>,
    pub downsample_color: bool,
    pub downsample_color_dpi: f64,
    pub downsample_mono: bool,
    pub downsample_mono_dpi: f64,
    pub object_marks: bool,
    // PostScript.
    pub dsc: bool,
    /// 0 none, 1 LZW, 2 JPEG.
    pub compression: usize,
    pub jpeg_quality: f64,
    pub opi: bool,
    pub dcs: bool,
    pub type1: bool,
    pub tt_to_type1: bool,
    /// 0 the page only, 1 full screen, 2 thumbnails.
    pub pdf_start: usize,
    pub hyperlinks: bool,
    pub bookmarks: bool,
    pub auto_flatness: bool,
    pub auto_fountain_steps: bool,
    pub optimize_fountains: bool,
    // Issues.
    /// Preflight checks the user asked not to see again, by id.
    pub skipped_checks: Vec<String>,
    /// The side preview is open.
    pub side_preview: bool,
}

impl Default for PrintExtra {
    fn default() -> Self {
        PrintExtra {
            use_ppd: false,
            to_file: false,
            file_mode: 0,
            parity: 0,
            style: String::new(),
            color_settings: 0,
            output_colors: 0,
            preserve_cmyk: true,
            preserve_black: true,
            intent: 1,
            overprints: 0,
            screen: 0,
            trap: 0,
            trap_width: 0.5,
            trap_text: 18.0,
            color_separations: false,
            show_empty_plates: false,
            excluded_plates: Vec::new(),
            position: 0,
            printer_marks: false,
            reposition_to: 0,
            tiles_x: 1,
            tiles_y: 1,
            tile_pages: false,
            tile_marks: false,
            tile_overlap: 0.0,
            tile_overlap_pct: 0.0,
            reg_style: 0,
            info_inside: false,
            crop_exterior: false,
            densities: vec![0, 5, 25, 50, 75, 95, 100],
            downsample_color: false,
            downsample_color_dpi: 300.0,
            downsample_mono: false,
            downsample_mono_dpi: 1200.0,
            object_marks: false,
            dsc: false,
            compression: 0,
            jpeg_quality: 2.0,
            opi: true,
            dcs: true,
            type1: true,
            tt_to_type1: true,
            pdf_start: 0,
            hyperlinks: false,
            bookmarks: false,
            auto_flatness: false,
            auto_fountain_steps: false,
            optimize_fountains: false,
            skipped_checks: Vec::new(),
            side_preview: false,
        }
    }
}

/// Screen frequencies offered after Default, in lines per inch.
pub const SCREENS: [u32; 6] = [60, 85, 100, 133, 150, 175];
/// Every density a densitometer scale can show, in percent.
const ALL_DENSITIES: [u8; 7] = [0, 5, 25, 50, 75, 95, 100];

/// What the dialog is: File > Print, or the print preview's options.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Print,
    PreviewOptions,
}

/// What a button of the dialog asked for this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Print,
    Ok,
    Cancel,
    Apply,
    Preview,
}

/// File > Print.
pub fn print_dialog(app: &mut App, ctx: &Context, st: &mut PrintState, close: &mut bool) {
    match show(app, ctx, st, Kind::Print, close) {
        Some(Outcome::Print) => {
            match crate::export::print(app, st) {
                Ok(msg) if !msg.is_empty() => app.status = msg,
                Ok(_) => {}
                Err(e) => app.status = format!("{}: {e}", tr("dialog.print_failed")),
            }
            if app.settings.printing.remember_last {
                app.settings.printing.remember(st);
                app.settings.save();
            }
            *close = true;
        }
        Some(Outcome::Apply) => {
            app.settings.printing.remember(st);
            app.settings.save();
        }
        Some(Outcome::Preview) => {
            app.print_preview(Some(st.clone()));
            *close = true;
        }
        Some(Outcome::Cancel) => *close = true,
        Some(Outcome::Ok) | None => {}
    }
}

/// The dialog of `kind` for `st`; returns the button picked this frame.
/// The close button and Escape answer Cancel.
pub fn show(
    app: &mut App,
    ctx: &Context,
    st: &mut PrintState,
    kind: Kind,
    close: &mut bool,
) -> Option<Outcome> {
    let issues: Vec<crate::preflight::Issue> = if app.settings.printing.preflight {
        st.preflight
            .get_or_insert_with(|| crate::ui::preflight_dialog::issues_of(app))
            .iter()
            .filter(|i| !st.ex.skipped_checks.contains(&i.check.id().to_string()))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };
    let title = match kind {
        Kind::Print => tr("prt.title"),
        Kind::PreviewOptions => tr("prt.options_title"),
    };
    let side = kind == Kind::Print && st.ex.side_preview;
    let size = if side {
        SIZE + Vec2::new(SIDE, 0.0)
    } else {
        SIZE
    };
    let mut out = None;
    let x_closed = chrome::dialog(ctx, "print_dialog", &title, size, |ui| {
        let body = ui.max_rect();
        let o = body.min;
        tab_strip(ui, o, st, issues.len());
        let panel = PANEL.translate(o.to_vec2());
        ui.painter().rect(
            panel,
            0.0,
            Tokens::panel(),
            Stroke::new(1.0, Tokens::control_border()),
            egui::StrokeKind::Inside,
        );
        let po = panel.min;
        ui.scope_builder(egui::UiBuilder::new().max_rect(panel), |ui| match st.tab {
            1 => tab_color(app, ui, po, st),
            2 => tab_separations(app, ui, po, st),
            3 => tab_layout(app, ui, po, st),
            4 => tab_prepress(app, ui, po, st),
            5 => tab_postscript(ui, po, st),
            6 => tab_issues(app, ui, po, st, &issues),
            _ => tab_general(app, ui, po, st),
        });
        if side {
            side_preview(app, ui, o, st);
        }
        out = button_row(app, ui, o, st, kind, size.x);
    });
    if x_closed || chrome::take_close_request(ctx) {
        out = Some(Outcome::Cancel);
    }
    if out == Some(Outcome::Cancel) || (out == Some(Outcome::Ok) && kind == Kind::Print) {
        *close = true;
    }
    out
}

// ----- tabs -------------------------------------------------------------------

/// The tab strip over the panel; the tab in front is blue.
fn tab_strip(ui: &mut Ui, o: Pos2, st: &mut PrintState, issues: usize) {
    let names = [
        tr("prt.tab_general"),
        tr("prt.tab_color"),
        tr("prt.tab_separations"),
        tr("prt.tab_layout"),
        tr("prt.tab_prepress"),
        tr("prt.tab_postscript"),
        issues_tab_name(issues),
    ];
    for (i, ((x, w), name)) in TABS.iter().zip(names.iter()).enumerate() {
        let r = Rect::from_min_size(o + Vec2::new(*x, 15.0), Vec2::new(*w, 24.0));
        let resp = ui.interact(r, egui::Id::new(("print_tab", i)), Sense::click());
        if resp.clicked() {
            st.tab = i;
        }
        let on = st.tab == i;
        let fill = if on {
            tab_blue()
        } else if resp.hovered() {
            Tokens::tool_hover()
        } else {
            Tokens::button()
        };
        ui.painter().rect(
            r,
            egui::CornerRadius {
                nw: 3,
                ne: 3,
                sw: 0,
                se: 0,
            },
            fill,
            Stroke::new(1.0, Tokens::control_border()),
            egui::StrokeKind::Inside,
        );
        let text = fit_text(ui, name, w - 10.0);
        ui.painter().text(
            r.center(),
            Align2::CENTER_CENTER,
            text,
            FontId::proportional(FONT),
            if on { Color32::WHITE } else { Tokens::text() },
        );
    }
}

/// "No Issues", "1 Issue" or "n Issues".
pub fn issues_tab_name(n: usize) -> String {
    match n {
        0 => tr("prt.tab_no_issues"),
        1 => tr("prt.tab_one_issue"),
        _ => trf("prt.tab_issues", &[("n", &n.to_string())]),
    }
}

/// The blue of the tab in front and of the default button.
fn tab_blue() -> Color32 {
    Color32::from_rgb(0x1E, 0x4D, 0x8C)
}

/// `text` cut to `width` with an ellipsis.
fn fit_text(ui: &Ui, text: &str, width: f32) -> String {
    let fits = |s: &str| {
        ui.painter()
            .layout_no_wrap(s.to_string(), FontId::proportional(FONT), Color32::WHITE)
            .size()
            .x
            <= width
    };
    if fits(text) {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    for n in (1..chars.len()).rev() {
        let s: String = chars[..n].iter().collect::<String>() + "...";
        if fits(&s) {
            return s;
        }
    }
    "...".into()
}

fn tab_general(app: &mut App, ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    group(
        ui,
        o,
        rect(16.0, 25.0, 581.0, 176.0),
        &tr("prt.destination"),
    );
    lbl(ui, o, 24.0, 47.0, &tr("prt.printer"), true);
    let printer_name = match (&st.printer, st.destination) {
        (Some(p), 0) => p.clone(),
        _ => tr("prt.viewer"),
    };
    combo(
        ui,
        "prt_printer",
        o,
        rect(109.0, 35.0, 400.0, 60.0),
        printer_name,
        true,
        |ui| {
            for p in st.printers.clone() {
                let on = st.destination == 0 && st.printer.as_deref() == Some(p.name.as_str());
                if ui.selectable_label(on, &p.name).clicked() {
                    st.destination = 0;
                    st.printer = Some(p.name.clone());
                }
            }
            if ui
                .selectable_label(st.printer.is_none(), tr("prt.viewer"))
                .clicked()
            {
                st.destination = 0;
                st.printer = None;
            }
        },
    );
    if gear(ui, "prt_printer_prefs", o + Vec2::new(421.0, 47.0)) {
        if let Some(p) = st.printer.clone() {
            if let Err(e) = crate::printers::open_preferences(&p) {
                app.status = e;
            }
        }
    }
    lbl(ui, o, 24.0, 80.0, &tr("prt.orientation"), true);
    let orient_doc = if app
        .doc()
        .pages
        .first()
        .map(|p| p.size.width > p.size.height)
        .unwrap_or(false)
    {
        tr("prt.landscape")
    } else {
        tr("prt.portrait")
    };
    let orients = [
        trf("prt.orient_match", &[("o", &orient_doc)]),
        tr("prt.portrait"),
        tr("prt.landscape"),
    ];
    combo(
        ui,
        "prt_orientation",
        o,
        rect(109.0, 68.0, 400.0, 93.0),
        orients[st.orientation.min(2)].clone(),
        true,
        |ui| {
            for (i, t) in orients.iter().enumerate() {
                if ui
                    .selectable_value(&mut st.orientation, i, t.as_str())
                    .clicked()
                    && i > 0
                {
                    st.landscape = i == 2;
                }
            }
        },
    );
    chk(
        ui,
        "prt_ppd",
        o,
        409.0,
        80.0,
        &mut st.ex.use_ppd,
        &tr("prt.use_ppd"),
        true,
    );
    let info = st
        .printer
        .as_ref()
        .and_then(|n| st.printers.iter().find(|p| &p.name == n))
        .cloned();
    lbl(ui, o, 24.0, 108.0, &tr("prt.status"), true);
    let status = match &info {
        Some(p) if !p.status.is_empty() => p.status.clone(),
        Some(p) if p.default => tr("prt.status_default"),
        _ => tr("prt.status_ready"),
    };
    lbl(ui, o, 109.0, 108.0, &status, true);
    lbl(ui, o, 24.0, 130.0, &tr("prt.location"), true);
    let location = info
        .as_ref()
        .map(|p| p.location.clone())
        .unwrap_or_default();
    lbl(ui, o, 109.0, 130.0, &location, true);
    chk(
        ui,
        "prt_to_file",
        o,
        409.0,
        130.0,
        &mut st.ex.to_file,
        &tr("prt.print_to_file"),
        true,
    );
    lbl(ui, o, 24.0, 158.0, &tr("prt.comment"), true);
    let comment = info.as_ref().map(|p| p.comment.clone()).unwrap_or_default();
    lbl(ui, o, 109.0, 158.0, &comment, true);
    let modes = [
        tr("prt.file_single"),
        tr("prt.file_pages"),
        tr("prt.file_plates"),
    ];
    combo(
        ui,
        "prt_file_mode",
        o,
        rect(409.0, 146.0, 574.0, 170.0),
        modes[st.ex.file_mode.min(2)].clone(),
        st.ex.to_file,
        |ui| {
            for (i, m) in modes.iter().enumerate() {
                ui.selectable_value(&mut st.ex.file_mode, i, m.as_str());
            }
        },
    );

    // Print range.
    group(ui, o, rect(16.0, 192.0, 265.0, 333.0), &tr("prt.range"));
    let docs = app.docs.len() > 1;
    let sel = !app.selection.is_empty();
    if radio(
        ui,
        "prt_r0",
        o,
        24.0,
        211.0,
        st.range == 0,
        &tr("prt.current_document"),
        true,
    ) {
        st.range = 0;
    }
    if radio(
        ui,
        "prt_r4",
        o,
        158.0,
        211.0,
        st.range == 4,
        &tr("prt.documents"),
        docs,
    ) {
        st.range = 4;
    }
    if radio(
        ui,
        "prt_r1",
        o,
        24.0,
        234.0,
        st.range == 1,
        &tr("prt.current_page"),
        true,
    ) {
        st.range = 1;
    }
    if radio(
        ui,
        "prt_r3",
        o,
        163.0,
        234.0,
        st.range == 3,
        &tr("prt.selection"),
        sel,
    ) {
        st.range = 3;
    }
    let many = app.doc().pages.len() > 1;
    if radio(
        ui,
        "prt_r2",
        o,
        24.0,
        261.0,
        st.range == 2,
        &tr("prt.pages"),
        many,
    ) {
        st.range = 2;
    }
    let pages_on = st.range == 2;
    if st.pages.is_empty() {
        st.pages = format!("1-{}", app.doc().pages.len());
    }
    field(
        ui,
        "prt_pages",
        o,
        rect(128.0, 249.0, 233.0, 273.0),
        &mut st.pages,
        pages_on,
    );
    let parities = [tr("prt.even_odd"), tr("prt.even"), tr("prt.odd")];
    combo(
        ui,
        "prt_parity",
        o,
        rect(128.0, 281.0, 233.0, 305.0),
        parities[st.ex.parity.min(2)].clone(),
        pages_on || st.range == 0,
        |ui| {
            for (i, p) in parities.iter().enumerate() {
                ui.selectable_value(&mut st.ex.parity, i, p.as_str());
            }
        },
    );

    // Copies.
    group(ui, o, rect(298.0, 191.0, 581.0, 300.0), &tr("prt.copies"));
    lbl(ui, o, 306.0, 215.0, &tr("prt.copies_n"), true);
    if let Some(v) = spin(
        ui,
        "prt_copies",
        o,
        rect(462.0, 203.0, 541.0, 227.0),
        st.copies as f64,
        0,
        "",
        1.0,
        1.0..=999.0,
        true,
    ) {
        st.copies = v.round() as u32;
    }
    collate_picture(ui, o + Vec2::new(306.0, 247.0), st.collate && st.copies > 1);
    chk(
        ui,
        "prt_collate",
        o,
        464.0,
        264.0,
        &mut st.collate,
        &tr("prt.collate"),
        st.copies > 1,
    );
    chk(
        ui,
        "prt_bitmap",
        o,
        301.0,
        320.0,
        &mut st.as_bitmap,
        &tr("prt.as_bitmap"),
        true,
    );
    if let Some(v) = spin(
        ui,
        "prt_bitmap_dpi",
        o,
        rect(462.0, 308.0, 543.0, 332.0),
        st.bitmap_dpi,
        0,
        "",
        1.0,
        36.0..=1200.0,
        st.as_bitmap,
    ) {
        st.bitmap_dpi = v;
    }
    lbl(ui, o, 551.0, 320.0, "dpi", true);

    // Print style.
    lbl(ui, o, 18.0, 372.0, &tr("prt.style"), true);
    style_combo(app, ui, o, st);
    if button(
        ui,
        "prt_save_style",
        o,
        rect(466.0, 360.0, 581.0, 385.0),
        &tr("prt.save_as"),
        true,
    ) {
        let name = if st.ex.style.is_empty() {
            tr("prt.new_style")
        } else {
            st.ex.style.clone()
        };
        ui.ctx()
            .data_mut(|d| d.insert_temp(egui::Id::new("prt_style_name"), Some(name)));
    }
    save_style_window(app, ui.ctx(), st);
    // Keep the older "all pages" choice in step with the range.
    st.all_pages = st.range != 1;
}

/// The print style list: Custom while the settings differ from every
/// saved style, then Default and the saved styles.
fn style_combo(app: &mut App, ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    let current = crate::prefs::PrintPrefs::of(st);
    let styles = app.settings.printing.styles.clone();
    let shown = styles
        .iter()
        .find(|s| s.name == st.ex.style && s.prefs.same_settings(&current))
        .map(|s| s.name.clone())
        .unwrap_or_else(|| tr("prt.style_custom"));
    let mut pick: Option<crate::prefs::PrintPrefs> = None;
    combo(
        ui,
        "prt_style",
        o,
        rect(141.0, 360.0, 440.0, 385.0),
        shown,
        true,
        |ui| {
            if ui
                .selectable_label(false, tr("prt.style_default"))
                .clicked()
            {
                pick = Some(crate::prefs::PrintPrefs::default());
            }
            for s in &styles {
                if ui
                    .selectable_label(st.ex.style == s.name, s.name.as_str())
                    .clicked()
                {
                    let mut p = s.prefs.clone();
                    p.extra.style = s.name.clone();
                    pick = Some(p);
                }
            }
        },
    );
    if let Some(p) = pick {
        let mut next = p.state();
        next.copies = st.copies;
        next.range = st.range;
        next.pages = st.pages.clone();
        next.printers = std::mem::take(&mut st.printers);
        next.preflight = st.preflight.take();
        next.tab = st.tab;
        next.ex.side_preview = st.ex.side_preview;
        *st = next;
    }
}

/// Save As: a small window asking the style's name.
fn save_style_window(app: &mut App, ctx: &Context, st: &mut PrintState) {
    let id = egui::Id::new("prt_style_name");
    let Some(Some(mut name)) = ctx.data(|d| d.get_temp::<Option<String>>(id)) else {
        return;
    };
    let mut done = false;
    let mut keep = true;
    let closed = chrome::dialog(
        ctx,
        "prt_save_style_dialog",
        &tr("prt.save_style_title"),
        Vec2::new(380.0, 150.0),
        |ui| {
            let o = ui.max_rect().min;
            lbl(ui, o, 16.0, 26.0, &tr("prt.style_name"), true);
            let r = Rect::from_min_max(o + Vec2::new(16.0, 40.0), o + Vec2::new(364.0, 64.0));
            chrome::text_field(ui, r, &mut name).request_focus();
            let ok = Rect::from_min_max(o + Vec2::new(178.0, 84.0), o + Vec2::new(268.0, 110.0));
            let cancel =
                Rect::from_min_max(o + Vec2::new(274.0, 84.0), o + Vec2::new(364.0, 110.0));
            if chrome::button_at(ui, ok, &tr("prt.ok")).clicked()
                || ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                done = true;
            }
            if chrome::button_at(ui, cancel, &tr("prt.cancel")).clicked() {
                keep = false;
            }
        },
    );
    let name_t = name.trim().to_string();
    if done && !name_t.is_empty() {
        st.ex.style = name_t.clone();
        let mut prefs = crate::prefs::PrintPrefs::of(st);
        prefs.styles.clear();
        let list = &mut app.settings.printing.styles;
        list.retain(|s| s.name != name_t);
        list.push(crate::prefs::PrintStyle {
            name: name_t,
            prefs,
        });
        app.settings.save();
        keep = false;
    }
    if closed {
        keep = false;
    }
    ctx.data_mut(|d| d.insert_temp(id, keep.then_some(name)));
}

/// The copies picture: two copies of three pages, as whole sets
/// (collated) or each page twice.
fn collate_picture(ui: &Ui, at: Pos2, collated: bool) {
    let p = ui.painter();
    let page = |x: f32, y: f32, n: usize| {
        let r = Rect::from_min_size(Pos2::new(x, y), Vec2::new(14.0, 19.0));
        p.rect(
            r,
            0.0,
            Color32::WHITE,
            Stroke::new(1.0, Color32::from_gray(40)),
            egui::StrokeKind::Inside,
        );
        p.text(
            r.right_top() + Vec2::new(-2.0, 2.0),
            Align2::RIGHT_TOP,
            n.to_string(),
            FontId::proportional(9.0),
            Color32::BLACK,
        );
    };
    if collated {
        for set in 0..2 {
            let x = at.x + set as f32 * 48.0;
            for k in (0..3).rev() {
                page(x + k as f32 * 8.0, at.y + 12.0 - k as f32 * 6.0, 3 - k);
            }
        }
    } else {
        for n in 0..3 {
            let x = at.x + n as f32 * 33.0;
            page(x + 9.0, at.y + 6.0, n + 1);
            page(x, at.y + 12.0, n + 1);
        }
    }
}

fn tab_color(app: &mut App, ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    lbl(ui, o, 32.0, 24.0, &tr("prt.color"), true);
    if radio(
        ui,
        "prt_comp",
        o,
        223.0,
        24.0,
        !st.separations,
        &tr("prt.composite"),
        true,
    ) {
        st.separations = false;
    }
    if radio(
        ui,
        "prt_seps",
        o,
        223.0,
        46.0,
        st.separations,
        &tr("prt.separations"),
        true,
    ) {
        st.separations = true;
    }
    hline(ui, o, 18.0, 510.0, 62.0);
    let proof = st.separations;
    lbl(ui, o, 32.0, 79.0, &tr("prt.settings"), !proof);
    if radio(
        ui,
        "prt_doc_color",
        o,
        223.0,
        79.0,
        st.ex.color_settings == 0,
        &tr("prt.doc_color"),
        !proof,
    ) {
        st.ex.color_settings = 0;
    }
    if radio(
        ui,
        "prt_proof",
        o,
        223.0,
        102.0,
        st.ex.color_settings == 1,
        &tr("prt.proof"),
        !proof,
    ) {
        st.ex.color_settings = 1;
    }
    lbl(ui, o, 32.0, 129.0, &tr("prt.conversions"), !proof);
    combo(
        ui,
        "prt_conv",
        o,
        rect(218.0, 117.0, 496.0, 141.0),
        tr("prt.engine"),
        !proof,
        |ui| {
            let _ = ui.selectable_label(true, tr("prt.engine"));
        },
    );
    lbl(ui, o, 32.0, 161.0, &tr("prt.output_colors"), !proof);
    let outs = [tr("prt.out_cmyk"), tr("prt.out_rgb"), tr("prt.out_gray")];
    combo(
        ui,
        "prt_out",
        o,
        rect(218.0, 149.0, 300.0, 173.0),
        outs[st.ex.output_colors.min(2)].clone(),
        !proof,
        |ui| {
            for (i, t) in outs.iter().enumerate() {
                ui.selectable_value(&mut st.ex.output_colors, i, t.as_str());
            }
        },
    );
    chk(
        ui,
        "prt_spot",
        o,
        218.0,
        194.0,
        &mut st.spot_to_cmyk,
        &tr("prt.spot_to_cmyk"),
        true,
    );
    lbl(ui, o, 32.0, 226.0, &tr("prt.profile"), true);
    let profile = trf(
        "prt.profile_default",
        &[("name", &app.settings.color.cmyk_profile)],
    );
    combo(
        ui,
        "prt_profile",
        o,
        rect(218.0, 214.0, 496.0, 238.0),
        profile.clone(),
        true,
        |ui| {
            let _ = ui.selectable_label(true, profile.as_str());
        },
    );
    chk(
        ui,
        "prt_keep_cmyk",
        o,
        218.0,
        254.0,
        &mut st.ex.preserve_cmyk,
        &tr("prt.preserve_cmyk"),
        false,
    );
    chk(
        ui,
        "prt_keep_black",
        o,
        218.0,
        277.0,
        &mut st.ex.preserve_black,
        &tr("prt.preserve_black"),
        true,
    );
    lbl(ui, o, 32.0, 305.0, &tr("prt.intent"), true);
    let intents = [
        tr("prt.intent_perceptual"),
        tr("prt.intent_relative"),
        tr("prt.intent_saturation"),
        tr("prt.intent_absolute"),
    ];
    combo(
        ui,
        "prt_intent",
        o,
        rect(218.0, 293.0, 496.0, 317.0),
        intents[st.ex.intent.min(3)].clone(),
        true,
        |ui| {
            for (i, t) in intents.iter().enumerate() {
                ui.selectable_value(&mut st.ex.intent, i, t.as_str());
            }
        },
    );
    hline(ui, o, 18.0, 510.0, 325.0);
    info_icon(ui, o + Vec2::new(26.0, 350.0));
    wrapped(ui, o, 47.0, 337.0, 470.0, &tr("prt.color_info"), true);
}

fn tab_separations(app: &mut App, ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    lbl(ui, o, 16.0, 25.0, &tr("prt.doc_overprints"), true);
    let ops = [tr("prt.preserve"), tr("prt.simulate"), tr("prt.ignore")];
    combo(
        ui,
        "prt_ops",
        o,
        rect(16.0, 40.0, 136.0, 64.0),
        ops[st.ex.overprints.min(2)].clone(),
        true,
        |ui| {
            for (i, t) in ops.iter().enumerate() {
                ui.selectable_value(&mut st.ex.overprints, i, t.as_str());
            }
        },
    );
    lbl(ui, o, 299.0, 25.0, &tr("prt.screen"), true);
    let screen_name = |i: usize| {
        if i == 0 {
            tr("prt.screen_default")
        } else {
            SCREENS
                .get(i - 1)
                .map(|v| v.to_string())
                .unwrap_or_default()
        }
    };
    combo(
        ui,
        "prt_screen",
        o,
        rect(299.0, 40.0, 418.0, 64.0),
        screen_name(st.ex.screen),
        true,
        |ui| {
            for i in 0..=SCREENS.len() {
                ui.selectable_value(&mut st.ex.screen, i, screen_name(i));
            }
        },
    );
    lbl(ui, o, 424.0, 52.0, &tr("prt.lpi"), true);

    group(ui, o, rect(16.0, 79.0, 293.0, 200.0), &tr("prt.trapping"));
    lbl(ui, o, 24.0, 98.0, &tr("prt.auto_trap"), true);
    let traps = [tr("prt.trap_none"), tr("prt.trap_all")];
    combo(
        ui,
        "prt_trap",
        o,
        rect(24.0, 113.0, 153.0, 137.0),
        traps[st.ex.trap.min(1)].clone(),
        true,
        |ui| {
            for (i, t) in traps.iter().enumerate() {
                ui.selectable_value(&mut st.ex.trap, i, t.as_str());
            }
        },
    );
    let trap_on = st.ex.trap > 0;
    if let Some(v) = spin(
        ui,
        "prt_trap_w",
        o,
        rect(205.0, 113.0, 282.0, 137.0),
        st.ex.trap_width,
        1,
        "pt",
        0.1,
        0.0..=10.0,
        trap_on,
    ) {
        st.ex.trap_width = v;
    }
    lbl(ui, o, 205.0, 154.0, &tr("prt.trap_text_above"), trap_on);
    if let Some(v) = spin(
        ui,
        "prt_trap_text",
        o,
        rect(205.0, 168.0, 282.0, 192.0),
        st.ex.trap_text,
        1,
        "pt",
        0.5,
        0.0..=144.0,
        trap_on,
    ) {
        st.ex.trap_text = v;
    }

    group(ui, o, rect(298.0, 79.0, 556.0, 189.0), &tr("prt.options"));
    chk(
        ui,
        "prt_text_black",
        o,
        305.0,
        98.0,
        &mut st.text_black,
        &tr("prt.text_black"),
        true,
    );
    chk(
        ui,
        "prt_ob",
        o,
        305.0,
        121.0,
        &mut st.overprint_black,
        &tr("prt.overprint_black"),
        true,
    );
    chk(
        ui,
        "prt_color_seps",
        o,
        305.0,
        144.0,
        &mut st.ex.color_separations,
        &tr("prt.color_seps"),
        st.separations,
    );
    if button(
        ui,
        "prt_adv",
        o,
        rect(305.0, 159.0, 408.0, 182.0),
        &tr("prt.advanced"),
        true,
    ) {
        ui.ctx()
            .data_mut(|d| d.insert_temp(egui::Id::new("prt_advanced_open"), true));
    }
    advanced_window(app, ui.ctx());
    chk(
        ui,
        "prt_empty",
        o,
        16.0,
        215.0,
        &mut st.ex.show_empty_plates,
        &tr("prt.show_empty"),
        true,
    );
    plates_table(app, ui, o, st);
}

/// The plates: Print ticks, the ink's colour and its name.
fn plates_table(app: &mut App, ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    let table = rect(16.0, 230.0, 556.0, 380.0).translate(o.to_vec2());
    let p = ui.painter().clone();
    p.rect(
        table,
        0.0,
        Tokens::field(),
        Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    let head = Rect::from_min_max(table.min, Pos2::new(table.right(), table.top() + 23.0));
    p.rect_filled(head.shrink(1.0), 0.0, Tokens::button());
    let split = table.left() + 82.0;
    p.vline(
        split,
        head.y_range(),
        Stroke::new(1.0, Tokens::control_border()),
    );
    p.hline(
        table.x_range(),
        head.bottom(),
        Stroke::new(1.0, Tokens::control_border()),
    );
    p.text(
        Pos2::new((table.left() + split) / 2.0, head.center().y),
        Align2::CENTER_CENTER,
        tr("prt.include"),
        FontId::proportional(FONT),
        Tokens::text(),
    );
    p.text(
        Pos2::new((split + table.right()) / 2.0, head.center().y),
        Align2::CENTER_CENTER,
        tr("prt.col_color"),
        FontId::proportional(FONT),
        Tokens::text(),
    );
    let doc = app.doc();
    let used = crate::export::plates_used(doc);
    for (row, (plate, name, color)) in plate_rows(doc, &used, st.ex.show_empty_plates)
        .into_iter()
        .enumerate()
    {
        let y = head.bottom() + 9.0 + row as f32 * 17.0;
        if y > table.bottom() - 18.0 {
            break;
        }
        let mut on = !st.ex.excluded_plates.contains(&plate);
        let before = on;
        chk(
            ui,
            &format!("prt_plate_{plate}"),
            Pos2::new(table.left(), head.bottom()),
            5.0,
            y - head.bottom(),
            &mut on,
            "",
            st.separations,
        );
        if on != before {
            if on {
                st.ex.excluded_plates.retain(|p| p != &plate);
            } else {
                st.ex.excluded_plates.push(plate.clone());
            }
        }
        let sw = Rect::from_min_max(
            Pos2::new(table.left() + 22.0, y - 6.0),
            Pos2::new(table.left() + 112.0, y + 6.0),
        );
        p.rect_filled(sw, 0.0, color);
        p.text(
            Pos2::new(sw.right() + 7.0, y),
            Align2::LEFT_CENTER,
            name,
            FontId::proportional(FONT),
            Tokens::text(),
        );
    }
    // The horizontal scroll bar along the bottom.
    let bar = Rect::from_min_max(
        Pos2::new(table.left() + 1.0, table.bottom() - 15.0),
        Pos2::new(table.right() - 1.0, table.bottom() - 1.0),
    );
    p.rect_filled(bar, 0.0, Tokens::button());
    p.rect_filled(
        bar.shrink2(Vec2::new(18.0, 4.0)),
        2.0,
        Tokens::control_border(),
    );
}

/// The plates of `sheets` with the colour of their swatch: the process
/// inks, then the spot inks; empty ones only when asked.
pub fn plate_rows(
    sheets: &tracedraw_core::Document,
    used: &[bool],
    show_empty: bool,
) -> Vec<(String, String, Color32)> {
    let process = [
        (tr("fxp.cyan"), Color32::from_rgb(0x00, 0xAE, 0xEF)),
        (tr("fxp.magenta"), Color32::from_rgb(0xEC, 0x00, 0x8C)),
        (tr("fxp.yellow"), Color32::from_rgb(0xFF, 0xF2, 0x00)),
        (tr("fxp.black"), Color32::from_rgb(0x23, 0x1F, 0x20)),
    ];
    let mut out = Vec::new();
    for (i, (plate, name)) in crate::export::plates(sheets).into_iter().enumerate() {
        if !show_empty && !used.get(i).copied().unwrap_or(false) {
            continue;
        }
        let (shown, color) = match process.get(i) {
            Some((n, c)) => (n.clone(), *c),
            None => {
                let c = sheets
                    .inks
                    .iter()
                    .find(|ink| crate::export::Plate::Spot(ink.id) == plate)
                    .map(|ink| {
                        let [r, g, b] = ink.color(1.0).to_rgb8();
                        Color32::from_rgb(r, g, b)
                    })
                    .unwrap_or(Color32::GRAY);
                (name.clone(), c)
            }
        };
        out.push((name, shown, color));
    }
    out
}

/// Separations > Advanced: the screen of each process ink.
fn advanced_window(_app: &mut App, ctx: &Context) {
    let id = egui::Id::new("prt_advanced_open");
    if !ctx.data(|d| d.get_temp::<bool>(id)).unwrap_or(false) {
        return;
    }
    let mut keep = true;
    let closed = chrome::dialog(
        ctx,
        "prt_advanced_dialog",
        &tr("prt.advanced_title"),
        Vec2::new(420.0, 250.0),
        |ui| {
            let o = ui.max_rect().min;
            lbl(ui, o, 20.0, 24.0, &tr("prt.col_color"), true);
            lbl(ui, o, 170.0, 24.0, &tr("prt.frequency"), true);
            lbl(ui, o, 290.0, 24.0, &tr("prt.angle"), true);
            let inks = [
                ("fxp.cyan", 15.0),
                ("fxp.magenta", 75.0),
                ("fxp.yellow", 0.0),
                ("fxp.black", 45.0),
            ];
            for (i, (key, angle)) in inks.iter().enumerate() {
                let y = 52.0 + i as f32 * 26.0;
                lbl(ui, o, 20.0, y, &tr(key), true);
                lbl(ui, o, 170.0, y, "60,0", true);
                lbl(ui, o, 290.0, y, &format!("{angle:.1}"), true);
            }
            let ok = Rect::from_min_max(o + Vec2::new(214.0, 175.0), o + Vec2::new(304.0, 201.0));
            let cancel =
                Rect::from_min_max(o + Vec2::new(310.0, 175.0), o + Vec2::new(400.0, 201.0));
            if chrome::button_at(ui, ok, &tr("prt.ok")).clicked()
                || chrome::button_at(ui, cancel, &tr("prt.cancel")).clicked()
            {
                keep = false;
            }
        },
    );
    if closed {
        keep = false;
    }
    ctx.data_mut(|d| d.insert_temp(id, keep));
}

fn tab_layout(app: &mut App, ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    group(ui, o, rect(17.0, 24.0, 576.0, 287.0), &tr("prt.image_pos"));
    if radio(
        ui,
        "prt_pos0",
        o,
        25.0,
        43.0,
        st.ex.position == 0,
        &tr("prt.as_document"),
        true,
    ) {
        st.ex.position = 0;
    }
    if radio(
        ui,
        "prt_pos1",
        o,
        25.0,
        67.0,
        st.ex.position == 1,
        &tr("prt.fit"),
        true,
    ) {
        st.ex.position = 1;
    }
    chk(
        ui,
        "prt_pmarks",
        o,
        51.0,
        89.0,
        &mut st.ex.printer_marks,
        &tr("prt.printer_marks"),
        st.ex.position == 1,
    );
    if radio(
        ui,
        "prt_pos2",
        o,
        25.0,
        116.0,
        st.ex.position == 2,
        &tr("prt.reposition"),
        true,
    ) {
        st.ex.position = 2;
    }
    st.fit_to_page = st.ex.position == 1;
    let places = [
        tr("prt.pos_center"),
        tr("prt.pos_top_left"),
        tr("prt.pos_top_right"),
        tr("prt.pos_bottom_left"),
        tr("prt.pos_bottom_right"),
    ];
    combo(
        ui,
        "prt_place",
        o,
        rect(175.0, 104.0, 303.0, 128.0),
        places[st.ex.reposition_to.min(4)].clone(),
        st.ex.position == 2,
        |ui| {
            for (i, t) in places.iter().enumerate() {
                ui.selectable_value(&mut st.ex.reposition_to, i, t.as_str());
            }
        },
    );
    position_picture(ui, rect(523.0, 52.0, 569.0, 111.0).translate(o.to_vec2()));

    lbl(ui, o, 23.0, 147.0, &tr("prt.settings_for"), true);
    let page_name = trf("prt.page_n", &[("n", "1")]);
    combo(
        ui,
        "prt_for_page",
        o,
        rect(149.0, 135.0, 229.0, 159.0),
        page_name.clone(),
        true,
        |ui| {
            let _ = ui.selectable_label(true, page_name.as_str());
        },
    );
    lbl(ui, o, 52.0, 175.0, &tr("prt.position"), false);
    lbl(ui, o, 172.0, 175.0, &tr("prt.size"), false);
    lbl(ui, o, 275.0, 175.0, &tr("prt.scale"), false);
    lbl(ui, o, 406.0, 175.0, &tr("prt.tiles"), false);
    let (x, y, w, h) = content_box(app);
    let free = st.ex.position == 2;
    lbl(ui, o, 30.0, 200.0, "x:", free);
    lbl(ui, o, 30.0, 217.0, "y:", free);
    small_value(ui, o, rect(42.0, 192.0, 128.0, 208.0), &mm(x), free);
    small_value(ui, o, rect(42.0, 209.0, 128.0, 225.0), &mm(y), free);
    lbl(ui, o, 156.0, 200.0, "\u{2194}", free);
    lbl(ui, o, 156.0, 217.0, "\u{2195}", free);
    small_value(ui, o, rect(172.0, 192.0, 253.0, 208.0), &mm(w), free);
    small_value(ui, o, rect(172.0, 209.0, 253.0, 225.0), &mm(h), free);
    let scale = format!("{:.0}", st.scale);
    small_value(ui, o, rect(279.0, 192.0, 343.0, 208.0), &scale, free);
    small_value(ui, o, rect(279.0, 209.0, 343.0, 225.0), &scale, free);
    lbl(ui, o, 348.0, 200.0, "%", free);
    lbl(ui, o, 348.0, 217.0, "%", free);
    lbl(ui, o, 408.0, 200.0, "\u{25A4}", st.ex.tile_pages);
    lbl(ui, o, 408.0, 217.0, "\u{25A5}", st.ex.tile_pages);
    if let Some(v) = spin(
        ui,
        "prt_tiles_x",
        o,
        rect(420.0, 192.0, 488.0, 208.0),
        st.ex.tiles_x as f64,
        0,
        "",
        1.0,
        1.0..=20.0,
        st.ex.tile_pages,
    ) {
        st.ex.tiles_x = v as u32;
    }
    if let Some(v) = spin(
        ui,
        "prt_tiles_y",
        o,
        rect(420.0, 209.0, 488.0, 225.0),
        st.ex.tiles_y as f64,
        0,
        "",
        1.0,
        1.0..=20.0,
        st.ex.tile_pages,
    ) {
        st.ex.tiles_y = v as u32;
    }
    lock_icon(ui, o + Vec2::new(521.0, 210.0));
    chk(
        ui,
        "prt_tile",
        o,
        23.0,
        242.0,
        &mut st.ex.tile_pages,
        &tr("prt.tile_pages"),
        true,
    );
    chk(
        ui,
        "prt_tile_marks",
        o,
        211.0,
        242.0,
        &mut st.ex.tile_marks,
        &tr("prt.tile_marks"),
        st.ex.tile_pages,
    );
    lbl(
        ui,
        o,
        40.0,
        269.0,
        &tr("prt.tile_overlap"),
        st.ex.tile_pages,
    );
    if let Some(v) = spin(
        ui,
        "prt_overlap",
        o,
        rect(201.0, 257.0, 276.0, 281.0),
        st.ex.tile_overlap,
        1,
        "mm",
        0.5,
        0.0..=100.0,
        st.ex.tile_pages,
    ) {
        st.ex.tile_overlap = v;
    }
    if let Some(v) = spin(
        ui,
        "prt_overlap_pct",
        o,
        rect(298.0, 257.0, 373.0, 281.0),
        st.ex.tile_overlap_pct,
        0,
        "",
        1.0,
        0.0..=50.0,
        st.ex.tile_pages,
    ) {
        st.ex.tile_overlap_pct = v;
    }
    lbl(
        ui,
        o,
        380.0,
        269.0,
        &tr("prt.page_width_pct"),
        st.ex.tile_pages,
    );

    chk(
        ui,
        "prt_bleed",
        o,
        44.0,
        324.0,
        &mut st.bleed,
        &tr("prt.bleed_limit"),
        true,
    );
    if let Some(v) = spin(
        ui,
        "prt_bleed_mm",
        o,
        rect(208.0, 312.0, 290.0, 336.0),
        st.bleed_limit,
        1,
        "mm",
        0.5,
        0.0..=50.0,
        st.bleed,
    ) {
        st.bleed_limit = v;
    }
    lbl(ui, o, 17.0, 377.0, &tr("prt.imposition"), true);
    let kinds = [
        tr("prt.imp_as_document"),
        tr("print.layout_n_up"),
        tr("print.layout_booklet"),
        tr("print.layout_repeat"),
    ];
    combo(
        ui,
        "prt_imposition",
        o,
        rect(149.0, 365.0, 475.0, 389.0),
        kinds[st.layout_kind.min(3)].clone(),
        true,
        |ui| {
            for (i, t) in kinds.iter().enumerate() {
                ui.selectable_value(&mut st.layout_kind, i, t.as_str());
            }
        },
    );
    if button(
        ui,
        "prt_imp_edit",
        o,
        rect(497.0, 365.0, 577.0, 389.0),
        &tr("prt.edit"),
        true,
    ) {
        let mut s = st.clone();
        s.tab = 3;
        app.print_preview(Some(s));
        app.dialog = Dialog::None;
    }
}

/// The page's objects' bounds, mm from the page's top left.
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

fn mm(v: f64) -> String {
    format!("{v:.2} mm").replace('.', ",")
}

fn tab_prepress(app: &mut App, ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    group(ui, o, rect(13.0, 23.0, 273.0, 83.0), &tr("prt.paper_film"));
    chk(
        ui,
        "prt_invert",
        o,
        21.0,
        43.0,
        &mut st.invert,
        &tr("prt.invert"),
        true,
    );
    chk(
        ui,
        "prt_mirror",
        o,
        21.0,
        66.0,
        &mut st.mirror,
        &tr("prt.mirror"),
        true,
    );
    film_icon(ui, o + Vec2::new(254.0, 50.0), st.mirror);

    group(ui, o, rect(320.0, 23.0, 590.0, 88.0), &tr("prt.reg_group"));
    chk(
        ui,
        "prt_reg",
        o,
        327.0,
        43.0,
        &mut st.registration_marks,
        &tr("prt.reg_marks"),
        true,
    );
    lbl(
        ui,
        o,
        327.0,
        70.0,
        &tr("prt.reg_style"),
        st.registration_marks,
    );
    reg_style(ui, o, st);

    group(ui, o, rect(13.0, 103.0, 273.0, 234.0), &tr("prt.file_info"));
    chk(
        ui,
        "prt_info",
        o,
        21.0,
        123.0,
        &mut st.page_info,
        &tr("prt.print_file_info"),
        true,
    );
    if st.info_text.is_empty() {
        st.info_text = app.document_name();
    }
    field(
        ui,
        "prt_info_text",
        o,
        rect(21.0, 138.0, 265.0, 161.0),
        &mut st.info_text,
        st.page_info,
    );
    chk(
        ui,
        "prt_numbers",
        o,
        21.0,
        178.0,
        &mut st.page_numbers,
        &tr("prt.page_numbers"),
        true,
    );
    chk(
        ui,
        "prt_inside",
        o,
        21.0,
        201.0,
        &mut st.ex.info_inside,
        &tr("prt.info_inside"),
        st.page_info || st.page_numbers,
    );

    group(
        ui,
        o,
        rect(320.0, 103.0, 590.0, 268.0),
        &tr("prt.calibration"),
    );
    chk(
        ui,
        "prt_bars",
        o,
        327.0,
        123.0,
        &mut st.color_bars,
        &tr("prt.color_bar"),
        true,
    );
    chk(
        ui,
        "prt_dens",
        o,
        327.0,
        145.0,
        &mut st.densitometer,
        &tr("prt.densitometer"),
        true,
    );
    lbl(ui, o, 327.0, 172.0, &tr("prt.densities"), st.densitometer);
    densities_list(ui, o, st);

    group(ui, o, rect(13.0, 275.0, 273.0, 363.0), &tr("prt.finish"));
    chk(
        ui,
        "prt_crop",
        o,
        21.0,
        302.0,
        &mut st.crop_marks,
        &tr("prt.crop_marks"),
        true,
    );
    chk(
        ui,
        "prt_ext",
        o,
        42.0,
        325.0,
        &mut st.ex.crop_exterior,
        &tr("prt.exterior"),
        st.crop_marks,
    );

    group(
        ui,
        o,
        rect(279.0, 282.0, 589.0, 363.0),
        &tr("prt.downsample"),
    );
    chk(
        ui,
        "prt_ds_color",
        o,
        287.0,
        306.0,
        &mut st.ex.downsample_color,
        &tr("prt.ds_color"),
        true,
    );
    if let Some(v) = spin(
        ui,
        "prt_ds_color_dpi",
        o,
        rect(464.0, 294.0, 547.0, 318.0),
        st.ex.downsample_color_dpi,
        0,
        "",
        1.0,
        36.0..=2400.0,
        st.ex.downsample_color,
    ) {
        st.ex.downsample_color_dpi = v;
    }
    lbl(ui, o, 571.0, 306.0, "dpi", true);
    chk(
        ui,
        "prt_ds_mono",
        o,
        287.0,
        338.0,
        &mut st.ex.downsample_mono,
        &tr("prt.ds_mono"),
        true,
    );
    if let Some(v) = spin(
        ui,
        "prt_ds_mono_dpi",
        o,
        rect(464.0, 326.0, 547.0, 350.0),
        st.ex.downsample_mono_dpi,
        0,
        "",
        1.0,
        72.0..=4800.0,
        st.ex.downsample_mono,
    ) {
        st.ex.downsample_mono_dpi = v;
    }
    lbl(ui, o, 571.0, 338.0, "dpi", true);
    chk(
        ui,
        "prt_obj_marks",
        o,
        14.0,
        378.0,
        &mut st.ex.object_marks,
        &tr("prt.object_marks"),
        st.crop_marks || st.registration_marks,
    );
}

/// The registration mark style: the mark drawn in a black box.
fn reg_style(ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    let r = rect(382.0, 61.0, 444.0, 79.0).translate(o.to_vec2());
    let on = st.registration_marks;
    let resp = ui.interact(
        r.expand2(Vec2::new(0.0, 2.0))
            .union(r.translate(Vec2::new(38.0, 0.0))),
        egui::Id::new("prt_reg_style"),
        Sense::click(),
    );
    let p = ui.painter();
    p.rect_filled(r, 0.0, Color32::BLACK);
    let c = r.center();
    let ink = if on {
        Color32::WHITE
    } else {
        Color32::from_gray(120)
    };
    p.circle_stroke(c, 6.0, Stroke::new(1.2, ink));
    p.hline((c.x - 10.0)..=(c.x + 10.0), c.y, Stroke::new(1.2, ink));
    p.vline(c.x, (c.y - 8.0)..=(c.y + 8.0), Stroke::new(1.2, ink));
    if st.reg_style_filled() {
        p.circle_filled(c, 3.0, ink);
    }
    let arrow = Pos2::new(r.right() + 16.0, c.y);
    p.add(egui::epaint::PathShape::convex_polygon(
        vec![
            arrow + Vec2::new(-3.5, -1.5),
            arrow + Vec2::new(3.5, -1.5),
            arrow + Vec2::new(0.0, 2.0),
        ],
        Tokens::text_dim(),
        Stroke::NONE,
    ));
    if on && resp.clicked() {
        st.ex.reg_style = (st.ex.reg_style + 1) % 2;
    }
}

impl PrintState {
    /// The second registration mark style has a solid centre.
    fn reg_style_filled(&self) -> bool {
        self.ex.reg_style == 1
    }
}

/// The densities list: every density, the ones on the scale picked.
fn densities_list(ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    let r = rect(408.0, 160.0, 510.0, 261.0).translate(o.to_vec2());
    let on = st.densitometer;
    let p = ui.painter().clone();
    p.rect(
        r,
        0.0,
        Tokens::field(),
        Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    let rows = ALL_DENSITIES.len();
    let first: usize = ui
        .ctx()
        .data(|d| d.get_temp(egui::Id::new("prt_dens_scroll")))
        .unwrap_or(0);
    let shown = 4usize;
    for k in 0..shown.min(rows - first) {
        let v = ALL_DENSITIES[first + k];
        let row = Rect::from_min_size(
            Pos2::new(r.left() + 1.0, r.top() + 1.0 + k as f32 * 24.5),
            Vec2::new(r.width() - 16.0, 24.0),
        );
        let resp = ui.interact(row, egui::Id::new(("prt_dens", v)), Sense::click());
        let picked = st.ex.densities.contains(&v);
        if picked && on {
            p.rect_filled(row, 0.0, Tokens::selected_row());
        }
        p.text(
            Pos2::new(row.left() + 5.0, row.center().y),
            Align2::LEFT_CENTER,
            v.to_string(),
            FontId::proportional(FONT),
            if on {
                Tokens::text()
            } else {
                Tokens::text_dim()
            },
        );
        if on && resp.clicked() {
            if picked {
                st.ex.densities.retain(|d| *d != v);
            } else {
                st.ex.densities.push(v);
                st.ex.densities.sort_unstable();
            }
        }
    }
    // The scroll bar with its arrows.
    let bar = Rect::from_min_max(
        Pos2::new(r.right() - 15.0, r.top() + 1.0),
        r.max - Vec2::splat(1.0),
    );
    p.rect_filled(bar, 0.0, Tokens::button());
    let span = bar.height() - 28.0;
    let thumb_h = span * shown as f32 / rows as f32;
    let thumb_y = bar.top() + 14.0 + span * first as f32 / rows as f32;
    p.rect_filled(
        Rect::from_min_size(
            Pos2::new(bar.left() + 3.0, thumb_y),
            Vec2::new(9.0, thumb_h),
        ),
        2.0,
        Tokens::control_border(),
    );
    let up = Rect::from_min_size(bar.min, Vec2::new(bar.width(), 14.0));
    let down = Rect::from_min_max(Pos2::new(bar.left(), bar.bottom() - 14.0), bar.max);
    arrow(&p, up.center(), true);
    arrow(&p, down.center(), false);
    let mut next = first;
    if ui
        .interact(up, egui::Id::new("prt_dens_up"), Sense::click())
        .clicked()
    {
        next = first.saturating_sub(1);
    }
    if ui
        .interact(down, egui::Id::new("prt_dens_down"), Sense::click())
        .clicked()
    {
        next = (first + 1).min(rows - shown);
    }
    let wheel = if ui.rect_contains_pointer(r) {
        ui.input(|i| i.smooth_scroll_delta.y)
    } else {
        0.0
    };
    if wheel < -1.0 {
        next = (first + 1).min(rows - shown);
    } else if wheel > 1.0 {
        next = first.saturating_sub(1);
    }
    ui.ctx()
        .data_mut(|d| d.insert_temp(egui::Id::new("prt_dens_scroll"), next));
}

fn tab_postscript(ui: &mut Ui, o: Pos2, st: &mut PrintState) {
    group(ui, o, rect(19.0, 25.0, 285.0, 127.0), &tr("prt.compat"));
    let levels = [2u8, 3];
    combo(
        ui,
        "prt_ps_level",
        o,
        rect(23.0, 48.0, 279.0, 72.0),
        trf("prt.ps_level", &[("n", &st.ps_level.to_string())]),
        true,
        |ui| {
            for l in levels {
                ui.selectable_value(
                    &mut st.ps_level,
                    l,
                    trf("prt.ps_level", &[("n", &l.to_string())]),
                );
            }
        },
    );
    chk(
        ui,
        "prt_dsc",
        o,
        23.0,
        100.0,
        &mut st.ex.dsc,
        &tr("prt.dsc"),
        true,
    );

    group(ui, o, rect(19.0, 148.0, 285.0, 392.0), &tr("prt.bitmaps"));
    lbl(ui, o, 25.0, 183.0, &tr("prt.compression"), true);
    let comps = [tr("prt.comp_none"), "LZW".to_string(), "JPEG".to_string()];
    combo(
        ui,
        "prt_comp",
        o,
        rect(193.0, 170.0, 279.0, 195.0),
        comps[st.ex.compression.min(2)].clone(),
        true,
        |ui| {
            for (i, t) in comps.iter().enumerate() {
                ui.selectable_value(&mut st.ex.compression, i, t.as_str());
            }
        },
    );
    let jpeg = st.ex.compression == 2;
    lbl(ui, o, 25.0, 211.0, &tr("prt.jpeg_quality"), jpeg);
    let slider = rect(28.0, 232.0, 240.0, 250.0).translate(o.to_vec2());
    ui.scope_builder(egui::UiBuilder::new().max_rect(slider), |ui| {
        ui.add_enabled_ui(jpeg, |ui| {
            ui.spacing_mut().slider_width = slider.width();
            ui.add(
                egui::Slider::new(&mut st.ex.jpeg_quality, 0.0..=10.0)
                    .step_by(1.0)
                    .show_value(false),
            );
        });
    });
    lbl(
        ui,
        o,
        262.0,
        241.0,
        &format!("{:.0}", st.ex.jpeg_quality),
        jpeg,
    );
    lbl(ui, o, 25.0, 288.0, &tr("prt.high"), jpeg);
    lbl(ui, o, 199.0, 288.0, &tr("prt.low"), jpeg);
    chk(
        ui,
        "prt_opi",
        o,
        28.0,
        327.0,
        &mut st.ex.opi,
        &tr("prt.opi"),
        true,
    );
    chk(
        ui,
        "prt_dcs",
        o,
        28.0,
        362.0,
        &mut st.ex.dcs,
        &tr("prt.dcs"),
        true,
    );

    group(ui, o, rect(310.0, 25.0, 590.0, 127.0), &tr("prt.fonts"));
    chk(
        ui,
        "prt_t1",
        o,
        320.0,
        56.0,
        &mut st.ex.type1,
        &tr("prt.type1"),
        true,
    );
    chk(
        ui,
        "prt_tt",
        o,
        343.0,
        91.0,
        &mut st.ex.tt_to_type1,
        &tr("prt.tt_type1"),
        st.ex.type1,
    );

    group(
        ui,
        o,
        rect(310.0, 148.0, 590.0, 285.0),
        &tr("prt.pdf_marks"),
    );
    lbl(ui, o, 316.0, 183.0, &tr("prt.pdf_start"), true);
    let starts = [
        tr("prt.start_page"),
        tr("prt.start_full"),
        tr("prt.start_thumbs"),
    ];
    combo(
        ui,
        "prt_pdf_start",
        o,
        rect(426.0, 170.0, 582.0, 195.0),
        starts[st.ex.pdf_start.min(2)].clone(),
        false,
        |ui| {
            for (i, t) in starts.iter().enumerate() {
                ui.selectable_value(&mut st.ex.pdf_start, i, t.as_str());
            }
        },
    );
    chk(
        ui,
        "prt_links",
        o,
        318.0,
        222.0,
        &mut st.ex.hyperlinks,
        &tr("prt.hyperlinks"),
        false,
    );
    chk(
        ui,
        "prt_marks",
        o,
        318.0,
        258.0,
        &mut st.ex.bookmarks,
        &tr("prt.bookmarks"),
        false,
    );
    chk(
        ui,
        "prt_flat",
        o,
        314.0,
        313.0,
        &mut st.ex.auto_flatness,
        &tr("prt.auto_flat"),
        true,
    );
    chk(
        ui,
        "prt_steps",
        o,
        314.0,
        348.0,
        &mut st.ex.auto_fountain_steps,
        &tr("prt.auto_steps"),
        true,
    );
    chk(
        ui,
        "prt_opt_fount",
        o,
        314.0,
        384.0,
        &mut st.ex.optimize_fountains,
        &tr("prt.opt_fountains"),
        true,
    );
}

fn tab_issues(
    app: &mut App,
    ui: &mut Ui,
    o: Pos2,
    st: &mut PrintState,
    issues: &[crate::preflight::Issue],
) {
    lbl(ui, o, 17.0, 30.0, &tr("prt.preflight_for"), true);
    let set = tr("prt.preflight_default");
    combo(
        ui,
        "prt_preflight_for",
        o,
        rect(166.0, 19.0, 565.0, 41.0),
        set.clone(),
        true,
        |ui| {
            let _ = ui.selectable_label(true, set.as_str());
        },
    );
    if gear(ui, "prt_preflight_settings", o + Vec2::new(587.0, 30.0)) {
        app.dialog = Dialog::Preflight(Default::default());
    }
    let list = rect(17.0, 49.0, 600.0, 155.0).translate(o.to_vec2());
    let p = ui.painter().clone();
    p.rect(
        list,
        0.0,
        Tokens::field(),
        Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    let sel_id = egui::Id::new("prt_issue_sel");
    let mut sel: Option<usize> = ui.ctx().data(|d| d.get_temp(sel_id)).unwrap_or(None);
    if sel.is_some_and(|s| s >= issues.len()) {
        sel = None;
    }
    if issues.is_empty() {
        doc_icon(&p, Pos2::new(list.left() + 13.0, list.top() + 10.0));
        p.text(
            Pos2::new(list.left() + 30.0, list.top() + 10.0),
            Align2::LEFT_CENTER,
            tr("prt.no_issues"),
            FontId::proportional(FONT),
            Tokens::text(),
        );
    }
    for (i, issue) in issues.iter().enumerate().take(5) {
        let row = Rect::from_min_size(
            Pos2::new(list.left() + 1.0, list.top() + 1.0 + i as f32 * 20.0),
            Vec2::new(list.width() - 2.0, 20.0),
        );
        let resp = ui.interact(row, egui::Id::new(("prt_issue", i)), Sense::click());
        if resp.clicked() {
            sel = Some(i);
        }
        if sel == Some(i) {
            p.rect_filled(row, 0.0, Tokens::selected_row());
        }
        crate::ui::preflight_dialog::paint_icon(
            &p,
            Pos2::new(row.left() + 12.0, row.center().y),
            issue.check.severity(),
        );
        let mut text = tr(&issue.check.key());
        if !issue.detail.is_empty() {
            text = format!("{text}: {}", issue.detail);
        }
        p.text(
            Pos2::new(row.left() + 28.0, row.center().y),
            Align2::LEFT_CENTER,
            text,
            FontId::proportional(FONT),
            Tokens::text(),
        );
    }
    ui.ctx().data_mut(|d| d.insert_temp(sel_id, sel));
    match sel.and_then(|s| issues.get(s)) {
        Some(issue) => {
            lbl(ui, o, 17.0, 171.0, &tr("prt.problem"), true);
            wrapped(
                ui,
                o,
                17.0,
                181.0,
                580.0,
                &tr(&format!("preflight.hint.{}", issue.check.id())),
                true,
            );
            let mut skip = false;
            chk(
                ui,
                "prt_skip",
                o,
                22.0,
                353.0,
                &mut skip,
                &tr("prt.dont_check"),
                true,
            );
            if skip {
                st.ex.skipped_checks.push(issue.check.id().to_string());
                st.preflight = None;
            }
        }
        None => {
            wrapped(ui, o, 17.0, 165.0, 580.0, &tr("prt.issues_desc1"), true);
            wrapped(ui, o, 17.0, 206.0, 580.0, &tr("prt.issues_desc2"), true);
            ui.painter().text(
                o + Vec2::new(17.0, 251.0),
                Align2::LEFT_CENTER,
                tr("prt.tip"),
                crate::theme::bold(FONT),
                Tokens::text(),
            );
            wrapped(ui, o, 17.0, 268.0, 580.0, &tr("prt.tip_text"), true);
            let mut skip = false;
            chk(
                ui,
                "prt_skip",
                o,
                22.0,
                353.0,
                &mut skip,
                &tr("prt.dont_check"),
                false,
            );
        }
    }
}

// ----- the bottom row and the side preview ------------------------------------------

fn button_row(
    app: &mut App,
    ui: &mut Ui,
    o: Pos2,
    st: &mut PrintState,
    kind: Kind,
    width: f32,
) -> Option<Outcome> {
    let mut out = None;
    let y0 = 481.0;
    let help = Rect::from_min_size(o + Vec2::new(12.0, y0), Vec2::new(24.0, 26.0));
    if chrome::button_at(ui, help, "?").clicked() {
        app.open_help(crate::help::dialog_topic(
            &Dialog::Print(st.clone()),
            app.options_page,
        ));
    }
    let right = width - 29.0;
    let enter =
        ui.input(|i| i.key_pressed(egui::Key::Enter)) && ui.ctx().memory(|m| m.focused().is_none());
    match kind {
        Kind::Print => {
            let pv = Rect::from_min_size(o + Vec2::new(42.0, y0), Vec2::new(180.0, 26.0));
            if ui
                .put(pv, egui::Button::new(underlined(&tr("prt.preview"), true)))
                .clicked()
            {
                out = Some(Outcome::Preview);
            }
            let pop = Rect::from_min_size(o + Vec2::new(229.0, y0 + 3.0), Vec2::new(20.0, 20.0));
            let resp = ui
                .interact(pop, egui::Id::new("prt_side_preview"), Sense::click())
                .on_hover_text(tr("prt.side_preview"));
            popout_icon(ui, pop, st.ex.side_preview || resp.hovered());
            if resp.clicked() {
                st.ex.side_preview = !st.ex.side_preview;
            }
            let print =
                Rect::from_min_size(o + Vec2::new(right - 256.0, y0), Vec2::new(88.0, 26.0));
            if primary(ui, print, &tr("prt.print")) || enter {
                out = Some(Outcome::Print);
            }
        }
        Kind::PreviewOptions => {
            let ok = Rect::from_min_size(o + Vec2::new(right - 256.0, y0), Vec2::new(88.0, 26.0));
            if chrome::button_at(ui, ok, &tr("prt.ok")).clicked() || enter {
                out = Some(Outcome::Ok);
            }
        }
    }
    let cancel = Rect::from_min_size(o + Vec2::new(right - 162.0, y0), Vec2::new(80.0, 26.0));
    if chrome::button_at(ui, cancel, &tr("prt.cancel")).clicked() {
        out = Some(Outcome::Cancel);
    }
    let apply = Rect::from_min_size(o + Vec2::new(right - 76.0, y0), Vec2::new(76.0, 26.0));
    if chrome::button_at(ui, apply, &tr("prt.apply")).clicked() {
        out = Some(Outcome::Apply);
    }
    out
}

/// The default button: blue, white text.
fn primary(ui: &mut Ui, r: Rect, text: &str) -> bool {
    let resp = ui.interact(r, egui::Id::new(("prt_primary", text)), Sense::click());
    let fill = if resp.hovered() {
        Color32::from_rgb(0x28, 0x5E, 0xA8)
    } else {
        tab_blue()
    };
    ui.painter().rect(
        r,
        2.0,
        fill,
        Stroke::new(1.0, Color32::from_rgb(0x4A, 0x7F, 0xC8)),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        r.center(),
        Align2::CENTER_CENTER,
        text,
        FontId::proportional(FONT),
        Color32::WHITE,
    );
    resp.clicked()
}

/// The side preview: the first sheet as it prints, beside the tabs.
fn side_preview(app: &mut App, ui: &mut Ui, o: Pos2, st: &PrintState) {
    let area = Rect::from_min_max(
        o + Vec2::new(SIZE.x - 6.0, 15.0),
        o + Vec2::new(SIZE.x + SIDE - 14.0, 461.0),
    );
    ui.painter().rect(
        area,
        0.0,
        Tokens::panel_dark(),
        Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    let ctx = ui.ctx().clone();
    if let Some(tex) =
        crate::ui::print_preview::thumbnail(app, &ctx, st, area.size() - Vec2::splat(24.0))
    {
        let size = tex.size_vec2();
        let r = Rect::from_center_size(area.center(), size);
        ui.painter().rect_filled(
            r.translate(Vec2::splat(3.0)),
            0.0,
            Color32::from_black_alpha(80),
        );
        ui.painter().image(
            tex.id(),
            r,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
    }
}

// ----- widgets at fixed places ----------------------------------------------------

/// A rect from panel coordinates.
fn rect(x0: f32, y0: f32, x1: f32, y1: f32) -> Rect {
    Rect::from_min_max(Pos2::new(x0, y0), Pos2::new(x1, y1))
}

/// `text` with the letter after `&` underlined (`&&` is an ampersand).
pub fn underlined(text: &str, enabled: bool) -> egui::text::LayoutJob {
    let color = if enabled {
        Tokens::text()
    } else {
        Tokens::text_dim()
    };
    let mut job = egui::text::LayoutJob::default();
    let font = FontId::proportional(FONT);
    let plain = |s: &str, job: &mut egui::text::LayoutJob| {
        if !s.is_empty() {
            job.append(
                s,
                0.0,
                egui::TextFormat {
                    font_id: font.clone(),
                    color,
                    ..Default::default()
                },
            );
        }
    };
    let mut buf = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            match chars.next() {
                Some('&') => buf.push('&'),
                Some(u) => {
                    plain(&buf, &mut job);
                    buf.clear();
                    job.append(
                        &u.to_string(),
                        0.0,
                        egui::TextFormat {
                            font_id: font.clone(),
                            color,
                            underline: Stroke::new(1.0, color),
                            ..Default::default()
                        },
                    );
                }
                None => {}
            }
        } else {
            buf.push(c);
        }
    }
    plain(&buf, &mut job);
    job
}

/// A label left-aligned from `(x, y)` (its vertical centre), dimmed when
/// off.
fn lbl(ui: &Ui, o: Pos2, x: f32, y: f32, text: &str, enabled: bool) {
    let galley = ui.painter().layout_job(underlined(text, enabled));
    let h = galley.size().y;
    ui.painter()
        .galley(o + Vec2::new(x, y - h / 2.0), galley, Tokens::text());
}

/// Text wrapped to `width` from its top left `(x, y)`.
fn wrapped(ui: &Ui, o: Pos2, x: f32, y: f32, width: f32, text: &str, enabled: bool) {
    let mut job = underlined(text, enabled);
    job.wrap.max_width = width;
    let galley = ui.painter().layout_job(job);
    ui.painter()
        .galley(o + Vec2::new(x, y), galley, Tokens::text());
}

/// A group box: a thin frame with its title in a gap of the top edge.
fn group(ui: &Ui, o: Pos2, r: Rect, title: &str) {
    let r = r.translate(o.to_vec2());
    let galley = ui.painter().layout_no_wrap(
        title.to_string(),
        FontId::proportional(FONT),
        Tokens::text(),
    );
    let stroke = Stroke::new(1.0, Tokens::control_border());
    let p = ui.painter();
    let (t0, t1) = (r.left() + 6.0, r.left() + 12.0 + galley.size().x);
    p.hline(r.left()..=t0, r.top(), stroke);
    p.hline(t1..=r.right(), r.top(), stroke);
    p.hline(r.left()..=r.right(), r.bottom(), stroke);
    p.vline(r.left(), r.top()..=r.bottom(), stroke);
    p.vline(r.right(), r.top()..=r.bottom(), stroke);
    p.galley(
        Pos2::new(t0 + 3.0, r.top() - galley.size().y / 2.0),
        galley,
        Tokens::text(),
    );
}

/// A line across the panel at `y`, from `x0` to `x1`.
fn hline(ui: &Ui, o: Pos2, x0: f32, x1: f32, y: f32) {
    ui.painter().hline(
        (o.x + x0)..=(o.x + x1),
        o.y + y,
        Stroke::new(1.0, Tokens::control_border()),
    );
}

/// A check box at `(x, y)` (the box's left edge and centre line) with its
/// label after it; returns true when it changed.
#[allow(clippy::too_many_arguments)]
fn chk(
    ui: &mut Ui,
    id: &str,
    o: Pos2,
    x: f32,
    y: f32,
    value: &mut bool,
    text: &str,
    enabled: bool,
) -> bool {
    let galley = ui.painter().layout_job(underlined(text, enabled));
    let w = 18.0 + galley.size().x;
    let hit = Rect::from_min_size(o + Vec2::new(x - 2.0, y - 9.0), Vec2::new(w + 4.0, 18.0));
    let resp = ui.interact(hit, egui::Id::new(id), Sense::click());
    let b = Rect::from_center_size(o + Vec2::new(x + 6.5, y), Vec2::splat(13.0));
    let border = if enabled && resp.hovered() {
        chrome::FIELD_FOCUS
    } else {
        Tokens::control_border()
    };
    let p = ui.painter();
    p.rect(
        b,
        2.0,
        if *value && enabled {
            tab_blue()
        } else {
            Tokens::field()
        },
        Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );
    if *value {
        let ink = if enabled {
            Color32::WHITE
        } else {
            Tokens::text_dim()
        };
        let c = b.center();
        p.line_segment(
            [c + Vec2::new(-3.5, 0.0), c + Vec2::new(-1.0, 2.8)],
            Stroke::new(1.6, ink),
        );
        p.line_segment(
            [c + Vec2::new(-1.0, 2.8), c + Vec2::new(3.8, -2.8)],
            Stroke::new(1.6, ink),
        );
    }
    let h = galley.size().y;
    p.galley(o + Vec2::new(x + 18.0, y - h / 2.0), galley, Tokens::text());
    if enabled && resp.clicked() {
        *value = !*value;
        return true;
    }
    false
}

/// A radio button at `(x, y)`; returns true when clicked.
#[allow(clippy::too_many_arguments)]
fn radio(
    ui: &mut Ui,
    id: &str,
    o: Pos2,
    x: f32,
    y: f32,
    selected: bool,
    text: &str,
    enabled: bool,
) -> bool {
    let galley = ui.painter().layout_job(underlined(text, enabled));
    let w = 18.0 + galley.size().x;
    let hit = Rect::from_min_size(o + Vec2::new(x - 2.0, y - 9.0), Vec2::new(w + 4.0, 18.0));
    let resp = ui.interact(hit, egui::Id::new(id), Sense::click());
    let c = o + Vec2::new(x + 6.5, y);
    let p = ui.painter();
    let border = if enabled && resp.hovered() {
        chrome::FIELD_FOCUS
    } else {
        Tokens::control_border()
    };
    p.circle(c, 6.5, Tokens::field(), Stroke::new(1.0, border));
    if selected {
        p.circle_filled(
            c,
            3.2,
            if enabled {
                Tokens::text()
            } else {
                Tokens::text_dim()
            },
        );
    }
    let h = galley.size().y;
    p.galley(o + Vec2::new(x + 18.0, y - h / 2.0), galley, Tokens::text());
    enabled && resp.clicked()
}

/// A list at `r` (panel coordinates), greyed when off.
#[allow(clippy::too_many_arguments)]
fn combo(
    ui: &mut Ui,
    id: &str,
    o: Pos2,
    r: Rect,
    selected: String,
    enabled: bool,
    add: impl FnOnce(&mut Ui),
) {
    let r = r.translate(o.to_vec2());
    ui.add_enabled_ui(enabled, |ui| {
        chrome::combo(ui, id, r, selected, |ui| add(ui));
    });
}

/// A spin box at `r`; returns the new value.
#[allow(clippy::too_many_arguments)]
fn spin(
    ui: &mut Ui,
    id: &str,
    o: Pos2,
    r: Rect,
    value: f64,
    decimals: usize,
    suffix: &str,
    step: f64,
    range: std::ops::RangeInclusive<f64>,
    enabled: bool,
) -> Option<f64> {
    let r = r.translate(o.to_vec2());
    ui.add_enabled_ui(enabled, |ui| {
        chrome::spin_field(ui, id, r, value, decimals, suffix, step, range)
    })
    .inner
}

/// A one-line text field at `r`.
fn field(ui: &mut Ui, id: &str, o: Pos2, r: Rect, text: &mut String, enabled: bool) {
    let r = r.translate(o.to_vec2());
    ui.push_id(id, |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            chrome::text_field(ui, r, text);
        });
    });
}

/// A value shown in a small greyed field (the Layout tab's figures).
fn small_value(ui: &Ui, o: Pos2, r: Rect, text: &str, enabled: bool) {
    let r = r.translate(o.to_vec2());
    let p = ui.painter();
    p.rect(
        r,
        0.0,
        Tokens::field(),
        Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    p.text(
        Pos2::new(r.left() + 4.0, r.center().y),
        Align2::LEFT_CENTER,
        text,
        FontId::proportional(FONT - 1.0),
        if enabled {
            Tokens::text()
        } else {
            Tokens::text_dim()
        },
    );
    arrow(p, Pos2::new(r.right() - 14.0, r.center().y), false);
    arrow(p, Pos2::new(r.right() - 5.0, r.center().y), true);
}

/// A push button at `r`; returns true when clicked.
fn button(ui: &mut Ui, id: &str, o: Pos2, r: Rect, text: &str, enabled: bool) -> bool {
    let r = r.translate(o.to_vec2());
    ui.push_id(id, |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            ui.put(r, egui::Button::new(underlined(text, enabled)))
                .clicked()
        })
        .inner
    })
    .inner
}

/// A gear button centred on `c`; returns true when clicked.
fn gear(ui: &mut Ui, id: &str, c: Pos2) -> bool {
    let r = Rect::from_center_size(c, Vec2::splat(20.0));
    let resp = ui.interact(r, egui::Id::new(id), Sense::click());
    let p = ui.painter();
    if resp.hovered() {
        p.rect_filled(r, 2.0, Tokens::tool_hover());
    }
    let ink = Tokens::text();
    for k in 0..8 {
        let a = k as f32 * std::f32::consts::TAU / 8.0;
        let d = Vec2::new(a.cos(), a.sin());
        let n = Vec2::new(-d.y, d.x);
        p.add(egui::epaint::PathShape::convex_polygon(
            vec![
                c + d * 5.0 - n * 2.2,
                c + d * 8.5 - n * 1.6,
                c + d * 8.5 + n * 1.6,
                c + d * 5.0 + n * 2.2,
            ],
            ink,
            Stroke::NONE,
        ));
    }
    p.circle_filled(c, 6.2, ink);
    p.circle_filled(c, 2.6, Tokens::panel());
    resp.clicked()
}

/// A small triangle pointing up or down.
fn arrow(p: &egui::Painter, c: Pos2, up: bool) {
    let pts = if up {
        vec![
            c + Vec2::new(-3.0, 1.5),
            c + Vec2::new(3.0, 1.5),
            c + Vec2::new(0.0, -1.5),
        ]
    } else {
        vec![
            c + Vec2::new(-3.0, -1.5),
            c + Vec2::new(3.0, -1.5),
            c + Vec2::new(0.0, 1.5),
        ]
    };
    p.add(egui::epaint::PathShape::convex_polygon(
        pts,
        Tokens::text_dim(),
        Stroke::NONE,
    ));
}

/// The blue round "i" before the Color tab's note.
fn info_icon(ui: &Ui, c: Pos2) {
    let p = ui.painter();
    p.circle_filled(c, 9.0, Color32::from_rgb(0x1E, 0x88, 0xE5));
    p.text(
        c,
        Align2::CENTER_CENTER,
        "i",
        crate::theme::bold(12.0),
        Color32::WHITE,
    );
}

/// A small document picture before "no issues".
fn doc_icon(p: &egui::Painter, c: Pos2) {
    let r = Rect::from_center_size(c, Vec2::new(9.0, 11.0));
    p.rect_stroke(
        r,
        0.0,
        Stroke::new(1.0, Tokens::text_dim()),
        egui::StrokeKind::Inside,
    );
    p.hline(
        (r.left() + 2.0)..=(r.right() - 2.0),
        r.top() + 4.0,
        Stroke::new(1.0, Tokens::text_dim()),
    );
    p.hline(
        (r.left() + 2.0)..=(r.right() - 2.0),
        r.top() + 7.0,
        Stroke::new(1.0, Tokens::text_dim()),
    );
}

/// The Layout tab's picture: a page with the art over its centre, a
/// lock on it.
fn position_picture(ui: &Ui, r: Rect) {
    let p = ui.painter();
    p.rect_filled(r, 0.0, Color32::WHITE);
    dashed_rect(p, r.shrink(2.0), Color32::BLACK);
    // The folded corner.
    let c = r.right_top();
    p.add(egui::epaint::PathShape::convex_polygon(
        vec![c + Vec2::new(-10.0, 0.0), c, c + Vec2::new(0.0, 10.0)],
        Color32::from_gray(80),
        Stroke::NONE,
    ));
    p.circle_filled(
        r.center() + Vec2::new(8.0, 6.0),
        18.0,
        Color32::from_gray(205),
    );
    let art = Rect::from_center_size(r.center(), Vec2::new(16.0, 16.0));
    p.rect_stroke(
        art,
        0.0,
        Stroke::new(1.0, Color32::BLACK),
        egui::StrokeKind::Inside,
    );
    lock_shape(p, art.center() + Vec2::new(3.0, 3.0), Color32::BLACK);
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

fn lock_icon(ui: &Ui, c: Pos2) {
    lock_shape(ui.painter(), c, Tokens::text());
}

fn lock_shape(p: &egui::Painter, c: Pos2, color: Color32) {
    let body = Rect::from_center_size(c + Vec2::new(0.0, 2.0), Vec2::new(9.0, 7.0));
    p.rect_filled(body, 1.0, color);
    p.circle_stroke(c + Vec2::new(0.0, -2.0), 3.0, Stroke::new(1.4, color));
}

/// The paper/film picture: a page with an E, mirrored with Mirror on.
fn film_icon(ui: &Ui, c: Pos2, mirrored: bool) {
    let p = ui.painter();
    let r = Rect::from_center_size(c, Vec2::new(18.0, 24.0));
    p.rect_stroke(
        r,
        1.0,
        Stroke::new(1.2, Tokens::text()),
        egui::StrokeKind::Inside,
    );
    let fold = r.right_top();
    p.line_segment(
        [fold + Vec2::new(-6.0, 0.0), fold + Vec2::new(0.0, 6.0)],
        Stroke::new(1.2, Tokens::text()),
    );
    let e = if mirrored { "\u{018E}" } else { "E" };
    p.text(
        c + Vec2::new(0.0, 2.0),
        Align2::CENTER_CENTER,
        e,
        crate::theme::bold(13.0),
        Tokens::text(),
    );
}

/// The button that opens the side preview: a square with an arrow out.
fn popout_icon(ui: &Ui, r: Rect, lit: bool) {
    let p = ui.painter();
    if lit {
        p.rect_filled(r, 2.0, Tokens::tool_hover());
    }
    let ink = Stroke::new(1.3, Tokens::text());
    let b = r.shrink(4.0);
    p.line_segment([b.left_top() + Vec2::new(0.0, 0.0), b.left_bottom()], ink);
    p.line_segment([b.left_bottom(), b.right_bottom()], ink);
    p.line_segment(
        [b.right_bottom(), b.right_bottom() + Vec2::new(0.0, -6.0)],
        ink,
    );
    p.line_segment([b.left_top(), b.left_top() + Vec2::new(6.0, 0.0)], ink);
    let tip = b.right_top();
    p.line_segment([tip + Vec2::new(-6.0, 6.0), tip], ink);
    p.line_segment([tip, tip + Vec2::new(-5.0, 0.0)], ink);
    p.line_segment([tip, tip + Vec2::new(0.0, 5.0)], ink);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ampersands_underline_the_next_letter() {
        let job = underlined("U&sar PPD", true);
        let text: String = job.text.clone();
        assert_eq!(text, "Usar PPD");
        let underlined: Vec<&str> = job
            .sections
            .iter()
            .filter(|s| s.format.underline.width > 0.0)
            .map(|s| &job.text[s.byte_range.start.0..s.byte_range.end.0])
            .collect();
        assert_eq!(underlined, vec!["s"]);
        assert_eq!(super::underlined("A && B", true).text, "A & B");
    }

    #[test]
    fn the_issues_tab_counts_the_issues() {
        crate::i18n::set_language("en");
        assert_eq!(issues_tab_name(0), "No Issues");
        assert_eq!(issues_tab_name(1), "1 Issue");
        assert_eq!(issues_tab_name(3), "3 Issues");
    }

    /// Every tab draws, in both dialogs, with nothing picked and with
    /// separations on.
    #[test]
    fn every_tab_draws() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        for kind in [Kind::Print, Kind::PreviewOptions] {
            for tab in 0..7 {
                for seps in [false, true] {
                    let mut st = PrintState {
                        tab,
                        separations: seps,
                        ..PrintState::default()
                    };
                    st.ex.side_preview = seps;
                    let input = egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(
                            Pos2::ZERO,
                            Vec2::new(1400.0, 900.0),
                        )),
                        ..Default::default()
                    };
                    let mut close = false;
                    let mut out = ctx.run_ui(input, |ui| {
                        show(&mut app, ui.ctx(), &mut st, kind, &mut close);
                    });
                    out.textures_delta.clear();
                    assert!(!close, "tab {tab}");
                }
            }
        }
    }

    #[test]
    fn plates_list_the_inks_in_use_unless_empty_ones_are_asked_for() {
        let doc =
            tracedraw_core::Document::new("t", tracedraw_core::geometry::Size::new(10.0, 10.0));
        let used = [true, false, false, true];
        let rows = plate_rows(&doc, &used, false);
        assert_eq!(rows.len(), 2);
        assert_eq!(plate_rows(&doc, &used, true).len(), 4);
    }
}
