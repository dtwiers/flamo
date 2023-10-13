
#[derive(Clone, Copy, Debug)]
pub struct Affine {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Affine {
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        let x1 = self.a * x + self.b * y + self.e;
        let y1 = self.c * x + self.d * y + self.f;
        (x1, y1)
    }
}