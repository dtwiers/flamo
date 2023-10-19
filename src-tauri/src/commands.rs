use std::sync::{Arc, RwLock};

use tauri::Wry;

pub fn make_commands<AppState>(builder: &mut tauri::Builder<Wry>, app: Arc<RwLock<AppState>>)
where
    AppState: 'static + Send + Sync,
{
    builder.invoke_handler(tauri::generate_handler![greet, make_image]);
}

fn greet(name: &str, _state: &mut ()) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn make_image(foo: (), _state: &mut ()) -> String {
    "foo".to_string()
}
