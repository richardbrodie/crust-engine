use std::ops::{Add, AddAssign, Mul, Sub};

use winit::dpi::{PhysicalPosition, PhysicalSize};

// x,y or w,h
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T>(pub T, pub T);

pub type Point = Vec2<f64>;
pub type Rect = Vec2<usize>;

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl<T: Add<Output = T> + std::ops::AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl<T: Mul<f64, Output = T>> Mul<f64> for Vec2<T> {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other, self.1 * other)
    }
}

impl From<PhysicalPosition<f64>> for Point {
    fn from(p: PhysicalPosition<f64>) -> Self {
        Self(p.x, p.y)
    }
}
impl From<PhysicalSize<u32>> for Rect {
    fn from(p: PhysicalSize<u32>) -> Self {
        Self(p.width as usize, p.height as usize)
    }
}
impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Self(p.0 as f64, p.1 as f64)
    }
}

pub fn point_in(anchor: Point, area: Rect, point: Point) -> bool {
    point.0 >= anchor.0
        && point.0 <= (anchor.0 + area.0 as f64)
        && point.1 >= anchor.1
        && point.1 <= (anchor.1 + area.1 as f64)
}

#[cfg(test)]
mod tests {
    use crate::geometry::{point_in, Point, Rect, Vec2};

    #[test]
    fn test_point_in() {
        let anchor: Point = Vec2(10.0, 10.0);
        let area: Rect = Vec2(10, 10);

        assert!(point_in(anchor, area, Vec2(11.0, 11.0)));
        assert!(point_in(anchor, area, Vec2(20.0, 20.0)));
        assert!(!point_in(anchor, area, Vec2(21.0, 21.0)));
        assert!(!point_in(anchor, area, Vec2(9.0, 9.0)));
        assert!(!point_in(anchor, area, Vec2(9.0, 21.0)));
    }
}
