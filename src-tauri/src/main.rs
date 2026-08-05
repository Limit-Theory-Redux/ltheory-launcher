// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::StreamExt;
use minisign_verify::{PublicKey, Signature};
use reqwest::{redirect::Policy, Client};
use std::io::{self, BufRead, BufReader, Read};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    mpsc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::Color,
    AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_opener::OpenerExt;
use tokio::{fs::File, io::AsyncWriteExt};
use winapi::shared::{minwindef::BOOL, windef::HWND};
use winapi::um::winuser::{
    EnumWindows, GetWindow, GetWindowThreadProcessId, IsWindowVisible, SetForegroundWindow,
    ShowWindow, GW_OWNER, SW_RESTORE,
};
use winreg::enums::*;
use winreg::RegKey;

mod speed_calc;

const GAME_RELEASE_DOWNLOAD_BASE: &str =
    "https://github.com/Limit-Theory-Redux/ltheory/releases/download";
const GAME_ARCHIVE_NAME: &str = "ltheory-windows.zip";
const GAME_SIGNATURE_NAME: &str = "ltheory-windows.zip.minisig";
const GAME_SIGNING_PUBLIC_KEY: &str = include_str!("../keys/game-release.pub");
const LAUNCHER_TRAY_ID: &str = "launcher-tray";
const MAX_DOWNLOAD_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const MAX_SIGNATURE_BYTES: usize = 16 * 1024;
const MAX_EXTRACTED_BYTES: u64 = 12 * 1024 * 1024 * 1024;
const MAX_ARCHIVE_ENTRIES: usize = 200_000;

fn game_asset_url(release_tag: &str, asset_name: &str) -> Result<String, String> {
    if release_tag.is_empty()
        || release_tag.len() > 128
        || !release_tag
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err("The selected game release tag is invalid".to_string());
    }

    Ok(format!(
        "{GAME_RELEASE_DOWNLOAD_BASE}/{release_tag}/{asset_name}"
    ))
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[derive(Clone, serde::Serialize)]
struct LaunchOutput {
    stream: String,
    line: String,
}

struct WindowSearch {
    process_id: u32,
    window: HWND,
}

#[derive(Default)]
struct PendingLaunch {
    state: Option<String>,
    window_label: Option<String>,
}

#[derive(Default)]
struct LaunchCoordinator {
    next_window_id: AtomicU64,
    pending: Mutex<PendingLaunch>,
    last_log: Mutex<GameLaunchLog>,
}

#[derive(Default)]
struct GameLaunchLog {
    state: Option<String>,
    entries: Vec<LaunchOutput>,
    running: bool,
    exit_code: Option<i32>,
    failure: Option<String>,
}

fn begin_game_launch_log(app: &AppHandle, state: &str) -> Result<(), String> {
    let coordinator = app.state::<LaunchCoordinator>();
    let mut log = coordinator
        .last_log
        .lock()
        .map_err(|_| "Could not lock the game launch log".to_string())?;
    *log = GameLaunchLog {
        state: Some(state.to_string()),
        entries: Vec::new(),
        running: true,
        exit_code: None,
        failure: None,
    };
    Ok(())
}

fn append_game_launch_output(app: &AppHandle, output: &LaunchOutput) {
    if let Ok(mut log) = app.state::<LaunchCoordinator>().last_log.lock() {
        log.entries.push(output.clone());
    }
}

fn finish_game_launch_log(app: &AppHandle, exit_code: Option<i32>, failure: Option<String>) {
    if let Ok(mut log) = app.state::<LaunchCoordinator>().last_log.lock() {
        log.running = false;
        log.exit_code = exit_code;
        log.failure = failure;
    }
}

#[tauri::command]
fn get_last_game_launch_log(app: AppHandle) -> Result<String, String> {
    let coordinator = app.state::<LaunchCoordinator>();
    let log = coordinator
        .last_log
        .lock()
        .map_err(|_| "Could not lock the game launch log".to_string())?;
    if log.state.is_none() && log.entries.is_empty() {
        return Err("No game launch log is available yet".to_string());
    }

    let mut lines = vec![
        "Limit Theory Redux game launch log".to_string(),
        format!("State: {}", log.state.as_deref().unwrap_or("unknown")),
        format!(
            "Status: {}",
            if log.running { "running" } else { "finished" }
        ),
    ];
    if let Some(exit_code) = log.exit_code {
        lines.push(format!("Exit code: {exit_code}"));
    }
    if let Some(failure) = &log.failure {
        lines.push(format!("Failure: {failure}"));
    }
    lines.push(String::new());
    lines.push("Process output:".to_string());
    if log.entries.is_empty() {
        lines.push("(no process output captured)".to_string());
    } else {
        lines.extend(log.entries.iter().map(|entry| {
            if entry.line.starts_with('[') {
                entry.line.clone()
            } else {
                format!("[{}] {}", entry.stream, entry.line)
            }
        }));
    }
    Ok(lines.join("\n"))
}

#[cfg(target_os = "windows")]
fn normalized_windows_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    if let Some(unc_path) = value.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{unc_path}")
    } else {
        value.strip_prefix(r"\\?\").unwrap_or(&value).to_string()
    }
}

#[cfg(target_os = "windows")]
fn save_installation_path(install_path: &Path) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"SOFTWARE\LTheoryRedux\LTheoryRedux";
    let (key, _disp) = hklm.create_subkey(path)?;

    key.set_value("InstallDir", &normalized_windows_path(install_path))?;
    Ok(())
}

#[tauri::command]
#[cfg(target_os = "windows")]
async fn set_game_installation_path(
    window: WebviewWindow,
    install_path: &str,
) -> Result<String, String> {
    if window.label() != "main" {
        return Err("The game location can only be changed from launcher settings".to_string());
    }

    let selected_path = Path::new(install_path);
    let canonical_path = selected_path
        .canonicalize()
        .map_err(|_| "The selected game folder does not exist".to_string())?;
    if !canonical_path.is_absolute() {
        return Err("The game folder must be an absolute path".to_string());
    }
    if !canonical_path.join("bin").join("ltr.exe").is_file() {
        return Err(
            "This folder is not a Limit Theory Redux installation (bin/ltr.exe is missing)"
                .to_string(),
        );
    }

    save_installation_path(&canonical_path)
        .map_err(|error| format!("Could not save the game location: {error}"))?;
    Ok(normalized_windows_path(&canonical_path))
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
        Err(_) => {
            return Ok(GameInfo {
                installed: false,
                version: None,
                states: vec!["LTheoryRedux".to_string()],
            })
        }
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
async fn open_config(app: AppHandle, window: WebviewWindow) -> Result<(), String> {
    if window.label() != "main" {
        return Err("Configuration can only be opened from the main launcher window".to_string());
    }
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
    let config_dir =
        dirs::config_dir().ok_or_else(|| "Could not find config directory".to_string())?;
    Ok(config_dir
        .join("LTheoryRedux")
        .join("LTheoryRedux")
        .join("data")
        .join("user.ini"))
}

#[tauri::command]
#[cfg(target_os = "windows")]
async fn prepare_game_launch(
    app: AppHandle,
    window: WebviewWindow,
    state: &str,
) -> Result<(), String> {
    if window.label() != "main" {
        return Err("Game launch can only be started from the main launcher window".to_string());
    }
    if !state.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Invalid state name".to_string());
    }

    let dir = get_installation_path_internal().map_err(|error| error.to_string())?;
    let binary_path = Path::new(&dir).join("bin").join("ltr.exe");
    if !binary_path.is_file() {
        return Err("Game executable was not found".to_string());
    }

    let coordinator = app.state::<LaunchCoordinator>();
    let (startup_label, reuse_existing) = {
        let mut pending = coordinator
            .pending
            .lock()
            .map_err(|_| "Could not lock the pending game launch state".to_string())?;
        if let Some(label) = pending.window_label.clone() {
            (label, true)
        } else {
            let window_id = coordinator.next_window_id.fetch_add(1, Ordering::Relaxed) + 1;
            let label = format!("game-startup-{window_id}");
            pending.state = Some(state.to_string());
            pending.window_label = Some(label.clone());
            (label, false)
        }
    };

    // Launch preparation is intentionally idempotent. A fast double click or a
    // second frontend request shares the in-flight startup window. Each actual
    // launch receives a fresh label so WebView event registrations cannot leak
    // into the next launch after a window is destroyed.
    if reuse_existing {
        if let Some(existing) = app.get_webview_window(&startup_label) {
            let _ = existing.set_focus();
        }
        return Ok(());
    }

    let build_result =
        WebviewWindowBuilder::new(&app, &startup_label, WebviewUrl::App("index.html".into()))
            .title("Starting Limit Theory Redux")
            .inner_size(640.0, 420.0)
            .background_color(Color(5, 11, 17, 255))
            .visible(false)
            .resizable(false)
            .maximizable(false)
            .minimizable(false)
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .center()
            .build();

    if let Err(error) = build_result {
        clear_pending_launch_if_label(&app, &startup_label);
        return Err(format!("Could not create the game startup window: {error}"));
    }

    Ok(())
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn show_game_startup(app: AppHandle, window: WebviewWindow) -> Result<(), String> {
    ensure_active_startup_window(&app, window.label())?;

    set_launcher_tray_visible(&app, true)?;

    let main_window = app
        .get_webview_window("main")
        .ok_or_else(|| "The main launcher window is unavailable".to_string())?;
    main_window
        .hide()
        .map_err(|error| format!("Could not hide the launcher: {error}"))?;

    if let Err(error) = window.show() {
        let _ = main_window.show();
        let _ = set_launcher_tray_visible(&app, false);
        return Err(format!("Could not show the game startup screen: {error}"));
    }
    let _ = window.set_focus();
    Ok(())
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn dismiss_game_startup(app: AppHandle, window: WebviewWindow) -> Result<(), String> {
    ensure_active_startup_window(&app, window.label())?;
    let startup_label = window.label().to_string();
    window
        .close()
        .map_err(|error| format!("Could not close the startup window: {error}"))?;
    clear_pending_launch_if_label(&app, &startup_label);
    restore_main_window(&app);
    Ok(())
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn launch_game(app: AppHandle, window: WebviewWindow) -> Result<(), String> {
    let startup_label = window.label().to_string();
    let state = {
        let coordinator = app.state::<LaunchCoordinator>();
        let mut pending = coordinator
            .pending
            .lock()
            .map_err(|_| "Could not lock the pending game launch state".to_string())?;
        if pending.window_label.as_deref() != Some(startup_label.as_str()) {
            return Err(
                "The game process must be started from the active startup window".to_string(),
            );
        }
        pending
            .state
            .take()
            .ok_or_else(|| "No game launch is pending".to_string())?
    };
    if !state.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Invalid state name".to_string());
    }

    let dir = get_installation_path_internal().map_err(|error| error.to_string())?;
    let binary_path = Path::new(&dir).join("bin").join("ltr.exe");
    if !binary_path.is_file() {
        return Err("Game executable was not found".to_string());
    }

    let ready_marker_path = Path::new(&dir)
        .join("script")
        .join("States")
        .join("App")
        .join("LTheoryRedux.lua");
    let supports_ready_marker = std::fs::read_to_string(ready_marker_path)
        .map(|content| content.contains("LTR_LAUNCHER_READY"))
        .unwrap_or(false);

    begin_game_launch_log(&app, &state)?;
    let child_result = Command::new(&binary_path)
        .arg(&state)
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .current_dir(&dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match child_result {
        Ok(child) => child,
        Err(error) => {
            let message = format!("Failed to start Limit Theory Redux: {error}");
            finish_game_launch_log(&app, None, Some(message.clone()));
            return Err(message);
        }
    };

    let process_id = child.id();
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Could not capture game output".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "Could not capture game errors".to_string())?;
    let (sender, receiver) = mpsc::channel::<LaunchOutput>();
    let stdout_thread = stream_game_output(stdout, "stdout", sender.clone());
    let stderr_thread = stream_game_output(stderr, "stderr", sender);

    let _ = app.emit("game-launch-status", "Initializing game engine");
    let started_output = LaunchOutput {
        stream: "launcher".to_string(),
        line: format!(
            "[launcher] Started {} (process {process_id})",
            binary_path.display()
        ),
    };
    append_game_launch_output(&app, &started_output);
    let _ = app.emit("game-launch-output", started_output);

    let monitor_app = app.clone();
    thread::spawn(move || {
        let mut ready_marker_seen = false;
        let mut handoff_complete = false;
        let mut visible_since: Option<Instant> = None;
        let mut last_status = "Initializing game engine".to_string();
        let mut stdout_thread = Some(stdout_thread);
        let mut stderr_thread = Some(stderr_thread);

        loop {
            while let Ok(output) = receiver.try_recv() {
                forward_game_launch_output(
                    &monitor_app,
                    output,
                    &mut ready_marker_seen,
                    &mut last_status,
                );
            }

            match child.try_wait() {
                Ok(Some(status)) => {
                    if let Some(handle) = stdout_thread.take() {
                        let _ = handle.join();
                    }
                    if let Some(handle) = stderr_thread.take() {
                        let _ = handle.join();
                    }
                    while let Ok(output) = receiver.try_recv() {
                        forward_game_launch_output(
                            &monitor_app,
                            output,
                            &mut ready_marker_seen,
                            &mut last_status,
                        );
                    }
                    if handoff_complete {
                        let failure = (!status.success()).then(|| {
                            status
                                .code()
                                .map(|code| format!("The game exited with code {code}."))
                                .unwrap_or_else(|| {
                                    "The game process terminated unexpectedly.".to_string()
                                })
                        });
                        finish_game_launch_log(&monitor_app, status.code(), failure);
                        let _ = monitor_app.emit("game-launch-exited", status.code());
                        restore_main_window(&monitor_app);
                    } else {
                        let message = status
                            .code()
                            .map(|code| format!("The game exited during startup with code {code}."))
                            .unwrap_or_else(|| {
                                "The game stopped before startup completed.".to_string()
                            });
                        finish_game_launch_log(&monitor_app, status.code(), Some(message.clone()));
                        let _ = monitor_app.emit("game-launch-failed", message);
                    }
                    break;
                }
                Ok(None) => {}
                Err(error) => {
                    let message = format!("Could not monitor the game process: {error}");
                    finish_game_launch_log(&monitor_app, None, Some(message.clone()));
                    let _ = monitor_app.emit("game-launch-failed", message);
                    break;
                }
            }

            if !handoff_complete {
                if let Some(game_window) = find_process_window(process_id) {
                    let first_seen = visible_since.get_or_insert_with(Instant::now);
                    let fallback_ready =
                        !supports_ready_marker && first_seen.elapsed() >= Duration::from_secs(2);
                    if ready_marker_seen || fallback_ready {
                        let _ = monitor_app.emit("game-launch-status", "Game ready");
                        let ready_output = LaunchOutput {
                            stream: "launcher".to_string(),
                            line: "[launcher] Game window is ready".to_string(),
                        };
                        append_game_launch_output(&monitor_app, &ready_output);
                        let _ = monitor_app.emit("game-launch-output", ready_output);
                        foreground_game_window(game_window);
                        if let Some(startup_window) = monitor_app.get_webview_window(&startup_label)
                        {
                            let _ = startup_window.close();
                        }
                        clear_pending_launch_if_label(&monitor_app, &startup_label);
                        handoff_complete = true;
                    }
                } else {
                    visible_since = None;
                }
            }

            thread::sleep(Duration::from_millis(60));
        }
    });

    Ok(())
}

#[cfg(target_os = "windows")]
fn forward_game_launch_output(
    app: &AppHandle,
    mut output: LaunchOutput,
    ready_marker_seen: &mut bool,
    last_status: &mut String,
) {
    if output.line.contains("LTR_LAUNCHER_READY") {
        *ready_marker_seen = true;
        output.line = "[game] Main menu is ready".to_string();
    }
    if let Some(status) = startup_status_for_line(&output.line) {
        if status != *last_status {
            *last_status = status.to_string();
            let _ = app.emit("game-launch-status", status);
        }
    }
    append_game_launch_output(app, &output);
    let _ = app.emit("game-launch-output", output);
}

#[cfg(target_os = "windows")]
fn stream_game_output<R>(
    reader: R,
    stream: &'static str,
    sender: mpsc::Sender<LaunchOutput>,
) -> thread::JoinHandle<()>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        for line in BufReader::new(reader).lines() {
            let Ok(line) = line else { break };
            let line = sanitize_console_line(&line);
            if line.is_empty() {
                continue;
            }
            if sender
                .send(LaunchOutput {
                    stream: stream.to_string(),
                    line,
                })
                .is_err()
            {
                break;
            }
        }
    })
}

#[cfg(target_os = "windows")]
fn sanitize_console_line(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(character) = chars.next() {
        if character == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if ('@'..='~').contains(&code) {
                    break;
                }
            }
            continue;
        }
        if !character.is_control() || character == '\t' {
            result.push(character);
        }
    }
    result.trim().to_string()
}

#[cfg(target_os = "windows")]
fn startup_status_for_line(line: &str) -> Option<&'static str> {
    let line = line.to_ascii_lowercase();
    if line.contains("ltr_launcher_ready") || line.contains("main_menu") {
        Some("Preparing the main menu")
    } else if line.contains("star system") || line.contains("universe") {
        Some("Generating the main-menu universe")
    } else if line.contains("music") || line.contains("audio") || line.contains("sound") {
        Some("Loading audio systems")
    } else if line.contains("config") || line.contains("lua") || line.contains("script") {
        Some("Loading game scripts")
    } else if line.contains("render") || line.contains("window") || line.contains("graphics") {
        Some("Initializing graphics")
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn find_window_callback(window: HWND, parameter: isize) -> BOOL {
    let search = &mut *(parameter as *mut WindowSearch);
    let mut process_id = 0;
    GetWindowThreadProcessId(window, &mut process_id);
    if process_id == search.process_id
        && IsWindowVisible(window) != 0
        && GetWindow(window, GW_OWNER).is_null()
    {
        search.window = window;
        return 0;
    }
    1
}

#[cfg(target_os = "windows")]
fn find_process_window(process_id: u32) -> Option<HWND> {
    let mut search = WindowSearch {
        process_id,
        window: std::ptr::null_mut(),
    };
    unsafe {
        EnumWindows(
            Some(find_window_callback),
            &mut search as *mut WindowSearch as isize,
        );
    }
    (!search.window.is_null()).then_some(search.window)
}

#[cfg(target_os = "windows")]
fn foreground_game_window(window: HWND) {
    unsafe {
        ShowWindow(window, SW_RESTORE);
        SetForegroundWindow(window);
    }
}

#[cfg(target_os = "windows")]
fn restore_main_window(app: &AppHandle) {
    clear_pending_launch_state(app);
    let _ = set_launcher_tray_visible(app, false);
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.emit("game-launch-reset", ());
        let _ = main_window.show();
        let _ = main_window.unminimize();
        let _ = main_window.set_focus();
    }
}

fn clear_pending_launch_state(app: &AppHandle) {
    if let Ok(mut pending) = app.state::<LaunchCoordinator>().pending.lock() {
        pending.state.take();
        pending.window_label.take();
    }
}

fn clear_pending_launch_if_label(app: &AppHandle, window_label: &str) {
    if let Ok(mut pending) = app.state::<LaunchCoordinator>().pending.lock() {
        if pending.window_label.as_deref() == Some(window_label) {
            pending.state.take();
            pending.window_label.take();
        }
    }
}

fn ensure_active_startup_window(app: &AppHandle, window_label: &str) -> Result<(), String> {
    let coordinator = app.state::<LaunchCoordinator>();
    let pending = coordinator
        .pending
        .lock()
        .map_err(|_| "Could not lock the pending game launch state".to_string())?;
    if pending.window_label.as_deref() == Some(window_label) {
        Ok(())
    } else {
        Err("This is not the active game startup window".to_string())
    }
}

fn show_active_launcher_window(app: &AppHandle) {
    let startup_label = app
        .state::<LaunchCoordinator>()
        .pending
        .lock()
        .ok()
        .and_then(|pending| pending.window_label.clone());
    if let Some(label) = startup_label {
        if let Some(startup_window) = app.get_webview_window(&label) {
            let _ = startup_window.show();
            let _ = startup_window.set_focus();
            return;
        }
    }
    restore_main_window(app);
}

fn set_launcher_tray_visible(app: &AppHandle, visible: bool) -> Result<(), String> {
    let tray = app
        .tray_by_id(LAUNCHER_TRAY_ID)
        .ok_or_else(|| "The launcher tray icon is unavailable".to_string())?;
    tray.set_visible(visible)
        .map_err(|error| format!("Could not update the launcher tray icon: {error}"))
}

#[tauri::command]
#[cfg(target_os = "windows")]
async fn download_game(
    app: AppHandle,
    window: WebviewWindow,
    install_path: Option<&str>,
    release_tag: &str,
    update_existing: bool,
) -> Result<(), String> {
    if window.label() != "main" {
        return Err(
            "Game installation can only be started from the main launcher window".to_string(),
        );
    }
    let (canonical_install_path, installation_path) = if update_existing {
        let registered_path = PathBuf::from(
            get_installation_path_internal()
                .map_err(|_| "No existing game installation is configured".to_string())?,
        );
        let installation_path = registered_path
            .canonicalize()
            .map_err(|_| "The configured game installation no longer exists".to_string())?;
        if !installation_path.join("bin").join("ltr.exe").is_file() {
            return Err("The configured game installation is incomplete".to_string());
        }
        let install_parent = installation_path
            .parent()
            .ok_or_else(|| "The configured game installation has no parent folder".to_string())?
            .to_path_buf();
        (install_parent, installation_path)
    } else {
        let selected_path = install_path
            .ok_or_else(|| "Choose a parent folder for the game installation".to_string())?;
        let install_parent = Path::new(selected_path)
            .canonicalize()
            .map_err(|error| format!("Invalid install path: {error}"))?;
        if !install_parent.is_absolute() {
            return Err("Install path must be absolute".to_string());
        }
        let installation_path = install_parent.join("Limit Theory Redux");
        (install_parent, installation_path)
    };

    let download_url = game_asset_url(release_tag, GAME_ARCHIVE_NAME)?;
    let signature_url = game_asset_url(release_tag, GAME_SIGNATURE_NAME)?;

    let client = Client::builder()
        .https_only(true)
        .redirect(Policy::limited(5))
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(30 * 60))
        .user_agent("LTheoryReduxLauncher/5")
        .build()
        .map_err(|error| format!("Could not initialize secure download: {error}"))?;

    let temp_file = tempfile::Builder::new()
        .prefix("ltr-download-")
        .suffix(".zip")
        .tempfile()
        .map_err(|error| format!("Could not create a temporary download: {error}"))?;
    let dl_file_path = temp_file.into_temp_path();
    let signature = download_game_signature(&client, &signature_url).await?;

    let response = client
        .get(download_url)
        .send()
        .await
        .map_err(|error| format!("Game download failed: {error}"))?
        .error_for_status()
        .map_err(|error| format!("Game download was rejected: {error}"))?;

    let total_size = response
        .content_length()
        .ok_or_else(|| "The download did not include a content length".to_string())?;
    if total_size == 0 || total_size > MAX_DOWNLOAD_BYTES {
        return Err(format!(
            "Refusing unexpected download size: {total_size} bytes"
        ));
    }

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
        downloaded = downloaded
            .checked_add(chunk.len() as u64)
            .ok_or_else(|| "Download size overflow".to_string())?;
        if downloaded > total_size || downloaded > MAX_DOWNLOAD_BYTES {
            return Err("Download exceeded the declared size".to_string());
        }

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
    }

    if downloaded != total_size {
        return Err(format!(
            "Download ended early: received {downloaded} of {total_size} bytes"
        ));
    }
    file.flush()
        .await
        .map_err(|error| format!("Could not flush downloaded file: {error}"))?;
    drop(file);

    verify_game_archive(&dl_file_path, &signature)?;

    main_window
        .emit("download-extracting", ())
        .map_err(|error| error.to_string())?;

    let staging_dir = tempfile::Builder::new()
        .prefix(".ltr-installing-")
        .tempdir_in(&canonical_install_path)
        .map_err(|error| format!("Could not create an installation staging directory: {error}"))?;
    extract_zip(&dl_file_path, staging_dir.path(), &main_window)?;

    let staged_binary = staging_dir.path().join("bin").join("ltr.exe");
    if !staged_binary.is_file() {
        return Err("Downloaded archive does not contain bin/ltr.exe".to_string());
    }

    let staging_path = staging_dir.keep();
    replace_installation(&staging_path, &installation_path)?;
    save_installation_path(&installation_path)
        .map_err(|error| format!("Could not save the installation path: {error}"))?;

    main_window
        .emit("install-complete", ())
        .map_err(|error| error.to_string())?;

    Ok(())
}

async fn download_game_signature(client: &Client, signature_url: &str) -> Result<String, String> {
    let response = client
        .get(signature_url)
        .send()
        .await
        .map_err(|error| format!("Game signature download failed: {error}"))?
        .error_for_status()
        .map_err(|error| format!("Game signature was rejected: {error}"))?;

    if response
        .content_length()
        .is_some_and(|length| length > MAX_SIGNATURE_BYTES as u64)
    {
        return Err("Refusing an unexpectedly large game signature".to_string());
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("Could not read the game signature: {error}"))?;
    if bytes.is_empty() || bytes.len() > MAX_SIGNATURE_BYTES {
        return Err("The game signature has an invalid size".to_string());
    }

    String::from_utf8(bytes.to_vec())
        .map_err(|_| "The game signature is not valid UTF-8".to_string())
}

fn verify_game_archive(archive_path: &Path, signature_text: &str) -> Result<(), String> {
    let public_key = PublicKey::decode(GAME_SIGNING_PUBLIC_KEY)
        .map_err(|error| format!("Embedded game signing key is invalid: {error}"))?;
    let signature = Signature::decode(signature_text)
        .map_err(|error| format!("Downloaded game signature is invalid: {error}"))?;
    let mut verifier = public_key
        .verify_stream(&signature)
        .map_err(|error| format!("Could not initialize game signature verification: {error}"))?;
    let mut archive = std::fs::File::open(archive_path)
        .map_err(|error| format!("Could not open the game archive for verification: {error}"))?;
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let bytes_read = archive
            .read(&mut buffer)
            .map_err(|error| format!("Could not verify the game archive: {error}"))?;
        if bytes_read == 0 {
            break;
        }
        verifier.update(&buffer[..bytes_read]);
    }

    verifier
        .finalize()
        .map_err(|error| format!("Game archive signature verification failed: {error}"))
}

fn extract_zip(
    zip_path: &Path,
    destination: &Path,
    main_window: &WebviewWindow,
) -> Result<(), String> {
    let file = std::fs::File::open(zip_path)
        .map_err(|error| format!("Could not open downloaded archive: {error}"))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|error| format!("Downloaded file is not a valid ZIP archive: {error}"))?;
    let archive_len = archive.len();
    if archive_len == 0 || archive_len > MAX_ARCHIVE_ENTRIES {
        return Err(format!("Refusing archive with {archive_len} entries"));
    }

    let mut extracted_bytes = 0_u64;

    for index in 0..archive_len {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("Could not read archive entry {index}: {error}"))?;
        let relative_path = entry
            .enclosed_name()
            .ok_or_else(|| format!("Archive entry {} has an unsafe path", entry.name()))?
            .to_owned();

        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170000 == 0o120000)
        {
            return Err(format!("Archive entry {} is a symbolic link", entry.name()));
        }

        extracted_bytes = extracted_bytes
            .checked_add(entry.size())
            .ok_or_else(|| "Extracted size overflow".to_string())?;
        if extracted_bytes > MAX_EXTRACTED_BYTES {
            return Err("Archive expands beyond the allowed size".to_string());
        }

        let output_path = destination.join(relative_path);
        if entry.is_dir() {
            std::fs::create_dir_all(&output_path)
                .map_err(|error| format!("Could not create {}: {error}", output_path.display()))?;
        } else {
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|error| format!("Could not create {}: {error}", parent.display()))?;
            }
            let mut output = std::fs::File::create(&output_path)
                .map_err(|error| format!("Could not create {}: {error}", output_path.display()))?;
            io::copy(&mut entry, &mut output)
                .map_err(|error| format!("Could not extract {}: {error}", output_path.display()))?;
        }

        main_window
            .emit("extracting-files", archive_len - index - 1)
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn replace_installation(staging_path: &Path, installation_path: &Path) -> Result<(), String> {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("System clock error: {error}"))?
        .as_millis();
    let parent = installation_path
        .parent()
        .ok_or_else(|| "Installation path has no parent directory".to_string())?;
    let backup_path: PathBuf = parent.join(format!(".ltr-backup-{suffix}"));
    let had_existing_installation = installation_path.exists();

    if had_existing_installation {
        std::fs::rename(installation_path, &backup_path)
            .map_err(|error| format!("Could not stage the existing installation: {error}"))?;
    }

    if let Err(error) = std::fs::rename(staging_path, installation_path) {
        if had_existing_installation {
            let _ = std::fs::rename(&backup_path, installation_path);
        }
        return Err(format!("Could not activate the new installation: {error}"));
    }

    if had_existing_installation {
        std::fs::remove_dir_all(&backup_path).map_err(|error| {
            format!("Installed successfully, but could not remove backup: {error}")
        })?;
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(LaunchCoordinator::default())
        .setup(|app| {
            let open_launcher =
                MenuItem::with_id(app, "open-launcher", "Open Launcher", true, None::<&str>)?;
            let quit_launcher =
                MenuItem::with_id(app, "quit-launcher", "Quit Launcher", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&open_launcher, &quit_launcher])?;
            let mut tray_builder = TrayIconBuilder::with_id(LAUNCHER_TRAY_ID)
                .tooltip("Limit Theory Redux Launcher")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "open-launcher" => show_active_launcher_window(app),
                    "quit-launcher" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if matches!(
                        event,
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        }
                    ) {
                        show_active_launcher_window(tray.app_handle());
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            let tray = tray_builder.build(app)?;
            tray.set_visible(false)?;

            let Some(main_window) = app.get_webview_window("main") else {
                return Ok(());
            };

            main_window.on_window_event(move |event| {
                if let WindowEvent::Resized(..) = event {
                    std::thread::sleep(std::time::Duration::from_nanos(1));
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit("single-instance", Payload { args: argv, cwd })
                .unwrap();

            show_active_launcher_window(app);
        }))
        .plugin(
            tauri_plugin_updater::Builder::new()
                .default_version_comparator(|current, update| update.version != current)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_installation_path,
            set_game_installation_path,
            get_game_info,
            check_config_exists,
            open_config,
            prepare_game_launch,
            show_game_startup,
            launch_game,
            dismiss_game_startup,
            get_last_game_launch_log,
            download_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri applications");
}

#[cfg(test)]
mod signature_tests {
    use super::*;

    const VALID_SIGNATURE: &str = "untrusted comment: signature from minisign secret key\nRUTFoQ7eoeD8I1fn5Gt3F8W3iu7J2n1QO4jXZMXZ6LymsABin/m0SyRkn6GbAK9SeDlZ0/IfprAp7QFW+laLvXYoGt1MzSCTMwQ=\ntrusted comment: LTR signature verification test\nVkYJq3m45p5dFVAA//iqWC+JUE5yjMlL7xKg/uMgIKNd8TEw3AQNnBxalJiQEwqo/zyCqFsREZ/X79P+Fi25DA==\n";

    #[test]
    fn removes_windows_extended_path_prefix_before_persisting() {
        assert_eq!(
            normalized_windows_path(Path::new(r"\\?\Z:\ltheory")),
            r"Z:\ltheory"
        );
        assert_eq!(
            normalized_windows_path(Path::new(r"\\?\UNC\server\games\LTR")),
            r"\\server\games\LTR"
        );
    }

    #[test]
    fn accepts_authentic_archive_and_rejects_tampering() {
        let mut archive = tempfile::NamedTempFile::new().expect("create test archive");
        std::io::Write::write_all(&mut archive, b"LTR signature verification test fixture\n")
            .expect("write authentic test archive");
        std::io::Write::flush(&mut archive).expect("flush authentic test archive");

        verify_game_archive(archive.path(), VALID_SIGNATURE)
            .expect("authentic archive should verify");

        std::io::Write::write_all(&mut archive, b"tampered").expect("tamper with test archive");
        std::io::Write::flush(&mut archive).expect("flush tampered test archive");

        assert!(verify_game_archive(archive.path(), VALID_SIGNATURE).is_err());
    }

    #[test]
    fn release_asset_urls_are_pinned_to_a_valid_tag() {
        assert_eq!(
            game_asset_url("v0.1.0-nightly_2", GAME_ARCHIVE_NAME).unwrap(),
            "https://github.com/Limit-Theory-Redux/ltheory/releases/download/v0.1.0-nightly_2/ltheory-windows.zip"
        );
        assert!(game_asset_url("../../latest", GAME_ARCHIVE_NAME).is_err());
        assert!(game_asset_url("https://example.com", GAME_ARCHIVE_NAME).is_err());
    }

    #[test]
    fn preserves_complete_game_output_lines() {
        let long_line = format!("begin-{}-end", "x".repeat(2_000));
        assert_eq!(sanitize_console_line(&long_line), long_line);
    }
}
