use num::Float;

use super::Point;


#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub struct Affine<Scalar: Float> {
    pub a: Scalar,
    pub b: Scalar,
    pub c: Scalar,
    pub d: Scalar,
    pub e: Scalar,
    pub f: Scalar,
}

impl<Scalar: Float> Affine<Scalar> {
    pub fn apply(&self, point: &Point<Scalar>) -> Point<Scalar> {
        let x1 = self.a * point.x + self.b * point.y + self.e;
        let y1 = self.c * point.x + self.d * point.y + self.f;
        Point::new(x1, y1, point.color.clone())
    }
}