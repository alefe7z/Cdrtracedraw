//! Approved reference artwork, isolated from labels and live field values.
use egui::{Color32, Painter, Pos2, Rect};

pub fn source(name: &str) -> Option<&'static str> {
    match name {
        "abrir" => Some(include_str!("../../../../assets/icons/reference-bar/abrir.svg")),
        "capitular" => Some(include_str!("../../../../assets/icons/reference-bar/capitular.svg")),
        "captura" => Some(include_str!("../../../../assets/icons/reference-bar/captura.svg")),
        "colar" => Some(include_str!("../../../../assets/icons/reference-bar/colar.svg")),
        "colunas" => Some(include_str!("../../../../assets/icons/reference-bar/colunas.svg")),
        "configuracoes" => Some(include_str!("../../../../assets/icons/reference-bar/configuracoes.svg")),
        "copiar" => Some(include_str!("../../../../assets/icons/reference-bar/copiar.svg")),
        "desfazer" => Some(include_str!("../../../../assets/icons/reference-bar/desfazer.svg")),
        "espelhar-horizontal" => Some(include_str!("../../../../assets/icons/reference-bar/espelhar-horizontal.svg")),
        "espelhar-vertical" => Some(include_str!("../../../../assets/icons/reference-bar/espelhar-vertical.svg")),
        "exportar" => Some(include_str!("../../../../assets/icons/reference-bar/exportar.svg")),
        "girar" => Some(include_str!("../../../../assets/icons/reference-bar/girar.svg")),
        "importar" => Some(include_str!("../../../../assets/icons/reference-bar/importar.svg")),
        "imprimir" => Some(include_str!("../../../../assets/icons/reference-bar/imprimir.svg")),
        "inicio" => Some(include_str!("../../../../assets/icons/reference-bar/inicio.svg")),
        "italico" => Some(include_str!("../../../../assets/icons/reference-bar/italico.svg")),
        "lista" => Some(include_str!("../../../../assets/icons/reference-bar/lista.svg")),
        "negrito" => Some(include_str!("../../../../assets/icons/reference-bar/negrito.svg")),
        "novo" => Some(include_str!("../../../../assets/icons/reference-bar/novo.svg")),
        "pdf" => Some(include_str!("../../../../assets/icons/reference-bar/pdf.svg")),
        "recortar" => Some(include_str!("../../../../assets/icons/reference-bar/recortar.svg")),
        "refazer" => Some(include_str!("../../../../assets/icons/reference-bar/refazer.svg")),
        "salvar" => Some(include_str!("../../../../assets/icons/reference-bar/salvar.svg")),
        "sublinhado" => Some(include_str!("../../../../assets/icons/reference-bar/sublinhado.svg")),
        "texto" => Some(include_str!("../../../../assets/icons/reference-bar/texto.svg")),
        "zoom-altura" => Some(include_str!("../../../../assets/icons/reference-bar/zoom-altura.svg")),
        "zoom-largura" => Some(include_str!("../../../../assets/icons/reference-bar/zoom-largura.svg")),
        "zoom-mais" => Some(include_str!("../../../../assets/icons/reference-bar/zoom-mais.svg")),
        "zoom-menos" => Some(include_str!("../../../../assets/icons/reference-bar/zoom-menos.svg")),
        "zoom-pagina" => Some(include_str!("../../../../assets/icons/reference-bar/zoom-pagina.svg")),
        "zoom-selecao" => Some(include_str!("../../../../assets/icons/reference-bar/zoom-selecao.svg")),
        "zoom-todos" => Some(include_str!("../../../../assets/icons/reference-bar/zoom-todos.svg")),
        _ => None,
    }
}

fn raster(name: &str, side: u32) -> Option<egui::ColorImage> {
    let imported = tracedraw_io::svg_import::parse(
        source(name)?, &mut tracedraw_core::id::IdSource::default(),
    ).ok()?;
    let zoom = f64::from(side) / imported.size.width;
    let mut doc = tracedraw_core::Document::new("Toolbar icon", imported.size);
    let page = doc.pages.first_mut()?;
    let page_id = page.id;
    page.layers.first_mut()?.shapes = imported.shapes;
    let pixels = tracedraw_render::render_page(&doc, page_id, &tracedraw_render::RenderOptions {
        width: side, height: side,
        view: tracedraw_render::ViewTransform { zoom, origin_x: 0.0, origin_y: f64::from(side) },
        ..Default::default()
    })?;
    Some(egui::ColorImage::from_rgba_premultiplied([side as usize; 2], pixels.data()))
}

/// Return false on failure so callers retain their original vector fallback.
/// Rasterize at physical display resolution; tint supplies theme and enabled state.
pub fn draw(painter: &Painter, rect: Rect, name: &'static str, color: Color32) -> bool {
    let ctx = painter.ctx();
    let side = (rect.width() * ctx.pixels_per_point()).ceil().clamp(8.0, 256.0) as u32;
    let key = egui::Id::new(("reference_bar_svg", name, side));
    let cached = ctx.data(|d| d.get_temp::<egui::TextureHandle>(key));
    let texture = match cached {
        Some(t) => t,
        None => {
            let Some(image) = raster(name, side) else { return false; };
            let t = ctx.load_texture(format!("reference_bar_{name}_{side}"), image, egui::TextureOptions::LINEAR);
            ctx.data_mut(|d| d.insert_temp(key, t.clone()));
            t
        }
    };
    painter.image(texture.id(), rect, Rect::from_min_max(Pos2::ZERO, egui::pos2(1.0, 1.0)), color);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_reference_icon_renders_with_transparent_background() {
        for name in ["abrir", "capitular", "captura", "colar", "colunas", "configuracoes", "copiar", "desfazer", "espelhar-horizontal", "espelhar-vertical", "exportar", "girar", "importar", "imprimir", "inicio", "italico", "lista", "negrito", "novo", "pdf", "recortar", "refazer", "salvar", "sublinhado", "texto", "zoom-altura", "zoom-largura", "zoom-mais", "zoom-menos", "zoom-pagina", "zoom-selecao", "zoom-todos"] {
            for side in [18, 24, 48] {
                let image = raster(name, side).expect(name);
                assert!(image.pixels.iter().any(|p| p.a() > 0), "{name}");
                assert!(image.pixels.iter().any(|p| p.a() == 0), "{name}");
            }
        }
    }
}
