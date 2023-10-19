pub mod project_file;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;

use crate::fractal::{
    application::render_image, Affine, Color, ComputeParameters, RenderParameters,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub id: String,
    pub render_parameters: RenderParameters,
    #[serde(skip)]
    pub status: Arc<Mutex<ProjectStatus>>,
    pub path: Option<String>,
    pub is_dirty: bool,
    pub display_path: Option<String>,
}

impl Default for Project {
    fn default() -> Self {
        let render_parameters = RenderParameters {
            width: 1280,
            height: 720,
            quality: 50,
            compute_parameters: ComputeParameters::default(),
        };
        Self {
            name: "New Project".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            render_parameters,
            status: ProjectStatus::NotStarted,
            path: None,
            is_dirty: true,
        }
    }
}

impl Project {
    pub async fn render(&mut self) {
        self.set_status(ProjectStatus::InProgress(0));
        self.render_parameters.compute_parameters.init_weight();
        let image_path = tauri::async_runtime::spawn_blocking(|| {
            render_image(&self.render_parameters, 1, &mut |progress| {
                self.set_status(ProjectStatus::InProgress(progress));
            })
        })
        .await
        .unwrap();
        self.set_status(ProjectStatus::Complete);
    }

    pub async fn set_status(&mut self, status: ProjectStatus) {
        if let Ok(mut self_status) = self.status.try_lock() {
            *self_status = status;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProjectStatus {
    NotStarted,
    InProgress(u16),
    Complete,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        Self::NotStarted
    }
}
