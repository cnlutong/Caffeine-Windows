//! Opt-in desktop integration harness. Not part of normal application builds.
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Manager};

static REOPENING: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub fn smoke_reopening() -> bool {
    REOPENING.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn smoke_reopen(app: AppHandle) -> Result<(), String> {
    if !enabled() {
        return Err("Desktop smoke harness is not enabled".into());
    }
    REOPENING.store(true, Ordering::SeqCst);
    app.get_webview_window("main")
        .ok_or("Missing panel")?
        .destroy()
        .map_err(|e| e.to_string())?;
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(200));
        super::show_settings(&app);
    });
    Ok(())
}

#[tauri::command]
pub fn smoke_checkpoint(app: AppHandle) -> Result<(), String> {
    if !enabled() {
        return Err("Desktop smoke harness is not enabled".into());
    }
    let state = app.state::<super::Shared>();
    let runtime = state.lock().map_err(|_| "State lock failed")?;
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("/usr/bin/pmset")
            .args(["-g", "assertions"])
            .output()
            .map_err(|e| e.to_string())?;
        let text = String::from_utf8_lossy(&output.stdout);
        let owner = format!("pid {}(", std::process::id());
        let owned: Vec<_> = text
            .lines()
            .filter(|line| line.contains(&owner) && line.contains("Caffeine: user enabled"))
            .collect();
        let expected = match runtime.controller.mode {
            super::Mode::Off => None,
            super::Mode::System => Some("PreventUserIdleSystemSleep"),
            super::Mode::Display => Some("PreventUserIdleDisplaySleep"),
        };
        if let Some(kind) = expected {
            if owned.len() != 1 || !owned[0].contains(kind) {
                return Err(format!(
                    "Wrong native assertions: {owned:?}, expected {kind}"
                ));
            }
        } else if !owned.is_empty() {
            return Err(format!("Leaked native assertions: {owned:?}"));
        }
    }
    #[cfg(not(target_os = "macos"))]
    let _ = runtime;
    Ok(())
}

pub fn enabled() -> bool {
    std::env::args().any(|arg| arg == "--smoke-test")
}

#[tauri::command]
pub fn smoke_complete(app: AppHandle, passed: bool, detail: String) -> Result<(), String> {
    if !enabled() {
        return Err("Desktop smoke harness is not enabled".into());
    }
    let state = app.state::<super::Shared>();
    let runtime = state.lock().map_err(|_| "State lock failed")?;
    let clean = runtime.controller.mode == super::Mode::Off;
    eprintln!(
        "DESKTOP_SMOKE {}: {}",
        if passed && clean { "PASS" } else { "FAIL" },
        detail
    );
    drop(runtime);
    app.exit(if passed && clean { 0 } else { 1 });
    Ok(())
}

pub fn script() -> &'static str {
    include_str!("../tests/desktop-smoke.js")
}
