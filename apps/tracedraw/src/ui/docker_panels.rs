//! Functional integrations for the five panels in the owner's reference.

use crate::app::{App, DockerTab};
use crate::i18n::{tr, trf};
use crate::settings::SavedView;
use egui::{Id, Ui};
use std::path::PathBuf;
use tracedraw_core::geometry::Point;

pub(super) fn show(app: &mut App, ui: &mut Ui, tab: DockerTab) {
    ui.push_id(tab.key(), |ui| match tab {
        DockerTab::LiveGuides => {
            egui::CollapsingHeader::new(tr("reference_panels.alignment"))
                .default_open(true)
                .show(ui, |ui| crate::ui::options::alignment_guides_page(app, ui));
            egui::CollapsingHeader::new(tr("reference_panels.dynamic"))
                .default_open(true)
                .show(ui, |ui| crate::ui::options::dynamic_guides_page(app, ui));
        }
        DockerTab::FitPath => fit_path(app, ui),
        DockerTab::ColorProof => proof(app, ui),
        DockerTab::Assets => assets(app, ui),
        DockerTab::Views => views(app, ui),
        _ => {}
    });
}

fn fit_path(app: &mut App, ui: &mut Ui) {
    ui.label(tr("reference_panels.fit_hint"));
    let prefs = &mut app.settings.fit_path;
    ui.checkbox(&mut prefs.rotate, tr("reference_panels.rotate"));
    ui.checkbox(&mut prefs.reverse, tr("reference_panels.reverse"));
    ui.horizontal(|ui| {
        ui.label(tr("reference_panels.start"));
        ui.add(
            crate::ui::field::NumField::new(&mut prefs.start_percent)
                .range(0.0..=100.0)
                .suffix("%"),
        );
    });
    ui.horizontal(|ui| {
        ui.label(tr("reference_panels.end"));
        ui.add(
            crate::ui::field::NumField::new(&mut prefs.end_percent)
                .range(0.0..=100.0)
                .suffix("%"),
        );
    });
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                app.selection.len() >= 2,
                egui::Button::new(tr("docker.apply")),
            )
            .clicked()
        {
            app.fit_objects_to_path();
        }
        if ui.button(tr("options.reset")).clicked() {
            app.settings.fit_path = Default::default();
        }
    });
}

fn proof(app: &mut App, ui: &mut Ui) {
    let mut changed = ui
        .checkbox(&mut app.proof_colors, tr("menu.view.proof_colors"))
        .changed();
    changed |= ui
        .checkbox(
            &mut app.simulate_overprints,
            tr("menu.view.simulate_overprints"),
        )
        .changed();
    if changed {
        app.raster.borrow_mut().invalidate();
    }
    ui.separator();
    let md = &app.doc().metadata;
    for (key, profile) in [
        ("reference_panels.rgb", md.rgb_profile.as_str()),
        ("reference_panels.cmyk", md.cmyk_profile.as_str()),
        ("reference_panels.gray", md.gray_profile.as_str()),
        ("reference_panels.intent", md.rendering_intent.as_str()),
    ] {
        ui.label(tr(key));
        ui.label(if profile.is_empty() {
            tr("reference_panels.default_profile")
        } else {
            profile.to_string()
        });
    }
    if ui.button(tr("menu.tools.color_management")).clicked() {
        app.dialog = crate::ui::dialogs::Dialog::ColorManagement(Default::default());
    }
}

fn add_asset_paths(app: &mut App, paths: Vec<PathBuf>) {
    for path in paths {
        if !app.settings.asset_files.contains(&path) {
            app.settings.asset_files.push(path);
        }
    }
}

/// Local file references, separate from the Shelf's embedded object snippets.
fn assets(app: &mut App, ui: &mut Ui) {
    let search_id = Id::new("reference_assets_search");
    let selected_id = Id::new("reference_assets_selected");
    let mut search = ui.data_mut(|d| d.get_temp::<String>(search_id).unwrap_or_default());
    let selected = ui.data_mut(|d| d.get_temp::<PathBuf>(selected_id));
    ui.add(
        egui::TextEdit::singleline(&mut search)
            .hint_text(tr("reference_panels.search"))
            .desired_width(ui.available_width()),
    );
    ui.data_mut(|d| d.insert_temp(search_id, search.clone()));
    ui.horizontal_wrapped(|ui| {
        if ui.button(tr("reference_panels.add_files")).clicked() {
            crate::files::Dialog::new()
                .add_filter(
                    tr("reference_panels.asset_files"),
                    &[
                        "cdr", "svg", "svgz", "pdf", "ai", "png", "jpg", "jpeg", "bmp", "gif",
                        "webp", "tif", "tiff", "eps", "dxf",
                    ],
                )
                .pick_files(app, add_asset_paths);
        }
        if ui
            .add_enabled(
                selected.is_some() && app.has_document(),
                egui::Button::new(tr("reference_panels.insert")),
            )
            .clicked()
        {
            if let Some(path) = &selected {
                app.import_path(path);
            }
        }
        if ui
            .add_enabled(selected.is_some(), egui::Button::new(tr("shelf.remove")))
            .clicked()
        {
            if let Some(path) = &selected {
                app.settings.asset_files.retain(|p| p != path);
                ui.data_mut(|d| d.remove::<PathBuf>(selected_id));
            }
        }
    });
    let files = app.settings.asset_files.clone();
    let query = search.to_lowercase();
    for path in files {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());
        if !name.to_lowercase().contains(&query) {
            continue;
        }
        let response = ui
            .selectable_label(selected.as_ref() == Some(&path), name)
            .on_hover_text(path.to_string_lossy().into_owned());
        if response.clicked() {
            ui.data_mut(|d| d.insert_temp(selected_id, path.clone()));
        }
        if response.double_clicked() && app.has_document() {
            app.import_path(&path);
        }
    }
    if app.settings.asset_files.is_empty() {
        ui.label(tr("reference_panels.assets_empty"));
    }
}

fn save_view(app: &mut App, name: String) -> bool {
    let name = name.trim();
    if name.is_empty() || !app.has_document() || !app.view.zoom().is_finite() {
        return false;
    }
    let center = app.view.to_page(app.canvas_rect.center());
    if !center.x.is_finite() || !center.y.is_finite() {
        return false;
    }
    let saved = SavedView {
        name: name.to_string(),
        page: app.page,
        zoom: app.view.zoom(),
        center: [center.x, center.y],
    };
    if let Some(old) = app
        .settings
        .saved_views
        .iter_mut()
        .find(|v| v.page == saved.page && v.name == saved.name)
    {
        *old = saved;
    } else {
        app.settings.saved_views.push(saved);
    }
    true
}

fn restore_view(app: &mut App, saved: &SavedView) -> bool {
    if !saved.zoom.is_finite() || saved.zoom <= 0.0 || !saved.center.iter().all(|n| n.is_finite()) {
        return false;
    }
    let Some(index) = app.doc().pages.iter().position(|p| p.id == saved.page) else {
        return false;
    };
    app.goto_page(index);
    app.fit_pending = false;
    app.view.set_zoom(saved.zoom);
    let center = Point::new(saved.center[0], saved.center[1]);
    let at = app.view.to_screen(center);
    app.view.pan(app.canvas_rect.center() - at);
    true
}

fn views(app: &mut App, ui: &mut Ui) {
    let id = Id::new("reference_view_name");
    let mut name = ui.data_mut(|d| d.get_temp::<String>(id).unwrap_or_default());
    ui.add(
        egui::TextEdit::singleline(&mut name)
            .hint_text(tr("reference_panels.view_name"))
            .desired_width(ui.available_width()),
    );
    ui.data_mut(|d| d.insert_temp(id, name.clone()));
    if ui
        .add_enabled(
            app.has_document(),
            egui::Button::new(tr("reference_panels.save_view")),
        )
        .clicked()
    {
        if name.trim().is_empty() {
            name = trf(
                "reference_panels.view_number",
                &[("n", &(app.settings.saved_views.len() + 1).to_string())],
            );
        }
        save_view(app, name);
    }
    let saved = app.settings.saved_views.clone();
    for (index, view) in saved.iter().enumerate() {
        // Persistent page ids keep views attached to the correct document.
        if app.doc().page(view.page).is_err() {
            continue;
        }
        ui.horizontal(|ui| {
            if ui.button(&view.name).clicked() {
                restore_view(app, view);
            }
            if ui
                .small_button(tr("reference_panels.update_view"))
                .clicked()
            {
                let page = view.page;
                if page == app.page {
                    save_view(app, view.name.clone());
                }
            }
            if ui.small_button(tr("shelf.remove")).clicked() {
                // Only one removal can occur per frame, indices match the snapshot.
                if index < app.settings.saved_views.len() {
                    app.settings.saved_views.remove(index);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_are_deduplicated_without_changing_existing_order() {
        let mut app = App::headless();
        app.settings.asset_files.clear();
        let a = PathBuf::from("a.svg");
        let b = PathBuf::from("b.png");
        add_asset_paths(&mut app, vec![a.clone(), b.clone(), a.clone()]);
        assert_eq!(app.settings.asset_files, vec![a, b]);
    }

    #[test]
    fn a_saved_view_restores_zoom_and_center_and_round_trips_settings() {
        let mut app = App::headless();
        app.settings.saved_views.clear();
        app.view.set_zoom(7.0);
        app.canvas_rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
        let center = app.view.to_page(app.canvas_rect.center());
        assert!(save_view(&mut app, " Detail ".into()));
        let saved = app.settings.saved_views[0].clone();
        app.view.set_zoom(2.0);
        app.view.set_origin(egui::pos2(20.0, 40.0));
        assert!(restore_view(&mut app, &saved));
        let restored = app.view.to_page(app.canvas_rect.center());
        assert!((restored.x - center.x).abs() < 1e-4);
        assert!((restored.y - center.y).abs() < 1e-4);
        assert_eq!(app.view.zoom(), 7.0);
        let encoded = serde_json::to_string(&app.settings).expect("settings serialize");
        let decoded: crate::settings::Settings =
            serde_json::from_str(&encoded).expect("settings parse");
        assert_eq!(decoded.saved_views, app.settings.saved_views);
        let mut invalid = saved;
        invalid.zoom = f32::NAN;
        assert!(!restore_view(&mut app, &invalid));
    }
}
