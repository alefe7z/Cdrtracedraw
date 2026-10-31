//! Hand-drawn toolbar vectors, theme colours and cached SVG rasterization.
use egui::{Color32, Painter, Pos2, Rect};

#[cfg(test)]
pub(crate) const NAMES: &[&str] = &[
    "abrir",
    "capitular",
    "captura",
    "colar",
    "colunas",
    "configuracoes",
    "copiar",
    "desfazer",
    "espelhar-horizontal",
    "espelhar-vertical",
    "exportar",
    "girar",
    "importar",
    "imprimir",
    "inicio",
    "italico",
    "lista",
    "negrito",
    "novo",
    "pdf",
    "recortar",
    "refazer",
    "salvar",
    "sublinhado",
    "texto",
    "zoom-altura",
    "zoom-largura",
    "zoom-mais",
    "zoom-menos",
    "zoom-pagina",
    "zoom-selecao",
    "zoom-todos",
];

pub fn source(name: &str) -> Option<&'static str> {
    match name {
        "abrir" => Some(include_str!(
            "../../../../assets/icons/reference-bar/abrir.svg"
        )),
        "capitular" => Some(include_str!(
            "../../../../assets/icons/reference-bar/capitular.svg"
        )),
        "captura" => Some(include_str!(
            "../../../../assets/icons/reference-bar/captura.svg"
        )),
        "colar" => Some(include_str!(
            "../../../../assets/icons/reference-bar/colar.svg"
        )),
        "colunas" => Some(include_str!(
            "../../../../assets/icons/reference-bar/colunas.svg"
        )),
        "configuracoes" => Some(include_str!(
            "../../../../assets/icons/reference-bar/configuracoes.svg"
        )),
        "copiar" => Some(include_str!(
            "../../../../assets/icons/reference-bar/copiar.svg"
        )),
        "desfazer" => Some(include_str!(
            "../../../../assets/icons/reference-bar/desfazer.svg"
        )),
        "espelhar-horizontal" => Some(include_str!(
            "../../../../assets/icons/reference-bar/espelhar-horizontal.svg"
        )),
        "espelhar-vertical" => Some(include_str!(
            "../../../../assets/icons/reference-bar/espelhar-vertical.svg"
        )),
        "exportar" => Some(include_str!(
            "../../../../assets/icons/reference-bar/exportar.svg"
        )),
        "girar" => Some(include_str!(
            "../../../../assets/icons/reference-bar/girar.svg"
        )),
        "importar" => Some(include_str!(
            "../../../../assets/icons/reference-bar/importar.svg"
        )),
        "imprimir" => Some(include_str!(
            "../../../../assets/icons/reference-bar/imprimir.svg"
        )),
        "inicio" => Some(include_str!(
            "../../../../assets/icons/reference-bar/inicio.svg"
        )),
        "italico" => Some(include_str!(
            "../../../../assets/icons/reference-bar/italico.svg"
        )),
        "lista" => Some(include_str!(
            "../../../../assets/icons/reference-bar/lista.svg"
        )),
        "negrito" => Some(include_str!(
            "../../../../assets/icons/reference-bar/negrito.svg"
        )),
        "novo" => Some(include_str!(
            "../../../../assets/icons/reference-bar/novo.svg"
        )),
        "pdf" => Some(include_str!(
            "../../../../assets/icons/reference-bar/pdf.svg"
        )),
        "recortar" => Some(include_str!(
            "../../../../assets/icons/reference-bar/recortar.svg"
        )),
        "refazer" => Some(include_str!(
            "../../../../assets/icons/reference-bar/refazer.svg"
        )),
        "salvar" => Some(include_str!(
            "../../../../assets/icons/reference-bar/salvar.svg"
        )),
        "sublinhado" => Some(include_str!(
            "../../../../assets/icons/reference-bar/sublinhado.svg"
        )),
        "texto" => Some(include_str!(
            "../../../../assets/icons/reference-bar/texto.svg"
        )),
        "zoom-altura" => Some(include_str!(
            "../../../../assets/icons/reference-bar/zoom-altura.svg"
        )),
        "zoom-largura" => Some(include_str!(
            "../../../../assets/icons/reference-bar/zoom-largura.svg"
        )),
        "zoom-mais" => Some(include_str!(
            "../../../../assets/icons/reference-bar/zoom-mais.svg"
        )),
        "zoom-menos" => Some(include_str!(
            "../../../../assets/icons/reference-bar/zoom-menos.svg"
        )),
        "zoom-pagina" => Some(include_str!(
            "../../../../assets/icons/reference-bar/zoom-pagina.svg"
        )),
        "zoom-selecao" => Some(include_str!(
            "../../../../assets/icons/reference-bar/zoom-selecao.svg"
        )),
        "zoom-todos" => Some(include_str!(
            "../../../../assets/icons/reference-bar/zoom-todos.svg"
        )),
        _ => None,
    }
}

fn raster(name: &str, side: u32, color: Color32, accents: bool) -> Option<egui::ColorImage> {
    let [r, g, b, _] = color.to_srgba_unmultiplied();
    let plain = format!("#{r:02x}{g:02x}{b:02x}");
    let svg = source(name)?
        .replace("#ffffff", &plain)
        .replace("#00ff00", if accents { "#1e9e3c" } else { &plain })
        .replace("#00ffff", if accents { "#149abd" } else { &plain })
        .replace("#ff0000", if accents { "#cf5950" } else { &plain });
    let imported =
        tracedraw_io::svg_import::parse(&svg, &mut tracedraw_core::id::IdSource::default()).ok()?;
    let zoom = f64::from(side) / imported.size.width;
    let mut doc = tracedraw_core::Document::new("Toolbar icon", imported.size);
    let page = doc.pages.first_mut()?;
    let page_id = page.id;
    page.layers.first_mut()?.shapes = imported.shapes;
    let pixels = tracedraw_render::render_page(
        &doc,
        page_id,
        &tracedraw_render::RenderOptions {
            width: side,
            height: side,
            view: tracedraw_render::ViewTransform {
                zoom,
                origin_x: 0.0,
                origin_y: f64::from(side),
            },
            ..Default::default()
        },
    )?;
    Some(egui::ColorImage::from_rgba_premultiplied(
        [side as usize; 2],
        pixels.data(),
    ))
}

/// Cache both a successfully loaded texture and a failed load.
/// Keeping None prevents a malformed resource being parsed on every frame.
pub(super) fn cached_texture(
    ctx: &egui::Context,
    key: egui::Id,
    load: impl FnOnce() -> Option<egui::TextureHandle>,
) -> Option<egui::TextureHandle> {
    if let Some(cached) = ctx.data(|d| d.get_temp::<Option<egui::TextureHandle>>(key)) {
        return cached;
    }
    let texture = load();
    ctx.data_mut(|d| d.insert_temp(key, texture.clone()));
    texture
}

/// Render at physical display resolution, with optional accents only when enabled.
pub fn draw(painter: &Painter, rect: Rect, name: &'static str, color: Color32) -> bool {
    let ctx = painter.ctx();
    let side = (rect.width() * ctx.pixels_per_point())
        .ceil()
        .clamp(8.0, 256.0) as u32;
    let accents = super::icons::colored() && color == crate::theme::Tokens::icon();
    let key = egui::Id::new(("reference_bar_vector_v2", name, side, color, accents));
    let texture = cached_texture(ctx, key, || {
        raster(name, side, color, accents).map(|image| {
            ctx.load_texture(
                format!("reference_bar_{name}_{side}"),
                image,
                egui::TextureOptions::LINEAR,
            )
        })
    });
    let Some(texture) = texture else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(Pos2::ZERO, egui::pos2(1.0, 1.0)),
        Color32::from_white_alpha(color.a()),
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_reference_icon_renders_with_transparent_background() {
        for name in NAMES {
            for side in [16, 24, 48] {
                let image = raster(name, side, Color32::WHITE, false).expect(name);
                assert!(image.pixels.iter().any(|p| p.a() > 0), "{name}");
                assert!(image.pixels.iter().any(|p| p.a() == 0), "{name}");
            }
        }
    }
    #[test]
    fn failed_rasterization_is_cached() {
        let ctx = egui::Context::default();
        let key = egui::Id::new("missing_icon");
        let mut attempts = 0;
        for _ in 0..4 {
            assert!(cached_texture(&ctx, key, || {
                attempts += 1;
                None
            })
            .is_none());
        }
        assert_eq!(attempts, 1);
    }

    #[test]
    fn theme_tint_and_optional_accents_reach_the_rendered_pixels() {
        let plain = Color32::from_rgb(80, 80, 80);
        for name in ["novo", "pdf", "lista", "captura", "colunas"] {
            let mono = raster(name, 48, plain, false).expect(name);
            assert!(
                mono.pixels
                    .iter()
                    .filter(|p| p.a() > 0)
                    .all(|p| p.r() == p.g() && p.g() == p.b()),
                "{name}"
            );
            let colored = raster(name, 48, plain, true).expect(name);
            assert!(
                colored.pixels.iter().any(|p| p.a() > 0 && p.r() != p.g()),
                "{name}"
            );
            assert_ne!(mono.pixels, colored.pixels, "{name}");
        }
        assert!(raster("not_an_icon", 24, plain, false).is_none());
    }
}
