use crate::geometry::{point, Point};

use super::Vector;

const PATH_COLOUR: [u8; 4] = [255, 255, 255, 255];
const BOX_COLOUR: [u8; 4] = [10, 10, 240, 255];
const GRAPH_COLOUR: [u8; 4] = [10, 240, 10, 255];

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl Line {
    pub fn intersects(&self, other: &Self) -> Option<Point> {
        let det = self.a * self.b - other.b * self.a;

        if det.abs() < f64::EPSILON {
            return None;
        }

        let x = (self.c * other.b - other.c * self.b) / det;
        let y = (self.a * other.c - other.a * self.c) / det;

        return Some(point(x, y));
    }
}
pub fn line(d: Vector, o: Point) -> Line {
    let c = o.y * (o.x + d.x) - o.x * (o.y + d.y);
    Line { a: d.x, b: d.y, c }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
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

        if cmpxr == 0.0 {
            return ((other.start.x - self.start.x < 0.0) != (other.start.x - self.end.x < 0.0))
                || ((other.start.y - self.start.y < 0.0) != (other.start.y - self.end.y < 0.0));
        }

        if rxs == 0.0 {
            return false; // Lines are parallel.
        }

        let rxsr = 1.0 / rxs;
        let t = cmpxs * rxsr;
        let u = cmpxr * rxsr;

        return (t >= 0.0) && (t <= 1.0) && (u >= 0.0) && (u <= 1.0);
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
}
pub fn lineseg(start: Point, end: Point) -> LineSegment {
    LineSegment { start, end }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct LineString(pub Vec<Point>);
impl LineString {
    pub fn new(v: Vec<Point>) -> Self {
        Self(v)
    }
    pub fn lines(&self) -> impl Iterator<Item = LineSegment> + '_ {
        self.0.windows(2).map(|w| lineseg(w[0], w[1]))
    }
    pub fn close(&mut self) {
        if !self.0.is_empty() && self.0.first() != self.0.last() {
            self.0.push(self.0[0]);
        }
    }
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
        line::{lerp, lerp_point, lineseg, LineString},
        point,
    };

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
        let line = lineseg(p0, p1);
        let points = line.points();
        assert_eq!(points.len(), 5);
    }

    #[test]
    fn test_closest_point() {
        let l0 = lineseg(point(1.0, 1.0), point(4.0, 4.0));
        let p0 = point(1.0, 3.0);
        assert_eq!(l0.closest_point(p0), point(2.0, 2.0));

        let l1 = lineseg(point(10.0, 10.0), point(40.0, 10.0));
        let p1 = point(20.0, 30.0);
        assert_eq!(l1.closest_point(p1), point(20.0, 10.0));

        let l1 = lineseg(point(10.0, 10.0), point(40.0, 40.0));
        let p1 = point(30.0, 40.0);
        assert_eq!(l1.closest_point(p1), point(35.0, 35.0));

        let l1 = lineseg(point(40.0, 40.0), point(10.0, 10.0));
        let p1 = point(30.0, 40.0);
        assert_eq!(l1.closest_point(p1), point(35.0, 35.0));

        let l2 = lineseg(point(88.0, 1111.0), point(555.0, 22.0));
        let p2 = point(1234.0, 101.0);
        assert_eq!(l2.closest_point(p2), point(555.0, 22.0));
    }

    #[test]
    fn test_lines_intersect() {
        let l1 = lineseg(point(0.0, 0.0), point(2.0, 8.0));
        let l2 = lineseg(point(8.0, 0.0), point(0.0, 20.0));
        assert!(!l1.intersects(&l2));

        let l1 = lineseg(point(0.0, 10.0), point(2.0, 0.0));
        let l2 = lineseg(point(10.0, 0.0), point(0.0, 5.0));
        assert!(l1.intersects(&l2));

        let l1 = lineseg(point(0.0, 0.0), point(0.0, 10.0));
        let l2 = lineseg(point(2.0, 0.0), point(2.0, 10.0));
        assert!(!l1.intersects(&l2));

        let l1 = lineseg(point(0.0, 0.0), point(5.0, 5.0));
        let l2 = lineseg(point(2.0, 0.0), point(7.0, 5.0));
        assert!(!l1.intersects(&l2));

        let l1 = lineseg(point(0.0, 0.0), point(5.0, 5.0));
        let l2 = lineseg(point(2.0, 2.0), point(7.0, 7.0));
        assert!(l1.intersects(&l2));

        let l1 = lineseg(point(0.0, 0.0), point(5.0, 5.0));
        let l2 = lineseg(point(7.0, 7.0), point(10.0, 10.0));
        assert!(!l1.intersects(&l2));

        let l1 = lineseg(point(4.0, 0.0), point(6.0, 10.0));
        let l2 = lineseg(point(0.0, 3.0), point(10.0, 7.0));
        assert!(l1.intersects(&l2));

        let l1 = lineseg(point(0.0, 0.0), point(1.0, 1.0));
        let l2 = lineseg(point(1.0, 2.0), point(4.0, 5.0));
        assert!(!l1.intersects(&l2));
    }

    // #[test]
    // fn test_linestring_lines() {
    //     let v = vec![
    //         point(5.0, 5.0),
    //         point(6.0, 6.0),
    //         point(7.0, 7.0),
    //         point(9.0, 15.0),
    //         point(13.0, 25.0),
    //     ];
    //     let ls = LineString::new(v);
    //     let mut ll = ls.lines();
    //
    //     assert_eq!(ll.next().unwrap().1, point(6.0, 6.0));
    //     assert_eq!(ll.next().unwrap().1, point(7.0, 7.0));
    //     ll.next();
    //     ll.next();
    //     let n = ll.next();
    //     assert!(n.is_none());
    // }
}
