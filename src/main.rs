mod offset;
mod titlebar;

use offset::WindowExt;
use tauri::Manager;
use titlebar::TitleBar;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

const INIT_SCRIPT: &str = include_str!("inject.js");

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&win, NSVisualEffectMaterial::Sidebar, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            win.set_transparent_titlebar(true);
            win.position_traffic_lights(18.0, 22.0);

            Ok(())
        })
        .on_page_load(|window, _| window.app_handle().get_window("main").unwrap().eval(INIT_SCRIPT).unwrap())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
