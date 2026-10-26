//! Tools > Options (the application options and the Customization, Tools,
//! Global and Workspaces dialogs) and Layout > Document Options: a white
//! page list on the left, the page on
//! the right, the help button at the bottom left, OK and Cancel at the
//! bottom right. Cancel puts back what the dialog changed.

use crate::app::{App, Units};
use crate::i18n::{tr, trf};
use crate::settings::{AutoCenter, NodeShape, NodeSize, Startup};
use crate::theme::Tokens;
use crate::ui::chrome;
use crate::ui::options_pages::{
    heading, note, off_button, off_check, off_combo, off_radio, sep, slider, tabbed,
};
use egui::{Context, Ui};
use tracedraw_core::{Color, Command, Fill};

/// Which category of Tools > Options a page belongs to: the six buttons
/// at the top right of the dialog, in their order there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsSet {
    Document,
    App,
    Customization,
    Tools,
    Global,
    Workspaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptionsPage {
    // CD Draw.
    #[default]
    General,
    TabletMode,
    Display,
    Edit,
    NodesAndHandles,
    ClipFrame,
    BitmapTracing,
    Snapping,
    Save,
    Text,
    PenSettings,
    Warnings,
    Macros,
    Plugins,
    // Customization.
    Appearance,
    CommandBars,
    Commands,
    ColorPalette,
    // Tools.
    Pick,
    Eraser,
    ZoomPan,
    Curves,
    SmartDrawing,
    Rectangle,
    Ellipse,
    GraphPaper,
    Spiral,
    Table,
    Dimension,
    AngularDimension,
    Connector,
    MeshFill,
    // Global.
    GlobalGeneral,
    Printing,
    FileLocations,
    FileFormats,
    GlobalColor,
    // Workspaces.
    Workspace,
    // Document (also Layout > Document Options).
    DocumentGeneral,
    PageSize,
    Layout,
    Background,
    Guidelines,
    Grid,
    Rulers,
    DocumentSave,
}

impl OptionsSet {
    /// The six categories in the order of their buttons.
    pub const ALL: [OptionsSet; 6] = [
        OptionsSet::Document,
        OptionsSet::App,
        OptionsSet::Customization,
        OptionsSet::Tools,
        OptionsSet::Global,
        OptionsSet::Workspaces,
    ];

    pub fn pages(self) -> &'static [OptionsPage] {
        use OptionsPage::*;
        match self {
            OptionsSet::App => &[
                General,
                TabletMode,
                Display,
                Edit,
                NodesAndHandles,
                ClipFrame,
                BitmapTracing,
                Snapping,
                Save,
                Text,
                PenSettings,
                Warnings,
                Macros,
                Plugins,
            ],
            OptionsSet::Customization => &[Appearance, CommandBars, Commands, ColorPalette],
            OptionsSet::Tools => &[
                Pick,
                Eraser,
                ZoomPan,
                Curves,
                SmartDrawing,
                Rectangle,
                Ellipse,
                GraphPaper,
                Spiral,
                Table,
                Dimension,
                AngularDimension,
                Connector,
                MeshFill,
            ],
            OptionsSet::Global => &[
                GlobalGeneral,
                Printing,
                FileLocations,
                FileFormats,
                GlobalColor,
            ],
            OptionsSet::Workspaces => &[Workspace],
            OptionsSet::Document => &[
                DocumentGeneral,
                PageSize,
                Layout,
                Background,
                Guidelines,
                Grid,
                Rulers,
                DocumentSave,
            ],
        }
    }

    /// The category's name, at the top left of the dialog and as its
    /// button's tip.
    pub fn title(self) -> String {
        match self {
            OptionsSet::App => "CD Draw".into(),
            OptionsSet::Customization => tr("opt.set.customization"),
            OptionsSet::Tools => tr("opt.set.tools"),
            OptionsSet::Global => tr("opt.set.global"),
            OptionsSet::Workspaces => tr("opt.set.workspaces"),
            OptionsSet::Document => tr("opt.set.document"),
        }
    }
}

impl OptionsPage {
    pub fn set(self) -> OptionsSet {
        OptionsSet::ALL
            .into_iter()
            .find(|s| s.pages().contains(&self))
            .unwrap_or(OptionsSet::App)
    }

    /// The page's name in the list (its i18n key).
    pub fn key(self) -> &'static str {
        use OptionsPage::*;
        match self {
            General | GlobalGeneral | DocumentGeneral => "options.general",
            TabletMode => "opt.page.tablet_mode",
            Display => "options.display",
            Edit => "options.edit",
            NodesAndHandles => "opt.page.nodes",
            ClipFrame => "options.clip_frame",
            BitmapTracing => "opt.page.tracing",
            Snapping => "opt.page.snapping",
            Save | DocumentSave => "options.save",
            Text => "options.text",
            PenSettings => "opt.page.pen",
            Warnings => "options.warnings",
            Macros => "opt.page.macros",
            Plugins => "opt.page.plugins",
            Appearance => "options.appearance",
            CommandBars => "opt.page.command_bars",
            Commands => "options.commands",
            ColorPalette => "options.color_palette",
            Pick => "tool.pick",
            Eraser => "tool.eraser",
            ZoomPan => "opt.page.zoom_pan",
            Curves => "opt.page.curves",
            SmartDrawing => "opt.page.smart_drawing",
            Rectangle => "tool.rectangle",
            Ellipse => "tool.ellipse",
            GraphPaper => "tool.graph_paper",
            Spiral => "tool.spiral",
            Table => "tool.table",
            Dimension => "opt.page.dimension",
            AngularDimension => "tool.angular_dimension",
            Connector => "opt.page.connector",
            MeshFill => "tool.mesh_fill",
            Printing => "opt.page.printing",
            FileLocations => "opt.page.file_locations",
            FileFormats => "opt.page.file_formats",
            GlobalColor => "opt.page.color",
            Workspace => "options.workspace",
            PageSize => "options.page_size",
            Layout => "options.layout",
            Background => "options.background",
            Guidelines => "options.guidelines",
            Grid => "options.grid",
            Rulers => "options.rulers",
        }
    }

    /// The tool whose picture the Tools list shows before the page's name.
    pub fn tool(self) -> Option<crate::tools::Tool> {
        use crate::tools::Tool as T;
        use OptionsPage::*;
        Some(match self {
            Pick => T::Pick,
            Eraser => T::Eraser,
            ZoomPan => T::Zoom,
            Curves => T::Freehand,
            SmartDrawing => T::ShapeRecognition,
            Rectangle => T::Rectangle,
            Ellipse => T::Ellipse,
            GraphPaper => T::GraphPaper,
            Spiral => T::Spiral,
            Table => T::Table,
            Dimension => T::ParallelDimension,
            AngularDimension => T::AngularDimension,
            Connector => T::Connector,
            MeshFill => T::MeshFill,
            _ => return None,
        })
    }
}

/// What the options dialogs may change, kept when one opens so that
/// Cancel can put it back. Document changes are commands, undone back to
/// the depth the history had.
pub struct OptionsSnapshot {
    settings: crate::settings::Settings,
    language: String,
    units: Units,
    nudge_mm: f64,
    duplicate_offset: tracedraw_core::geometry::Vec2,
    show_rulers: bool,
    show_grid: bool,
    show_guides: bool,
    show_baseline_grid: bool,
    show_pixel_grid: bool,
    show_page_border: bool,
    show_status_bar: bool,
    show_standard_toolbar: bool,
    show_property_bar: bool,
    show_toolbox: bool,
    show_text_toolbar: bool,
    show_zoom_toolbar: bool,
    show_transform_toolbar: bool,
    show_dockers: bool,
    docker_tab: crate::app::DockerTab,
    toolbox_last: Vec<crate::tools::Tool>,
    proof_colors: bool,
    snap: crate::snap::SnapSettings,
    polygon_points: u32,
    star_sharpness: f64,
    rect_corners: tracedraw_core::Corners,
    corners_together: bool,
    ellipse_arc: Option<tracedraw_core::EllipseArc>,
    spiral_revolutions: u32,
    spiral_logarithmic: bool,
    graph_rows: u32,
    graph_cols: u32,
    table_rows: u32,
    table_cols: u32,
    eraser_width: f64,
    eraser_square: bool,
    mesh_rows: u32,
    mesh_cols: u32,
    dimension_style: crate::dimensions::DimensionStyle,
    text_font: String,
    text_size_pt: f64,
    text_hyphenation: bool,
    show_non_printing: bool,
    workspace: crate::app::Workspace,
    undo_depth: usize,
}

impl OptionsSnapshot {
    pub fn take(app: &App) -> Self {
        OptionsSnapshot {
            settings: app.settings.clone(),
            language: crate::i18n::language(),
            units: app.units,
            nudge_mm: app.nudge_mm,
            duplicate_offset: app.duplicate_offset,
            show_rulers: app.show_rulers,
            show_grid: app.show_grid,
            show_guides: app.show_guides,
            show_baseline_grid: app.show_baseline_grid,
            show_pixel_grid: app.show_pixel_grid,
            show_page_border: app.show_page_border,
            show_status_bar: app.show_status_bar,
            show_standard_toolbar: app.show_standard_toolbar,
            show_property_bar: app.show_property_bar,
            show_toolbox: app.show_toolbox,
            show_text_toolbar: app.show_text_toolbar,
            show_zoom_toolbar: app.show_zoom_toolbar,
            show_transform_toolbar: app.show_transform_toolbar,
            show_dockers: app.show_dockers,
            docker_tab: app.docker_tab,
            toolbox_last: app.toolbox_last.clone(),
            proof_colors: app.proof_colors,
            snap: app.snap,
            polygon_points: app.polygon_points,
            star_sharpness: app.star_sharpness,
            rect_corners: app.rect_corners,
            corners_together: app.corners_together,
            ellipse_arc: app.ellipse_arc,
            spiral_revolutions: app.spiral_revolutions,
            spiral_logarithmic: app.spiral_logarithmic,
            graph_rows: app.graph_rows,
            graph_cols: app.graph_cols,
            table_rows: app.table_rows,
            table_cols: app.table_cols,
            eraser_width: app.eraser_width,
            eraser_square: app.eraser_square,
            mesh_rows: app.mesh_rows,
            mesh_cols: app.mesh_cols,
            dimension_style: app.dimension_style.clone(),
            text_font: app.text_font.clone(),
            text_size_pt: app.text_size_pt,
            text_hyphenation: app.text_hyphenation,
            show_non_printing: app.show_non_printing,
            workspace: app.workspace,
            undo_depth: undo_depth(app),
        }
    }

    /// Put everything back (Cancel).
    pub fn restore(self, app: &mut App) {
        if app.has_document() {
            while undo_depth(app) > self.undo_depth {
                if app.engine.undo().is_err() {
                    break;
                }
            }
        }
        crate::i18n::set_language(&self.language);
        app.settings = self.settings;
        // The workspace first: choosing one sets the bars, which the
        // snapshot then puts back as they were.
        if app.workspace != self.workspace {
            app.set_workspace(self.workspace);
        }
        app.units = self.units;
        app.nudge_mm = self.nudge_mm;
        app.duplicate_offset = self.duplicate_offset;
        app.show_rulers = self.show_rulers;
        app.show_grid = self.show_grid;
        app.show_guides = self.show_guides;
        app.show_baseline_grid = self.show_baseline_grid;
        app.show_pixel_grid = self.show_pixel_grid;
        app.show_page_border = self.show_page_border;
        app.show_status_bar = self.show_status_bar;
        app.show_standard_toolbar = self.show_standard_toolbar;
        app.show_property_bar = self.show_property_bar;
        app.show_toolbox = self.show_toolbox;
        app.show_text_toolbar = self.show_text_toolbar;
        app.show_zoom_toolbar = self.show_zoom_toolbar;
        app.show_transform_toolbar = self.show_transform_toolbar;
        app.proof_colors = self.proof_colors;
        app.snap = self.snap;
        app.polygon_points = self.polygon_points;
        app.star_sharpness = self.star_sharpness;
        app.rect_corners = self.rect_corners;
        app.corners_together = self.corners_together;
        app.ellipse_arc = self.ellipse_arc;
        app.spiral_revolutions = self.spiral_revolutions;
        app.spiral_logarithmic = self.spiral_logarithmic;
        app.graph_rows = self.graph_rows;
        app.graph_cols = self.graph_cols;
        app.table_rows = self.table_rows;
        app.table_cols = self.table_cols;
        app.eraser_width = self.eraser_width;
        app.eraser_square = self.eraser_square;
        app.mesh_rows = self.mesh_rows;
        app.mesh_cols = self.mesh_cols;
        app.dimension_style = self.dimension_style;
        app.text_font = self.text_font;
        app.text_size_pt = self.text_size_pt;
        app.text_hyphenation = self.text_hyphenation;
        app.show_non_printing = self.show_non_printing;
        app.show_dockers = self.show_dockers;
        app.docker_tab = self.docker_tab;
        app.toolbox_last = self.toolbox_last;
        app.apply_runtime_settings();
    }
}

fn undo_depth(app: &App) -> usize {
    app.engine.history_labels().0.len()
}

/// The dialog's size at 100 % interface size (borders and title bar in).
pub const SIZE: egui::Vec2 = egui::vec2(802.0, 676.0);
/// The page list's width.
const PAGE_LIST_W: f32 = 180.0;
/// Rows of the page list.
const LIST_ROW: f32 = 25.0;

/// Draw Tools > Options: the title "Options", the category's name at the
/// top left and its six buttons at the top right, the page list on the
/// left (with Save as Default under it for the document, Import for the
/// workspaces), the page on the right, the help button at the bottom left,
/// OK and Cancel at the bottom right. Cancel puts back what it changed.
pub fn options_dialog(app: &mut App, ctx: &Context, close: &mut bool) {
    if app.options_snapshot.is_none() {
        app.options_snapshot = Some(OptionsSnapshot::take(app));
    }
    let set = app.options_page.set();
    let mut ok = false;
    let mut cancel = false;
    let x_closed = chrome::dialog(ctx, "options_dialog", &tr("opt.title"), SIZE, |ui| {
        let body = ui.max_rect();
        let left = body.left() + 1.0 + 10.0;
        let right = body.left() + SIZE.x - 10.0;
        let top = body.top();
        header(
            app,
            ui,
            set,
            egui::Rect::from_x_y_ranges(left..=right, top..=top + 36.0),
        );
        let bottom = body.top() + SIZE.y - chrome::TITLE_BAR - 44.0;
        let list = egui::Rect::from_x_y_ranges(left..=left + PAGE_LIST_W, top + 38.0..=bottom);
        let page = egui::Rect::from_x_y_ranges(list.right() + 7.0..=right, list.y_range());
        list_column(app, ui, list, set);
        ui.scope_builder(egui::UiBuilder::new().max_rect(page), |ui| {
            frame_box(ui, page);
            let inner = page.shrink2(egui::vec2(12.0, 10.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
                // Both ways, so a page wider than the dialog scrolls
                // instead of widening it.
                egui::ScrollArea::both()
                    .id_salt(("options_page", app.options_page.key(), set.title()))
                    .auto_shrink([false, false])
                    .max_height(inner.height())
                    .show(ui, |ui| {
                        ui.set_max_width(inner.width() - 14.0);
                        ui.spacing_mut().item_spacing.y = 6.0;
                        draw_page(app, ui);
                    });
            });
        });
        let row = egui::Rect::from_x_y_ranges(left..=right, bottom + 10.0..=bottom + 34.0);
        if ui
            .put(
                egui::Rect::from_min_size(row.min, egui::vec2(24.0, 24.0)),
                egui::Button::new("?"),
            )
            .on_hover_text(tr("dialog.help"))
            .clicked()
        {
            app.open_help(crate::help::options_topic(app.options_page));
        }
        let cancel_r = egui::Rect::from_min_size(
            egui::pos2(row.right() - 80.0, row.top()),
            egui::vec2(80.0, 24.0),
        );
        let ok_r = cancel_r.translate(egui::vec2(-86.0, 0.0));
        if accent_button(ui, ok_r, &tr("dialog.ok")).clicked() {
            ok = true;
        }
        if ui
            .put(cancel_r, egui::Button::new(tr("dialog.cancel")))
            .clicked()
        {
            cancel = true;
        }
    });
    if ok {
        app.options_snapshot = None;
        app.apply_runtime_settings();
        app.save_settings();
        *close = true;
    } else if cancel || x_closed {
        if let Some(s) = app.options_snapshot.take() {
            s.restore(app);
        }
        *close = true;
    }
}

/// A push button filled with the accent colour (the default button).
pub(crate) fn accent_button(ui: &mut Ui, rect: egui::Rect, text: &str) -> egui::Response {
    ui.put(
        rect,
        egui::Button::new(egui::RichText::new(text).color(egui::Color32::WHITE))
            .fill(Tokens::ACCENT),
    )
}

/// The frame of the page list and of the page.
fn frame_box(ui: &Ui, rect: egui::Rect) {
    ui.painter().rect(
        rect,
        0.0,
        Tokens::field(),
        egui::Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
}

/// The category's name at the left and its six buttons at the right.
fn header(app: &mut App, ui: &mut Ui, set: OptionsSet, rect: egui::Rect) {
    ui.painter().text(
        rect.left_center(),
        egui::Align2::LEFT_CENTER,
        set.title(),
        crate::theme::bold(13.0),
        Tokens::text(),
    );
    let side = 26.0;
    let gap = 6.0;
    for (i, s) in OptionsSet::ALL.iter().enumerate() {
        let x = rect.right() - (6 - i) as f32 * (side + gap) + gap;
        let r = egui::Rect::from_center_size(
            egui::pos2(x + side / 2.0, rect.center().y),
            egui::vec2(side, side),
        );
        let resp = ui
            .interact(r, egui::Id::new(("options_set", i)), egui::Sense::click())
            .on_hover_text(s.title());
        let on = *s == set;
        if on {
            ui.painter().rect_filled(r, 3.0, Tokens::ACCENT);
        } else if resp.hovered() {
            ui.painter().rect_filled(r, 3.0, Tokens::hovered_row());
        }
        let color = if on {
            egui::Color32::WHITE
        } else {
            Tokens::icon()
        };
        set_icon(ui.painter(), r.shrink(5.0), *s, color);
        if resp.clicked() && !on {
            app.options_page = s.pages()[0];
        }
    }
}

/// The pictures of the six categories: a page, the program's mark, three
/// sliders, the pointer, a bulb and a screen.
fn set_icon(p: &egui::Painter, r: egui::Rect, set: OptionsSet, color: egui::Color32) {
    let st = egui::Stroke::new(1.3, color);
    let c = r.center();
    let w = r.width();
    match set {
        OptionsSet::Document => {
            let page = egui::Rect::from_center_size(c, egui::vec2(w * 0.62, w * 0.86));
            p.rect_stroke(page, 1.0, st, egui::StrokeKind::Inside);
        }
        OptionsSet::App => {
            // A drop and a pen nib in a rounded square.
            p.rect_stroke(r.shrink(0.5), 3.0, st, egui::StrokeKind::Inside);
            p.circle_stroke(c + egui::vec2(-1.5, 1.5), w * 0.2, st);
            p.line_segment(
                [
                    c + egui::vec2(0.5, -0.5),
                    c + egui::vec2(w * 0.32, -w * 0.32),
                ],
                st,
            );
        }
        OptionsSet::Customization => {
            for (i, k) in [0.3f32, 0.7, 0.45].iter().enumerate() {
                let y = r.top() + w * (0.22 + i as f32 * 0.28);
                p.line_segment([egui::pos2(r.left(), y), egui::pos2(r.right(), y)], st);
                let x = r.left() + w * k;
                p.rect_filled(
                    egui::Rect::from_center_size(egui::pos2(x, y), egui::vec2(3.0, 5.0)),
                    0.5,
                    color,
                );
            }
        }
        OptionsSet::Tools => crate::ui::icons::draw(p, r, crate::tools::Tool::Pick, color),
        OptionsSet::Global => {
            let bulb = c + egui::vec2(0.0, -w * 0.12);
            p.circle_filled(bulb, w * 0.32, color);
            p.rect_filled(
                egui::Rect::from_center_size(
                    c + egui::vec2(0.0, w * 0.36),
                    egui::vec2(w * 0.3, w * 0.16),
                ),
                1.0,
                color,
            );
        }
        OptionsSet::Workspaces => {
            let screen = egui::Rect::from_min_max(
                r.min + egui::vec2(0.0, w * 0.08),
                egui::pos2(r.right(), r.top() + w * 0.72),
            );
            p.rect_stroke(screen, 1.0, st, egui::StrokeKind::Inside);
            p.line_segment(
                [
                    egui::pos2(c.x, screen.bottom()),
                    egui::pos2(c.x, r.bottom() - 1.0),
                ],
                st,
            );
            p.line_segment(
                [
                    egui::pos2(c.x - w * 0.25, r.bottom() - 1.0),
                    egui::pos2(c.x + w * 0.25, r.bottom() - 1.0),
                ],
                st,
            );
        }
    }
}

/// The column at the left: the pages of the category (the workspaces for
/// Workspaces), with Save as Default under the document's pages and
/// Import under the workspaces.
fn list_column(app: &mut App, ui: &mut Ui, rect: egui::Rect, set: OptionsSet) {
    let below = matches!(set, OptionsSet::Document | OptionsSet::Workspaces);
    let list = if below {
        egui::Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.bottom() - 30.0))
    } else {
        rect
    };
    frame_box(ui, list);
    if set == OptionsSet::Workspaces {
        crate::ui::options_pages::workspace_list(app, ui, list);
    } else {
        for (i, p) in set.pages().iter().enumerate() {
            let row = egui::Rect::from_min_size(
                list.min + egui::vec2(2.0, 3.0 + i as f32 * LIST_ROW),
                egui::vec2(list.width() - 4.0, LIST_ROW - 1.0),
            );
            let selected = app.options_page == *p;
            if list_row(
                ui,
                row,
                &tr(p.key()),
                p.tool(),
                selected,
                ("options_list", i),
            )
            .clicked()
            {
                app.options_page = *p;
            }
        }
    }
    if below {
        let b = egui::Rect::from_min_max(egui::pos2(rect.left(), rect.bottom() - 24.0), rect.max);
        if set == OptionsSet::Document {
            let enabled = app.has_document();
            if ui
                .put(
                    b,
                    egui::Button::new(tr("opt.save_as_default")).sense(if enabled {
                        egui::Sense::click()
                    } else {
                        egui::Sense::hover()
                    }),
                )
                .clicked()
            {
                app.settings.document_defaults = crate::settings::DocumentDefaults::of(app.doc());
                app.status = tr("status.settings_saved");
            }
        } else if ui.put(b, egui::Button::new(tr("opt.import"))).clicked() {
            let parts = app.settings.workspace_parts;
            app.import_workspace_dialog(parts, false);
        }
    }
}

/// One row of the list at the left: the grey bar when picked or under the
/// pointer, a tool's picture when it has one, the name.
pub(crate) fn list_row(
    ui: &mut Ui,
    row: egui::Rect,
    text: &str,
    tool: Option<crate::tools::Tool>,
    selected: bool,
    id: impl std::hash::Hash + std::fmt::Debug,
) -> egui::Response {
    let resp = ui.interact(row, egui::Id::new(id), egui::Sense::click());
    if selected {
        ui.painter().rect_filled(row, 0.0, Tokens::control_fill());
    } else if resp.hovered() {
        ui.painter().rect_filled(row, 0.0, Tokens::hovered_row());
    }
    let mut x = row.left() + 6.0;
    if let Some(t) = tool {
        let r = egui::Rect::from_center_size(
            egui::pos2(x + 8.0, row.center().y),
            egui::vec2(16.0, 16.0),
        );
        crate::ui::icons::draw(
            ui.painter(),
            r,
            t,
            crate::ui::icons::tool_color(t, Tokens::icon()),
        );
        x += 22.0;
    }
    ui.painter().text(
        egui::pos2(x, row.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        egui::FontId::proportional(12.5),
        Tokens::text(),
    );
    resp
}

fn draw_page(app: &mut App, ui: &mut Ui) {
    use crate::ui::options_pages as pg;
    use OptionsPage::*;
    // A command waits for its keys only while its page is shown.
    if !matches!(app.options_page, Commands) {
        app.shortcut_editor.capture = None;
    }
    match app.options_page {
        General => general_page(app, ui),
        TabletMode => pg::tablet_mode_page(app, ui),
        Display => display_page(app, ui),
        Edit => edit_page(app, ui),
        NodesAndHandles => nodes_page(app, ui),
        ClipFrame => clip_frame_page(app, ui),
        BitmapTracing => crate::ui::options_global::bitmap_tracing_page(app, ui),
        Snapping => snapping_page(app, ui),
        Save => save_page(app, ui),
        Text => text_page(app, ui),
        PenSettings => pg::pen_page(app, ui),
        Warnings => crate::ui::options_global::warnings_page(app, ui),
        Macros => pg::macros_page(app, ui),
        Plugins => pg::plugins_page(app, ui),
        Appearance => appearance_page(app, ui),
        Commands => commands_page(app, ui),
        CommandBars => command_bars_page(app, ui),
        ColorPalette => color_palette_page(app, ui),
        Pick => pick_page(app, ui),
        Eraser => eraser_page(app, ui),
        ZoomPan => zoom_pan_page(app, ui),
        Curves => curves_page(app, ui),
        SmartDrawing => pg::smart_drawing_page(app, ui),
        Rectangle => rectangle_page(app, ui),
        Ellipse => ellipse_page(app, ui),
        GraphPaper => graph_paper_page(app, ui),
        Spiral => spiral_page(app, ui),
        Table => table_page(app, ui),
        Dimension => pg::dimension_page(app, ui, false),
        AngularDimension => pg::dimension_page(app, ui, true),
        Connector => pg::connector_page(app, ui),
        MeshFill => pg::mesh_fill_page(app, ui),
        GlobalGeneral => global_general_page(app, ui),
        Printing => crate::ui::options_global::printing_page(app, ui),
        FileLocations => file_locations_page(app, ui),
        FileFormats => crate::ui::options_global::file_formats_page(app, ui),
        GlobalColor => pg::global_color_page(app, ui),
        Workspace => workspace_page(app, ui),
        DocumentGeneral => document_general_page(app, ui),
        PageSize => crate::ui::dialogs::page_size_page(app, ui),
        Layout => layout_page(app, ui),
        Background => background_page(app, ui),
        Guidelines => crate::ui::layout_options::guidelines_page(app, ui),
        Grid => crate::ui::layout_options::grid_page(app, ui),
        Rulers => crate::ui::layout_options::rulers_page(app, ui),
        DocumentSave => pg::document_save_page(app, ui),
    }
}

/// A label right-aligned in a grid's first column.
pub(crate) fn row_label(ui: &mut Ui, text: &str) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(text);
    });
}

pub(crate) fn grid(id: &str) -> egui::Grid {
    egui::Grid::new(id).num_columns(2).spacing([8.0, 8.0])
}

pub(crate) fn combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    id: &str,
    value: &mut T,
    options: &[T],
    label: impl Fn(T) -> String,
    width: f32,
) -> bool {
    let mut changed = false;
    egui::ComboBox::from_id_salt(id)
        .selected_text(label(*value))
        .width(width)
        .show_ui(ui, |ui| {
            for o in options {
                if ui.selectable_label(*value == *o, label(*o)).clicked() {
                    *value = *o;
                    changed = true;
                }
            }
        });
    changed
}

fn rgb_button(ui: &mut Ui, rgb: &mut [u8; 3]) -> bool {
    let resp = ui.color_edit_button_srgb(rgb);
    // A frame, so a white colour shows on the white page.
    ui.painter().rect_stroke(
        resp.rect,
        0.0,
        egui::Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    resp.changed()
}

// ----- CD Draw options -------------------------------------------------

fn general_page(app: &mut App, ui: &mut Ui) {
    heading(ui, &tr("opt.getting_started"));
    grid("startup").show(ui, |ui| {
        row_label(ui, &trf("opt.on_startup", &[("app", "CD Draw")]));
        let mut s = app.settings.startup;
        if combo(ui, "startup", &mut s, &Startup::ALL, |v| tr(v.key()), 150.0) {
            app.settings.startup = s;
            app.settings.show_welcome_on_start = s == Startup::WelcomeScreen;
        }
        ui.end_row();
    });
    ui.checkbox(
        &mut app.settings.show_new_document_dialog,
        tr("opt.show_new_document_dialog"),
    );
    ui.checkbox(&mut app.settings.show_splash, tr("options.show_splash"));
    sep(ui);
    heading(ui, &tr("opt.undo_levels"));
    grid("undo_grid").show(ui, |ui| {
        row_label(ui, &tr("opt.undo_regular"));
        ui.add_sized(
            [64.0, 22.0],
            egui::DragValue::new(&mut app.settings.undo_levels).range(1..=99_999),
        );
        ui.end_row();
    });
}

fn display_page(app: &mut App, ui: &mut Ui) {
    ui.checkbox(&mut app.proof_colors, tr("opt.proof_colors"));
    sep(ui);
    heading(ui, &tr("options.other_controls"));
    ui.columns(2, |c| {
        off_check(&mut c[0], false, &tr("opt.interruptible_refresh"));
        off_check(&mut c[0], false, &tr("opt.manual_refresh"));
        off_check(&mut c[0], true, &tr("opt.auto_panning"));
        c[0].checkbox(&mut app.settings.show_tooltips, tr("opt.show_tooltips"));
        off_check(&mut c[1], true, &tr("opt.antialiased_bitmaps"));
        c[1].checkbox(
            &mut app.settings.hide_bbox_curve_tools,
            tr("opt.hide_bbox_curve_tools"),
        );
        off_check(&mut c[1], false, &tr("opt.highlight_selected_outline"));
    });
    sep(ui);
    heading(ui, &tr("options.full_screen_preview"));
    off_radio(ui, false, &tr("opt.use_normal_view"));
    off_radio(ui, true, &tr("opt.use_enhanced_view"));
    ui.checkbox(
        &mut app.settings.preview_page_border,
        tr("options.show_page_border"),
    );
    sep(ui);
    heading(ui, &tr("opt.navigation"));
    grid("display_wheel").show(ui, |ui| {
        row_label(ui, &tr("opt.wheel_default_action"));
        let mut zoom = app.settings.wheel_zooms;
        combo(
            ui,
            "wheel",
            &mut zoom,
            &[true, false],
            |v| {
                tr(if v {
                    "opt.wheel_zoom"
                } else {
                    "opt.wheel_scroll"
                })
            },
            110.0,
        );
        app.settings.wheel_zooms = zoom;
        ui.end_row();
        for key in ["opt.zoom_rate", "opt.alternate_zoom_rate"] {
            row_label(ui, &tr(key));
            ui.horizontal(|ui| {
                off_combo(ui, key, "100%", 70.0);
                off_button(ui, &tr("options.reset"));
            });
            ui.end_row();
        }
    });
}

/// Two rulers drawn at the calibrated resolution, 100 mm and 4 inches
/// long, to hold a real ruler against.
fn calibration_rulers(ui: &mut Ui, ppi: f32) {
    let per_mm = ppi / 25.4;
    let pad = 8.0;
    // Room at the right end for the unit's name.
    let width = 100.0 * per_mm + pad + 30.0;
    for metric in [true, false] {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 36.0), egui::Sense::hover());
        let p = ui.painter();
        p.rect(
            rect,
            0.0,
            crate::theme::Tokens::field(),
            egui::Stroke::new(1.0, crate::theme::Tokens::control_border()),
            egui::StrokeKind::Inside,
        );
        let x0 = rect.left() + pad;
        let top = rect.top() + 1.0;
        let font = egui::FontId::proportional(10.0);
        let (count, step, label_every, unit) = if metric {
            (100, per_mm, 10, "mm")
        } else {
            // Eighths of an inch.
            (32, ppi / 8.0, 8, "in")
        };
        for i in 0..=count {
            let x = (x0 + i as f32 * step).round() + 0.5;
            let long = i % label_every == 0;
            let mid = if metric { i % 5 == 0 } else { i % 4 == 0 };
            let h = if long {
                14.0
            } else if mid {
                9.0
            } else {
                5.0
            };
            p.line_segment(
                [egui::pos2(x, top), egui::pos2(x, top + h)],
                egui::Stroke::new(1.0, Tokens::text()),
            );
            if long {
                let n = if metric { i } else { i / 8 };
                p.text(
                    egui::pos2(x, top + 16.0),
                    egui::Align2::CENTER_TOP,
                    n.to_string(),
                    font.clone(),
                    Tokens::text(),
                );
            }
        }
        p.text(
            egui::pos2(rect.right() - 4.0, rect.bottom() - 3.0),
            egui::Align2::RIGHT_BOTTOM,
            unit,
            font,
            Tokens::text_dim(),
        );
        ui.add_space(4.0);
    }
}

fn edit_page(app: &mut App, ui: &mut Ui) {
    grid("edit").show(ui, |ui| {
        row_label(ui, &tr("opt.constrain_angle"));
        ui.horizontal(|ui| {
            ui.add_sized(
                [64.0, 22.0],
                egui::DragValue::new(&mut app.settings.constrain_angle)
                    .range(0.1..=90.0)
                    .max_decimals(1)
                    .speed(0.5),
            );
            ui.label(tr("options.degrees"));
        });
        ui.end_row();
        row_label(ui, &tr("opt.drawing_precision"));
        ui.horizontal(|ui| {
            ui.add_sized(
                [64.0, 22.0],
                egui::DragValue::new(&mut app.settings.precision).range(0..=10),
            );
            ui.label(tr("options.decimal_places"));
        });
        ui.end_row();
    });
    off_check(ui, true, &tr("opt.double_click_edit_bitmaps"));
    sep(ui);
    heading(ui, &tr("opt.inactive_visibility"));
    ui.horizontal(|ui| {
        let mut v = app.settings.focus_visibility as f32;
        if slider(
            ui,
            egui::Slider::new(&mut v, 0.0..=100.0)
                .suffix(" %")
                .integer(),
        )
        .changed()
        {
            app.settings.focus_visibility = v.round() as u8;
        }
        if ui.button(tr("options.reset")).clicked() {
            app.settings.focus_visibility = 20;
        }
    });
    note(ui, &tr("options.focus_inactive_hint"));
    ui.add_space(4.0);
    ui.checkbox(
        &mut app.settings.focus_double_click,
        tr("opt.focus_double_click"),
    );
}

fn shape_label(s: NodeShape) -> String {
    match s {
        NodeShape::Square => "\u{25A1}".into(),
        NodeShape::Circle => "\u{25CB}".into(),
        NodeShape::Diamond => "\u{25C7}".into(),
    }
}

fn nodes_page(app: &mut App, ui: &mut Ui) {
    let n = &mut app.settings.nodes;
    grid("node_size").show(ui, |ui| {
        row_label(ui, &tr("opt.node_size"));
        combo(
            ui,
            "node_size",
            &mut n.size,
            &NodeSize::ALL,
            |v| tr(v.key()),
            90.0,
        );
        ui.end_row();
    });
    sep(ui);
    heading(ui, &tr("opt.node_shape"));
    grid("node_shapes").show(ui, |ui| {
        row_label(ui, &tr("opt.node_cusp"));
        combo(ui, "cusp", &mut n.cusp, &NodeShape::ALL, shape_label, 40.0);
        ui.end_row();
        row_label(ui, &tr("opt.node_smooth"));
        combo(
            ui,
            "smooth",
            &mut n.smooth,
            &NodeShape::ALL,
            shape_label,
            40.0,
        );
        ui.end_row();
        row_label(ui, &tr("opt.node_symmetrical"));
        combo(
            ui,
            "symmetrical",
            &mut n.symmetrical,
            &NodeShape::ALL,
            shape_label,
            40.0,
        );
        ui.end_row();
        ui.label("");
        if ui.button(tr("options.reset")).clicked() {
            let d = crate::settings::NodePrefs::default();
            n.cusp = d.cusp;
            n.smooth = d.smooth;
            n.symmetrical = d.symmetrical;
        }
        ui.end_row();
    });
    ui.checkbox(&mut n.show_direction, tr("opt.show_curve_direction"));
    ui.checkbox(&mut app.settings.node_tracking, tr("opt.node_tracking"));
    ui.horizontal(|ui| {
        ui.label(tr("options.node_pick_distance"));
        ui.add(
            egui::DragValue::new(&mut n.pick_distance)
                .range(2.0..=20.0)
                .speed(0.2)
                .max_decimals(0)
                .suffix(" px"),
        )
        .on_hover_text(tr("options.node_pick_distance_tip"));
    });
    sep(ui);
    heading(ui, &tr("options.colors"));
    // The scheme is Custom as soon as a colour is not the default one;
    // choosing Default puts both back.
    let d = crate::settings::NodePrefs::default();
    let mut custom = n.main_rgb != d.main_rgb || n.secondary_rgb != d.secondary_rgb;
    let custom_id = ui.id().with("node_scheme_custom");
    custom |= ui.data(|m| m.get_temp(custom_id)).unwrap_or(false);
    grid("node_colors").show(ui, |ui| {
        row_label(ui, &tr("opt.color_scheme"));
        let before = custom;
        combo(
            ui,
            "node_scheme",
            &mut custom,
            &[false, true],
            |v| {
                tr(if v {
                    "opt.scheme_custom"
                } else {
                    "opt.scheme_default"
                })
            },
            150.0,
        );
        if before && !custom {
            n.main_rgb = d.main_rgb;
            n.secondary_rgb = d.secondary_rgb;
        }
        ui.end_row();
        row_label(ui, &tr("opt.main_color"));
        ui.add_enabled_ui(custom, |ui| {
            ui.horizontal(|ui| {
                rgb_button(ui, &mut n.main_rgb);
                if ui.button(tr("options.reset")).clicked() {
                    n.main_rgb = d.main_rgb;
                }
            });
        });
        ui.end_row();
        row_label(ui, &tr("opt.secondary_color"));
        ui.add_enabled_ui(custom, |ui| {
            ui.horizontal(|ui| {
                rgb_button(ui, &mut n.secondary_rgb);
                if ui.button(tr("options.reset")).clicked() {
                    n.secondary_rgb = d.secondary_rgb;
                }
            });
        });
        ui.end_row();
    });
    ui.data_mut(|m| m.insert_temp(custom_id, custom));
    ui.checkbox(&mut n.unselected_filled, tr("opt.show_highlight"));
    off_check(ui, false, &tr("opt.node_types_colors"));
}

fn clip_frame_page(app: &mut App, ui: &mut Ui) {
    let p = &mut app.settings.clip_frame;
    ui.horizontal(|ui| {
        ui.label(tr("opt.clip_drag_into"));
        off_combo(ui, "clip_drag", &tr("opt.clip_frame_with_content"), 170.0);
    });
    ui.indent("clip_drag_choices", |ui| {
        ui.add_space(4.0);
        off_radio(ui, true, &tr("opt.clip_ignore_frame"));
        off_radio(ui, false, &tr("opt.clip_add_to_frame"));
        off_radio(ui, false, &tr("opt.clip_replace_contents"));
    });
    sep(ui);
    ui.label(tr("opt.auto_center"));
    for (v, key) in [
        (AutoCenter::WhenOutside, "opt.auto_center_outside"),
        (AutoCenter::Always, "opt.auto_center_always"),
        (AutoCenter::Never, "opt.auto_center_never"),
    ] {
        ui.radio_value(&mut p.auto_center, v, tr(key));
    }
    note(ui, &tr("opt.auto_center_note"));
    sep(ui);
    ui.checkbox(&mut p.empty_lines, tr("opt.empty_frame_lines"));
    off_radio(ui, true, &tr("opt.empty_lines_always"));
    off_radio(ui, false, &tr("opt.empty_lines_screen"));
    sep(ui);
    note(ui, &tr("opt.clip_note"));
}

fn snapping_page(app: &mut App, ui: &mut Ui) {
    ui.checkbox(&mut app.snap.objects, tr("opt.snap_to_objects"));
    off_check(ui, true, &tr("opt.snap_to_self"));
    ui.checkbox(&mut app.snap.page, tr("opt.snap_to_page"));
    ui.horizontal(|ui| {
        ui.label(tr("opt.snapping_radius"));
        ui.add_sized(
            [64.0, 22.0],
            egui::DragValue::new(&mut app.settings.snap.threshold_px).range(1.0..=100.0),
        );
        ui.label(tr("options.pixels"));
    });
    ui.checkbox(&mut app.settings.snap.show_marks, tr("opt.show_snap_marks"));
    ui.add_enabled(
        app.settings.snap.show_marks,
        egui::Checkbox::new(
            &mut app.settings.snap.screen_tips,
            tr("opt.snap_screen_tip"),
        ),
    );
    sep(ui);
    heading(ui, &tr("opt.snap_modes"));
    let modes = &mut app.settings.snap.modes;
    egui::Frame::new()
        .fill(Tokens::field())
        .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
        .inner_margin(egui::Margin::same(4))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            egui::ScrollArea::vertical()
                .id_salt("snap_modes_list")
                .max_height(150.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    for m in crate::snap_points::SnapMode::ALL {
                        ui.horizontal(|ui| {
                            let mut on = modes.get(m);
                            if ui.checkbox(&mut on, "").changed() {
                                modes.set(m, on);
                            }
                            let (r, _) = ui
                                .allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                            crate::canvas::draw_snap_glyph(
                                ui.painter(),
                                r.center(),
                                m,
                                Tokens::icon(),
                            );
                            ui.label(tr(m.key()));
                        });
                    }
                });
        });
    ui.horizontal(|ui| {
        if ui.button(tr("opt.select_all")).clicked() {
            *modes = crate::snap_points::SnapModes::default();
        }
        if ui.button(tr("opt.deselect_all")).clicked() {
            *modes = crate::snap_points::SnapModes::none();
        }
    });
}

/// A length typed in the document's units, kept in millimetres.
fn length_field(ui: &mut Ui, units: Units, mm: &mut f64, max_mm: f64) {
    let mut v = units.world_from_mm(*mm);
    let resp = ui.add_sized(
        [80.0, 22.0],
        egui::DragValue::new(&mut v)
            .range(0.0..=units.world_from_mm(max_mm))
            .max_decimals(3)
            .speed(0.1),
    );
    if resp.changed() && v.is_finite() {
        *mm = units.world_to_mm(v).clamp(0.0, max_mm);
    }
    ui.label(units.short());
}

/// The colour and line style of a kind of guide.
fn guide_look(
    ui: &mut Ui,
    id: &str,
    color: &mut [u8; 3],
    style: &mut crate::live_guides::GuideStyle,
) {
    grid(id).show(ui, |ui| {
        row_label(ui, &tr("options.guide_color"));
        rgb_button(ui, color);
        ui.end_row();
        row_label(ui, &tr("options.guide_style"));
        combo(
            ui,
            &format!("{id}_style"),
            style,
            &crate::live_guides::GuideStyle::ALL,
            |v| tr(v.key()),
            140.0,
        );
        ui.end_row();
    });
}

/// Options > Alignment Guides: what lines up with what while objects are
/// moved, drawn or resized.
pub(crate) fn alignment_guides_page(app: &mut App, ui: &mut Ui) {
    ui.checkbox(
        &mut app.snap.alignment_guides,
        tr("options.alignment_guides_on"),
    );
    let units = app.units;
    let on = app.snap.alignment_guides;
    let a = &mut app.settings.snap.align;
    ui.add_enabled_ui(on, |ui| {
        chrome::section(ui, &tr("options.align_moving"));
        ui.indent("align_moving", |ui| {
            ui.radio_value(
                &mut a.each_object,
                false,
                tr("options.align_selection_as_one"),
            );
            ui.radio_value(&mut a.each_object, true, tr("options.align_each_object"));
        });
        chrome::section(ui, &tr("options.align_what"));
        ui.indent("align_what", |ui| {
            ui.checkbox(&mut a.edges, tr("options.align_edges"));
            ui.checkbox(&mut a.centers, tr("options.align_centers"));
            ui.checkbox(&mut a.page, tr("options.align_page"));
            ui.checkbox(&mut a.equal_spacing, tr("options.align_equal_spacing"));
            ui.checkbox(&mut a.equal_size, tr("options.align_equal_size"));
        });
        chrome::section(ui, &tr("options.align_margins"));
        ui.indent("align_margins", |ui| {
            ui.checkbox(&mut a.margins, tr("options.align_margins_on"));
            ui.add_enabled_ui(a.margins, |ui| {
                grid("align_margin_values").show(ui, |ui| {
                    row_label(ui, &tr("options.align_offset"));
                    ui.horizontal(|ui| length_field(ui, units, &mut a.offset_mm, 1000.0));
                    ui.end_row();
                    row_label(ui, &tr("options.align_inset"));
                    ui.horizontal(|ui| length_field(ui, units, &mut a.inset_mm, 1000.0));
                    ui.end_row();
                });
            });
        });
        chrome::section(ui, &tr("options.guide_look"));
        guide_look(ui, "align_look", &mut a.color, &mut a.style);
        if ui.button(tr("options.reset")).clicked() {
            *a = crate::live_guides::AlignOptions::default();
        }
    });
}

/// The angles offered as check boxes on the Dynamic Guides page.
const DYNAMIC_PRESETS: [f64; 12] = [
    0.0, 15.0, 30.0, 45.0, 60.0, 75.0, 90.0, 105.0, 120.0, 135.0, 150.0, 165.0,
];

/// Options > Dynamic Guides: the angles of the guides from acquired snap
/// points, their tip and ticks.
pub(crate) fn dynamic_guides_page(app: &mut App, ui: &mut Ui) {
    ui.checkbox(
        &mut app.snap.dynamic_guides,
        tr("options.dynamic_guides_on"),
    );
    let units = app.units;
    let on = app.snap.dynamic_guides;
    let d = &mut app.settings.snap.dynamic;
    ui.add_enabled_ui(on, |ui| {
        chrome::section(ui, &tr("options.dynamic_angles"));
        ui.indent("dyn_angles", |ui| {
            let mut angles = d.clean_angles();
            egui::Grid::new("dyn_angle_grid")
                .num_columns(4)
                .show(ui, |ui| {
                    for (i, a) in DYNAMIC_PRESETS.iter().enumerate() {
                        let mut has = angles.iter().any(|b| (b - a).abs() < 1e-6);
                        if ui.checkbox(&mut has, format!("{a:.0}\u{00B0}")).changed() {
                            if has {
                                angles.push(*a);
                            } else {
                                angles.retain(|b| (b - a).abs() >= 1e-6);
                            }
                        }
                        if i % 4 == 3 {
                            ui.end_row();
                        }
                    }
                });
            // Other angles, each with a button that takes it away.
            let others: Vec<f64> = angles
                .iter()
                .copied()
                .filter(|a| !DYNAMIC_PRESETS.iter().any(|b| (b - a).abs() < 1e-6))
                .collect();
            if !others.is_empty() {
                ui.horizontal_wrapped(|ui| {
                    for a in others {
                        if ui
                            .button(format!("{a:.1}\u{00B0} \u{2715}"))
                            .on_hover_text(tr("options.dynamic_remove_angle"))
                            .clicked()
                        {
                            angles.retain(|b| (b - a).abs() >= 1e-6);
                        }
                    }
                });
            }
            ui.horizontal(|ui| {
                let id = egui::Id::new("dyn_new_angle");
                let mut v: f64 = ui.data_mut(|m| *m.get_temp_mut_or(id, 22.5));
                ui.add_sized(
                    [70.0, 22.0],
                    egui::DragValue::new(&mut v)
                        .range(0.0..=179.9)
                        .max_decimals(1),
                );
                ui.label(tr("options.degrees"));
                if ui.button(tr("options.dynamic_add_angle")).clicked() && v.is_finite() {
                    angles.push(v);
                }
                ui.data_mut(|m| m.insert_temp(id, v));
            });
            angles.sort_by(|a, b| a.total_cmp(b));
            if angles != d.angles {
                d.angles = angles;
            }
        });
        chrome::section(ui, &tr("options.dynamic_display"));
        ui.indent("dyn_display", |ui| {
            ui.checkbox(&mut d.tooltip, tr("options.dynamic_tooltip"));
            ui.checkbox(&mut d.ticks, tr("options.dynamic_ticks"));
            ui.add_enabled_ui(d.ticks, |ui| {
                grid("dyn_ticks").show(ui, |ui| {
                    row_label(ui, &tr("options.dynamic_tick_spacing"));
                    ui.horizontal(|ui| length_field(ui, units, &mut d.tick_mm, 10_000.0));
                    ui.end_row();
                });
                ui.checkbox(&mut d.snap_ticks, tr("options.dynamic_snap_ticks"));
            });
        });
        chrome::section(ui, &tr("options.guide_look"));
        guide_look(ui, "dyn_look", &mut d.color, &mut d.style);
        if ui.button(tr("options.reset")).clicked() {
            *d = crate::live_guides::DynamicOptions::default();
        }
    });
}

/// A read-only path with a Browse button that picks a folder (desktop).
fn path_field(ui: &mut Ui, dir: &mut Option<std::path::PathBuf>, fallback: String) {
    let text = dir
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or(fallback);
    ui.horizontal(|ui| {
        ui.add_enabled(
            false,
            egui::TextEdit::singleline(&mut text.clone()).desired_width(320.0),
        );
        if ui
            .add_enabled(true, egui::Button::new(tr("options.browse")))
            .clicked()
        {
            if let Some(p) = crate::files::pick_folder() {
                *dir = Some(p);
            }
        }
    });
}

fn save_page(app: &mut App, ui: &mut Ui) {
    let b = &mut app.settings.backup;
    heading(ui, &tr("opt.backup"));
    ui.checkbox(&mut b.before_save, tr("opt.backup_before_save"));
    ui.add_enabled_ui(b.before_save, |ui| {
        let mut choose = b.before_save_dir.is_some();
        ui.horizontal_top(|ui| {
            ui.label(tr("opt.backup_to"));
            ui.vertical(|ui| {
                ui.radio_value(&mut choose, false, tr("opt.same_folder"));
                ui.radio_value(&mut choose, true, tr("opt.choose_location"));
            });
        });
        if choose && b.before_save_dir.is_none() {
            b.before_save_dir = Some(std::env::temp_dir());
        } else if !choose {
            b.before_save_dir = None;
        }
        ui.add_enabled_ui(choose, |ui| {
            path_field(
                ui,
                &mut b.before_save_dir,
                std::env::temp_dir().display().to_string(),
            );
        });
    });
    sep(ui);
    heading(ui, &tr("opt.auto_backup"));
    ui.horizontal(|ui| {
        ui.checkbox(&mut b.auto, tr("opt.backup_every"));
        ui.add_enabled_ui(b.auto, |ui| {
            combo(
                ui,
                "backup_minutes",
                &mut b.minutes,
                &[1, 2, 5, 10, 15, 20, 30, 45, 60],
                |m| m.to_string(),
                60.0,
            );
        });
        ui.label(tr("options.minutes"));
    });
    ui.add_enabled_ui(b.auto, |ui| {
        let mut choose = b.auto_dir.is_some();
        ui.horizontal_top(|ui| {
            ui.label(tr("opt.backup_to"));
            ui.vertical(|ui| {
                ui.radio_value(&mut choose, false, tr("opt.user_temp_folder"));
                ui.radio_value(&mut choose, true, tr("opt.choose_location"));
            });
        });
        let fallback = crate::app::App::auto_backup_dir_default()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        if choose && b.auto_dir.is_none() {
            b.auto_dir = crate::app::App::auto_backup_dir_default();
        } else if !choose {
            b.auto_dir = None;
        }
        ui.add_enabled_ui(choose, |ui| path_field(ui, &mut b.auto_dir, fallback));
    });
    sep(ui);
    heading(ui, &tr("opt.save_older_versions"));
    off_check(ui, true, &tr("opt.ask_when_saving"));
    off_radio(ui, true, &tr("opt.keep_appearance"));
    off_radio(ui, false, &tr("opt.keep_editable"));
}

fn text_page(app: &mut App, ui: &mut Ui) {
    let labels = [
        tr("options.display"),
        tr("opt.paragraph_text"),
        tr("opt.fonts"),
        tr("opt.spelling"),
        tr("opt.quick_correct"),
    ];
    tabbed(ui, "text", &labels, |ui, tab| match tab {
        0 => {
            off_check(ui, true, &tr("opt.show_handles_editing"));
            off_check(ui, false, &tr("opt.auto_keyboard"));
            off_check(ui, false, &tr("opt.find_font_language"));
            ui.add_space(8.0);
            grid("text_display").show(ui, |ui| {
                row_label(ui, &tr("opt.keyboard_text_increment"));
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [64.0, 22.0],
                        egui::DragValue::new(&mut app.settings.text_increment_pt)
                            .range(0.1..=100.0)
                            .max_decimals(1),
                    );
                    ui.label(tr("options.points"));
                });
                ui.end_row();
                row_label(ui, &tr("opt.default_text_units"));
                off_combo(ui, "text_units", &tr("options.points"), 100.0);
                ui.end_row();
            });
        }
        1 => {
            ui.checkbox(
                &mut app.settings.show_text_frames,
                tr("menu.text.show_text_frames"),
            );
            ui.checkbox(&mut app.text_hyphenation, tr("menu.text.use_hyphenation"));
            ui.checkbox(
                &mut app.show_non_printing,
                tr("menu.text.show_non_printing"),
            );
        }
        2 => {
            grid("text_fonts").show(ui, |ui| {
                row_label(ui, &tr("options.default_font"));
                crate::ui::font_list::text_font_list(app, ui, "options_default_font", 220.0);
                ui.end_row();
                row_label(ui, &tr("options.default_size"));
                ui.add(
                    egui::DragValue::new(&mut app.text_size_pt)
                        .range(1.0..=999.0)
                        .suffix(" pt"),
                );
                ui.end_row();
            });
        }
        3 => {
            off_check(ui, true, &tr("opt.spell_as_you_type"));
            off_check(ui, false, &tr("opt.spell_uppercase"));
        }
        _ => {
            let a = &mut app.settings.autocorrect;
            ui.checkbox(&mut a.enabled, tr("autocorrect.enabled"));
            ui.add_enabled_ui(a.enabled, |ui| {
                ui.checkbox(&mut a.capitalize_sentences, tr("autocorrect.capitalize"));
                ui.checkbox(
                    &mut a.fix_two_initial_capitals,
                    tr("autocorrect.two_capitals"),
                );
                ui.checkbox(&mut a.typographic_quotes, tr("autocorrect.quotes"));
            });
        }
    });
}

// ----- Customization -----------------------------------------------------

/// The interface sizes the Appearance slider offers, in percent.
const UI_SIZE_RANGE: std::ops::RangeInclusive<f32> = 75.0..=250.0;

fn appearance_page(app: &mut App, ui: &mut Ui) {
    ui.checkbox(&mut app.settings.center_dialogs, tr("opt.center_dialogs"));
    heading(ui, &tr("opt.size"));
    // The size takes effect when the slider is let go or the field is
    // left, so the page does not grow under the pointer.
    let pending = ui.id().with("ui_size_pending");
    let current = crate::ui::interface_size(app.settings.ui_size) * 100.0;
    let mut pct: f32 = ui.data(|d| d.get_temp(pending)).unwrap_or(current);
    let mut reset = false;
    let busy = ui
        .horizontal(|ui| {
            let slider = ui
                .scope(|ui| {
                    ui.spacing_mut().slider_width = 140.0;
                    let w = &mut ui.visuals_mut().widgets;
                    w.inactive.bg_fill = crate::theme::Tokens::slider_rail();
                    w.inactive.fg_stroke = egui::Stroke::new(1.0, Tokens::control_border());
                    ui.visuals_mut().selection.bg_fill = Tokens::ACCENT;
                    ui.add(
                        egui::Slider::new(&mut pct, UI_SIZE_RANGE)
                            .step_by(5.0)
                            .show_value(false)
                            .trailing_fill(true),
                    )
                })
                .inner;
            let field = ui.add_sized(
                [52.0, 22.0],
                egui::DragValue::new(&mut pct)
                    .range(UI_SIZE_RANGE)
                    .speed(1.0)
                    .max_decimals(0)
                    .suffix(" %"),
            );
            reset = ui.button(tr("opt.reset_to_default")).clicked();
            slider.dragged() || field.dragged() || field.has_focus()
        })
        .inner;
    if reset {
        app.settings.ui_size = 0.0;
        ui.data_mut(|d| d.remove::<f32>(pending));
    } else if busy {
        ui.data_mut(|d| d.insert_temp(pending, pct));
    } else {
        ui.data_mut(|d| d.remove::<f32>(pending));
        if (pct - current).abs() > 0.5 {
            app.settings.ui_size = pct.round() / 100.0;
        }
    }
    note(ui, &tr("opt.ui_scale_hint"));
    let sys = crate::system_prefs::get();
    ui.checkbox(
        &mut app.settings.follow_text_size,
        crate::i18n::trf(
            "options.follow_text_size",
            &[("n", &format!("{:.0}", sys.text_scale * 100.0))],
        ),
    );
    sep(ui);
    heading(ui, &tr("opt.color"));
    grid("appearance_color").show(ui, |ui| {
        row_label(ui, &tr("options.theme"));
        let current = crate::theme::Theme::from_id(&app.settings.theme);
        egui::ComboBox::from_id_salt("ui_theme")
            .selected_text(tr(current.key()))
            .width(130.0)
            .show_ui(ui, |ui| {
                for t in crate::theme::Theme::ALL {
                    if ui.selectable_label(t == current, tr(t.key())).clicked() {
                        app.settings.theme = t.id().to_string();
                    }
                }
            });
        ui.end_row();
        row_label(ui, &tr("opt.window_border"));
        ui.add_enabled_ui(false, |ui| {
            let mut c = [0u8, 0, 0];
            rgb_button(ui, &mut c);
        });
        ui.end_row();
        row_label(ui, &tr("opt.desktop"));
        ui.horizontal(|ui| {
            rgb_button(ui, &mut app.settings.desktop_rgb);
            if ui.button(tr("options.reset")).clicked() {
                app.settings.desktop_rgb = [255, 255, 255];
            }
        });
        ui.end_row();
    });
    let on = sys.high_contrast.is_some();
    ui.checkbox(
        &mut app.settings.follow_high_contrast,
        tr(if on {
            "options.follow_high_contrast_on"
        } else {
            "options.follow_high_contrast"
        }),
    );
    ui.checkbox(&mut app.settings.ui_shadows, tr("options.ui_shadows"))
        .on_hover_text(tr("options.ui_shadows_tip"));
    ui.checkbox(&mut app.settings.colored_icons, tr("options.colored_icons"))
        .on_hover_text(tr("options.colored_icons_tip"));
}

fn color_palette_page(app: &mut App, ui: &mut Ui) {
    let p = &mut app.settings.palette;
    ui.checkbox(&mut p.auto_update_document, tr("opt.palette_auto_update"));
    ui.checkbox(&mut p.show_document, tr("options.palette_show_document"));
    heading(ui, &tr("opt.palette_options"));
    ui.horizontal(|ui| {
        ui.label(tr("opt.palette_max_rows"));
        ui.add_enabled_ui(false, |ui| {
            let mut rows = 1u32;
            ui.add_sized([50.0, 22.0], egui::DragValue::new(&mut rows));
        });
    });
    heading(ui, &tr("opt.color_swatches"));
    off_check(ui, false, &tr("opt.wide_borders"));
    off_check(ui, false, &tr("opt.large_swatches"));
    ui.checkbox(&mut p.show_no_color, tr("opt.palette_no_color"));
    heading(ui, &tr("opt.right_button"));
    ui.radio_value(
        &mut p.right_click_outline,
        false,
        tr("opt.palette_context_menu"),
    );
    ui.radio_value(
        &mut p.right_click_outline,
        true,
        tr("opt.palette_set_outline"),
    );
    ui.add_space(6.0);
    note(ui, &tr("opt.palette_hold_hint"));
}

/// Options > Customization > Commands: the menu commands found, the
/// search text, the command waiting for its keys and what the last
/// shortcut set took over.
#[derive(Debug, Clone, Default)]
pub struct ShortcutEditor {
    pub entries: Vec<crate::ui::menus::invoke::Entry>,
    pub filter: String,
    pub capture: Option<String>,
    pub note: Option<String>,
    /// The list shows one menu (its title key), the toolbox
    /// ([`TOOLBOX`]) or everything (`None`).
    pub category: Option<String>,
    /// The command picked in the list (a menu key or a tool id).
    pub selected: Option<String>,
    /// The tab beside the list: General (false) or Shortcut Keys (true).
    pub keys_tab: bool,
    /// Reset all shortcuts was pressed: the question is shown.
    pub confirm_reset_all: bool,
}

/// The Commands page's category for the toolbox's tools.
pub const TOOLBOX: &str = "toolbox";
/// The commands category of the loaded VBA macros.
pub const MACROS: &str = "macros";

/// The user's shortcut for `id` (a menu key or a tool id).
fn user_shortcut(app: &App, id: &str) -> Option<String> {
    app.settings
        .shortcuts
        .iter()
        .find(|(k, _)| k == id)
        .map(|(_, v)| v.clone())
}

/// Give the menu command `id` the shortcut `text`. Another command of the
/// user's with it loses it; commands and tools that have it built in are
/// named in the note, since the shortcut now runs this command.
pub(crate) fn set_menu_shortcut(app: &mut App, id: &str, text: &str) {
    use crate::shortcuts::same;
    let mut others: Vec<String> = Vec::new();
    for (k, v) in &app.settings.shortcuts {
        if k != id && same(v, text) {
            others.push(menu_or_tool_name(app, k));
        }
    }
    app.settings
        .shortcuts
        .retain(|(k, v)| k == id || !same(v, text));
    for e in &app.shortcut_editor.entries {
        if e.key != id && same(&e.shortcut, text) && user_shortcut(app, &e.key).is_none() {
            others.push(command_label(e));
        }
    }
    for t in crate::tools::Tool::ALL {
        if user_shortcut(app, t.id()).is_none() && t.shortcut().is_some_and(|s| same(s, text)) {
            others.push(t.name());
        }
    }
    app.settings.shortcuts.retain(|(k, _)| k != id);
    app.settings
        .shortcuts
        .push((id.to_string(), text.to_string()));
    others.dedup();
    app.shortcut_editor.note = (!others.is_empty()).then(|| {
        trf(
            "options.shortcut_taken",
            &[("keys", text), ("others", &others.join(", "))],
        )
    });
}

/// "File > Export...": the menus a command sits in and its name.
fn command_label(e: &crate::ui::menus::invoke::Entry) -> String {
    let mut parts: Vec<String> = e.path.iter().map(|k| tr(k)).collect();
    parts.push(match crate::macros::command_name(&e.key) {
        Some((name, _)) => name,
        None => tr(&e.key),
    });
    parts
        .iter()
        .map(|p| p.trim_end_matches("...").to_string())
        .collect::<Vec<_>>()
        .join(" > ")
}

fn menu_or_tool_name(app: &App, id: &str) -> String {
    if let Some(e) = app.shortcut_editor.entries.iter().find(|e| e.key == id) {
        return command_label(e);
    }
    crate::tools::Tool::ALL
        .iter()
        .find(|t| t.id() == id)
        .map(|t| t.name())
        .unwrap_or_else(|| id.to_string())
}

/// One row of the command list: a menu command or a toolbox tool.
#[derive(Debug, Clone)]
struct CommandItem {
    id: String,
    name: String,
    /// "File > Export" for a menu command, the toolbox for a tool.
    place: String,
    built_in: String,
    tool: Option<crate::tools::Tool>,
}

/// The rows of `category`, filtered by `filter` (lower case).
fn command_items(app: &App, category: Option<&str>, filter: &str) -> Vec<CommandItem> {
    let mut out = Vec::new();
    if category.is_none_or(|c| c != TOOLBOX) {
        for e in &app.shortcut_editor.entries {
            let top = e.path.first().map(String::as_str);
            let is_macro = e.key.starts_with("menu.macro.");
            match category {
                Some(MACROS) if !is_macro => continue,
                Some(c) if c != MACROS && top != Some(c) => continue,
                _ => {}
            }
            let label = command_label(e);
            let name = match crate::macros::command_name(&e.key) {
                Some((name, _)) => name,
                None => tr(&e.key).trim_end_matches("...").to_string(),
            };
            out.push(CommandItem {
                id: e.key.clone(),
                name,
                place: label,
                built_in: e.shortcut.clone(),
                tool: None,
            });
        }
    }
    if category.is_none_or(|c| c == TOOLBOX) {
        for t in crate::tools::Tool::ALL {
            out.push(CommandItem {
                id: t.id().to_string(),
                name: t.name(),
                place: tr("menu.window.toolbar_toolbox"),
                built_in: t.shortcut().unwrap_or("").to_string(),
                tool: Some(t),
            });
        }
    }
    if !filter.is_empty() {
        out.retain(|c| {
            c.name.to_lowercase().contains(filter) || c.place.to_lowercase().contains(filter)
        });
    }
    out
}

const LIST_W: f32 = 240.0;
const LIST_H: f32 = 330.0;
const ROW_H: f32 = 24.0;

/// Options > Customization > Commands: a category and a search above the
/// list of commands (with their pictures); beside it the picked
/// command's General and Shortcut Keys tabs.
fn commands_page(app: &mut App, ui: &mut Ui) {
    if app.shortcut_editor.entries.is_empty() {
        let ctx = ui.ctx().clone();
        app.shortcut_editor.entries = crate::ui::menus::commands(app, &ctx);
    }
    let category_name = |c: Option<&str>| match c {
        None => tr("options.all_commands"),
        Some(TOOLBOX) => tr("menu.window.toolbar_toolbox"),
        Some(MACROS) => tr("options.macros_category"),
        Some(k) => tr(k),
    };
    ui.horizontal(|ui| {
        let current = app.shortcut_editor.category.clone();
        egui::ComboBox::from_id_salt("command_category")
            .selected_text(category_name(current.as_deref()))
            .width(LIST_W - 8.0)
            .show_ui(ui, |ui| {
                let mut pick = current.clone();
                ui.selectable_value(&mut pick, None, category_name(None));
                for k in crate::ui::menus::menu_titles() {
                    ui.selectable_value(&mut pick, Some(k.to_string()), category_name(Some(k)));
                }
                ui.selectable_value(
                    &mut pick,
                    Some(TOOLBOX.to_string()),
                    category_name(Some(TOOLBOX)),
                );
                ui.selectable_value(
                    &mut pick,
                    Some(MACROS.to_string()),
                    category_name(Some(MACROS)),
                );
                app.shortcut_editor.category = pick;
            });
        ui.add_space(12.0);
        ui.label(tr("options.find_command"));
        ui.add(egui::TextEdit::singleline(&mut app.shortcut_editor.filter).desired_width(180.0));
    });
    ui.add_space(4.0);
    let filter = app.shortcut_editor.filter.trim().to_lowercase();
    let items = command_items(app, app.shortcut_editor.category.as_deref(), &filter);
    if app
        .shortcut_editor
        .selected
        .as_ref()
        .is_none_or(|s| !items.iter().any(|c| &c.id == s))
    {
        app.shortcut_editor.selected = items.first().map(|c| c.id.clone());
    }
    ui.horizontal_top(|ui| {
        egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
            .fill(crate::theme::Tokens::field())
            .show(ui, |ui| {
                ui.set_width(LIST_W);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    egui::ScrollArea::vertical()
                        .id_salt("command_list")
                        .auto_shrink([false, false])
                        .max_height(LIST_H)
                        .min_scrolled_height(LIST_H)
                        .show_rows(ui, ROW_H, items.len(), |ui, rows| {
                            for i in rows {
                                if let Some(c) = items.get(i) {
                                    command_list_row(app, ui, c);
                                }
                            }
                        });
                });
            });
        ui.add_space(10.0);
        ui.vertical(|ui| {
            ui.set_max_width(ui.available_width());
            let selected = app
                .shortcut_editor
                .selected
                .as_ref()
                .and_then(|s| items.iter().find(|c| &c.id == s))
                .cloned();
            ui.horizontal(|ui| {
                let keys = &mut app.shortcut_editor.keys_tab;
                ui.selectable_value(keys, false, tr("options.general"));
                ui.selectable_value(keys, true, tr("options.shortcut_keys"));
            });
            ui.separator();
            match selected {
                None => {
                    ui.label(
                        egui::RichText::new(tr("options.no_command")).color(Tokens::text_dim()),
                    );
                }
                Some(c) if !app.shortcut_editor.keys_tab => command_general(app, ui, &c),
                Some(c) => command_keys(app, ui, &c),
            }
        });
    });
}

/// One row of the list: the command's picture and name.
fn command_list_row(app: &mut App, ui: &mut Ui, c: &CommandItem) {
    let (row, resp) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ROW_H),
        egui::Sense::click(),
    );
    let on = app.shortcut_editor.selected.as_deref() == Some(c.id.as_str());
    if on {
        ui.painter()
            .rect_filled(row, 0.0, crate::theme::Tokens::selected_row());
    } else if resp.hovered() {
        ui.painter()
            .rect_filled(row, 0.0, crate::theme::Tokens::hovered_row());
    }
    let icon = egui::Rect::from_center_size(
        egui::pos2(row.left() + 14.0, row.center().y),
        egui::vec2(16.0, 16.0),
    );
    match c.tool {
        Some(t) => crate::ui::icons::draw(ui.painter(), icon, t, Tokens::icon()),
        None => {
            crate::ui::menus::draw_menu_picture(ui.painter(), icon, &c.id, Tokens::icon());
        }
    }
    let galley = ui.painter().layout_no_wrap(
        c.name.clone(),
        egui::FontId::proportional(13.0),
        Tokens::text(),
    );
    ui.painter().with_clip_rect(row).galley(
        egui::pos2(row.left() + 30.0, row.center().y - galley.size().y / 2.0),
        galley,
        Tokens::text(),
    );
    let resp = resp.on_hover_text(&c.place);
    if resp.clicked() {
        app.shortcut_editor.selected = Some(c.id.clone());
        app.shortcut_editor.capture = None;
        app.shortcut_editor.note = None;
    }
}

/// General: the command's name, where it is and its keys.
fn command_general(app: &App, ui: &mut Ui, c: &CommandItem) {
    grid("command_general").show(ui, |ui| {
        row_label(ui, &tr("options.command_name"));
        ui.add(egui::Label::new(egui::RichText::new(&c.name).strong()).wrap());
        ui.end_row();
        row_label(ui, &tr("options.available_on"));
        ui.add(egui::Label::new(&c.place).wrap());
        ui.end_row();
        row_label(ui, &format!("{}:", tr("options.shortcut_keys")));
        let keys = current_keys(app, c);
        ui.label(if keys.is_empty() {
            "-".to_string()
        } else {
            keys.join(", ")
        });
        ui.end_row();
    });
}

/// The keys that run `c` now: the user's, else the built-in ones. The
/// user's may be empty: the command's shortcut was removed.
fn current_keys(app: &App, c: &CommandItem) -> Vec<String> {
    match user_shortcut(app, &c.id) {
        Some(k) if k.is_empty() => Vec::new(),
        Some(k) => vec![k],
        None if !c.built_in.is_empty() => vec![c.built_in.clone()],
        None => Vec::new(),
    }
}

/// Shortcut Keys: the key waiting for a press, the current keys with
/// Delete, and Reset All.
fn command_keys(app: &mut App, ui: &mut Ui, c: &CommandItem) {
    ui.label(tr("options.new_shortcut"));
    let capturing = app.shortcut_editor.capture.as_deref() == Some(c.id.as_str());
    let text = if capturing {
        tr("options.press_keys")
    } else {
        tr("options.set_shortcut")
    };
    let button = ui
        .add_sized([200.0, 24.0], egui::Button::new(text).selected(capturing))
        .on_hover_text(tr("options.set_shortcut_hint"));
    if button.clicked() {
        app.shortcut_editor.capture = (!capturing).then(|| c.id.clone());
        app.shortcut_editor.note = None;
        // The keys pressed next are the shortcut, not typing or a press
        // of this button.
        ui.ctx().memory_mut(|m| m.stop_text_input());
        button.surrender_focus();
    }
    ui.add_space(6.0);
    ui.label(tr("options.current_shortcuts"));
    let custom = user_shortcut(app, &c.id);
    egui::Frame::new()
        .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
        .fill(crate::theme::Tokens::field())
        .inner_margin(4)
        .show(ui, |ui| {
            ui.set_width(260.0);
            ui.set_min_height(70.0);
            match &custom {
                // Removed: no keys.
                Some(k) if k.is_empty() => {}
                Some(k) => {
                    ui.label(k);
                }
                None if !c.built_in.is_empty() => {
                    ui.label(format!("{}  ({})", c.built_in, tr("options.built_in")));
                }
                None => {}
            }
        });
    let has_keys = !current_keys(app, c).is_empty();
    ui.horizontal(|ui| {
        if ui
            .add_enabled(has_keys, egui::Button::new(tr("options.delete")))
            .on_hover_text(tr("options.clear_shortcut"))
            .clicked()
        {
            remove_shortcut(app, c);
        }
        // The commands of the category shown; with every command shown,
        // the same as resetting all, so it asks first.
        if ui.button(tr("options.reset_shortcuts")).clicked() {
            match app.shortcut_editor.category.clone() {
                Some(category) => reset_category_shortcuts(app, &category),
                None => app.shortcut_editor.confirm_reset_all = true,
            }
        }
    });
    if ui.button(tr("options.reset_all_shortcuts")).clicked() {
        app.shortcut_editor.confirm_reset_all = true;
    }
    if app.shortcut_editor.confirm_reset_all {
        ui.label(tr("options.reset_all_shortcuts_question"));
        ui.horizontal(|ui| {
            if ui.button(tr("dialog.reset")).clicked() {
                app.settings.shortcuts.clear();
                app.shortcut_editor.note = None;
                app.shortcut_editor.capture = None;
                app.shortcut_editor.confirm_reset_all = false;
            }
            if ui.button(tr("dialog.cancel")).clicked() {
                app.shortcut_editor.confirm_reset_all = false;
            }
        });
    }
    // What the last shortcut set took over.
    if let Some(note) = &app.shortcut_editor.note {
        ui.label(egui::RichText::new(note).color(Tokens::ACCENT));
    }
}

/// Leave `c` without a shortcut: the user's goes, and a built-in one is
/// overridden by an empty shortcut, so its keys no longer run it.
fn remove_shortcut(app: &mut App, c: &CommandItem) {
    app.settings.shortcuts.retain(|(k, _)| k != &c.id);
    if !c.built_in.is_empty() {
        app.settings.shortcuts.push((c.id.clone(), String::new()));
    }
    app.shortcut_editor.note = None;
}

/// Reset to defaults on the Commands page: the shortcuts of the commands
/// in `category` (a menu's title key, the toolbox or the macros) only.
fn reset_category_shortcuts(app: &mut App, category: &str) {
    let ids: Vec<String> = command_items(app, Some(category), "")
        .into_iter()
        .map(|c| c.id)
        .collect();
    app.settings.shortcuts.retain(|(k, _)| !ids.contains(k));
    app.shortcut_editor.note = None;
    app.shortcut_editor.capture = None;
}

fn command_bars_page(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(tr("opt.global_button_size"));
        let mut size = app.settings.button_size;
        if combo(
            ui,
            "global_button_size",
            &mut size,
            &crate::settings::ButtonSize::ALL,
            |v| tr(v.key()),
            90.0,
        ) {
            app.settings.button_size = size;
        }
    });
    ui.add_space(6.0);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(185.0);
            egui::Frame::new()
                .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
                .fill(crate::theme::Tokens::field())
                .inner_margin(4)
                .show(ui, |ui| {
                    ui.set_width(175.0);
                    ui.set_min_height(300.0);
                    ui.spacing_mut().item_spacing.y = 3.0;
                    // The menu bar is always there.
                    ui.add_enabled(
                        false,
                        egui::Checkbox::new(&mut true, tr("menu.window.toolbar_menu_bar")),
                    );
                    let palette = &mut app.settings.palette.show_palette;
                    for (flag, key) in [
                        (&mut app.show_status_bar, "menu.window.toolbar_status_bar"),
                        (
                            &mut app.show_standard_toolbar,
                            "menu.window.toolbar_standard",
                        ),
                        (&mut app.show_zoom_toolbar, "menu.window.toolbar_zoom"),
                        (
                            &mut app.show_property_bar,
                            "menu.window.toolbar_property_bar",
                        ),
                        (&mut app.show_toolbox, "menu.window.toolbar_toolbox"),
                        (&mut app.show_text_toolbar, "menu.window.toolbar_text"),
                        (
                            &mut app.show_transform_toolbar,
                            "menu.window.toolbar_transform",
                        ),
                        (palette, "bars.palette"),
                    ] {
                        ui.checkbox(flag, tr(key));
                    }
                });
            ui.horizontal(|ui| {
                off_button(ui, &tr("opt.new"));
                if ui
                    .add(egui::Button::new(tr("options.reset")).min_size(egui::vec2(96.0, 22.0)))
                    .clicked()
                {
                    app.reset_command_bars();
                }
            });
        });
        ui.add_space(14.0);
        ui.vertical(|ui| {
            heading(ui, &tr("opt.button_properties"));
            grid("button_props").show(ui, |ui| {
                row_label(ui, &tr("opt.button_size"));
                let mut size = app
                    .settings
                    .toolbox_button_size
                    .unwrap_or(app.settings.button_size);
                if combo(
                    ui,
                    "toolbox_button_size",
                    &mut size,
                    &crate::settings::ButtonSize::ALL,
                    |v| tr(v.key()),
                    100.0,
                ) {
                    app.settings.toolbox_button_size = Some(size);
                }
                ui.end_row();
                row_label(ui, &tr("opt.button_style"));
                off_combo(ui, "button_style", &tr("opt.style_default"), 130.0);
                ui.end_row();
            });
            heading(ui, &tr("opt.toolbar_properties"));
            off_check(ui, true, &tr("opt.title_when_floating"));
            ui.checkbox(&mut app.settings.bars.locked, tr("opt.lock_toolbars"));
            ui.checkbox(
                &mut app.settings.show_outline_flyout,
                tr("options.show_outline_flyout"),
            );
            heading(ui, &tr("opt.menu_bar_mode"));
            off_radio(ui, true, &tr("opt.menu_mode_normal"));
            off_radio(ui, false, &tr("opt.menu_mode_file"));
        });
    });
}

// ----- Tools -------------------------------------------------------------

fn pick_page(app: &mut App, ui: &mut Ui) {
    ui.checkbox(
        &mut app.settings.crosshair_cursor,
        tr("opt.crosshair_cursor"),
    );
    ui.checkbox(
        &mut app.settings.treat_all_filled,
        tr("opt.treat_all_filled"),
    );
    ui.checkbox(
        &mut app.settings.marquee_touching,
        tr("options.marquee_touching"),
    )
    .on_hover_text(tr("options.marquee_touching_tip"));
    ui.add_space(8.0);
    ui.label(tr("opt.ctrl_shift_keys"));
    off_radio(ui, true, &tr("opt.keys_traditional"));
    ui.indent("keys_trad", |ui| note(ui, &tr("opt.keys_traditional_hint")));
    off_radio(ui, false, &tr("opt.keys_windows"));
    ui.indent("keys_win", |ui| note(ui, &tr("opt.keys_windows_hint")));
}

fn zoom_pan_page(app: &mut App, ui: &mut Ui) {
    heading(ui, &tr("opt.zoom_tool"));
    ui.label(tr("opt.right_button_action"));
    ui.radio_value(
        &mut app.settings.zoom_right_click_out,
        true,
        tr("opt.zoom_out"),
    );
    ui.radio_value(
        &mut app.settings.zoom_right_click_out,
        false,
        tr("opt.context_menu"),
    );
    ui.add_space(4.0);
    off_check(ui, false, &tr("opt.zoom_relative_1_1"));
    off_check(ui, true, &tr("opt.center_mouse_wheel_zoom"));
    sep(ui);
    heading(ui, &tr("opt.pan_tool"));
    ui.label(tr("opt.right_button_action"));
    off_radio(ui, true, &tr("opt.pan_reduce"));
    off_radio(ui, false, &tr("opt.context_menu"));
    sep(ui);
    let open_id = ui.id().with("calibrate_open");
    let mut open: bool = ui.data(|d| d.get_temp(open_id)).unwrap_or(false);
    if ui.button(tr("opt.calibrate_zoom")).clicked() {
        open = !open;
    }
    ui.data_mut(|d| d.insert_temp(open_id, open));
    note(ui, &tr("opt.calibrate_zoom_hint"));
    if open {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(tr("options.screen_resolution"));
            let mut ppi = crate::settings::screen_ppi(&app.settings);
            if ui
                .add_sized(
                    [70.0, 22.0],
                    egui::DragValue::new(&mut ppi)
                        .range(48.0..=400.0)
                        .speed(0.1)
                        .max_decimals(1),
                )
                .changed()
            {
                app.settings.screen_ppi = ppi;
            }
            ui.label(tr("options.points_per_inch"));
            if ui.button(tr("options.reset")).clicked() {
                app.settings.screen_ppi = 96.0;
            }
        });
        note(ui, &tr("options.calibrate_help"));
        calibration_rulers(ui, crate::settings::screen_points_per_inch(&app.settings));
    }
}

fn rectangle_page(app: &mut App, ui: &mut Ui) {
    use tracedraw_core::{CornerKind, Corners};
    let mut c = app.rect_corners;
    let u = app.units;
    ui.horizontal(|ui| {
        for (kind, key) in [
            (CornerKind::Round, "toolbar.corner_round"),
            (CornerKind::Scallop, "toolbar.corner_scallop"),
            (CornerKind::Chamfer, "toolbar.corner_chamfer"),
        ] {
            let (r, resp) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
            if c.kind == kind {
                ui.painter().rect_filled(r, 2.0, Tokens::ACCENT);
            } else if resp.hovered() {
                ui.painter().rect_filled(r, 2.0, Tokens::hovered_row());
            }
            corner_glyph(
                ui.painter(),
                r.shrink(5.0),
                kind,
                if c.kind == kind {
                    egui::Color32::WHITE
                } else {
                    Tokens::icon()
                },
            );
            if resp.on_hover_text(tr(key)).clicked() {
                c.kind = kind;
            }
        }
    });
    ui.add_space(4.0);
    ui.label(tr("opt.corners"));
    let together = app.corners_together;
    let mut lock = together;
    egui::Grid::new("rect_corners_grid")
        .num_columns(5)
        .spacing([6.0, 6.0])
        .show(ui, |ui| {
            for (row, pair) in [
                [Corners::TOP_LEFT, Corners::TOP_RIGHT],
                [Corners::BOTTOM_LEFT, Corners::BOTTOM_RIGHT],
            ]
            .into_iter()
            .enumerate()
            {
                for (col, i) in pair.into_iter().enumerate() {
                    if col == 1 {
                        if row == 0 {
                            let (r, resp) = ui
                                .allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                            if lock {
                                ui.painter().rect_filled(r, 2.0, Tokens::ACCENT);
                            } else if resp.hovered() {
                                ui.painter().rect_filled(r, 2.0, Tokens::hovered_row());
                            }
                            lock_glyph(
                                ui.painter(),
                                r.center(),
                                lock,
                                if lock {
                                    egui::Color32::WHITE
                                } else {
                                    Tokens::icon()
                                },
                            );
                            if resp.on_hover_text(tr("toolbar.corners_together")).clicked() {
                                lock = !lock;
                            }
                        } else {
                            ui.label("");
                        }
                    }
                    let mut v = u.world_from_mm(c.radii[i]);
                    if ui
                        .add_sized(
                            [76.0, 22.0],
                            egui::DragValue::new(&mut v)
                                .range(0.0..=10_000.0)
                                .speed(0.1)
                                .max_decimals(1)
                                .suffix(format!(" {}", u.short())),
                        )
                        .changed()
                    {
                        let v = u.world_to_mm(v).max(0.0);
                        if together {
                            c.radii = [v; 4];
                        } else {
                            c.radii[i] = v;
                        }
                    }
                }
                ui.end_row();
            }
        });
    app.corners_together = lock;
    let mut relative = !c.fixed;
    if ui
        .checkbox(&mut relative, tr("opt.scale_corners"))
        .changed()
    {
        c.fixed = !relative;
    }
    app.rect_corners = c;
}

fn ellipse_page(app: &mut App, ui: &mut Ui) {
    let mut kind = match app.ellipse_arc {
        None => 0,
        Some(a) if a.pie => 1,
        Some(_) => 2,
    };
    let before = kind;
    ui.horizontal_top(|ui| {
        ui.label(tr("opt.type"));
        ui.vertical(|ui| {
            ui.radio_value(&mut kind, 0, tr("options.ellipse"));
            ui.radio_value(&mut kind, 1, tr("options.pie"));
            ui.radio_value(&mut kind, 2, tr("options.arc"));
        });
    });
    let mut arc = app.ellipse_arc.unwrap_or(tracedraw_core::EllipseArc {
        start_deg: 90.0,
        end_deg: 90.0,
        pie: true,
    });
    if kind != before {
        app.ellipse_arc = match kind {
            0 => None,
            k => {
                arc.pie = k == 1;
                Some(arc)
            }
        };
    }
    ui.add_space(6.0);
    let on = app.ellipse_arc.is_some();
    let mut a = app.ellipse_arc.unwrap_or(arc);
    ui.add_enabled_ui(on, |ui| {
        grid("arc").show(ui, |ui| {
            row_label(ui, &tr("opt.starting_angle"));
            ui.horizontal(|ui| {
                ui.add_sized(
                    [64.0, 22.0],
                    egui::DragValue::new(&mut a.start_deg)
                        .range(-360.0..=360.0)
                        .max_decimals(1),
                );
                ui.label(tr("options.degrees"));
            });
            ui.end_row();
            row_label(ui, &tr("opt.ending_angle"));
            ui.horizontal(|ui| {
                ui.add_sized(
                    [64.0, 22.0],
                    egui::DragValue::new(&mut a.end_deg)
                        .range(-360.0..=360.0)
                        .max_decimals(1),
                );
                ui.label(tr("options.degrees"));
            });
            ui.end_row();
        });
    });
    if on {
        app.ellipse_arc = Some(a);
    }
    ui.add_space(6.0);
    ui.horizontal_top(|ui| {
        ui.label(tr("opt.direction"));
        ui.vertical(|ui| {
            off_radio(ui, false, &tr("opt.clockwise"));
            off_radio(ui, true, &tr("opt.counterclockwise"));
        });
    });
}

fn spiral_page(app: &mut App, ui: &mut Ui) {
    ui.horizontal_top(|ui| {
        ui.label(tr("opt.type"));
        ui.vertical(|ui| {
            ui.radio_value(&mut app.spiral_logarithmic, false, tr("opt.symmetrical"));
            ui.radio_value(&mut app.spiral_logarithmic, true, tr("opt.logarithmic"));
            spiral_preview(ui, app.spiral_revolutions, app.spiral_logarithmic);
        });
    });
    grid("spiral").show(ui, |ui| {
        row_label(ui, &tr("opt.revolutions"));
        ui.add_sized(
            [64.0, 22.0],
            egui::DragValue::new(&mut app.spiral_revolutions).range(1..=100),
        );
        ui.end_row();
        row_label(ui, &tr("opt.expansion"));
        ui.add_enabled_ui(false, |ui| {
            let mut v = 100.0f32;
            slider(ui, egui::Slider::new(&mut v, 1.0..=100.0).integer());
        });
        ui.end_row();
    });
}

fn graph_paper_page(app: &mut App, ui: &mut Ui) {
    grid("graph").show(ui, |ui| {
        row_label(ui, &tr("opt.columns_count"));
        ui.add_sized(
            [64.0, 22.0],
            egui::DragValue::new(&mut app.graph_cols).range(1..=99),
        );
        ui.end_row();
        row_label(ui, &tr("opt.rows_count"));
        ui.add_sized(
            [64.0, 22.0],
            egui::DragValue::new(&mut app.graph_rows).range(1..=99),
        );
        ui.end_row();
    });
}

fn table_page(app: &mut App, ui: &mut Ui) {
    grid("table").show(ui, |ui| {
        row_label(ui, &tr("opt.rows"));
        ui.add_sized(
            [64.0, 22.0],
            egui::DragValue::new(&mut app.table_rows).range(1..=99),
        );
        ui.end_row();
        row_label(ui, &tr("opt.columns"));
        ui.add_sized(
            [64.0, 22.0],
            egui::DragValue::new(&mut app.table_cols).range(1..=99),
        );
        ui.end_row();
    });
    ui.add_space(4.0);
    let p = &mut app.settings.table;
    ui.label(tr("opt.table_tab_key"));
    ui.radio_value(&mut p.tab_inserts_tab, true, tr("opt.table_tab_inserts"));
    ui.radio_value(&mut p.tab_inserts_tab, false, tr("opt.table_tab_moves"));
    ui.horizontal(|ui| {
        ui.label(tr("opt.table_tab_order"));
        combo(
            ui,
            "table_tab_order",
            &mut p.tab_by_columns,
            &[false, true],
            |c| {
                tr(if c {
                    "opt.table_order_columns"
                } else {
                    "opt.table_order_rows"
                })
            },
            250.0,
        );
    });
}

fn curves_page(app: &mut App, ui: &mut Ui) {
    let c = &mut app.settings.curves;
    grid("curves").show(ui, |ui| {
        row_label(ui, &tr("opt.freehand_smoothing"));
        ui.horizontal(|ui| {
            let mut v = c.smoothing.min(100);
            slider(ui, egui::Slider::new(&mut v, 0..=100).show_value(false));
            ui.add_sized([52.0, 22.0], egui::DragValue::new(&mut v).range(0..=100));
            c.smoothing = v;
        });
        ui.end_row();
        for key in ["opt.corner_threshold", "opt.straight_line_threshold"] {
            row_label(ui, &tr(key));
            ui.horizontal(|ui| {
                ui.add_enabled_ui(false, |ui| {
                    let mut v = 5u32;
                    ui.add_sized([52.0, 22.0], egui::DragValue::new(&mut v));
                });
                ui.label(tr("options.pixels"));
            });
            ui.end_row();
        }
        row_label(ui, &tr("opt.auto_join"));
        ui.horizontal(|ui| {
            let mut px = if c.join_px.is_finite() {
                c.join_px
            } else {
                5.0
            };
            ui.add_sized(
                [52.0, 22.0],
                egui::DragValue::new(&mut px)
                    .range(1.0..=50.0)
                    .speed(0.2)
                    .max_decimals(0),
            );
            c.join_px = px;
            ui.label(tr("options.pixels"));
        });
        ui.end_row();
    });
    sep(ui);
    heading(ui, &tr("options.pen"));
    ui.checkbox(&mut c.pen_preview, tr("options.pen_preview"));
    ui.checkbox(
        &mut c.pen_auto_add_delete,
        tr("options.pen_auto_add_delete"),
    );
    ui.horizontal(|ui| {
        ui.label(tr("options.constrain_key"));
        for k in crate::prefs::ConstrainKey::ALL {
            ui.radio_value(&mut c.constrain_key, k, tr(k.i18n_key()));
        }
    });
}

fn eraser_page(app: &mut App, ui: &mut Ui) {
    grid("eraser").show(ui, |ui| {
        row_label(ui, &tr("opt.thickness"));
        let u = app.units;
        let mut v = u.from_mm(app.eraser_width);
        if ui
            .add_sized(
                [76.0, 22.0],
                egui::DragValue::new(&mut v)
                    .speed(0.1)
                    .max_decimals(1)
                    .suffix(format!(" {}", u.short())),
            )
            .changed()
        {
            app.eraser_width = u.to_mm(v).max(0.01);
        }
        ui.end_row();
    });
    off_check(ui, true, &tr("opt.eraser_reduce_nodes"));
    ui.add_space(6.0);
    ui.radio_value(&mut app.eraser_square, false, tr("options.eraser_round"));
    ui.radio_value(&mut app.eraser_square, true, tr("options.eraser_square"));
}

// ----- Global ------------------------------------------------------------

fn global_general_page(app: &mut App, ui: &mut Ui) {
    off_check(ui, false, &tr("opt.background_tasks"));
    grid("global").show(ui, |ui| {
        row_label(ui, &tr("opt.ui_language"));
        let current = crate::i18n::language();
        let name = crate::i18n::LANGUAGES
            .iter()
            .find(|(c, _)| *c == current)
            .map(|(_, n)| *n)
            .unwrap_or("English");
        egui::ComboBox::from_id_salt("opt_lang")
            .selected_text(name)
            .width(140.0)
            .show_ui(ui, |ui| {
                for (code, n) in crate::i18n::LANGUAGES {
                    if ui.selectable_label(current == code, n).clicked() {
                        crate::i18n::set_language(code);
                        app.settings.language = code.to_string();
                    }
                }
            });
        ui.end_row();
    });
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        off_check(ui, true, &tr("opt.hardware_acceleration"));
        off_combo(ui, "gpu", &tr("opt.gpu_default"), 200.0);
    });
    ui.indent("gpu_uses", |ui| {
        off_check(ui, true, &tr("opt.gpu_display"));
        off_check(ui, true, &tr("opt.gpu_effects"));
    });
    ui.add_space(8.0);
    note(ui, &tr("options.reset_all_hint"));
    if ui.button(tr("options.reset_all")).clicked() {
        // The question replaces the options; nothing changes unless it is
        // answered Reset.
        if let Some(snapshot) = app.options_snapshot.take() {
            snapshot.restore(app);
        }
        app.dialog = crate::ui::dialogs::Dialog::ConfirmResetSettings;
    }
}

fn file_locations_page(app: &mut App, ui: &mut Ui) {
    let auto = app
        .settings
        .backup
        .auto_dir
        .clone()
        .or_else(crate::app::App::auto_backup_dir_default)
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let before = app
        .settings
        .backup
        .before_save_dir
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| tr("opt.same_folder"));
    let settings = crate::settings::Settings::path()
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let rows = [
        (tr("opt.loc_auto_backup"), auto),
        (tr("opt.loc_backup"), before),
        (tr("opt.loc_settings"), settings),
    ];
    let pick_id = ui.id().with("loc_pick");
    let mut pick: usize = ui.data(|d| d.get_temp(pick_id)).unwrap_or(0);
    let (frame, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 240.0),
        egui::Sense::hover(),
    );
    ui.painter().rect(
        frame,
        0.0,
        Tokens::field(),
        egui::Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    let col = frame.left() + 210.0;
    let font = egui::FontId::proportional(12.5);
    let header = egui::Rect::from_min_size(
        frame.min + egui::vec2(1.0, 1.0),
        egui::vec2(frame.width() - 2.0, 22.0),
    );
    ui.painter()
        .rect_filled(header, 0.0, Tokens::control_fill());
    ui.painter().text(
        egui::pos2(frame.left() + 8.0, header.center().y),
        egui::Align2::LEFT_CENTER,
        tr("opt.content_type"),
        font.clone(),
        Tokens::text(),
    );
    ui.painter().text(
        egui::pos2(col + 8.0, header.center().y),
        egui::Align2::LEFT_CENTER,
        tr("opt.location"),
        font.clone(),
        Tokens::text(),
    );
    for (i, (kind, path)) in rows.iter().enumerate() {
        let row = egui::Rect::from_min_size(
            egui::pos2(frame.left() + 1.0, header.bottom() + 2.0 + i as f32 * 22.0),
            egui::vec2(frame.width() - 2.0, 21.0),
        );
        let resp = ui.interact(row, ui.id().with(("loc_row", i)), egui::Sense::click());
        if pick == i {
            ui.painter().rect_filled(row, 0.0, Tokens::selected_row());
        } else if resp.hovered() {
            ui.painter().rect_filled(row, 0.0, Tokens::hovered_row());
        }
        let clip = ui.painter().with_clip_rect(row);
        clip.with_clip_rect(egui::Rect::from_x_y_ranges(
            row.left()..=col - 4.0,
            row.y_range(),
        ))
        .text(
            egui::pos2(row.left() + 7.0, row.center().y),
            egui::Align2::LEFT_CENTER,
            kind,
            font.clone(),
            Tokens::text(),
        );
        clip.with_clip_rect(egui::Rect::from_x_y_ranges(
            col..=row.right() - 4.0,
            row.y_range(),
        ))
        .text(
            egui::pos2(col + 8.0, row.center().y),
            egui::Align2::LEFT_CENTER,
            path,
            font.clone(),
            Tokens::text(),
        );
        if resp.on_hover_text(path).clicked() {
            pick = i;
        }
    }
    ui.data_mut(|d| d.insert_temp(pick_id, pick));
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let editable = pick < 2;
            if ui
                .add_enabled(editable, egui::Button::new(tr("opt.edit")))
                .clicked()
            {
                if let Some(p) = crate::files::pick_folder() {
                    if pick == 0 {
                        app.settings.backup.auto_dir = Some(p);
                    } else {
                        app.settings.backup.before_save_dir = Some(p);
                    }
                }
            }
            if ui
                .add_enabled(editable, egui::Button::new(tr("options.reset")))
                .clicked()
            {
                if pick == 0 {
                    app.settings.backup.auto_dir = None;
                } else {
                    app.settings.backup.before_save_dir = None;
                }
            }
        });
    });
}

// ----- Workspaces --------------------------------------------------------

fn workspace_page(app: &mut App, ui: &mut Ui) {
    crate::ui::options_pages::workspace_buttons(app, ui);
    sep(ui);
    heading(ui, &tr("options.workspace_transfer"));
    note(ui, &tr("options.workspace_transfer_hint"));
    let mut parts = app.settings.workspace_parts;
    for (on, key) in parts.flags_mut() {
        ui.checkbox(on, tr(key));
    }
    app.settings.workspace_parts = parts;
}

// ----- Document Options --------------------------------------------------

fn document_general_page(app: &mut App, ui: &mut Ui) {
    if !app.has_document() {
        return;
    }
    let mut meta = app.doc().metadata.clone();
    heading(ui, &tr("opt.preferences"));
    ui.checkbox(&mut meta.fill_open_curves, tr("opt.fill_open_curves"));
    let mut inflate = !meta.no_auto_inflate;
    if ui
        .checkbox(&mut inflate, tr("opt.auto_inflate_bitmaps"))
        .changed()
    {
        meta.no_auto_inflate = !inflate;
    }
    if meta != app.doc().metadata {
        app.run(Command::SetMetadata { metadata: meta });
    }
    grid("doc_view").show(ui, |ui| {
        row_label(ui, &tr("opt.view_mode"));
        use crate::app::ViewMode as V;
        let mut mode = app.view_mode;
        let name = |m: V| {
            tr(match m {
                V::Wireframe => "menu.view.wireframe",
                V::Normal => "menu.view.normal",
                V::Enhanced => "menu.view.enhanced",
                V::Pixels => "menu.view.pixels",
            })
        };
        if combo(
            ui,
            "doc_view_mode",
            &mut mode,
            &[V::Wireframe, V::Normal, V::Enhanced, V::Pixels],
            name,
            130.0,
        ) {
            app.view_mode = mode;
            app.wireframe = mode == V::Wireframe;
        }
        ui.end_row();
        row_label(ui, &tr("opt.page_view"));
        ui.horizontal(|ui| {
            for (multi, key) in [(false, "opt.single_page"), (true, "opt.multipage")] {
                let on = app.multipage == multi;
                let (r, resp) =
                    ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
                if on {
                    ui.painter().rect_filled(r, 2.0, Tokens::ACCENT);
                } else if resp.hovered() {
                    ui.painter().rect_filled(r, 2.0, Tokens::hovered_row());
                }
                let color = if on {
                    egui::Color32::WHITE
                } else {
                    Tokens::icon()
                };
                let st = egui::Stroke::new(1.2, color);
                let page = egui::Rect::from_center_size(r.center(), egui::vec2(9.0, 12.0));
                if multi {
                    ui.painter().rect_stroke(
                        page.translate(egui::vec2(-2.0, -2.0)),
                        1.0,
                        st,
                        egui::StrokeKind::Inside,
                    );
                    ui.painter().rect_stroke(
                        page.translate(egui::vec2(2.0, 2.0)),
                        1.0,
                        st,
                        egui::StrokeKind::Inside,
                    );
                } else {
                    ui.painter()
                        .rect_stroke(page, 1.0, st, egui::StrokeKind::Inside);
                }
                if resp.on_hover_text(tr(key)).clicked() && !on {
                    app.multipage = multi;
                    if multi {
                        app.page_sorter = false;
                    }
                    app.fit_pending = true;
                }
            }
        });
        ui.end_row();
    });
    sep(ui);
    // Ctrl+D places duplicates this far from the original.
    heading(ui, &tr("opt.duplicate_offset"));
    let u = app.units;
    let (mut dx, mut dy) = (
        u.world_from_mm(app.duplicate_offset.x),
        u.world_from_mm(app.duplicate_offset.y),
    );
    let mut changed = false;
    grid("dup").show(ui, |ui| {
        for (key, v) in [("opt.horizontal", &mut dx), ("opt.vertical", &mut dy)] {
            row_label(ui, &tr(key));
            changed |= ui
                .add_sized(
                    [80.0, 22.0],
                    egui::DragValue::new(v)
                        .speed(0.1)
                        .max_decimals(1)
                        .suffix(format!(" {}", u.short())),
                )
                .changed();
            ui.end_row();
        }
    });
    if changed {
        app.duplicate_offset =
            tracedraw_core::geometry::Vec2::new(u.world_to_mm(dx), u.world_to_mm(dy));
    }
}

fn layout_page(app: &mut App, ui: &mut Ui) {
    let s = app.page_size();
    let u = app.units;
    ui.horizontal_top(|ui| {
        grid("doc_layout").show(ui, |ui| {
            row_label(ui, &tr("options.layout"));
            off_combo(ui, "doc_layout_kind", &tr("opt.full_page"), 120.0);
            ui.end_row();
            row_label(ui, &tr("opt.width"));
            ui.label(format!("{:.0} {}", u.world_from_mm(s.width), u.short()));
            ui.end_row();
            row_label(ui, &tr("opt.height"));
            ui.label(format!("{:.0} {}", u.world_from_mm(s.height), u.short()));
            ui.end_row();
            row_label(ui, &tr("opt.description"));
            ui.label(tr("opt.one_page_per_sheet"));
            ui.end_row();
        });
        ui.add_space(16.0);
        let (r, _) = ui.allocate_exact_size(egui::vec2(160.0, 78.0), egui::Sense::hover());
        ui.painter().rect_filled(r, 0.0, egui::Color32::WHITE);
        let ink = egui::Color32::from_gray(60);
        for i in 0..3 {
            let p = egui::Rect::from_min_size(
                r.min + egui::vec2(30.0 + i as f32 * 36.0, 24.0),
                egui::vec2(22.0, 30.0),
            );
            ui.painter().rect_stroke(
                p,
                0.0,
                egui::Stroke::new(1.0, ink),
                egui::StrokeKind::Inside,
            );
            for k in 0..4 {
                let y = p.top() + 7.0 + k as f32 * 6.0;
                ui.painter().hline(
                    p.left() + 4.0..=p.right() - 4.0,
                    y,
                    egui::Stroke::new(1.0, ink),
                );
            }
        }
    });
    sep(ui);
    off_check(ui, false, &tr("opt.facing_pages"));
    note(ui, &tr("opt.facing_pages_hint"));
    ui.horizontal(|ui| {
        ui.add_enabled(false, egui::Label::new(tr("opt.start_on")));
        off_combo(ui, "facing_start", &tr("opt.right_side"), 90.0);
    });
    sep(ui);
    let n = app.doc().pages.len();
    ui.label(trf("options.pages_count", &[("n", &n.to_string())]));
    ui.checkbox(&mut app.page_sorter, tr("menu.view.page_sorter"));
    heading(ui, &tr("options.master_layers"));
    let masters: Vec<(tracedraw_core::LayerId, String, tracedraw_core::MasterScope)> = app
        .doc()
        .master
        .iter()
        .map(|l| (l.id, l.name.clone(), l.scope))
        .collect();
    for (id, name, scope) in masters {
        ui.horizontal(|ui| {
            ui.label(&name);
            for (s, k) in [
                (tracedraw_core::MasterScope::All, "options.scope_all"),
                (tracedraw_core::MasterScope::Odd, "options.scope_odd"),
                (tracedraw_core::MasterScope::Even, "options.scope_even"),
            ] {
                if ui.selectable_label(scope == s, tr(k)).clicked() {
                    app.run(Command::SetLayerScope {
                        layer: id,
                        scope: s,
                    });
                }
            }
        });
    }
    ui.horizontal(|ui| {
        for (s, k) in [
            (tracedraw_core::MasterScope::All, "options.new_master_all"),
            (tracedraw_core::MasterScope::Odd, "options.new_master_odd"),
            (tracedraw_core::MasterScope::Even, "options.new_master_even"),
        ] {
            if ui.button(tr(k)).clicked() {
                crate::ui::objects_docker::new_master_layer(app, s);
            }
        }
    });
}

fn background_page(app: &mut App, ui: &mut Ui) {
    let page = app.page;
    let current = app.doc().page(page).ok().and_then(|p| p.background.clone());
    let was = match &current {
        None => 0,
        Some(Fill::Solid(_)) => 1,
        Some(_) => 2,
    };
    let mut kind = was;
    ui.radio_value(&mut kind, 0, tr("opt.no_background"));
    let mut rgb = match &current {
        Some(Fill::Solid(c)) => {
            let [r, g, b] = c.to_rgb8();
            [r, g, b]
        }
        _ => [255, 255, 255],
    };
    let mut color_changed = false;
    ui.horizontal(|ui| {
        ui.radio_value(&mut kind, 1, tr("opt.solid"));
        ui.add_space(20.0);
        color_changed = rgb_button(ui, &mut rgb);
    });
    let mut browse = false;
    ui.horizontal(|ui| {
        ui.radio_value(&mut kind, 2, tr("opt.bitmap"));
        ui.add_space(ui.available_width() - 100.0);
        browse = ui.button(tr("opt.browse")).clicked();
    });
    ui.horizontal(|ui| {
        ui.add_enabled(false, egui::Label::new(tr("opt.bitmap_source")));
        off_radio(ui, false, &tr("opt.linked"));
        off_radio(ui, true, &tr("opt.embedded"));
    });
    sep(ui);
    heading(ui, &tr("opt.bitmap_size"));
    off_radio(ui, true, &tr("opt.default_size"));
    off_radio(ui, false, &tr("opt.custom_size"));
    sep(ui);
    off_check(ui, true, &tr("opt.print_export_background"));
    if color_changed && kind != 1 {
        kind = 1;
    }
    match kind {
        0 => {
            if current.is_some() {
                app.run(Command::SetPageBackground {
                    page,
                    background: None,
                });
            }
        }
        1 => {
            if color_changed || !matches!(current, Some(Fill::Solid(_))) {
                let c = Color::rgb8(rgb[0], rgb[1], rgb[2]);
                app.run(Command::SetPageBackground {
                    page,
                    background: Some(Fill::Solid(c)),
                });
            }
        }
        _ => browse |= was != 2,
    }
    if browse {
        crate::files::Dialog::new()
            .add_filter(
                "Images",
                &["png", "jpg", "jpeg", "bmp", "gif", "webp", "tif", "tiff"],
            )
            .pick_file(app, move |app, p| app.set_page_background_image(page, &p));
    }
}

/// The picture of a rectangle corner: rounded, scalloped or chamfered.
fn corner_glyph(
    p: &egui::Painter,
    r: egui::Rect,
    kind: tracedraw_core::CornerKind,
    color: egui::Color32,
) {
    use tracedraw_core::CornerKind;
    let st = egui::Stroke::new(1.3, color);
    let (a, b) = (r.left_bottom(), r.right_top());
    let k = r.width() * 0.5;
    let pts: Vec<egui::Pos2> = match kind {
        CornerKind::Round => (0..=8)
            .map(|i| {
                let t = std::f32::consts::FRAC_PI_2 * i as f32 / 8.0;
                egui::pos2(a.x + k - k * t.cos(), b.y + k - k * t.sin())
            })
            .collect(),
        CornerKind::Scallop => (0..=8)
            .map(|i| {
                let t = std::f32::consts::FRAC_PI_2 * i as f32 / 8.0;
                egui::pos2(a.x + k * t.sin(), b.y + k * t.cos())
            })
            .rev()
            .collect(),
        CornerKind::Chamfer => vec![egui::pos2(a.x, b.y + k), egui::pos2(a.x + k, b.y)],
    };
    p.line_segment([a, *pts.first().unwrap_or(&a)], st);
    p.line(pts.clone(), st);
    p.line_segment([*pts.last().unwrap_or(&b), b], st);
}

/// A padlock, shut or open.
fn lock_glyph(p: &egui::Painter, c: egui::Pos2, shut: bool, color: egui::Color32) {
    let body = egui::Rect::from_center_size(c + egui::vec2(0.0, 2.5), egui::vec2(10.0, 7.0));
    p.rect_filled(body, 1.0, color);
    let st = egui::Stroke::new(1.5, color);
    let lift = if shut { 0.0 } else { 3.0 };
    let (l, r) = (body.left() + 2.5, body.right() - 2.5);
    let top = body.top() - 4.5 - lift;
    let pts: Vec<egui::Pos2> = (0..=10)
        .map(|i| {
            let t = std::f32::consts::PI * i as f32 / 10.0;
            egui::pos2(c.x - (r - l) / 2.0 * t.cos(), top + 2.5 - 2.5 * t.sin())
        })
        .collect();
    p.line(pts, st);
    p.line_segment(
        [
            egui::pos2(l, top + 2.5),
            egui::pos2(l, body.top() - if shut { 0.0 } else { lift }),
        ],
        st,
    );
    p.line_segment([egui::pos2(r, top + 2.5), egui::pos2(r, body.top())], st);
}

/// A small picture of the spiral the tool draws.
fn spiral_preview(ui: &mut Ui, revolutions: u32, logarithmic: bool) {
    let (r, _) = ui.allocate_exact_size(egui::vec2(58.0, 58.0), egui::Sense::hover());
    ui.painter().rect(
        r,
        0.0,
        egui::Color32::WHITE,
        egui::Stroke::new(1.0, Tokens::control_border()),
        egui::StrokeKind::Inside,
    );
    let turns = revolutions.clamp(1, 12) as f32;
    let max = r.width() * 0.42;
    let steps = (turns * 48.0) as usize;
    let pts: Vec<egui::Pos2> = (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            let a = t * turns * std::f32::consts::TAU;
            let rad = if logarithmic { max * t * t } else { max * t };
            r.center() + egui::vec2(a.cos(), a.sin()) * rad
        })
        .collect();
    ui.painter()
        .line(pts, egui::Stroke::new(1.0, egui::Color32::from_gray(40)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_list_filters_by_menu_toolbox_and_text() {
        let mut app = App::headless();
        let ctx = egui::Context::default();
        let mut entries = Vec::new();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            entries = crate::ui::menus::commands(&mut app, ui.ctx());
        });
        out.textures_delta.clear();
        app.shortcut_editor.entries = entries;
        let all = command_items(&app, None, "");
        let file = command_items(&app, Some("menu.file"), "");
        let tools = command_items(&app, Some(TOOLBOX), "");
        assert!(file
            .iter()
            .any(|c| c.id == "menu.file.open" && c.built_in == "Ctrl+O"));
        assert!(file.iter().all(|c| c.tool.is_none()));
        assert_eq!(tools.len(), crate::tools::Tool::ALL.len());
        assert_eq!(all.len(), app.shortcut_editor.entries.len() + tools.len());
        let found = command_items(&app, None, "open");
        assert!(!found.is_empty() && found.len() < all.len());
        // A user's key shows instead of the built-in one.
        let open = file
            .iter()
            .find(|c| c.id == "menu.file.open")
            .cloned()
            .expect("open");
        set_menu_shortcut(&mut app, "menu.file.open", "Ctrl+Shift+O");
        assert_eq!(current_keys(&app, &open), vec!["Ctrl+Shift+O".to_string()]);
    }

    /// The app with the menu commands collected, as the Commands page has.
    fn app_with_commands() -> App {
        let mut app = App::headless();
        let ctx = egui::Context::default();
        let mut entries = Vec::new();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            entries = crate::ui::menus::commands(&mut app, ui.ctx());
        });
        out.textures_delta.clear();
        app.shortcut_editor.entries = entries;
        app
    }

    #[test]
    fn a_built_in_shortcut_can_be_removed() {
        let mut app = app_with_commands();
        let file = command_items(&app, Some("menu.file"), "");
        let open = file
            .iter()
            .find(|c| c.id == "menu.file.open")
            .cloned()
            .expect("open");
        assert_eq!(current_keys(&app, &open), vec!["Ctrl+O".to_string()]);
        // Delete on a built-in shortcut: an empty one overrides it.
        remove_shortcut(&mut app, &open);
        assert!(current_keys(&app, &open).is_empty());
        assert!(app
            .settings
            .shortcuts
            .contains(&("menu.file.open".to_string(), String::new())));
        // A user's own key on a command with none built in just goes.
        let tool = command_items(&app, Some(TOOLBOX), "")
            .into_iter()
            .find(|c| c.built_in.is_empty())
            .expect("a tool without a built-in key");
        set_menu_shortcut(&mut app, &tool.id, "Alt+Shift+K");
        assert_eq!(current_keys(&app, &tool), vec!["Alt+Shift+K".to_string()]);
        remove_shortcut(&mut app, &tool);
        assert!(current_keys(&app, &tool).is_empty());
        assert!(!app.settings.shortcuts.iter().any(|(k, _)| k == &tool.id));
    }

    #[test]
    fn reset_to_defaults_touches_the_category_shown_only() {
        let mut app = app_with_commands();
        set_menu_shortcut(&mut app, "menu.file.open", "Ctrl+Shift+O");
        set_menu_shortcut(&mut app, "rectangle", "Alt+R");
        reset_category_shortcuts(&mut app, TOOLBOX);
        // The tool's key went, the menu command's stayed.
        assert_eq!(
            app.settings.shortcuts,
            vec![("menu.file.open".to_string(), "Ctrl+Shift+O".to_string())]
        );
        reset_category_shortcuts(&mut app, "menu.file");
        assert!(app.settings.shortcuts.is_empty());
    }

    #[test]
    fn customization_pages_center_dialogs_and_reset_bars() {
        use OptionsPage::*;
        assert_eq!(
            OptionsSet::Customization.pages(),
            &[Appearance, CommandBars, Commands, ColorPalette]
        );
        let mut app = App::headless();
        assert!(app.settings.center_dialogs);
        // Dialogs follow the setting given for the frame.
        let ctx = egui::Context::default();
        assert!(crate::ui::chrome::center_dialogs(&ctx));
        crate::ui::chrome::set_center_dialogs(&ctx, false);
        assert!(!crate::ui::chrome::center_dialogs(&ctx));
        // Reset brings back the bars of a new installation.
        app.show_toolbox = false;
        app.show_zoom_toolbar = true;
        app.settings.palette.show_palette = false;
        app.settings.bars.locked = true;
        app.reset_command_bars();
        assert!(app.show_toolbox && !app.show_zoom_toolbar);
        assert!(app.settings.palette.show_palette && !app.settings.bars.locked);
    }

    /// The six categories and their pages as the reference shows them;
    /// every Tools page has its tool's picture.
    #[test]
    fn categories_follow_the_reference() {
        use OptionsPage::*;
        assert_eq!(OptionsSet::ALL[0], OptionsSet::Document);
        assert_eq!(OptionsSet::ALL[5], OptionsSet::Workspaces);
        assert_eq!(OptionsSet::App.pages().len(), 14);
        assert_eq!(OptionsSet::Tools.pages().len(), 14);
        assert_eq!(OptionsSet::Document.pages().len(), 8);
        assert_eq!(BitmapTracing.set(), OptionsSet::App);
        assert_eq!(Warnings.set(), OptionsSet::App);
        assert!(OptionsSet::Tools.pages().iter().all(|p| p.tool().is_some()));
        assert!(OptionsSet::App.pages().iter().all(|p| p.tool().is_none()));
    }

    #[test]
    fn every_page_belongs_to_one_dialog() {
        let sets = [
            OptionsSet::App,
            OptionsSet::Customization,
            OptionsSet::Tools,
            OptionsSet::Global,
            OptionsSet::Workspaces,
            OptionsSet::Document,
        ];
        let mut seen = Vec::new();
        for s in sets {
            for p in s.pages() {
                assert!(!seen.contains(p), "{p:?} twice");
                assert_eq!(p.set(), s);
                seen.push(*p);
            }
        }
        assert_eq!(OptionsPage::Rulers.set(), OptionsSet::Document);
        assert_eq!(OptionsPage::General.set(), OptionsSet::App);
    }

    #[test]
    fn cancel_puts_settings_and_document_back() {
        let mut app = App::headless();
        let snap = OptionsSnapshot::take(&app);
        app.settings.undo_levels = 3;
        app.show_grid = !app.show_grid;
        let mut meta = app.doc().metadata.clone();
        meta.fill_open_curves = true;
        app.run(Command::SetMetadata { metadata: meta });
        snap.restore(&mut app);
        assert_eq!(app.settings.undo_levels, 150);
        assert!(!app.doc().metadata.fill_open_curves);
    }

    #[test]
    fn every_page_draws() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        for set in [
            OptionsSet::App,
            OptionsSet::Customization,
            OptionsSet::Tools,
            OptionsSet::Global,
            OptionsSet::Workspaces,
            OptionsSet::Document,
        ] {
            for p in set.pages() {
                app.options_page = *p;
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1600.0, 1000.0),
                    )),
                    ..Default::default()
                };
                let mut out = ctx.run_ui(input, |ui| {
                    let mut close = false;
                    options_dialog(&mut app, ui.ctx(), &mut close);
                    assert!(!close, "{p:?} closed by itself");
                });
                out.textures_delta.clear();
            }
        }
        // Drawing a page never changes what it shows (fields clamp the
        // values they hold to their ranges).
        assert_eq!(app.star_sharpness, 0.5);
    }

    /// A frame of the whole window with `events` as its input.
    fn frame(ctx: &Context, app: &mut App, events: Vec<egui::Event>) {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 1000.0),
            )),
            events,
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| crate::ui::root(app, ui));
        out.textures_delta.clear();
    }

    fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    #[test]
    fn commands_take_the_keys_pressed_as_their_shortcut() {
        use crate::ui::dialogs::Dialog;
        use egui::{Key, Modifiers};
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        app.show_welcome = false;
        app.dialog = Dialog::Options;
        app.options_page = OptionsPage::Commands;
        frame(&ctx, &mut app, vec![]);
        assert!(app
            .shortcut_editor
            .entries
            .iter()
            .any(|e| e.key == "menu.file.export"));
        // Export waits for its keys; Ctrl+D, Duplicate's, is pressed.
        app.shortcut_editor.capture = Some("menu.file.export".into());
        frame(
            &ctx,
            &mut app,
            vec![key(Key::ControlLeft, Modifiers::COMMAND)],
        );
        assert!(app.shortcut_editor.capture.is_some(), "Ctrl alone waits");
        frame(&ctx, &mut app, vec![key(Key::D, Modifiers::COMMAND)]);
        assert_eq!(
            user_shortcut(&app, "menu.file.export").as_deref(),
            Some("Ctrl+D")
        );
        assert!(app.shortcut_editor.capture.is_none());
        let note = app.shortcut_editor.note.clone().unwrap_or_default();
        assert!(note.contains(&tr("menu.edit.duplicate")), "{note}");
        // Every key is a shortcut, Enter and Esc alone too; Esc taken so
        // does not close the options.
        app.shortcut_editor.capture = Some("menu.file.import".into());
        frame(&ctx, &mut app, vec![key(Key::Enter, Modifiers::NONE)]);
        assert_eq!(
            user_shortcut(&app, "menu.file.import").as_deref(),
            Some("Enter")
        );
        app.shortcut_editor.capture = Some("menu.file.import".into());
        frame(&ctx, &mut app, vec![key(Key::Escape, Modifiers::NONE)]);
        assert_eq!(
            user_shortcut(&app, "menu.file.import").as_deref(),
            Some("Esc")
        );
        assert!(app.shortcut_editor.capture.is_none());
        assert!(matches!(app.dialog, Dialog::Options));
        // A modifier tapped alone is one too.
        app.shortcut_editor.capture = Some("menu.file.import".into());
        frame(
            &ctx,
            &mut app,
            vec![
                key(Key::ShiftRight, Modifiers::SHIFT),
                egui::Event::Key {
                    key: Key::ShiftRight,
                    physical_key: None,
                    pressed: false,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
        assert_eq!(
            user_shortcut(&app, "menu.file.import").as_deref(),
            Some("RShift")
        );
        // Keys of the user's move to the command given them last.
        set_menu_shortcut(&mut app, "menu.file.import", "ctrl+d");
        assert!(user_shortcut(&app, "menu.file.export").is_none());
        assert!(app
            .shortcut_editor
            .note
            .as_deref()
            .is_some_and(|n| n.contains(&tr("menu.file.export").replace("...", ""))));
        // Another page, or no options, and nothing waits any more.
        app.shortcut_editor.capture = Some("menu.file.export".into());
        app.options_page = OptionsPage::General;
        frame(&ctx, &mut app, vec![]);
        assert!(app.shortcut_editor.capture.is_none());
        app.options_page = OptionsPage::Commands;
        app.shortcut_editor.capture = Some("menu.file.export".into());
        app.dialog = Dialog::None;
        frame(&ctx, &mut app, vec![key(Key::F7, Modifiers::COMMAND)]);
        assert!(app.shortcut_editor.capture.is_none());
        assert!(user_shortcut(&app, "menu.file.export").is_none());
    }
}
