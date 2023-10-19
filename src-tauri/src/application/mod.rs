use std::fs::{self, File};

use serde::{Deserialize, Serialize};

pub mod project;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    pub is_configured: bool,
    pub projects: Vec<project::Project>,
    pub recent_projects: Vec<RecentProject>,
    pub current_project_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecentProject {
    pub name: String,
    pub path: String,
}

impl Application {
    pub fn init() -> Application {
        info!("Loading config");
        let app_dirs = platform_dirs::AppDirs::new(Some("flamo"), true).unwrap();
        let config_file_path = app_dirs.config_dir.join("flamo.json");
        fs::create_dir_all(&app_dirs.config_dir).unwrap();
        if config_file_path.exists() {
            let file = File::open(config_file_path).unwrap();
            let application: Application = serde_json::from_reader(file).unwrap();
            application
        } else {
            Application {
                is_configured: false,
                projects: Vec::new(),
                recent_projects: Vec::new(),
                current_project_id: None,
            }
        }
    }

    pub fn new_project(&mut self) -> project::Project {
        let project = project::Project::default();
        self.projects.push(project.clone());
        self.current_project_id = Some(project.id.clone());
        project
    }
}
