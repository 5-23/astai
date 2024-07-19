// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Args, Parser, Subcommand};
use std::{
    process::Command,
    sync::Mutex,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::Manager;
use window_vibrancy::*;

use window_vibrancy::*;
#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub command: String,
    pub path: Option<String>,
}
fn main() {
    let args = Arg::parse();
    let path = if args.path.is_none() {
        args.path.unwrap()
    } else {
        "./".to_string()
    };

    match args.command.trim() {
        "open" => {
            let mut window = tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![])
                .setup(|app| {
                    let windows = app.get_window("main").unwrap();
                    #[cfg(target_os = "windows")]
                    {
                        window.open_devtools();
                        window.close_devtools();
                    }
                    #[cfg(target_os = "macos")]
                    apply_vibrancy(&windows, NSVisualEffectMaterial::HudWindow, None, None).expect(
                        "Unsupported platform! 'apply_vibrancy' is only supported on macOS",
                    );

                    #[cfg(target_os = "windows")]
                    apply_acrylic(&windows, Some((18, 18, 18, 125)))
                        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        _ => println!("UNKNOWN COMMANDS"),
    }
}
