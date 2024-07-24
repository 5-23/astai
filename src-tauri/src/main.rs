// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use astai::*;
use tauri::{CustomMenuItem, Manager, Menu, Submenu, SystemTray, SystemTrayMenu};

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
        Menu::new()
            .add_item({
                let mut item = CustomMenuItem::new("settings", "Settings...");
                item.keyboard_accelerator = Some("cmdOrctrl + ,".to_string());
                item
            })
            .add_item({
                let mut item = CustomMenuItem::new("quit", "quit");
                item.keyboard_accelerator = Some("cmdOrctrl + q".to_string());
                item
            })
            .add_item({
                let mut item = CustomMenuItem::new("close", "close");
                item.keyboard_accelerator = Some("cmdOrctrl + w".to_string());
                item
            }),
    ));

    tauri::Builder::default()
        .system_tray(tray)
        .menu(menu.clone())
        .invoke_handler(tauri::generate_handler![
            commands::get_images,
            commands::get_image,
            commands::get_folder,
            commands::get_class
        ])
        .setup(|app| {
            let setting_window = tauri::WindowBuilder::new(
                &app.handle(),
                "setting", /* the unique window label */
                tauri::WindowUrl::App("settings".into()),
            )
            .build()
            .unwrap();
            setting_window.hide().unwrap();
            setting_window.allow_alpha();

            let main_window: tauri::Window = app.get_window("main").unwrap();
            main_window.allow_alpha();

            Ok(())
        })
        .on_menu_event(menu::menu_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
