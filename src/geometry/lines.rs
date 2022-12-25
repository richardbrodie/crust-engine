use crate::geometry::{point, Point};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
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
}
pub fn line(start: Point, end: Point) -> Line {
    Line { start, end }
}

pub struct LineString(pub Vec<Point>);
impl LineString {
    pub fn new(v: Vec<Point>) -> Self {
        Self(v)
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

#[cfg(test)]
mod tests {
    use crate::geometry::{
        lines::{lerp, lerp_point, Line},
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
    fn test_line() {
        let p0 = point(8.0, 3.0);
        let p1 = point(4.0, 8.0);
        let line = Line::new(p0, p1);
        let points = line.points();
        assert_eq!(points.len(), 5);
    }
}
