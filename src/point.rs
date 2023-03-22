use std::f64;
use std::fmt::{Display, Formatter};
use std::cmp::{PartialEq, Eq};
use std::ops::{Add, Sub, Mul, Div};

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const DECIMAL_PLACES: i32 = 2;
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x, y };
    }
    pub fn round_to(num: f64, places: i32) -> f64 {
        let power_of_ten = f64::powi(10., places);
        return (num * power_of_ten).round() / power_of_ten;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", Self::round_to(self.x, Point::DECIMAL_PLACES), Self::round_to(self.y, Point::DECIMAL_PLACES));
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Eq for Point {
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        return Self::Output { x: self.x + other.x, y: self.y + other.y };
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        return Self::Output { x: self.x - other.x, y: self.y - other.y };
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, a: f64) -> Self::Output {
        return Self::Output { x: self.x * a, y: self.y * a };
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, other: Point) -> Self::Output {
        return Self::Output { x: self * other.x, y: self * other.y };
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, a: f64) -> Self::Output {
        if a == 0. {
            panic!("Division by zero!");
        }
        return Self::Output { x: self.x / a, y: self.y / a };
    }
}