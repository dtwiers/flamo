use std::f64::consts::PI;

use num::Float;

use crate::variation;

use super::{
    color::Color,
    variation_util::{alpha, gaussian_estimate, omega, psi},
    Affine, Point, Variation,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct LinearVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(LinearVariation, Scalar, self, point, {
    Point::new(point.x, point.y, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct SinusoidalVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(SinusoidalVariation, Scalar, self, point, {
    Point::new(point.x.sin(), point.y.sin(), point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct SphericalVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(SphericalVariation, Scalar, self, point, {
    let r_squared = point.r_squared();
    Point::new(point.x, point.y, point.color.merge(&self.color)) / r_squared
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct SwirlVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(SwirlVariation, Scalar, self, point, {
    let r_squared = point.r_squared();
    let sin_r_squared = r_squared.sin();
    let cos_r_squared = r_squared.cos();
    Point::new(
        point.x * sin_r_squared - point.y * cos_r_squared,
        point.x * cos_r_squared + point.y * sin_r_squared,
        point.color.merge(&self.color),
    )
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct HorseshoeVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(HorseshoeVariation, Scalar, self, point, {
    let r = point.r();
    let x = point.x;
    let y = point.y;
    let two = Scalar::one() + Scalar::one();
    Point::new(
        (x - y) * (x + y),
        two * x * y,
        point.color.merge(&self.color),
    ) / r
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct PolarVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(PolarVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let pi: Scalar = Scalar::from::<f64>(PI).unwrap();
    Point::new(
        theta / pi,
        r - Scalar::one(),
        point.color.merge(&self.color),
    )
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct HandkerchiefVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(HandkerchiefVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let x = (theta + r).sin();
    let y = (theta - r).cos();
    Point::new(x, y, point.color.merge(&self.color)) * r
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct HeartVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(HeartVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let x = (theta * r).sin();
    let y = -(theta * r).cos();
    Point::new(x, y, point.color.merge(&self.color)) * r
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct DiscVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(DiscVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let pi: Scalar = Scalar::from::<f64>(PI).unwrap();
    let x = r.sin();
    let y = r.cos();
    Point::new(x, y, point.color.merge(&self.color)) * (theta / pi)
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct SpiralVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(SpiralVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let x = (theta.cos() + r.sin()) / r;
    let y = (theta.sin() - r.cos()) / r;
    Point::new(x, y, point.color.merge(&self.color)) / r
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct HyperbolicVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(HyperbolicVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let x = theta.sin() / r;
    let y = theta.cos() * r;
    Point::new(x, y, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct DiamondVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(DiamondVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let x = theta.sin() * r.cos();
    let y = theta.cos() * r.sin();
    Point::new(x, y, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct ExVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(ExVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let p0_cubed = (theta + r).sin().powi(3);
    let p1_cubed = (theta - r).cos().powi(3);
    let x = r * (p0_cubed + p1_cubed);
    let y = r * (p0_cubed - p1_cubed);
    Point::new(x, y, point.color.merge(&self.color)) * r
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct JuliaVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(JuliaVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let omega = omega::<Scalar>();
    let two = Scalar::one() + Scalar::one();
    let inner = theta / two + omega;
    let x = inner.cos();
    let y = inner.sin();
    Point::new(x, y, point.color.merge(&self.color)) * r.sqrt()
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct BentVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(BentVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let two = Scalar::one() + Scalar::one();
    let x1 = if x >= Scalar::zero() { x } else { x * two };
    let y1 = if y >= Scalar::zero() { y } else { y / two };
    Point::new(x1, y1, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct WavesVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(WavesVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let xinner = y / self.affine.c.powi(2);
    let yinner = x / self.affine.f.powi(2);
    let x1 = x + xinner.sin() * self.affine.b;
    let y1 = y + yinner.sin() * self.affine.e;
    Point::new(x1, y1, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct FisheyeVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(FisheyeVariation, Scalar, self, point, {
    let r = point.r();
    let r_squared = r * r;
    let two = Scalar::one() + Scalar::one();
    let x = two / (r_squared + Scalar::one());
    let y = two / (r_squared + Scalar::one());
    Point::new(x, y, point.color.merge(&self.color)) * r / two
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct PopcornVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(PopcornVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let three = Scalar::one() + Scalar::one() + Scalar::one();
    let x = x + ((y * three).tan()).sin() * self.affine.c;
    let y = y + ((x * three).tan()).sin() * self.affine.f;
    Point::new(x, y, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct ExponentialVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(ExponentialVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let pi = Scalar::from::<f64>(PI).unwrap();
    let e = Scalar::from::<f64>(std::f64::consts::E).unwrap();
    let e_to_x = e.powf(x - Scalar::one());
    let inner = pi * y;
    Point::new(inner.cos(), inner.sin(), point.color.merge(&self.color)) * e_to_x
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct PowerVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(PowerVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let magnitude = r.powf(theta.sin());
    Point::new(theta.cos(), theta.sin(), point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct CosineVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(CosineVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let pi = Scalar::from::<f64>(PI).unwrap();
    let x1 = (pi * x).cos() * y.cosh();
    let y1 = -(pi * x).sin() * y.sinh();
    Point::new(x1, y1, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct RingsVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(RingsVariation, Scalar, self, point, {
    let r = point.r();
    let theta = point.theta();
    let two = Scalar::one() + Scalar::one();
    let c2 = self.affine.c.powi(2);
    let magnitude = (r + c2) % (two * c2) - c2 + r * (Scalar::one() - c2);
    Point::new(theta.cos(), theta.sin(), point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct FanVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(FanVariation, Scalar, self, point, {
    let pi = Scalar::from::<f64>(PI).unwrap();
    let t = pi * self.affine.c.powi(2);
    let theta = point.theta();
    let two = Scalar::one() + Scalar::one();
    let condition = (theta + self.affine.f) % t > t / two;
    let new_color = point.color.merge(&self.color);
    if condition {
        let r = point.r();
        let theta = point.theta();
        let x = (theta - t / two).cos();
        let y = (theta - t / two).sin();
        Point::new(x, y, new_color) * r
    } else {
        let r = point.r();
        let theta = point.theta();
        let x = (theta + t / two).cos();
        let y = (theta + t / two).sin();
        Point::new(x, y, new_color) * r
    }
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct BlobVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub high: Scalar,
    pub low: Scalar,
    pub waves: Scalar,
}

variation!(BlobVariation, Scalar, self, point, {
    let high = self.high;
    let low = self.low;
    let waves = self.waves;
    let r = point.r();
    let theta = point.theta();
    let two = Scalar::one() + Scalar::one();
    let magnitude = r * ((high - low) / two * ((theta * waves).sin() + Scalar::one()) + low);
    Point::new(theta.cos(), theta.sin(), point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct PDJVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub a: Scalar,
    pub b: Scalar,
    pub c: Scalar,
    pub d: Scalar,
}

variation!(PDJVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let a = self.a;
    let b = self.b;
    let c = self.c;
    let d = self.d;
    let x1 = (a * y).sin() - (b * x).cos();
    let y1 = (c * x).sin() - (d * y).cos();
    Point::new(x1, y1, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct Fan2Variation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub x: Scalar,
    pub y: Scalar,
}

variation!(Fan2Variation, Scalar, self, point, {
    let theta = point.theta();
    let two = Scalar::one() + Scalar::one();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let p1 = pi * (self.x).powi(2);
    let p2 = self.y;
    let r = point.r();
    let t = theta + p2 - p1 * (two * theta * p2 / p1).trunc();
    let new_color = point.color.merge(&self.color);
    let condition = t > p2 / two;
    if condition {
        let x = (theta - p1 / two).sin();
        let y = (theta - p1 / two).cos();
        Point::new(x, y, new_color) * r
    } else {
        let x = (theta + p1 / two).sin();
        let y = (theta + p1 / two).cos();
        Point::new(x, y, new_color) * r
    }
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct Rings2Variation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub val: Scalar,
}

variation!(Rings2Variation, Scalar, self, point, {
    let p = self.val.powi(2);
    let r = point.r();
    let theta = point.theta();
    let two = Scalar::one() + Scalar::one();
    let t = r - two * p * ((r + p) / (two * p)).trunc();
    Point::new(theta.sin(), theta.cos(), point.color.merge(&self.color)) * t
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct EyefishVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(EyefishVariation, Scalar, self, point, {
    let r = point.r();
    let two = Scalar::one() + Scalar::one();
    let magnitude = two / (r + Scalar::one());
    Point::new(point.x, point.y, point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct BubbleVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(BubbleVariation, Scalar, self, point, {
    let r2 = point.r_squared();
    let four = Scalar::from(4).unwrap();
    let magnitude = four / (r2 + four);
    Point::new(point.x, point.y, point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct CylinderVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(CylinderVariation, Scalar, self, point, {
    Point::new(point.x.sin(), point.y, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct PerspectiveVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub angle: Scalar,
    pub dist: Scalar,
}

variation!(PerspectiveVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let angle = self.angle;
    let dist = self.dist;
    let magnitude = dist / (dist - y * angle.sin());
    Point::new(x, y * angle.cos(), point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct NoiseVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(NoiseVariation, Scalar, self, point, {
    let psi1 = psi();
    let psi2 = psi();
    let x = point.x;
    let y = point.y;
    let two = Scalar::one() + Scalar::one();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let x1 = x * (two * pi * psi2).cos();
    let y1 = y * (two * pi * psi1).sin();
    Point::new(x1, y1, point.color.merge(&self.color)) * psi1
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct JuliaNVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub power: Scalar,
    pub dist: Scalar,
}

variation!(JuliaNVariation, Scalar, self, point, {
    let power = self.power;
    let dist = self.dist;
    let p3 = power.abs() * psi();
    let two = Scalar::one() + Scalar::one();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let t = (point.phi() + two * pi * p3) / power;
    let r = point.r();
    let magnitude = r.powf(dist / power);
    Point::new(t.cos(), t.sin(), point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct JuliaScopeVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub power: Scalar,
    pub dist: Scalar,
}

variation!(JuliaScopeVariation, Scalar, self, point, {
    let power = self.power;
    let dist = self.dist;
    let p3 = power.abs() * psi();
    let two = Scalar::one() + Scalar::one();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let t = (point.phi() * alpha() + two * pi * p3) / power;
    let r = point.r();
    let magnitude = r.powf(dist / power);
    Point::new(t.cos(), t.sin(), point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct BlurVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(BlurVariation, Scalar, self, point, {
    let psi1 = psi();
    let psi2 = psi();
    let two = Scalar::one() + Scalar::one();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let inner = two * pi * psi2;
    Point::new(inner.cos(), inner.sin(), point.color.merge(&self.color)) * psi1
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct GaussianVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(GaussianVariation, Scalar, self, point, {
    let two = Scalar::one() + Scalar::one();
    let gaussian = gaussian_estimate();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let inner = two * pi * psi();
    Point::new(inner.cos(), inner.sin(), point.color.merge(&self.color)) * gaussian
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct RadialBlurVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub angle: Scalar,
}

variation!(RadialBlurVariation, Scalar, self, point, {
    let gaussian = gaussian_estimate::<Scalar>();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let two = Scalar::one() + Scalar::one();
    let p1 = self.angle * pi / two;
    let t2 = point.phi() + gaussian * p1.sin();
    let t3 = gaussian * p1.cos() - Scalar::one();
    let r = point.r();
    let x = point.x;
    let y = point.y;
    let x1 = r * t2.cos() + t3 * x;
    let y1 = r * t2.sin() + t3 * y;
    Point::new(x1, y1, point.color.merge(&self.color)) * (self.weight.recip())
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct PieVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub slices: Scalar,
    pub rotation: Scalar,
    pub thickness: Scalar,
}

variation!(PieVariation, Scalar, self, point, {
    let slices = self.slices;
    let rotation = self.rotation;
    let thickness = self.thickness;
    let half = Scalar::from(0.5).unwrap();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let t1 = (psi::<Scalar>() * slices + half).trunc();

    let t2 = rotation + pi / (half * slices) * (t1 + thickness * psi());
    Point::new(t2.cos(), t2.sin(), point.color.merge(&self.color)) * psi()
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct NgonVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub power: Scalar,
    pub sides: Scalar,
    pub corners: Scalar,
    pub circle: Scalar,
}

variation!(NgonVariation, Scalar, self, point, {
    let two = Scalar::one() + Scalar::one();
    let r = point.r();
    let p1 = self.power;
    let p2 = Scalar::from(2.0 * PI).unwrap() / self.sides;
    let p3 = self.corners;
    let p4 = self.circle;
    let t3 = point.phi() % p2;
    let t4 = if t3 > (p2 / two) { t3 } else { t3 - p2 };
    let k = (p3 * (Scalar::one() / t4.cos() - Scalar::one()) + p4) / r.powf(p1);
    Point::new(point.x, point.y, point.color.merge(&self.color)) * k
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct CurlVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub c1: Scalar,
    pub c2: Scalar,
}

variation!(CurlVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let c1 = self.c1;
    let c2 = self.c2;
    let one = Scalar::one();
    let two = one + one;
    let t1 = one + c1 * x + c2 * (x * x - y * y);
    let t2 = c1 * y + two * c2 * x * y;
    let magnitude = (t1 * t1 + t2 * t2).recip();
    let x1 = x * t1 + y * t2;
    let y1 = y * t1 - x * t2;
    Point::new(x1, y1, point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct RectanglesVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
    pub x: Scalar,
    pub y: Scalar,
}

variation!(RectanglesVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let p1 = self.x;
    let p2 = self.y;
    let one = Scalar::one();
    let two = one + one;
    let x1 = (two * (x / p1).floor() + one) * p1 - x;
    let y1 = (two * (y / p2).floor() + one) * p2 - y;
    Point::new(x1, y1, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct ArchVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(ArchVariation, Scalar, self, point, {
    let weight = self.weight;
    let pi = Scalar::from::<f64>(PI).unwrap();
    let inner = weight * pi * psi();
    let x1 = inner.sin();
    let y1 = inner.sin().powi(2) / inner.cos();
    Point::new(x1, y1, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct TangentVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(TangentVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let cosy = y.cos();
    Point::new(
        x.sin() / cosy,
        y.sin() / cosy,
        point.color.merge(&self.color),
    )
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct SquareVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(SquareVariation, Scalar, self, point, {
    let half = Scalar::from(0.5).unwrap();
    Point::new(
        psi::<Scalar>() - half,
        psi::<Scalar>() - half,
        point.color.merge(&self.color),
    )
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct RaysVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(RaysVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let weight = self.weight;
    let pi = Scalar::from::<f64>(PI).unwrap();
    let r2 = point.r_squared();
    let magnitude = weight * (weight * pi * psi()).tan() / r2;
    Point::new(x.cos(), y.sin(), point.color.merge(&self.color)) * magnitude
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct BladeVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(BladeVariation, Scalar, self, point, {
    let x = point.x;
    let weight = self.weight;
    let r = point.r();
    let inner = weight * r * psi();
    Point::new(
        inner.cos() + inner.sin(),
        inner.cos() - inner.sin(),
        point.color.merge(&self.color),
    ) * x
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct SecantVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(SecantVariation, Scalar, self, point, {
    let x = point.x;
    let weight = self.weight;
    let r = point.r();
    let y1 = (weight * (weight * r).cos()).recip();
    Point::new(x, y1, point.color.merge(&self.color))
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct TwintrianVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(TwintrianVariation, Scalar, self, point, {
    let x = point.x;
    let weight = self.weight;
    let r = point.r();
    let inner = weight * psi() * r;
    let t = inner.sin().powi(2).log10() + inner.cos();
    let pi = Scalar::from::<f64>(PI).unwrap();
    let y1 = t - inner.sin() * pi;
    Point::new(t, y1, point.color.merge(&self.color)) * x
});

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct CrossVariation<Scalar: Float> {
    pub affine: Affine<Scalar>,
    pub weight: Scalar,
    pub color: Color<Scalar>,
}

variation!(CrossVariation, Scalar, self, point, {
    let x = point.x;
    let y = point.y;
    let magnitude = (x.powi(2) - y.powi(2)).recip();
    Point::new(x, y, point.color.merge(&self.color)) * magnitude
});
