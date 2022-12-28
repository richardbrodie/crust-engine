use crate::geometry::{point, Point};

const PATH_COLOUR: [u8; 4] = [255, 255, 255, 255];
const BOX_COLOUR: [u8; 4] = [10, 10, 240, 255];
const GRAPH_COLOUR: [u8; 4] = [10, 240, 10, 255];

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
impl Line {
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
        let cmp = point(other.start.x - self.start.x, other.start.y - self.start.y);
        let r = point(self.end.x - self.start.x, self.end.y - self.start.y);
        let s = point(other.end.x - other.start.x, other.end.y - other.start.y);

        let cmpxr = cmp.x * r.y - cmp.y * r.x;
        let cmpxs = cmp.x * s.y - cmp.y * s.x;
        let rxs = r.x * s.y - r.y * s.x;

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
        let atop = p - self.start;
        let atob = self.end - self.start;
        let ablensq = atob.x * atob.x + atob.y * atob.y;
        let dot = atop.x * atob.x + atop.y * atob.y;
        let dist = dot / ablensq;
        if dist < 0.0 {
            self.start
        } else if dist > 1.0 {
            self.end
        } else {
            self.start + atob * dist
        }
    }
}
pub fn line(start: Point, end: Point) -> Line {
    Line { start, end }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct LineString(pub Vec<Point>);
impl LineString {
    pub fn new(v: Vec<Point>) -> Self {
        Self(v)
    }
    pub fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        self.0.windows(2).map(|w| line(w[0], w[1]))
    }
    pub fn close(&mut self) {
        if !self.0.is_empty() && self.0.first() != self.0.last() {
            self.0.push(self.0[0]);
        }
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a as f64 * (1.0 - t) + b as f64 * t
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
        lines::{lerp, lerp_point, line, LineString},
        point,
    };

    use super::Line;

    fn make_lines(ax: u8, ay: u8, bx: u8, by: u8, cx: u8, cy: u8, dx: u8, dy: u8) -> [Line; 2] {
        [
            line(point(ax as f64, ay as f64), point(bx as f64, by as f64)),
            line(point(cx as f64, cy as f64), point(dx as f64, dy as f64)),
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
    fn test_line() {
        let p0 = point(8.0, 3.0);
        let p1 = point(4.0, 8.0);
        let line = line(p0, p1);
        let points = line.points();
        assert_eq!(points.len(), 5);
    }

    #[test]
    fn test_closest_point() {
        let l0 = line(point(1.0, 1.0), point(4.0, 4.0));
        let p0 = point(2.0, 3.0);
        let p1 = point(2.5, 2.5);
        assert_eq!(l0.closest_point(p0), p1);
        // let l2 = line(point(0.0, 1.0), point(0.0, 4.0));
        // let l3 = line(point(1.0, 8.0), point(1.0, 4.0));
        // assert_eq!(l2.closest_point(l3), point(1.0, 1.0));
    }

    #[test]
    fn test_lines_intersect() {
        let l0 = make_lines(0, 0, 2, 8, 8, 0, 0, 20); // No intersect
        assert!(!l0[0].intersects(&l0[1]));
        let l1 = make_lines(0, 10, 2, 0, 10, 0, 0, 5); // Intersect
        assert!(l1[0].intersects(&l1[1]));
        let l2 = make_lines(0, 0, 0, 10, 2, 0, 2, 10); // Parallel, vertical
        assert!(!l2[0].intersects(&l2[1]));
        let l3 = make_lines(0, 0, 5, 5, 2, 0, 7, 5); // Parallel, diagonal
        assert!(!l3[0].intersects(&l3[1]));
        let l4 = make_lines(0, 0, 5, 5, 2, 2, 7, 7); // Collinear, overlap
        assert!(l4[0].intersects(&l4[1]));
        let l5 = make_lines(0, 0, 5, 5, 7, 7, 10, 10); // Collinear, no overlap
        assert!(!l5[0].intersects(&l5[1]));
    }

    #[test]
    fn test_linestring_lines() {
        let v = vec![
            point(5.0, 5.0),
            point(6.0, 6.0),
            point(7.0, 7.0),
            point(9.0, 15.0),
            point(13.0, 25.0),
        ];
        let ls = LineString::new(v);
        let mut ll = ls.lines();

        assert_eq!(ll.next().unwrap().end, point(6.0, 6.0));
        assert_eq!(ll.next().unwrap().end, point(7.0, 7.0));
        ll.next();
        ll.next();
        let n = ll.next();
        assert!(n.is_none());
    }

    // #[test]
    // fn test_closest_point() {
    //     let l0 = line(point(0.0, 0.0), point(0.0, 10.0));
    //     let p = point(5.0, 5.0);
    //     assert_eq!(l0.closest_point(p), point(0.0, 5.0));
    // }
}
