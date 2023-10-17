

pub struct ImageMatrixPixel {
    r: u64,
    g: u64,
    b: u64,
    point_count: u64
}

impl ImageMatrixPixel {
    pub fn new(r: u64, g: u64, b: u64, point_count: u64) -> Self {
        Self { r, g, b, point_count }
    }

    pub fn add_point(&self, r: u64, g: u64, b: u64) -> Self {
        Self::new(
            self.r + r,
            self.g + g,
            self.b + b,
            self.point_count + 1
        )
    }
}

pub struct ImageMatrix {

}