//! Cutting a path down to a rectangle, so that a path far larger than the
//! screen (an object seen at an enormous zoom) can be filled and stroked
//! with what lies near the screen only.
//!
//! The parts of the path that stay inside the rectangle are kept as they
//! are, curves included. A curve that crosses its edge is split until the
//! pieces are either inside, outside or short and flat enough to be taken
//! as lines; what lies outside becomes straight lines, and those lines are
//! then cut to the rectangle (Sutherland and Hodgman's method, one edge of
//! the rectangle at a time). Seen inside the rectangle the result fills
//! with the same winding as the path did, under either fill rule, and its
//! outline is the path's outline wherever the rectangle is not crossed.

use crate::geometry::{BezPath, PathEl, Point, Rect};

/// How many times a curve crossing the rectangle's edge may be halved.
const MAX_DEPTH: u32 = 40;

/// One drawn piece of a subpath, from the point before it to its end.
#[derive(Debug, Clone, Copy)]
enum Piece {
    Line(Point),
    Quad(Point, Point),
    Cubic(Point, Point, Point),
}

impl Piece {
    fn end(self) -> Point {
        match self {
            Piece::Line(p) | Piece::Quad(_, p) | Piece::Cubic(_, _, p) => p,
        }
    }
}

/// `path` cut down to `clip`: the same as `path` wherever `clip` reaches.
/// A path already inside `clip` comes back unchanged.
pub fn clip_path_to_rect(path: &BezPath, clip: Rect) -> BezPath {
    let b = path.control_box();
    if b.x0 >= clip.x0 && b.y0 >= clip.y0 && b.x1 <= clip.x1 && b.y1 <= clip.y1 {
        return path.clone();
    }
    let mut out = BezPath::new();
    let mut start: Option<Point> = None;
    let mut pieces: Vec<Piece> = Vec::new();
    let mut closed = false;
    let flush = |start: Option<Point>, pieces: &mut Vec<Piece>, closed: bool, out: &mut BezPath| {
        if let Some(s) = start {
            emit(out, s, std::mem::take(pieces), closed, clip);
        }
        pieces.clear();
    };
    let mut last = Point::ZERO;
    for el in path.elements() {
        match *el {
            PathEl::MoveTo(p) => {
                flush(start, &mut pieces, closed, &mut out);
                start = Some(p);
                closed = false;
                last = p;
            }
            PathEl::LineTo(p) => {
                start.get_or_insert(last);
                pieces.push(Piece::Line(p));
                last = p;
            }
            PathEl::QuadTo(c, p) => {
                start.get_or_insert(last);
                split(&mut pieces, last, Piece::Quad(c, p), clip, 0);
                last = p;
            }
            PathEl::CurveTo(c1, c2, p) => {
                start.get_or_insert(last);
                split(&mut pieces, last, Piece::Cubic(c1, c2, p), clip, 0);
                last = p;
            }
            PathEl::ClosePath => {
                closed = true;
                flush(start, &mut pieces, closed, &mut out);
                start = None;
                closed = false;
            }
        }
    }
    flush(start, &mut pieces, closed, &mut out);
    out
}

fn inside(r: Rect, p: Point) -> bool {
    p.x >= r.x0 && p.x <= r.x1 && p.y >= r.y0 && p.y <= r.y1
}

/// Keep `piece` (starting at `from`) when its control points are all
/// inside `clip`, take it as a line when they are all on one side of it
/// or it cannot be halved any more, else halve it and look again.
fn split(out: &mut Vec<Piece>, from: Point, piece: Piece, clip: Rect, depth: u32) {
    let pts: &[Point] = &match piece {
        Piece::Line(p) => [from, p, p, p],
        Piece::Quad(c, p) => [from, c, p, p],
        Piece::Cubic(c1, c2, p) => [from, c1, c2, p],
    };
    if pts.iter().all(|p| inside(clip, *p)) {
        out.push(piece);
        return;
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for p in pts {
        x0 = x0.min(p.x);
        y0 = y0.min(p.y);
        x1 = x1.max(p.x);
        y1 = y1.max(p.y);
    }
    let apart = x1 < clip.x0 || x0 > clip.x1 || y1 < clip.y0 || y0 > clip.y1;
    if apart || depth >= MAX_DEPTH || matches!(piece, Piece::Line(_)) {
        out.push(Piece::Line(piece.end()));
        return;
    }
    let mid = |a: Point, b: Point| a.midpoint(b);
    match piece {
        Piece::Quad(c, p) => {
            let (a, b) = (mid(from, c), mid(c, p));
            let m = mid(a, b);
            split(out, from, Piece::Quad(a, m), clip, depth + 1);
            split(out, m, Piece::Quad(b, p), clip, depth + 1);
        }
        Piece::Cubic(c1, c2, p) => {
            let (a, b, c) = (mid(from, c1), mid(c1, c2), mid(c2, p));
            let (d, e) = (mid(a, b), mid(b, c));
            let m = mid(d, e);
            split(out, from, Piece::Cubic(a, d, m), clip, depth + 1);
            split(out, m, Piece::Cubic(e, c, p), clip, depth + 1);
        }
        Piece::Line(_) => {}
    }
}

/// One edge of the rectangle: which side of it is inside.
#[derive(Clone, Copy)]
enum Edge {
    Left(f64),
    Right(f64),
    Bottom(f64),
    Top(f64),
}

impl Edge {
    fn keeps(self, p: Point) -> bool {
        match self {
            Edge::Left(x) => p.x >= x,
            Edge::Right(x) => p.x <= x,
            Edge::Bottom(y) => p.y >= y,
            Edge::Top(y) => p.y <= y,
        }
    }

    /// Where the line from `a` to `b` crosses the edge.
    fn cross(self, a: Point, b: Point) -> Point {
        let t = |va: f64, vb: f64, at: f64| {
            if (vb - va).abs() < f64::MIN_POSITIVE {
                0.0
            } else {
                ((at - va) / (vb - va)).clamp(0.0, 1.0)
            }
        };
        match self {
            Edge::Left(x) | Edge::Right(x) => {
                let k = t(a.x, b.x, x);
                Point::new(x, a.y + (b.y - a.y) * k)
            }
            Edge::Bottom(y) | Edge::Top(y) => {
                let k = t(a.y, b.y, y);
                Point::new(a.x + (b.x - a.x) * k, y)
            }
        }
    }
}

/// Cut one subpath (`start`, then `pieces`) to `clip` and add it to `out`.
fn emit(out: &mut BezPath, start: Point, pieces: Vec<Piece>, closed: bool, clip: Rect) {
    let mut s = start;
    let mut list = pieces;
    for edge in [
        Edge::Left(clip.x0),
        Edge::Right(clip.x1),
        Edge::Bottom(clip.y0),
        Edge::Top(clip.y1),
    ] {
        // A closed subpath is cut with its closing side included.
        let mut work = list.clone();
        let last = work.last().map_or(s, |p| p.end());
        if closed && last != s {
            work.push(Piece::Line(s));
        }
        let mut next: Vec<Piece> = Vec::with_capacity(work.len() + 4);
        let mut new_start: Option<Point> = None;
        let mut prev = s;
        let put = |p: Piece, next: &mut Vec<Piece>, new_start: &mut Option<Point>| {
            if new_start.is_none() {
                *new_start = Some(p.end());
            } else {
                next.push(p);
            }
        };
        if edge.keeps(s) {
            new_start = Some(s);
        }
        for piece in work {
            let e = piece.end();
            match (edge.keeps(prev), edge.keeps(e)) {
                // Curves only ever lie wholly inside.
                (true, true) => put(piece, &mut next, &mut new_start),
                (true, false) => put(Piece::Line(edge.cross(prev, e)), &mut next, &mut new_start),
                (false, true) => {
                    put(Piece::Line(edge.cross(prev, e)), &mut next, &mut new_start);
                    put(Piece::Line(e), &mut next, &mut new_start);
                }
                (false, false) => {}
            }
            prev = e;
        }
        let Some(ns) = new_start else {
            return;
        };
        // The closing side is part of the list now; drop it when it only
        // leads back to the start, so the subpath closes on its own.
        if closed
            && next
                .last()
                .is_some_and(|p| matches!(p, Piece::Line(q) if *q == ns))
        {
            next.pop();
        }
        s = ns;
        list = next;
    }
    if list.is_empty() && !closed {
        return;
    }
    out.move_to(s);
    for p in list {
        match p {
            Piece::Line(q) => out.line_to(q),
            Piece::Quad(c, q) => out.quad_to(c, q),
            Piece::Cubic(c1, c2, q) => out.curve_to(c1, c2, q),
        }
    }
    if closed {
        out.close_path();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{ellipse_path, Shape as _};

    fn winding_at(path: &BezPath, p: Point) -> i32 {
        path.winding(p)
    }

    #[test]
    fn a_path_inside_comes_back_as_it_was() {
        let p = ellipse_path(Rect::new(10.0, 10.0, 50.0, 30.0));
        let c = clip_path_to_rect(&p, Rect::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(c, p);
    }

    /// A circle a billion times larger than the window: what is left is a
    /// handful of pieces, it fills the window where the circle does and
    /// leaves it empty outside, and every point stays near the window.
    #[test]
    fn a_huge_circle_is_cut_to_the_window_and_fills_the_same() {
        let r = 1.0e12;
        // The circle's edge runs through the window's middle.
        let circle = ellipse_path(Rect::new(-r, 500.0 - 2.0 * r, r, 500.0));
        let window = Rect::new(0.0, 0.0, 1000.0, 1000.0);
        let clip = window.inflate(1000.0, 1000.0);
        let c = clip_path_to_rect(&circle, clip);
        assert!(c.elements().len() < 400, "{}", c.elements().len());
        let b = c.control_box();
        assert!(b.x0 >= clip.x0 - 1e-6 && b.x1 <= clip.x1 + 1e-6, "{b:?}");
        assert!(b.y0 >= clip.y0 - 1e-6 && b.y1 <= clip.y1 + 1e-6, "{b:?}");
        for (p, want) in [
            (Point::new(500.0, 100.0), true),
            (Point::new(500.0, 900.0), false),
            (Point::new(10.0, 490.0), true),
            (Point::new(990.0, 510.0), false),
        ] {
            assert_eq!(winding_at(&c, p) != 0, want, "{p:?}");
            assert_eq!(winding_at(&circle, p) != 0, want, "{p:?}");
        }
    }

    /// An open curve crossing the window keeps its part inside; a path
    /// wholly outside leaves nothing.
    #[test]
    fn open_paths_and_paths_outside() {
        let mut p = BezPath::new();
        p.move_to((-1.0e9, 50.0));
        p.curve_to((-1.0e8, 50.0), (1.0e8, 50.0), (1.0e9, 50.0));
        let window = Rect::new(0.0, 0.0, 100.0, 100.0);
        let c = clip_path_to_rect(&p, window);
        let b = c.control_box();
        assert!((b.y0 - 50.0).abs() < 1e-6 && (b.y1 - 50.0).abs() < 1e-6);
        assert!(b.x0 <= 0.0 + 1e-6 && b.x1 >= 100.0 - 1e-6);
        let far = ellipse_path(Rect::new(1.0e6, 1.0e6, 1.0e6 + 5.0, 1.0e6 + 5.0));
        assert!(clip_path_to_rect(&far, window).elements().is_empty());
    }
}
