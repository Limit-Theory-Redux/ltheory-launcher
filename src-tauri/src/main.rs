// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use tauri::WindowEvent;
use tauri::{AppHandle, Manager, Runtime};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[cfg(target_os = "windows")]
fn save_installation_path(install_path: &Path) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LTheoryRedux\LTheoryRedux";
    let (key, _disp) = hklm.create_subkey(&path)?;

    key.set_value("InstallationPath", &install_path.to_str().unwrap())?;
    Ok(())
}

#[tauri::command]
#[cfg(target_os = "windows")]
async fn get_installation_path() -> Result<String, String> {
    match get_installation_path_internal() {
        Ok(path) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(target_os = "windows")]
fn get_installation_path_internal() -> io::Result<String> {
    let hklm = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LTheoryRedux\LTheoryRedux";

    let key = hklm.open_subkey(path)?;

    let install_path: String = key.get_value("InstallationPath")?;

    Ok(install_path)
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn launch_game<R: Runtime>(app: AppHandle<R>) {
    let mut binding = Command::new("cmd");

    let binary_path = r"bin\\lt64.exe";

    let dir = match get_installation_path_internal() {
        Ok(install_path) => install_path,
        Err(e) => panic!("Error while reading installation path registry key: {}", e),
    };

    let _game = binding
        .args(&["/C", "start", &binary_path])
        .creation_flags(winapi::um::winbase::DETACHED_PROCESS) // ONLY WINDOWS;
        .current_dir(&dir)
        .spawn()
        .expect("Failed to start LTheoryRedux");

    // Exit Launcher
    app.exit(0)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let Some(main_window) = app.get_window("main") else {
                return Ok(());
            };

            // try to mitigate resize lag
            main_window.on_window_event(|event| match event {
                WindowEvent::Resized(..) => std::thread::sleep(std::time::Duration::from_nanos(1)),
                _ => {}
            });

            //TODO: show when frontend is loaded
            tauri::async_runtime::spawn(async move {
                // ! dirty code :d
                std::thread::sleep(std::time::Duration::from_secs(5));
                main_window.show().unwrap();
            });

            //TODO: this is temporary, adjust to whatever the user inputs for installation & at installation
            let install_path = Path::new("Z:\\ltheory-redux");

            match save_installation_path(install_path) {
                Ok(_) => println!("Installation path registry key successfully created"),
                Err(e) => println!("Error while creating installation path registry key: {}", e),
            }

            match get_installation_path_internal() {
                Ok(install_path) => println!("Installation path: {}", install_path),
                Err(e) => eprintln!("Error while reading installation path registry key: {}", e),
            }

            Ok(())
        })
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
        .invoke_handler(tauri::generate_handler![get_installation_path, launch_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri applications");
}
