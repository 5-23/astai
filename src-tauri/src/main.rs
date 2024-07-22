// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use lazy_static::lazy_static;
use tauri::Manager;
use window_vibrancy::*;
lazy_static! {
    static ref PATH: Mutex<String> = Mutex::new("/Users/dev523/data/test".to_string());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_images, get_image, get_folder])
        .setup(|app| {
            let windows: tauri::Window = app.get_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                window.open_devtools();
                window.close_devtools();
            }
            #[cfg(target_os = "macos")]
            apply_vibrancy(&windows, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_acrylic(&windows, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_images() -> Vec<String> {
    let path = PATH.lock();
    let mut images = vec![];
    if path.is_ok() {
        let path = path.unwrap();
        let dir = std::fs::read_dir(path.to_owned()).unwrap();
        for entry in dir.into_iter() {
            let name = entry.unwrap().file_name();
            let name = name.to_str().unwrap();
            if name.ends_with(".png")
                | name.ends_with(".jpg")
                | name.ends_with(".jpeg")
                | name.ends_with(".webp")
            {
                images.push(name.to_string())
            }
        }
    }
    images
}
#[tauri::command]
fn get_image(name: String) {
    let path = PATH.lock();
    if path.is_ok() {
        let path = path.unwrap().to_string();
        let base64 = image_base64::to_base64(&format!("{path}/{name}"));
        println!("{base64}")
    }
}

#[tauri::command]
fn get_folder() -> String {
    let a = rfd::FileDialog::new()
        .pick_folder()
        .unwrap_or("/".to_owned().into());
    let a = a.to_str().unwrap_or("/");

    a.to_string()
}
