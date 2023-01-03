use std::ops::{Add, Div, Mul, Sub};

use super::Point;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}
pub fn vector(x: f64, y: f64) -> Vector {
    Vector { x, y }
}
impl Vector {
    pub fn lerp_point(&self, o: Self, t: f64) -> Self {
        *self + ((o - *self) * t)
    }
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
    pub fn dot(&self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
    pub fn max_element(&self) -> f64 {
        self.x.max(self.y)
    }
    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }
    pub fn length_sq(&self) -> f64 {
        self.dot(*self)
    }
    pub fn normalise(&self) -> Self {
        *self / self.length()
    }
    pub fn cross(&self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

// vector
impl From<Point> for Vector {
    fn from(v: Point) -> Self {
        Vector { x: v.x, y: v.y }
    }
}
impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl Sub<Point> for Vector {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::vector;

    fn round(x: f64, decimals: i32) -> f64 {
        let sig = 10f64.powi(decimals);
        let xi = (x * sig + 0.5) as usize;
        xi as f64 / sig
    }

    #[test]
    fn test_round() {
        assert_eq!(round(10.123456789, 2), 10.12);
        assert_eq!(round(10.987654321, 2), 10.99);
        assert_eq!(round(10.123456789, 5), 10.12346);
        assert_eq!(round(10.987654321, 5), 10.98765);
        assert_eq!(round(10.123456789, 9), 10.123456789);
        assert_eq!(round(10.123456789, 10), 10.1234567890);
    }

    #[test]
    fn test_dot() {
        let d = vector(1.0, 3.5).dot(vector(5.3, 9.1));
        assert_eq!(d, 37.15)
    }

    #[test]
    fn test_length() {
        let l = vector(14.6, 82.3).length();
        assert_eq!(round(l, 5), 83.58499)
    }

    #[test]
    fn test_length_sq() {
        let l = vector(8.2, 9.3).length_sq();
        assert_eq!(round(l, 5), 153.73)
    }

    #[test]
    fn test_normalise() {
        let l = vector(88.8, 4.44).normalise();
        let ll = vector(round(l.x, 5), round(l.y, 5));
        assert_eq!(ll, vector(0.99875, 0.04994));
    }
}
