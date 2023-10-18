use crossbeam::channel::unbounded;
use image::RgbaImage;
use rand::{thread_rng, Rng};
use std::{sync::Arc, thread};

use super::{image_matrix::ImageMatrix, render::RenderParameters, Color, Point};

pub fn render_image(
    render_parameters: &RenderParameters,
    thread_count: u32,
    update_progress: Box<dyn Fn(u16) + Send>,
) -> Result<RgbaImage, String> {
    let render_parameters = Arc::new(render_parameters.clone());
    let (tx, rx) = unbounded();

    let point_count = (render_parameters.width as u64)
        * (render_parameters.height as u64)
        * (render_parameters.quality as u64);
    let points_per_thread = point_count / (thread_count as u64);
    println!(
        "Rendering {} points per thread, {} threads",
        points_per_thread, thread_count
    );
    (0..thread_count)
        .map(|_| init_point(&render_parameters))
        .for_each(|initial_point| {
            let mut point = initial_point.clone();
            let render_parameters = render_parameters.as_ref();
            println!("Rendering thread started");
            for _ in 0..points_per_thread {
                point = compute_point(render_parameters, point, false);
                tx.send(point.clone()).unwrap();
            }
        });

    let mut counter = 0f64;
    let f_point_count = point_count as f64;
    let mut image_matrix = ImageMatrix::new(render_parameters.width, render_parameters.height);

    while let Ok(point) = rx.recv() {
        println!("Rendering point {}/{}", counter, f_point_count);
        let (x, y) = render_parameters.get_coordinates(&point);
        let color = point.color;
        update_progress((counter / f_point_count * 65535.0) as u16);
        counter += 1.0;
        image_matrix.plot_point(x, y, color);
    }
    println!("Rendering complete");

    Ok(image_matrix.into())
}

fn init_point(render_parameters: &RenderParameters) -> Point<f64> {
    println!("Initializing point");
    let mut point = Point::new(
        thread_rng().gen_range(-1.0..1.0),
        thread_rng().gen_range(-1.0..1.0),
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        },
    );
    for _ in 0..20 {
        point = compute_point(render_parameters, point, true);
    }
    println!("Initialized point: {:?}", point);
    point
}

const ESCAPE_ITERATIONS: u32 = 1000;

fn compute_point(render_parameters: &RenderParameters, point: Point<f64>, ignore_bounds: bool) -> Point<f64> {
    let mut new_point = point.clone();
    let mut counter = 0;
    println!("Computing point");
    loop {
        counter += 1;
        let variation = render_parameters.compute_parameters.choose();
        new_point = variation.affine().apply(&new_point);
        new_point = variation.apply(new_point);
        // todo: final variation

        new_point = render_parameters
            .compute_parameters
            .post_transform
            .apply(&new_point);
        println!("Point: {:?}", new_point);
        if ignore_bounds || new_point.is_in_bounds(super::BoundingBox::default()) {
            return new_point;
        } else {
            println!("Point escaped: {:?}", new_point);
        }
        if counter > ESCAPE_ITERATIONS {
            println!("Point escaped, resetting");
            new_point = init_point(&render_parameters);
            counter = 0;
        }
    }
}
