use std::ops::{Add, AddAssign, Mul, Sub};

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
