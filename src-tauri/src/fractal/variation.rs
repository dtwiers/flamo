use num::Float;

use super::Point;

pub trait Variation<Scalar: Float> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar>;
}
