// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use lazy_static::lazy_static;
use tauri::{
    CustomMenuItem, Manager, Menu, MenuItem, Runtime, Submenu, SystemTray, SystemTrayMenu, Window,
};
use window_vibrancy::*;
lazy_static! {
    static ref PATH: Mutex<String> = Mutex::new("/Users/dev523/data/test".to_string());
}

fn main() {
    let tray = SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("adsf".to_string(), "title"))
            .add_native_item(tauri::SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("adsf".to_string(), "title"))
            .add_item(CustomMenuItem::new("adsf".to_string(), "title")),
    );
    let default_menu = Menu::default();

    let menu = default_menu.add_submenu(Submenu::new(
        "astai",
        Menu::new().add_item({
            let mut item = CustomMenuItem::new("settings", "Settings...");
            item.keyboard_accelerator = Some("cmdOrctrl + ,".to_string());
            item
        }),
    ));

    tauri::Builder::default()
        .system_tray(tray)
        .menu(menu.clone())
        .invoke_handler(tauri::generate_handler![
            get_images, get_image, get_folder, get_class
        ])
        .setup(|app| {
            let windows: tauri::Window = app.get_window("main").unwrap();
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
fn get_class() -> Vec<String> {
    let path = PATH.lock();
    if path.is_ok() {
        let path = path.unwrap();
        let content =
            std::fs::read_to_string(format!("{}/class.astai", *path)).unwrap_or_else(|_| {
                std::fs::write(format!("{}/class.astai", (*path).clone()), "[]".to_string())
                    .unwrap();
                "".to_string()
            });
        return content.split("\n").map(|x| x.to_string()).collect();
    }
    vec![]
}

#[tauri::command]
fn get_folder() -> String {
    let path = PATH.lock();
    if path.is_ok() {
        let mut path = path.unwrap();
        let path_value = path.clone();
        let a = rfd::FileDialog::new()
            .pick_folder()
            .unwrap_or(path_value.clone().into());
        let a = a.to_str().unwrap_or(&path_value);
        *path = a.to_string();
        return a.to_string();
    }
    "/".to_string()
}
