use crate::geometry::{point, Point};

use super::Vector;

const GRAPH_COLOUR: [u8; 4] = [255, 255, 255, 255];
const BOX_COLOUR: [u8; 4] = [10, 10, 240, 255];
const PATH_COLOUR: [u8; 4] = [10, 240, 10, 255];

#[derive(Default, Debug, PartialEq, Clone, Copy, PartialOrd)]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl Line {
    // pub fn intersects(&self, other: &Self) -> Option<Point> {
    //     let det = self.a * self.b - other.b * self.a;
    //
    //     if det.abs() < f64::EPSILON {
    //         return None;
    //     }
    //
    //     let x = (self.c * other.b - other.c * self.b) / det;
    //     let y = (self.a * other.c - other.a * self.c) / det;
    //
    //     return Some(point(x, y));
    // }
}
pub fn line(start: Point, end: Point) -> Line {
    let a = end.y - start.y;
    let b = start.x - end.x;
    let c = (end.x * start.y) - (start.x * end.y);
    Line { a, b, c }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}
impl LineSegment {
    pub fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        let n: f64 = diag_dist(self.start, self.end);
        for step in 0..n as usize {
            let t = if n == 0.0 { 0.0 } else { step as f64 / n };
            let p = lerp_point(self.start, self.end, t);
            points.push(point(p.x.round(), p.y.round()))
        }

        return points;
    }
    pub fn intersects(&self, other: &Self) -> bool {
        let cmp = other.start - self.start;
        let r = self.end - self.start;
        let s = other.end - other.start;

        let cmpxr = cmp.cross(r);
        let cmpxs = cmp.cross(s);
        let rxs = r.cross(s);

        if cmpxr.abs() < f64::EPSILON {
            return ((other.start.x - self.start.x < 0.0) != (other.start.x - self.end.x < 0.0))
                || ((other.start.y - self.start.y < 0.0) != (other.start.y - self.end.y < 0.0));
        }

        if rxs.abs() < f64::EPSILON {
            return false; // Lines are parallel.
        }

        let rxsr = 1.0 / rxs;
        let t = cmpxs * rxsr;
        let u = cmpxr * rxsr;

        return (t >= 0.0) && (t <= 1.0) && (u >= 0.0) && (u <= 1.0);
    }
    pub fn intersects2(&self, other: &Self) -> Option<Point> {
        let ov = other.end - other.start;
        let sv = self.end - self.start;
        let denom = (ov.y * sv.x) - (ov.x * sv.y);
        let d = (sv.y * ov.x) - (sv.x * ov.y);

        if denom.abs() < f64::EPSILON || d.abs() < f64::EPSILON {
            return None;
        }

        let nume_a =
            (ov.x * (self.start.y - other.start.y)) - (ov.y * (self.start.x - other.start.x));
        let nume_b =
            (sv.x * (other.start.y - self.start.y)) - (sv.y * (other.start.x - self.start.x));

        let ua = nume_a / denom;
        let ub = nume_b / d;

        if ua < 0.0 || ua > 1.0 || ub < 0.0 || ub > 1.0 {
            return None;
        }

        let p = self.start + sv * ua;
        return Some(p);
    }
    pub fn intersects3(&self, ls: &LineSegment) -> bool {
        let first_line = line(self.start, self.end);
        let d1 = (first_line.a * ls.start.x) + (first_line.b * ls.start.y) + first_line.c;
        let d2 = (first_line.a * ls.end.x) + (first_line.b * ls.end.y) + first_line.c;
        if (d1 > 0.0 && d2 > 0.0) || (d1 < 0.0 && d2 < 0.0) {
            return false;
        }

        let other_line = line(ls.start, ls.end);
        let d1 = (other_line.a * self.start.x) + (other_line.b * self.start.y) + other_line.c;
        let d2 = (other_line.a * self.end.x) + (other_line.b * self.end.y) + other_line.c;
        if (d1 > 0.0 && d2 > 0.0) || (d1 < 0.0 && d2 < 0.0) {
            return false;
        }

        if (first_line.a * other_line.b) - (other_line.a * first_line.b) == 0.0 {
            return false;
        }
        return true;
    }
    pub fn crosses(&self, other: &Self) -> bool {
        let denominator = ((self.end.x - self.start.x) * (other.end.y - other.start.y))
            - ((self.end.y - self.start.y) * (other.end.x - other.start.x));

        if denominator == 0.0 {
            return false;
        }

        let numerator1 = ((self.start.y - other.start.y) * (other.end.x - other.start.x))
            - ((self.start.x - other.start.x) * (other.end.y - other.start.y));

        let numerator2 = ((self.start.y - other.start.y) * (self.end.x - self.start.x))
            - ((self.start.x - other.start.x) * (self.end.y - self.start.y));

        if numerator1 == 0.0 || numerator2 == 0.0 {
            return false;
        }

        let r = numerator1 / denominator;
        let s = numerator2 / denominator;

        return (r > 0.0 && r < 1.0) && (s > 0.0 && s < 1.0);
    }
    pub fn closest_point(&self, p: Point) -> Point {
        let sv = self.end - self.start;
        let l2 = sv.length_sq();
        if l2 == 0.0 {
            return (p - self.start).into();
        }
        let ps = p - self.start;
        let t = ((ps.x * sv.x + ps.y * sv.y) / l2).clamp(0.0, 1.0);
        self.start + (sv * t)
    }
    pub fn length(&self) -> f64 {
        (self.end - self.start).length()
    }
}

pub fn line_segment(start: Point, end: Point) -> LineSegment {
    LineSegment { start, end }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

fn lerp_point(p0: Point, p1: Point, t: f64) -> Point {
    point(lerp(p0.x, p1.x, t), lerp(p0.y, p1.y, t))
}

fn diag_dist(p0: Point, p1: Point) -> f64 {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    dx.abs().max(dy.abs())
}

pub enum LineType {
    Path,
    Box,
    Graph,
}
impl LineType {
    pub fn colour(&self) -> &[u8] {
        match self {
            Self::Path => &PATH_COLOUR,
            Self::Box => &BOX_COLOUR,
            Self::Graph => &GRAPH_COLOUR,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        line::{lerp, lerp_point, line_segment},
        point,
    };

    use super::LineSegment;

    fn linesegments() -> Vec<(LineSegment, LineSegment, bool)> {
        vec![
            (
                line_segment(point(0.0, 0.0), point(2.0, 8.0)),
                line_segment(point(8.0, 0.0), point(0.0, 20.0)),
                false,
            ),
            (
                line_segment(point(0.0, 10.0), point(2.0, 0.0)),
                line_segment(point(10.0, 0.0), point(0.0, 5.0)),
                true,
            ),
            (
                line_segment(point(0.0, 0.0), point(0.0, 10.0)),
                line_segment(point(2.0, 0.0), point(2.0, 10.0)),
                false,
            ),
            (
                line_segment(point(0.0, 0.0), point(5.0, 5.0)),
                line_segment(point(2.0, 0.0), point(7.0, 5.0)),
                false,
            ),
            (
                line_segment(point(0.0, 0.0), point(5.0, 5.0)),
                line_segment(point(2.0, 2.0), point(7.0, 2.0)),
                true,
            ),
            (
                line_segment(point(0.0, 0.0), point(5.0, 5.0)),
                line_segment(point(7.0, 7.0), point(10.0, 10.0)),
                false,
            ),
            (
                line_segment(point(4.0, 0.0), point(6.0, 10.0)),
                line_segment(point(0.0, 3.0), point(10.0, 7.0)),
                true,
            ),
            (
                line_segment(point(0.0, 0.0), point(1.0, 1.0)),
                line_segment(point(1.0, 2.0), point(4.0, 5.0)),
                false,
            ),
        ]
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(lerp(0.0, 100.0, 0.5), 50.0);
        assert_eq!(lerp(3.0, 5.0, 0.5), 4.0);
        assert_eq!(lerp(5.0, 3.0, 0.5), 4.0);
    }

    #[test]
    fn test_lerp_point() {
        assert_eq!(
            lerp_point(point(0.0, 10.0), point(10.0, 0.0), 0.5),
            point(5.0, 5.0)
        );
    }

    #[test]
    fn test_lineseg() {
        let p0 = point(8.0, 3.0);
        let p1 = point(4.0, 8.0);
        let line = line_segment(p0, p1);
        let points = line.points();
        assert_eq!(points.len(), 5);
    }

    #[test]
    fn test_closest_point() {
        let l0 = line_segment(point(1.0, 1.0), point(4.0, 4.0));
        let p0 = point(1.0, 3.0);
        assert_eq!(l0.closest_point(p0), point(2.0, 2.0));

        let l1 = line_segment(point(10.0, 10.0), point(40.0, 10.0));
        let p1 = point(20.0, 30.0);
        assert_eq!(l1.closest_point(p1), point(20.0, 10.0));

        let l1 = line_segment(point(10.0, 10.0), point(40.0, 40.0));
        let p1 = point(30.0, 40.0);
        assert_eq!(l1.closest_point(p1), point(35.0, 35.0));

        let l1 = line_segment(point(40.0, 40.0), point(10.0, 10.0));
        let p1 = point(30.0, 40.0);
        assert_eq!(l1.closest_point(p1), point(35.0, 35.0));

        let l2 = line_segment(point(88.0, 1111.0), point(555.0, 22.0));
        let p2 = point(1234.0, 101.0);
        assert_eq!(l2.closest_point(p2), point(555.0, 22.0));
    }

    #[test]
    fn test_linesegment_crosses() {
        for (a, b, c) in linesegments() {
            assert_eq!(a.intersects(&b), c);
            assert_eq!(a.intersects2(&b).is_some(), c);
            assert_eq!(a.intersects3(&b), c);
        }
    }

    #[test]
    fn test_line_ord() {
        let l1 = line_segment(point(10.0, 20.0), point(20.0, 30.0));
        let l2 = line_segment(point(10.0, 20.0), point(21.0, 31.0));
        assert!(l1 < l2);
    }
}
