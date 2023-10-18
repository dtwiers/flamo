use image::RgbaImage;
use rayon::prelude::*;

use super::Color;

pub struct ImageMatrixPixel {
    r: u64,
    g: u64,
    b: u64,
    point_count: u64,
}

impl ImageMatrixPixel {
    pub fn new(r: u64, g: u64, b: u64, point_count: u64) -> Self {
        Self {
            r,
            g,
            b,
            point_count,
        }
    }

    pub fn add_point(&self, r: u64, g: u64, b: u64) -> Self {
        Self::new(self.r + r, self.g + g, self.b + b, self.point_count + 1)
    }

    pub fn normalize(&mut self, factor: u64) {
        self.r /= factor;
        self.g /= factor;
        self.b /= factor;
    }
}

impl Default for ImageMatrixPixel {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

pub struct ImageMatrix {
    width: u32,
    height: u32,
    pixels: Vec<Vec<ImageMatrixPixel>>,
    is_normalized: bool,
}

impl ImageMatrix {
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(ImageMatrixPixel::default());
            }
            pixels.push(row);
        }
        Self {
            width,
            height,
            pixels,
            is_normalized: false,
        }
    }

    pub fn plot_point(&mut self, x: u32, y: u32, color: Color<f64>) {
        println!("Plotting point ({}, {})", x, y);
        let pixel = &mut self.pixels[y as usize][x as usize];

        let r = (color.red * 255.0) as u64;
        let g = (color.green * 255.0) as u64;
        let b = (color.blue * 255.0) as u64;
        println!("Adding point ({}, {}, {}) to pixel", r, g, b);
        *pixel = pixel.add_point(r, g, b);
    }

    pub fn normalize(&mut self) {
        println!("Normalizing image");
        let max_count = self
            .pixels
            .par_iter()
            .map(|row| row.par_iter().map(|pixel| pixel.point_count).max().unwrap())
            .max()
            .unwrap();
        let normalization_factor = max_count * 255;
        println!("Normalization factor: {}", normalization_factor);
        self.pixels.par_iter_mut().for_each(|row| {
            row.par_iter_mut().for_each(|pixel| {
                pixel.normalize(normalization_factor);
            });
        });
        println!("Image normalized");
        self.is_normalized = true;
    }
}

impl Into<image::RgbaImage> for ImageMatrix {
    fn into(self) -> image::RgbaImage {
        let mut img = RgbaImage::new(self.width, self.height);
        for (y, row) in self.pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let r = pixel.r as u8;
                let g = pixel.g as u8;
                let b = pixel.b as u8;
                let a = 255;
                img.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
            }
        }
        img
    }
}
