use crate::{buffer::Buffer, geometry::Point};

fn plot_low(x0: isize, y0: isize, x1: isize, y1: isize) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 0;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = y0;

    for x in x0..x1 {
        // plot(x, y)
        if d > 0 {
            y = y + yi;
            d = d + (2 * (dx - dy));
        } else {
            d = d + 2 * dx;
        }
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a as f64 * (1.0 - t) + b as f64 * t
}

fn lerp_point(p0: Point, p1: Point, t: f64) -> Point {
    Point::new(lerp(p0.x, p1.x, t), lerp(p0.y, p1.y, t))
}

pub fn line(p0: Point, p1: Point) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    let n: f64 = diag_dist(p0, p1);
    for step in 0..n as usize {
        let t = if n == 0.0 { 0.0 } else { step as f64 / n };
        let p = lerp_point(p0, p1, t);
        points.push(Point::new(p.x.round(), p.y.round()))
    }

    return points;
}

fn diag_dist(p0: Point, p1: Point) -> f64 {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    dx.abs().max(dy.abs())
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::Point,
        lines::{lerp, lerp_point, line},
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
            lerp_point(Point::new(0.0, 10.0), Point::new(10.0, 0.0), 0.5),
            Point::new(5.0, 5.0)
        );
    }

    #[test]
    fn test_line() {
        let p0 = Point::new(8.0, 3.0);
        let p1 = Point::new(14.0, 28.0);
        let points = line(p0, p1);
        dbg!(&points);
        assert_eq!(points.len(), 10);
    }
}
