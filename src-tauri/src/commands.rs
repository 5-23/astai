use std::{sync::Mutex};

use lazy_static::lazy_static;
lazy_static! {
    static ref PATH: Mutex<String> = Mutex::new("/Users/dev523/data/test".to_string());
}
#[tauri::command]
pub fn get_images() -> Vec<String> {
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
pub fn get_image(name: String) {
    let path = PATH.lock();
    if path.is_ok() {
        let path = path.unwrap().to_string();
        let base64 = image_base64::to_base64(&format!("{path}/{name}"));
        println!("{base64}")
    }
}

#[tauri::command]
pub fn get_class() -> Vec<String> {
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
pub fn get_folder() -> String {
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
