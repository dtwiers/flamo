use crate::fractal::{affine::Affine, variation::Variation};



pub struct SinusoidalVariation {
    pub affine: Affine,
}

impl Variation for SinusoidalVariation {
    fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        let (x1, y1) = self.affine.apply(x, y);
        (x1.sin(), y1.sin())
    }
}