use num::Float;
use std::ops::{Add, Div, Mul};

use super::{color::Color, BoundingBox};

#[derive(Clone, Debug)]
pub struct Point<Scalar: Float> {
    pub x: Scalar,
    pub y: Scalar,
    pub color: Color<Scalar>,
}

impl<Scalar: Float> Point<Scalar> {
    pub fn new(x: Scalar, y: Scalar, color: Color<Scalar>) -> Self {
        Self { x, y, color }
    }

    pub fn r_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y
    }

    pub fn r(&self) -> Scalar {
        self.r_squared().sqrt()
    }

    pub fn theta(&self) -> Scalar {
        self.y.atan2(self.x)
    }

    pub fn phi(&self) -> Scalar {
        self.x.atan2(self.y)
    }

    pub fn is_in_bounds(&self, bounding_box: BoundingBox) -> bool {
        return self.x.to_f64().unwrap() >= bounding_box.x_min
            && self.x.to_f64().unwrap() <= bounding_box.x_max
            && self.y.to_f64().unwrap() >= bounding_box.y_min
            && self.y.to_f64().unwrap() <= bounding_box.y_max;
    }
}

impl<Scalar: Float> Mul<Scalar> for Point<Scalar> {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.color)
    }
}

impl<Scalar: Float> Add<Scalar> for Point<Scalar> {
    type Output = Self;

    fn add(self, rhs: Scalar) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.color)
    }
}

impl<Scalar: Float> Add<Point<Scalar>> for Point<Scalar> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.color)
    }
}

impl<Scalar: Float> Div<Scalar> for Point<Scalar> {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.color)
    }
}
