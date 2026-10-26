//! Mapping between page space (mm, Y up, origin bottom-left) and screen
//! space (pixels, Y down).

use egui::{Pos2, Rect as ERect, Vec2};
use tracedraw_core::geometry::{Affine, Point, Rect};

/// The closest the view zooms in, pixels per millimetre: about 26 trillion
/// percent. Beyond it a millimetre's double-precision coordinates no
/// longer tell neighbouring pixels apart, so this is as deep as zooming
/// can show anything new.
pub const MAX_ZOOM: f64 = 1.0e12;
/// The farthest the view zooms out, pixels per millimetre: a kilometre is
/// a millionth of a pixel. Far enough that the whole drawing is a dot.
pub const MIN_ZOOM: f64 = 1.0e-9;

/// The view onto the page: its zoom and where the page origin sits on
/// screen, kept in double precision so it zooms in and out without limit
/// in practice (see [`MAX_ZOOM`], [`MIN_ZOOM`]) and pans anywhere.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct View {
    /// Pixels per millimetre.
    zoom: f64,
    /// Screen position of the page origin (bottom-left corner).
    ox: f64,
    oy: f64,
}

impl Default for View {
    fn default() -> Self {
        View::new(3.0, Pos2::new(100.0, 800.0))
    }
}

impl View {
    /// A view at `zoom` pixels per mm with the page origin at `origin`.
    pub fn new(zoom: f32, origin: Pos2) -> Self {
        let mut v = View {
            zoom: 3.0,
            ox: origin.x as f64,
            oy: origin.y as f64,
        };
        v.set_zoom64(zoom as f64);
        v
    }

    /// A view at `zoom` pixels per mm with the page origin at (`ox`, `oy`)
    /// on screen, in full precision.
    pub fn new64(zoom: f64, ox: f64, oy: f64) -> Self {
        let mut v = View { zoom: 3.0, ox, oy };
        v.set_zoom64(zoom);
        v
    }

    /// Pixels per millimetre.
    pub fn zoom(&self) -> f32 {
        self.zoom as f32
    }

    /// Pixels per millimetre, in full precision.
    pub fn zoom64(&self) -> f64 {
        self.zoom
    }

    /// Set the zoom (kept within [`MIN_ZOOM`] and [`MAX_ZOOM`]; anything
    /// that is not a positive number leaves it as it was). The origin
    /// stays.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.set_zoom64(zoom as f64);
    }

    pub fn set_zoom64(&mut self, zoom: f64) {
        if zoom.is_finite() && zoom > 0.0 {
            self.zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        }
    }

    /// Screen position of the page origin.
    #[cfg(test)]
    pub fn origin(&self) -> Pos2 {
        Pos2::new(self.ox as f32, self.oy as f32)
    }

    /// The page origin's screen position in full precision.
    pub fn origin64(&self) -> (f64, f64) {
        (self.ox, self.oy)
    }

    pub fn set_origin64(&mut self, ox: f64, oy: f64) {
        self.ox = ox;
        self.oy = oy;
    }

    #[cfg(test)]
    pub fn set_origin(&mut self, origin: Pos2) {
        self.ox = origin.x as f64;
        self.oy = origin.y as f64;
    }

    /// Page to screen, as an affine map in full precision.
    pub fn affine(&self) -> Affine {
        Affine::new([self.zoom, 0.0, 0.0, -self.zoom, self.ox, self.oy])
    }

    pub fn to_screen(&self, p: Point) -> Pos2 {
        Pos2::new(
            (self.ox + p.x * self.zoom) as f32,
            (self.oy - p.y * self.zoom) as f32,
        )
    }

    pub fn to_page(&self, s: Pos2) -> Point {
        Point::new(
            (s.x as f64 - self.ox) / self.zoom,
            (self.oy - s.y as f64) / self.zoom,
        )
    }

    /// A page rectangle on screen. Its sides are kept within a few
    /// million points of the screen: at an enormous zoom a page edge can
    /// lie farther away than single precision places, and a side that far
    /// off is off screen either way.
    pub fn rect_to_screen(&self, r: Rect) -> ERect {
        const FAR: f64 = 4.0e6;
        let x = |v: f64| (self.ox + v * self.zoom).clamp(-FAR, FAR) as f32;
        let y = |v: f64| (self.oy - v * self.zoom).clamp(-FAR, FAR) as f32;
        ERect::from_min_max(Pos2::new(x(r.x0), y(r.y1)), Pos2::new(x(r.x1), y(r.y0)))
    }

    /// Zoom around a screen point, keeping it fixed.
    pub fn zoom_at(&mut self, anchor: Pos2, factor: f32) {
        self.zoom_at64(anchor, factor as f64);
    }

    pub fn zoom_at64(&mut self, anchor: Pos2, factor: f64) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let page = self.to_page(anchor);
        self.set_zoom64(self.zoom * factor);
        self.ox = anchor.x as f64 - page.x * self.zoom;
        self.oy = anchor.y as f64 + page.y * self.zoom;
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.ox += delta.x as f64;
        self.oy += delta.y as f64;
    }

    /// Page-space rectangle currently visible in the canvas.
    pub fn visible_page_rect(&self, screen: ERect) -> Rect {
        let a = self.to_page(screen.min);
        let b = self.to_page(screen.max);
        Rect::from_points(a, b)
    }

    /// Fit a page-space rectangle (the page, the drawing, the selection or
    /// a zoom box anywhere on the page) into the screen rect with a margin,
    /// centred.
    pub fn fit(&mut self, r: Rect, screen: ERect) {
        let margin = 40.0;
        let w = r.width().max(1e-300);
        let h = r.height().max(1e-300);
        let zx = ((screen.width() - 2.0 * margin).max(1.0) as f64) / w;
        let zy = ((screen.height() - 2.0 * margin).max(1.0) as f64) / h;
        self.set_zoom64(zx.min(zy));
        let c = r.center();
        self.ox = screen.center().x as f64 - c.x * self.zoom;
        self.oy = screen.center().y as f64 + c.y * self.zoom;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A rectangle away from the page origin ends up centred on screen:
    /// zoom to fit, to the selection and the zoom box all rely on this.
    #[test]
    fn fit_centres_rectangles_anywhere_on_the_page() {
        let screen = ERect::from_min_size(Pos2::new(50.0, 80.0), Vec2::new(800.0, 600.0));
        let mut v = View::default();
        let r = Rect::new(41.6, 168.6, 124.7, 226.8);
        v.fit(r, screen);
        let c = v.to_screen(r.center());
        assert!((c.x - screen.center().x).abs() < 0.01, "{c:?}");
        assert!((c.y - screen.center().y).abs() < 0.01, "{c:?}");
        let s = v.rect_to_screen(r);
        assert!(screen.contains_rect(s), "{s:?} inside {screen:?}");
        // The whole page as before: centred too.
        v.fit(Rect::new(0.0, 0.0, 210.0, 297.0), screen);
        let pc = v.to_screen(Point::new(105.0, 148.5));
        assert!((pc - screen.center()).length() < 0.01);
    }

    #[test]
    fn zoom_at_keeps_the_anchor_fixed() {
        let mut v = View::default();
        let anchor = Pos2::new(400.0, 300.0);
        let before = v.to_page(anchor);
        v.zoom_at(anchor, 3.0);
        let after = v.to_page(anchor);
        assert!((before - after).hypot() < 1e-3);
    }

    /// Zooming in by a factor of ten a hundred times, then out again, comes
    /// back to the same view: the zoom is limited only by the precision of
    /// the numbers, and the point under the pointer stays put all the way.
    #[test]
    fn the_zoom_goes_deep_in_and_far_out_and_back() {
        let mut v = View::default();
        let anchor = Pos2::new(400.0, 300.0);
        let start = v.to_page(anchor);
        for _ in 0..12 {
            v.zoom_at(anchor, 10.0);
            let p = v.to_page(anchor);
            assert!((p - start).hypot() < 1e-9, "{p:?}");
        }
        assert!(v.zoom64() > 1.0e11, "{}", v.zoom64());
        // Two neighbouring pixels are two different places a trillionth
        // of a millimetre apart.
        let a = v.to_page(anchor);
        let b = v.to_page(anchor + Vec2::new(1.0, 0.0));
        assert!(b.x > a.x && b.x - a.x < 1e-11);
        for _ in 0..30 {
            v.zoom_at(anchor, 0.1);
        }
        assert_eq!(v.zoom64(), MIN_ZOOM);
        assert!((v.to_page(anchor) - start).hypot() < 1e-3);
        // The limits hold, and nonsense leaves the zoom alone.
        v.set_zoom64(1.0e300);
        assert_eq!(v.zoom64(), MAX_ZOOM);
        v.set_zoom(f32::NAN);
        v.zoom_at(anchor, 0.0);
        assert_eq!(v.zoom64(), MAX_ZOOM);
        // The page seen from inside: its rectangle still covers the screen.
        let page = v.rect_to_screen(Rect::new(-1.0e3, -1.0e3, 1.0e3, 1.0e3));
        assert!(page.contains(anchor) && page.width() > 1.0e6, "{page:?}");
    }
}
