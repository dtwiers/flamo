use num::Integer;


pub struct ImageMatrixPixel<Scalar: Integer> {
    r: Scalar,
    g: Scalar,
    b: Scalar,
    point_count: Scalar
}

impl<Scalar: Integer> ImageMatrixPixel<Scalar> {
    pub fn new(r: Scalar, g: Scalar, b: Scalar, point_count: Scalar) -> Self {
        Self { r, g, b, point_count }
    }

    pub fn add_point(&self, r: Scalar, g: Scalar, b: Scalar) -> Self {
        let me = self.clone();
        Self::new(
            me.r + r,
            me.g + g,
            me.b + b,
            self.point_count + Scalar::one()
        )
    }
}

pub struct ImageMatrix {

}