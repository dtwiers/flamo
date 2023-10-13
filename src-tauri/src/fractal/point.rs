use std::ops::{Mul, Add, Div};
use num::Float;


#[derive(Clone, Copy, Debug)]
pub struct Point<Scalar: Float> {
    pub x: Scalar,
    pub y: Scalar,
}

impl<Scalar: Float> Point<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
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
}

impl<Scalar: Float> Mul<Scalar> for Point<Scalar> {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<Scalar: Float> Add<Scalar> for Point<Scalar> {
    type Output = Self;

    fn add(self, rhs: Scalar) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl<Scalar: Float> Add<Point<Scalar>> for Point<Scalar> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<Scalar: Float> Div<Scalar> for Point<Scalar> {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
