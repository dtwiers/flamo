use super::{compute_parameters::ComputeParameters, Point};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RenderParameters {
    pub width: u32,
    pub height: u32,
    pub quality: u32,
    pub compute_parameters: ComputeParameters<f64>,
}

impl RenderParameters {
    pub fn get_coordinates(&self, point: &Point<f64>) -> (u32, u32) {
        let x = ((point.x + 1.0) * (self.width as f64) / 2.0) as u32;
        let y = ((point.y + 1.0) * (self.height as f64) / 2.0) as u32;
        (x, y)
    }
}
