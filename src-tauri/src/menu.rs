use tauri::{Manager, WindowMenuEvent};

pub fn menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
            event.window().hide().unwrap();
        }
        "settings" => {
            let setting = event.window().get_window("setting").unwrap();
            setting.show().unwrap();
        }
        _ => {}
    }
}
