use num::Float;

use super::{Point, Affine, Color};

pub trait Variation<Scalar: Float> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar>;
    fn weight(&self) -> Scalar;
    fn affine(&self) -> Affine<Scalar>;
    fn name(&self) -> String;
    fn color(&self) -> Color<Scalar>;
}
#[macro_export]
macro_rules! variation {
    ($struct_name:ident, $generic_type:ident, $self_:ident, $point_:ident, $apply_body:block) => {
        impl<$generic_type: Float> Variation<$generic_type> for $struct_name<$generic_type> {
            fn weight(&self) -> Scalar {
                self.weight
            }

            fn affine(&self) -> Affine<Scalar> {
                self.affine
            }

            fn name(&self) -> String {
                stringify!($struct_name).to_string()
            }

            fn color(&self) -> Color<Scalar> {
                self.color.clone()
            }

            fn apply(&$self_, $point_: Point<$generic_type>) -> Point<$generic_type> {
                $apply_body
            }
        }
    };
}