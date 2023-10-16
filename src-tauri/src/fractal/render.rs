use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use super::{compute_parameters::ComputeParameters, Color, Point};

pub struct RenderParameters {
    pub width: u32,
    pub height: u32,
    pub quality: u32,
    pub compute_parameters: ComputeParameters<f64>,
}

impl RenderParameters {
    pub fn render(&self, thread_count: u32) {
        let mut threads = Vec::<JoinHandle<Vec<Point<f64>>>>::new();
        let thread_quality = self.quality / thread_count;
        for _ in 0..thread_count {
            let join = std::thread::spawn(|| {
                let mut point = self.init_point();
                let mut points = Vec::<Point<f64>>::new();
                for _ in 0..5000 {
                    while point.x < -1.0 || point.x > 1.0 || point.y < -1.0 || point.y > 1.0 {
                        point = self.compute_point(point);
                    }
                    points.push(point);
                }
                points
            });
            threads.push(join);
        }
        for thread in threads {
            let points = thread.join().unwrap();
        }
    }
    pub fn init_point(&self) -> Point<f64> {
        let mut point = Point::new(
            0.0,
            0.0,
            Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            },
        );
        for _ in 0..20 {
            point = self.compute_point(point);
        }
        point
    }

    pub fn compute_point(&self, point: Point<f64>) -> Point<f64> {
        let variation = self.compute_parameters.choose();
        let point = variation.affine().apply(point);
        let point = variation.apply(point);
        // todo: final variation
        let point = self.compute_parameters.post_transform.apply(point);
        point
    }
}
