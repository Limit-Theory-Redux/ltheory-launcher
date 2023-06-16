// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, Manager, Runtime
};
use std::process::Command;
use std::os::windows::process::CommandExt;

const DETACHED_PROCESS: u32 = 0x00000008;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[tauri::command]
fn launch_game<R: Runtime>(app: AppHandle<R>, dir: String, path: String) {
    let mut binding = Command::new("cmd");
    let _game = binding.args(&["/C", "start", &path])
        .creation_flags(DETACHED_PROCESS) // ONLY WINDOWS;
        .current_dir(&dir)
        .spawn().expect("Failed to start LTheoryRedux");

    // Exit Launcher
    app.exit(0)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();

            // Show Window
            let window = app.get_window("main").unwrap();
            let window_visible = window.is_visible().unwrap();

            if !window_visible {
                window.show().unwrap();
            };
        }))
        .invoke_handler(tauri::generate_handler![launch_game])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            _ => {}
        })
}
