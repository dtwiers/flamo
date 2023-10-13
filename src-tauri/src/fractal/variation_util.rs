use std::f64::consts::PI;

use rand::distributions::{Bernoulli, Distribution};

pub struct VariationUtil {
    bernoulli: Bernoulli,
    rng: rand::rngs::ThreadRng,
}

impl VariationUtil {
    pub fn new() -> Self {
        Self {
            bernoulli: Bernoulli::new(0.5).unwrap(),
            rng: rand::thread_rng(),
        }
    }
    pub fn omega(&mut self) -> f64 {
        let switch = self.bernoulli.sample(&mut self.rng);
        match switch {
            true => PI,
            false => 0.0,
        }
    }
}