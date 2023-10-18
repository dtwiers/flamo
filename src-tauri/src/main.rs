// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

use std::io::Cursor;

use tauri::{AboutMetadata, Menu, MenuItem, Submenu};

use crate::fractal::{ComputeParameters, RenderParameters};
use base64::prelude::*;
mod fractal;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn make_image() -> String {
    let mut compute_parameters = ComputeParameters::default();
    compute_parameters.linear.weight = 1.0;
    // compute_parameters.spiral.weight = 1.0;
    let compute_parameters = compute_parameters.init_weight();
    let img = fractal::application::render_image(
        &RenderParameters {
            width: 1,
            height: 1,
            quality: 1,
            compute_parameters,
        },
        1,
        Box::new(|_| ()),
    )
    .unwrap();
    let mut buff = Vec::new();
    img.write_to(&mut Cursor::new(&mut buff), image::ImageOutputFormat::Png)
        .unwrap();
    let b64 = BASE64_URL_SAFE_NO_PAD.encode(&buff);
    format!("data:image/png;base64,{}", b64)
}

fn main() {
    pretty_env_logger::init();
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
        .invoke_handler(tauri::generate_handler![greet, make_image])
        .menu(
            Menu::new()
                .add_submenu(file_submenu)
                .add_submenu(about_submenu),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
