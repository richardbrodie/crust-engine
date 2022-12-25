use std::ops::{Add, AddAssign, Mul, Sub};

use winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub width: usize,
    pub height: usize,
}
impl Rect {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
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
impl From<PhysicalSize<u32>> for Rect {
    fn from(p: PhysicalSize<u32>) -> Self {
        Self {
            width: p.width as usize,
            height: p.height as usize,
        }
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

pub fn point_in(anchor: Point, area: Rect, point: Point) -> bool {
    point.x >= anchor.x
        && point.x <= (anchor.x + area.width as f64)
        && point.y >= anchor.y
        && point.y <= (anchor.y + area.height as f64)
}

#[cfg(test)]
mod tests {
    use crate::geometry::{point_in, Point, Rect};

    #[test]
    fn test_point_in() {
        let anchor: Point = Point::new(10.0, 10.0);
        let area: Rect = Rect {
            width: 10,
            height: 10,
        };

        assert!(point_in(anchor, area, Point::new(11.0, 11.0)));
        assert!(point_in(anchor, area, Point::new(20.0, 20.0)));
        assert!(!point_in(anchor, area, Point::new(21.0, 21.0)));
        assert!(!point_in(anchor, area, Point::new(9.0, 9.0)));
        assert!(!point_in(anchor, area, Point::new(9.0, 21.0)));
    }
}
