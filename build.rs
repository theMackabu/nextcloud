fn main() {
    #[cfg(not(target_os = "macos"))]
    compile_error!("This project is only supported on MacOS.");
    tauri_build::build()
}
