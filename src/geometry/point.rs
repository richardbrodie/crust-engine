use std::ops::{Add, AddAssign, Mul, Sub};

use winit::dpi::PhysicalPosition;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn xy(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
pub fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl From<PhysicalPosition<f64>> for Point {
    fn from(p: PhysicalPosition<f64>) -> Self {
        Self { x: p.x, y: p.y }
    }
}
impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Self {
            x: p.0 as f64,
            y: p.1 as f64,
        }
    }
}

pub fn point_in(anchor: Point, area: Point, point: Point) -> bool {
    point.x >= anchor.x
        && point.x <= (anchor.x + area.x as f64)
        && point.y >= anchor.y
        && point.y <= (anchor.y + area.y as f64)
}

#[cfg(test)]
mod tests {
    use crate::geometry::point;

    // #[test]
    // fn test_point_in() {
    //     let anchor = vec2(10.0, 10.0);
    //     let area = vec2(10.0, 10.0);
    //
    //     assert!(point_in(anchor, area, vec2(11.0, 11.0)));
    //     assert!(point_in(anchor, area, vec2(20.0, 20.0)));
    //     assert!(!point_in(anchor, area, vec2(21.0, 21.0)));
    //     assert!(!point_in(anchor, area, vec2(9.0, 9.0)));
    //     assert!(!point_in(anchor, area, vec2(9.0, 21.0)));
    // }

    #[test]
    fn test_point_add() {
        assert_eq!(point(11.0, 11.0) + point(3.0, 0.0), point(14.0, 11.0));
        assert_eq!(point(4.0, 1.0) + point(0.0, 10.0), point(4.0, 11.0));
        assert_eq!(point(-4.0, 1.0) + point(0.0, -10.0), point(-4.0, -9.0));
    }

    #[test]
    fn test_point_add_assign() {
        let mut a = point(11.0, 11.0);
        a += point(3.0, 0.0);
        let mut b = point(4.0, 1.0);
        b += point(0.0, 10.0);
        let mut c = point(-4.0, 1.0);
        c += point(0.0, -10.0);

        assert_eq!(a, point(14.0, 11.0));
        assert_eq!(b, point(4.0, 11.0));
        assert_eq!(c, point(-4.0, -9.0));
    }
}
