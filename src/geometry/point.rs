use std::ops::{Add, Mul, Sub};

use winit::dpi::PhysicalPosition;

use super::Vector;

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

impl From<Vector> for Point {
    fn from(v: Vector) -> Self {
        Point { x: v.x, y: v.y }
    }
}

impl Add<Point> for Point {
    type Output = Vector;
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add<f64> for Point {
    type Output = Vector;
    fn add(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, other: Vector) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self::Output {
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

#[cfg(test)]
mod tests {
    use crate::geometry::{point, vector};

    #[test]
    fn test_point_add() {
        assert_eq!(point(11.0, 11.0) + point(3.0, 0.0), vector(14.0, 11.0));
        assert_eq!(point(4.0, 1.0) + point(0.0, 10.0), vector(4.0, 11.0));
        assert_eq!(point(-4.0, 1.0) + point(0.0, -10.0), vector(-4.0, -9.0));
    }
}
