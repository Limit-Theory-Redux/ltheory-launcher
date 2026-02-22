// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::StreamExt;
use reqwest::Client;
use std::env;
use std::io;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow, WindowEvent};
use tauri_plugin_opener::OpenerExt;
use tokio::{fs::File, io::AsyncWriteExt};
use winreg::enums::*;
use winreg::RegKey;

mod speed_calc;

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

    key.set_value("InstallDir", &install_path.to_str().unwrap())?;
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
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LTheoryRedux\LTheoryRedux";

    let key = hklm.open_subkey(path)?;

    let install_path_string: String = key.get_value("InstallDir")?;

    Ok(install_path_string)
}

#[derive(serde::Serialize)]
struct GameInfo {
    installed: bool,
    version: Option<String>,
    states: Vec<String>,
}

#[tauri::command]
#[cfg(target_os = "windows")]
async fn get_game_info() -> Result<GameInfo, String> {
    let install_path = match get_installation_path_internal() {
        Ok(path) => path,
        Err(_) => return Ok(GameInfo {
            installed: false,
            version: None,
            states: vec!["LTheoryRedux".to_string()],
        }),
    };

    let binary_path = Path::new(&install_path).join("bin").join("ltr.exe");
    
    if !binary_path.exists() {
        return Ok(GameInfo {
            installed: false,
            version: None,
            states: vec!["LTheoryRedux".to_string()],
        });
    }

    let version = get_game_version_internal(&install_path).ok();
    let states = get_available_states_internal(&install_path);

    Ok(GameInfo {
        installed: true,
        version,
        states,
    })
}

#[cfg(target_os = "windows")]
fn get_game_version_internal(install_path: &str) -> Result<String, String> {
    let version_path = Path::new(install_path)
        .join("script")
        .join("Config")
        .join("Version.lua");

    let content = std::fs::read_to_string(&version_path)
        .map_err(|e| format!("Failed to read Version.lua: {}", e))?;

    for line in content.lines() {
        if line.contains("Config.gameVersion") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    return Ok(line[start + 1..start + 1 + end].to_string());
                }
            }
        }
    }

    Err("Version not found in Version.lua".to_string())
}

#[cfg(target_os = "windows")]
fn get_available_states_internal(install_path: &str) -> Vec<String> {
    let mut states = vec!["LTheoryRedux".to_string()];
    
    let states_path = Path::new(install_path)
        .join("script")
        .join("States")
        .join("App");

    if let Ok(entries) = std::fs::read_dir(&states_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(children) = std::fs::read_dir(&path) {
                    for child in children.flatten() {
                        let child_path = child.path();
                        if let Some(ext) = child_path.extension() {
                            if ext == "lua" {
                                if let Some(name) = child_path.file_stem() {
                                    states.push(name.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    states
}

#[tauri::command]
#[cfg(target_os = "windows")]
async fn check_config_exists() -> Result<bool, String> {
    let config_path = get_config_path()?;
    Ok(config_path.exists())
}

#[tauri::command]
async fn open_config(app: AppHandle) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let config_path = get_config_path()?;
        if config_path.exists() {
            app.opener()
                .open_path(config_path.to_string_lossy().to_string(), None::<&str>)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn get_config_path() -> Result<std::path::PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "Could not find config directory".to_string())?;
    Ok(config_dir
        .join("LTheoryRedux")
        .join("LTheoryRedux")
        .join("data")
        .join("user.ini"))
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn launch_game(app: AppHandle, state: &str) {
    if !state.chars().all(|c| c.is_alphanumeric() || c == '_') {
        eprintln!("Invalid state name: {}", state);
        return;
    }

    let mut binding = Command::new("cmd");

    let binary_path = r"bin\\ltr.exe";

    let dir = match get_installation_path_internal() {
        Ok(install_path) => install_path,
        Err(e) => panic!("Error while reading installation path registry key: {}", e),
    };

    let _game = binding
        .args(&["/C", "start", "", &binary_path, state])
        .creation_flags(winapi::um::winbase::DETACHED_PROCESS)
        .current_dir(&dir)
        .spawn()
        .expect("Failed to start LTheoryRedux");

    app.exit(0)
}

#[tauri::command]
#[cfg(target_os = "windows")]
async fn download_game(app: AppHandle, install_path: &str) -> Result<(), String> {
    let install_path = Path::new(install_path);
    
    let canonical_install_path = install_path
        .canonicalize()
        .map_err(|e| format!("Invalid install path: {}", e))?;
    
    if !canonical_install_path.is_absolute() {
        return Err("Install path must be absolute".to_string());
    }
    
    if canonical_install_path.to_string_lossy().contains("..") {
        return Err("Invalid path".to_string());
    }

    let client = Client::new();
    let temp_dir = std::env::temp_dir();

    let url = String::from_str("https://github.com/Limit-Theory-Redux/ltheory/releases/download/latest/ltheory-windows.zip").unwrap();
    let dl_file_path = temp_dir.join("ltheory-windows.zip");
    let installation_path = canonical_install_path.join("Limit Theory Redux");

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| format!("Get error for: '{}'", &url))?;

    let total_size = response
        .content_length()
        .ok_or_else(|| format!("Total length of '{}' not accessible", &url))?;

    let mut file = File::create(&dl_file_path)
        .await
        .map_err(|_| format!("Error while creating '{}'", dl_file_path.display()))?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    let mut speed_calculator = speed_calc::SpeedCalculator::new(5000);
    let mut start_time = std::time::Instant::now();
    let mut last_speed_emit_time = std::time::Instant::now();
    let mut last_downloaded = 0_u64;

    let Some(main_window) = app.get_webview_window("main") else {
        return Ok(());
    };

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|_| "Error while downloading file")?;
        file.write_all(&chunk)
            .await
            .map_err(|_| "Error while writing file")?;
        downloaded += chunk.len() as u64;

        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        let elapsed_time = start_time.elapsed().as_secs_f64();

        main_window
            .emit("download-progress", progress)
            .map_err(|e| e.to_string())?;

        if elapsed_time > 0.0 {
            let speed = ((downloaded - last_downloaded) as f64 / 1024.0) / elapsed_time;
            speed_calculator.add_speed(speed);
            let average_speed = speed_calculator.average_speed();

            if last_speed_emit_time.elapsed() > std::time::Duration::new(0, 250000000) {
                println!(
                        "Downloaded: {} | Total size: {} | Progress: {:.2}% | Average Download speed: {:.2} KB/s",
                        downloaded, total_size, progress, average_speed
                    );

                main_window
                    .emit("download-speed", average_speed)
                    .map_err(|e| e.to_string())?;

                last_speed_emit_time = std::time::Instant::now();
            }

            last_downloaded = downloaded;
            start_time = std::time::Instant::now();
        }

        if downloaded == total_size {
            if let Ok(dir) = std::fs::read_dir(&installation_path) {
                delete_directory_contents(dir);
                println!("Successfully deleted old installation contents.");
            }

            main_window
                .emit("download-extracting", ())
                .map_err(|e| e.to_string())?;

            match extract_zip(&dl_file_path, &installation_path, &main_window).await {
                Ok(_) => println!("Zip successfully extracted!"),
                Err(e) => panic!("{}{}", "Error while extracting Zip", e),
            }

            match std::fs::remove_file(&dl_file_path) {
                Ok(()) => println!("Downloaded zip deleted"),
                Err(e) => println!("Error while deleting downloaded zip: {}", e),
            }

            match save_installation_path(&installation_path) {
                Ok(_) => println!("Installation path registry key successfully created"),
                Err(e) => println!("Error while creating installation path registry key: {}", e),
            }

            main_window
                .emit("install-complete", ())
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

async fn extract_zip(
    zip_path: &Path,
    path: &Path,
    main_window: &WebviewWindow,
) -> Result<(), String> {
    if !path.exists() {
        match std::fs::create_dir(&path) {
            Ok(_) => match env::set_current_dir(&path) {
                Ok(_) => println!(
                    "Successfully changed working directory to {}!",
                    path.display()
                ),
                Err(e) => panic!("Error while switching working directory: {}", e),
            },
            Err(e) => panic!("{}", e),
        };
    } else {
        match env::set_current_dir(&path) {
            Ok(_) => println!(
                "Successfully changed working directory to {}!",
                path.display()
            ),
            Err(e) => panic!("Error while switching working directory: {}", e),
        }
    }

    let file = std::fs::File::open(&zip_path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    let archive_len = &archive.len();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        main_window
            .emit("extracting-files", archive_len - i)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn delete_directory_contents(dir: std::fs::ReadDir) {
    for entry in dir {
        let path = entry.unwrap().path();
        if path.is_dir() {
            delete_directory_contents(std::fs::read_dir(&path).unwrap());
            std::fs::remove_dir(&path).unwrap();
        } else {
            std::fs::remove_file(&path).unwrap();
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let Some(main_window) = app.get_webview_window("main") else {
                return Ok(());
            };

            main_window.on_window_event(move |event| match event {
                WindowEvent::Resized(..) => std::thread::sleep(std::time::Duration::from_nanos(1)),
                _ => {}
            });

            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit("single-instance", Payload { args: argv, cwd })
                .unwrap();

            let window = app.get_webview_window("main").unwrap();
            let window_visible = window.is_visible().unwrap();

            if !window_visible {
                window.show().unwrap();
            };
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_installation_path,
            get_game_info,
            check_config_exists,
            open_config,
            launch_game,
            download_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri applications");
}
