
pub trait Variation {
    fn apply(&self, x: f64, y: f64) -> (f64, f64);
}