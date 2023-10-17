// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

use tauri::{AboutMetadata, Menu, MenuItem, Submenu};

use crate::fractal::RenderParameters;

mod fractal;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn make_image() -> String {
    fractal::application::render_image(
        &RenderParameters {
            width: 800,
            height: 600,
            quality: 100,
            compute_parameters: todo!(),
        },
        4,
        Box::new(|_| ()),
    );
    todo!()
}

fn main() {
    let file_submenu = Submenu::new("File", Menu::new().add_native_item(MenuItem::Quit));
    let about_submenu = Submenu::new(
        "About",
        Menu::new().add_native_item(MenuItem::About(
            "Flamo".to_string(),
            AboutMetadata::new()
                .version(env!("CARGO_PKG_VERSION"))
                .authors(vec!["Derek Wiers".to_string()])
                .comments("A fractal flame generator")
                .license("GPL"),
        )),
    );

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .menu(
            Menu::new()
                .add_submenu(file_submenu)
                .add_submenu(about_submenu),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
