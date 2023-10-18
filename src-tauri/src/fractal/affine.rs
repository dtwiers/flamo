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

// impl Default for Affine<f64> {
//     fn default() -> Self {
//         Self {
//             a: 1.0,
//             b: 0.0,
//             c: 0.0,
//             d: 1.0,
//             e: 0.0,
//             f: 0.0,
//         }
//     }
// }

// impl Default for Affine<f32> {
//     fn default() -> Self {
//         Self {
//             a: 1.0,
//             b: 0.0,
//             c: 0.0,
//             d: 1.0,
//             e: 0.0,
//             f: 0.0,
//         }
//     }
// }

impl<Scalar: Float> Default for Affine<Scalar> {
    fn default() -> Self {
        Self {
            a: Scalar::one(),
            b: Scalar::zero(),
            c: Scalar::zero(),
            d: Scalar::one(),
            e: Scalar::zero(),
            f: Scalar::zero(),
        }
    }
}
