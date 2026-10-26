//! Options > Global > Printing and File Formats, CD Draw > Bitmap Tracing
//! and Warnings
//! (see `prefs.rs` for what each choice does and docs/behavior/options.md).

use crate::app::App;
use crate::i18n::tr;
use crate::prefs::ImportPlace;
use crate::theme::Tokens;
use crate::ui::dialogs::{sheet_name, EXPORT_FORMATS, PDF_STANDARDS, SHEETS};
use crate::ui::options::{combo, grid, row_label};
use crate::ui::options_pages::{
    heading, note, off_button, off_check, off_combo, off_radio, sep, tabbed,
};
use egui::Ui;

fn hint(ui: &mut Ui, key: &str) {
    ui.label(egui::RichText::new(tr(key)).color(Tokens::text_dim()));
}

/// Options > Global > Printing: the default print style and what the
/// Print dialog starts with (tab Print Style), the printer driver
/// (tab Driver Compatibility).
pub fn printing_page(app: &mut App, ui: &mut Ui) {
    let labels = [tr("opt.print_style_tab"), tr("opt.driver_tab")];
    tabbed(ui, "printing", &labels, |ui, tab| {
        if tab == 1 {
            note(ui, &tr("opt.driver_hint"));
            off_check(ui, false, &tr("opt.driver_bitmap_fonts"));
            off_check(ui, false, &tr("opt.driver_no_gradients"));
            return;
        }
        let p = &mut app.settings.printing;
        grid("print_style_default").show(ui, |ui| {
            row_label(ui, &tr("opt.default_print_style"));
            off_combo(ui, "print_style_default", &tr("opt.app_defaults"), 160.0);
            ui.end_row();
        });
        off_check(ui, false, &tr("opt.print_current_page_only"));
        sep(ui);
        heading(ui, &tr("options.print_starts_with"));
        ui.checkbox(&mut p.fit_to_page, tr("dialog.fit_to_page"));
        ui.checkbox(&mut p.separations, tr("dialog.separations"));
        ui.checkbox(&mut p.bleed, tr("dialog.print_bleed"));
        ui.checkbox(&mut p.crop_marks, tr("dialog.crop_marks"));
        ui.checkbox(&mut p.registration_marks, tr("dialog.registration_marks"));
        ui.checkbox(&mut p.color_bars, tr("print.color_bars"));
        ui.checkbox(&mut p.densitometer, tr("print.densitometer"));
        ui.checkbox(&mut p.page_info, tr("print.page_info_mark"));
        grid("printing_layout").show(ui, |ui| {
            row_label(ui, &tr("print.sheet"));
            ui.horizontal(|ui| {
                let sheets: Vec<usize> = (0..SHEETS.len()).collect();
                if p.sheet >= SHEETS.len() {
                    p.sheet = 0;
                }
                combo(
                    ui,
                    "opt_print_sheet",
                    &mut p.sheet,
                    &sheets,
                    sheet_name,
                    130.0,
                );
                ui.add_enabled(
                    p.sheet > 0,
                    egui::Checkbox::new(&mut p.landscape, tr("print.landscape")),
                );
            });
            ui.end_row();
            row_label(ui, &tr("print.gutter"));
            ui.add(
                egui::DragValue::new(&mut p.gutter)
                    .range(0.0..=50.0)
                    .speed(0.5)
                    .suffix(" mm"),
            );
            ui.end_row();
        });
        ui.checkbox(&mut p.remember_last, tr("options.print_remember"));
        ui.checkbox(&mut p.preflight, tr("options.print_preflight"));
        sep(ui);
        heading(ui, &tr("opt.special_settings"));
        grid("print_special").show(ui, |ui| {
            for (key, value) in [
                ("opt.spot_separations_warning", "opt.more_than_two_spots"),
                ("opt.preview_color", "opt.auto_simulate"),
                ("opt.preview_separations", "opt.auto_simulate"),
                ("opt.preview_image", "opt.on_at_startup"),
                ("opt.print_bitmaps_in_chunks", "opt.no"),
            ] {
                row_label(ui, &tr(key));
                off_combo(ui, key, &tr(value), 120.0);
                ui.end_row();
            }
            for (key, value) in [
                ("opt.black_overprint_threshold", "95"),
                ("opt.bitmap_output_limit", "4096"),
                ("opt.small_fonts", "7"),
                ("opt.low_resolution_image", "96"),
            ] {
                row_label(ui, &tr(key));
                off_combo(ui, key, value, 60.0);
                ui.end_row();
            }
        });
    });
}

/// Options > Global > File Formats: the formats CD Draw reads by kind and
/// the active filters (fixed), then what File > Export starts with, where
/// imported objects go and the `.cdr` version new drawings are saved in.
pub fn file_formats_page(app: &mut App, ui: &mut Ui) {
    let kinds: [(&str, &[&str]); 4] = [
        (
            "opt.formats_raster",
            &[
                "png", "jpg", "jpeg", "bmp", "gif", "webp", "tif", "tiff", "psd", "psb",
            ],
        ),
        (
            "opt.formats_vector",
            &[
                "cdr", "cdt", "cdrt", "cmx", "svg", "svgz", "pdf", "ai", "eps", "ps", "prn", "dxf",
                "emf", "wmf", "plt", "hpgl", "hgl",
            ],
        ),
        ("opt.formats_text", &["txt", "rtf", "docx"]),
        ("opt.formats_animation", &["gif"]),
    ];
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(150.0);
            ui.label(tr("opt.available_types"));
            egui::Frame::new()
                .fill(Tokens::field())
                .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
                .inner_margin(4)
                .show(ui, |ui| {
                    ui.set_width(140.0);
                    ui.set_min_height(240.0);
                    for (key, exts) in kinds {
                        egui::CollapsingHeader::new(tr(key))
                            .id_salt(key)
                            .default_open(false)
                            .show(ui, |ui| {
                                for e in exts {
                                    ui.label(e.to_uppercase());
                                }
                            });
                    }
                });
        });
        ui.vertical(|ui| {
            ui.add_space(18.0);
            for key in ["opt.add", "opt.remove"] {
                off_button(ui, &tr(key));
            }
            ui.add_space(12.0);
            for key in ["opt.move_up", "opt.move_down"] {
                off_button(ui, &tr(key));
            }
        });
        ui.vertical(|ui| {
            ui.label(tr("opt.active_filters"));
            egui::Frame::new()
                .fill(Tokens::field())
                .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
                .inner_margin(4)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width().max(140.0));
                    egui::ScrollArea::vertical()
                        .id_salt("active_filters")
                        .max_height(240.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            for e in crate::app::IMPORT_EXTENSIONS {
                                ui.label(e.to_uppercase());
                            }
                        });
                });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                off_button(ui, &tr("opt.reset_to_defaults"));
            });
        });
    });
    sep(ui);
    let f = &mut app.settings.formats;
    heading(ui, &tr("options.export_starts_with"));
    if f.export_format >= EXPORT_FORMATS.len() {
        f.export_format = 0;
    }
    if f.pdf_standard >= PDF_STANDARDS.len() {
        f.pdf_standard = 0;
    }
    grid("formats_export").show(ui, |ui| {
        row_label(ui, &tr("dialog.format"));
        let formats: Vec<usize> = (0..EXPORT_FORMATS.len()).collect();
        combo(
            ui,
            "opt_export_format",
            &mut f.export_format,
            &formats,
            |i| EXPORT_FORMATS[i].0.to_string(),
            130.0,
        );
        ui.end_row();
        row_label(ui, &tr("dialog.resolution"));
        ui.add(
            egui::DragValue::new(&mut f.export_dpi)
                .range(crate::ui::dialogs::MIN_EXPORT_DPI..=2400.0)
                .suffix(" dpi"),
        );
        ui.end_row();
        row_label(ui, &tr("dialog.quality"));
        ui.add(egui::DragValue::new(&mut f.export_quality).range(1..=100));
        ui.end_row();
        row_label(ui, &tr("dialog.pdf_standard"));
        let standards: Vec<usize> = (0..PDF_STANDARDS.len()).collect();
        combo(
            ui,
            "opt_pdf_standard",
            &mut f.pdf_standard,
            &standards,
            |i| PDF_STANDARDS[i].to_string(),
            130.0,
        );
        ui.end_row();
        row_label(ui, &tr("dialog.output_condition"));
        ui.add(egui::TextEdit::singleline(&mut f.output_condition).desired_width(130.0));
        ui.end_row();
        row_label(ui, &tr("dialog.bleed"));
        ui.add(
            egui::DragValue::new(&mut f.pdf_bleed_mm)
                .range(0.0..=50.0)
                .speed(0.5)
                .suffix(" mm"),
        );
        ui.end_row();
    });
    ui.checkbox(
        &mut f.export_transparent,
        tr("dialog.transparent_background"),
    );
    ui.checkbox(&mut f.remember_last, tr("options.export_remember"));
    hint(ui, "options.export_presets_hint");
    sep(ui);
    heading(ui, &tr("options.import_section"));
    ui.label(tr("options.import_place"));
    ui.radio_value(
        &mut f.import_place,
        ImportPlace::AsInFile,
        tr("options.import_as_in_file"),
    );
    ui.radio_value(
        &mut f.import_place,
        ImportPlace::ViewCenter,
        tr("options.import_view_center"),
    );
    sep(ui);
    heading(ui, &tr("options.cdr_save_section"));
    f.cdr_version = f.cdr_version.clamp(
        tracedraw_cdr::write::CHECKED_SINCE,
        tracedraw_cdr::write::NEWEST_VERSION,
    );
    grid("formats_cdr").show(ui, |ui| {
        row_label(ui, &tr("dialog.cdr_version"));
        combo(
            ui,
            "opt_cdr_version",
            &mut f.cdr_version,
            &crate::ui::save_as::version_choices(),
            crate::ui::save_as::version_label,
            130.0,
        );
        ui.end_row();
    });
    hint(ui, "options.cdr_version_hint");
}

/// Options > CD Draw > Bitmap Tracing: Quick Trace's method, how colours
/// are merged and what every trace starts with.
pub fn bitmap_tracing_page(app: &mut App, ui: &mut Ui) {
    let t = &mut app.settings.tracing;
    grid("tracing").show(ui, |ui| {
        row_label(ui, &tr("opt.quick_trace_method"));
        combo(
            ui,
            "opt_quick_trace",
            &mut t.quick_preset,
            &crate::trace::Preset::ALL,
            // The menu's names, without the dots of a menu item.
            |p| {
                tr(p.key())
                    .trim_end_matches("...")
                    .trim_end_matches('\u{2026}')
                    .to_string()
            },
            170.0,
        );
        ui.end_row();
        row_label(ui, &tr("opt.merge_colors"));
        ui.vertical(|ui| {
            off_radio(ui, true, &tr("opt.merge_average"));
            off_radio(ui, false, &tr("opt.merge_first"));
        });
        ui.end_row();
    });
    sep(ui);
    heading(ui, &tr("options.trace_starts_with"));
    ui.checkbox(&mut t.remove_background, tr("dialog.remove_background"));
    ui.checkbox(&mut t.delete_original, tr("dialog.delete_original"));
    hint(ui, "options.trace_hint");
}

/// Options > CD Draw > Warnings: the messages that show, in a framed list.
pub fn warnings_page(app: &mut App, ui: &mut Ui) {
    let w = &mut app.settings.warnings;
    ui.label(tr("opt.warnings_show"));
    egui::Frame::new()
        .fill(Tokens::field())
        .stroke(egui::Stroke::new(1.0, Tokens::control_border()))
        .inner_margin(egui::Margin::same(6))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_min_height(250.0);
            ui.spacing_mut().item_spacing.y = 3.0;
            ui.checkbox(&mut w.missing_fonts, tr("options.warn_missing_fonts"));
            ui.checkbox(&mut w.unread_objects, tr("options.warn_unread_objects"));
            ui.checkbox(&mut w.shelf_remove, tr("options.warn_shelf_remove"));
        });
    ui.add_space(4.0);
    hint(ui, "options.warnings_hint");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every new page draws, with nonsense values put right.
    #[test]
    fn the_global_pages_draw() {
        let ctx = crate::theme::ui_context();
        let mut app = App::headless();
        app.settings.printing.sheet = 99;
        app.settings.formats.export_format = 99;
        app.settings.formats.pdf_standard = 42;
        for page in [
            printing_page as fn(&mut App, &mut Ui),
            file_formats_page,
            bitmap_tracing_page,
            warnings_page,
        ] {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| page(&mut app, ui));
            out.textures_delta.clear();
        }
        assert_eq!(app.settings.printing.sheet, 0);
        assert_eq!(app.settings.formats.export_format, 0);
        assert_eq!(app.settings.formats.pdf_standard, 0);
    }
}
