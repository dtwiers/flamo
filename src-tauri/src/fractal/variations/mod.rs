use std::f64::consts::PI;

use num::Float;

use super::{Affine, Point, Variation, variation_util::omega};

pub struct LinearVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for LinearVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        point
    }
}

pub struct SinusoidalVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for SinusoidalVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        Point::new(point.x.sin(), point.y.sin())
    }
}

pub struct SphericalVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for SphericalVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r_squared = point.r_squared();
        point / r_squared
    }
}

pub struct SwirlVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for SwirlVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r_squared = point.r_squared();
        let sin_r_squared = r_squared.sin();
        let cos_r_squared = r_squared.cos();
        Point::new(
            point.x * sin_r_squared - point.y * cos_r_squared,
            point.x * cos_r_squared + point.y * sin_r_squared,
        )
    }
}

pub struct HorseshoeVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for HorseshoeVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let x = point.x;
        let y = point.y;
        let two = Scalar::one() + Scalar::one();
        Point::new((x - y) * (x + y), two * x * y) / r
    }
}

pub struct PolarVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for PolarVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let pi: Scalar = Scalar::from::<f64>(PI).unwrap();
        Point::new(theta / pi, r - Scalar::one())
    }
}

pub struct HandkerchiefVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for HandkerchiefVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let x = (theta + r).sin();
        let y = (theta - r).cos();
        Point::new(x, y) * r
    }
}

pub struct HeartVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for HeartVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let x = (theta * r).sin();
        let y = -(theta * r).cos();
        Point::new(x, y) * r
    }
}

pub struct DiscVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for DiscVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let pi: Scalar = Scalar::from::<f64>(PI).unwrap();
        let x = r.sin();
        let y = r.cos();
        Point::new(x, y) * (theta / pi)
    }
}

pub struct SpiralVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for SpiralVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let x = (theta.cos() + r.sin()) / r;
        let y = (theta.sin() - r.cos()) / r;
        Point::new(x, y) / r
    }
}

pub struct HyperbolicVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for HyperbolicVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let x = theta.sin() / r;
        let y = theta.cos() * r;
        Point::new(x, y)
    }
}

pub struct DiamondVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for DiamondVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let x = theta.sin() * r.cos();
        let y = theta.cos() * r.sin();
        Point::new(x, y)
    }
}

pub struct ExVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for ExVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let p0_cubed = (theta + r).sin().powi(3);
        let p1_cubed = (theta - r).cos().powi(3);
        let x = r * (p0_cubed + p1_cubed);
        let y = r * (p0_cubed - p1_cubed);
        Point::new(x, y) * r
    }
}

pub struct JuliaVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for JuliaVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let theta = point.theta();
        let omega = omega::<Scalar>();
        let two = Scalar::one() + Scalar::one();
        let inner = theta / two + omega;
        let x = inner.cos();
        let y = inner.sin();
        Point::new(x, y) * r.sqrt()
    }
}

pub struct BentVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for BentVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let x = point.x;
        let y = point.y;
        let two = Scalar::one() + Scalar::one();
        let x1 = if x >= Scalar::zero() {
            x
        } else {
            x * two
        };
        let y1 = if y >= Scalar::zero() {
            y
        } else {
            y / two
        };
        Point::new(x1, y1)
    }
}

pub struct WavesVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for WavesVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let x = point.x;
        let y = point.y;
        let xinner = y / self.affine.c.powi(2);
        let yinner = x / self.affine.f.powi(2);
        let x1 = x + xinner.sin() * self.affine.b;
        let y1 = y + yinner.sin() * self.affine.e;
        Point::new(x1, y1)
    }
}

pub struct FisheyeVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
}

impl<Scalar: Float> Variation<Scalar> for FisheyeVariation<Scalar> {
    fn apply(&self, point: Point<Scalar>) -> Point<Scalar> {
        let r = point.r();
        let r_squared = r * r;
        let two = Scalar::one() + Scalar::one();
        let x = two / (r_squared + Scalar::one());
        let y = two / (r_squared + Scalar::one());
        Point::new(x, y) * r / two
    }
}
