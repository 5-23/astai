use tauri::Window;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
pub mod commands;
pub mod menu;

pub trait AllowAlpha {
    fn allow_alpha(&self) {}
}
impl AllowAlpha for Window {
    fn allow_alpha(&self) {
        #[cfg(target_os = "macos")]
        apply_vibrancy(self, NSVisualEffectMaterial::HudWindow, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

        #[cfg(target_os = "windows")]
        apply_acrylic(self, Some((18, 18, 18, 125)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
    }
}
