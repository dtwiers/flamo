
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn r(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn theta(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn phi(&self) -> f64 {
        self.x.atan2(self.y)
    }
}