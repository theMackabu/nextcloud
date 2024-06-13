#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod offset;
mod titlebar;

use offset::WindowExt;
use tauri::Manager;
use titlebar::TitleBar;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

const INIT_SCRIPT: &str = r#"
const dragRegionProperties = {
    zIndex: 1000,
    height: '18px',
    width: '100%',
    position: 'fixed',
    top: '0',
    left: '0'
};

const smallRegionProperties = {
    zIndex: 1000,
    height: '50px',
    width: '39%',
    position: 'fixed',
    top: '0',
    left: '45%'
};

document.addEventListener('DOMContentLoaded', () => {    
    const styleElement = document.createElement('style');
    const cssRule = 'html, body { background: transparent; }';
    
    styleElement.appendChild(document.createTextNode(cssRule));
    document.head.appendChild(styleElement);
    
    const dragRegion = document.createElement('div');
    const smallDragRegion = document.createElement('div');
    
    const logo = document.querySelector('.logo-icon');
    const anchor = document.getElementById('nextcloud');
    const header = document.querySelector('div.header-left');
    
    if (logo) { logo.parentNode.removeChild(logo); }
    if (anchor) { anchor.parentNode.removeChild(anchor); }
    if (header) { header.style.marginLeft = '100px'; }
    
    Object.assign(dragRegion.style, dragRegionProperties);
    dragRegion.setAttribute('data-tauri-drag-region', '');
    document.documentElement.appendChild(dragRegion);
    
    Object.assign(smallDragRegion.style, smallRegionProperties);
    smallDragRegion.setAttribute('data-tauri-drag-region', '');
    document.documentElement.appendChild(smallDragRegion);
});
"#;

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
