use num::Float;
use std::f64::consts::PI;

use rand::Rng;

pub fn omega<Scalar: Float>() -> Scalar {
    let rndbool = rand::thread_rng().gen::<u8>() & 1 == 1;
    Scalar::from(rndbool as i32).unwrap() * Scalar::from(PI).unwrap()
}

pub fn alpha<Scalar: Float>() -> Scalar {
    let rndbool = rand::thread_rng().gen::<u8>() & 1 == 1;
    Scalar::from(rndbool as i32 * 2 - 1).unwrap()
}

pub fn psi<Scalar: Float>() -> Scalar {
    Scalar::from(rand::thread_rng().gen_range(0.0..1.0)).unwrap()
}

pub fn gaussian_estimate<Scalar: Float>() -> Scalar {
    let two = Scalar::one() + Scalar::one();
    let mut value = Scalar::zero();
    for _ in 0..4 {
        value = value + psi();
    }
    value - two
}
