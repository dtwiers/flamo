use crate::fractal::{affine::Affine, variation::Variation};

pub struct LinearVariation {
    pub affine: Affine,
}

impl Variation for LinearVariation {
    fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        self.affine.apply(x, y)
    }
}