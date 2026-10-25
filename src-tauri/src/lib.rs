mod model;
mod power;
mod settings;
#[cfg(feature = "desktop-smoke")]
mod smoke;

use model::{Controller, Language, Mode, PowerBackend, Settings, Snapshot};
use power::PowerRequest;
use std::{path::PathBuf, sync::Mutex};
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, RunEvent, State, WebviewWindowBuilder,
};

struct Runtime {
    controller: Controller<PowerRequest>,
    settings_path: PathBuf,
    system_locale: String,
    warning: Option<String>,
    error: Option<String>,
}

impl Runtime {
    fn snapshot(&self) -> Snapshot {
        Snapshot {
            mode: self.controller.mode,
            last_active_mode: self.controller.settings.last_active_mode,
            language: self.controller.settings.language,
            locale: model::locale(self.controller.settings.language, &self.system_locale),
            platform: std::env::consts::OS,
            version: env!("CARGO_PKG_VERSION"),
            revision: self.controller.revision,
            warning: self.warning.clone(),
            error: self.error.clone(),
        }
    }

    fn save(&mut self) {
        self.warning = settings::save(&self.settings_path, &self.controller.settings).err();
        if let Some(warning) = &self.warning {
            eprintln!("{warning}");
        }
    }
}

type Shared = Mutex<Runtime>;

#[tauri::command]
fn get_state(state: State<'_, Shared>) -> Result<Snapshot, String> {
    Ok(state.lock().map_err(|_| "State lock failed")?.snapshot())
}

fn change(
    app: &AppHandle,
    operation: impl FnOnce(&mut Controller<PowerRequest>) -> Result<(), String>,
) -> Result<Snapshot, String> {
    let (snapshot, result) = {
        let state = app.state::<Shared>();
        let mut runtime = state.lock().map_err(|_| "State lock failed")?;
        let result = operation(&mut runtime.controller);
        runtime.error = result.as_ref().err().cloned();
        // Every published mutation gets a monotonic revision, including errors and language changes.
        runtime.controller.revision += 1;
        if result.is_ok() {
            runtime.save();
        }
        (runtime.snapshot(), result)
    };
    publish(app, &snapshot);
    result.map(|()| snapshot)
}

#[tauri::command]
fn set_mode(app: AppHandle, mode: Mode) -> Result<Snapshot, String> {
    change(&app, |controller| controller.set_mode(mode))
}

#[tauri::command]
fn set_language(app: AppHandle, language: Language) -> Result<Snapshot, String> {
    change(&app, |controller| {
        controller.settings.language = language;
        Ok(())
    })
}

#[tauri::command]
fn quit_app(app: AppHandle) -> Result<(), String> {
    change(&app, |controller| controller.set_mode(Mode::Off))?;
    app.exit(0);
    Ok(())
}

fn labels(locale: &str) -> [&'static str; 6] {
    if locale == "zh" {
        [
            "允许休眠",
            "仅防休眠",
            "屏幕常亮",
            "设置…",
            "退出 Caffeine",
            "Caffeine：",
        ]
    } else {
        [
            "Allow sleep",
            "Keep system awake",
            "Keep screen on",
            "Settings…",
            "Quit Caffeine",
            "Caffeine: ",
        ]
    }
}

fn menu(app: &AppHandle, snapshot: &Snapshot) -> tauri::Result<Menu<tauri::Wry>> {
    let text = labels(snapshot.locale);
    let off = CheckMenuItem::with_id(
        app,
        "off",
        text[0],
        true,
        snapshot.mode == Mode::Off,
        None::<&str>,
    )?;
    let system = CheckMenuItem::with_id(
        app,
        "system",
        text[1],
        true,
        snapshot.mode == Mode::System,
        None::<&str>,
    )?;
    let display = CheckMenuItem::with_id(
        app,
        "display",
        text[2],
        true,
        snapshot.mode == Mode::Display,
        None::<&str>,
    )?;
    let separator = PredefinedMenuItem::separator(app)?;
    let settings = MenuItem::with_id(app, "settings", text[3], true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", text[4], true, None::<&str>)?;
    Menu::with_items(
        app,
        &[&off, &system, &display, &separator, &settings, &quit],
    )
}

fn tray_image(mode: Mode) -> tauri::image::Image<'static> {
    let bytes: &[u8] = match (cfg!(target_os = "macos"), mode) {
        (true, Mode::Off) => include_bytes!("../icons/tray-mac-off.png"),
        (true, Mode::System) => include_bytes!("../icons/tray-mac-system.png"),
        (true, Mode::Display) => include_bytes!("../icons/tray-mac-display.png"),
        (false, Mode::Off) => include_bytes!("../icons/tray-off.png"),
        (false, Mode::System) => include_bytes!("../icons/tray-system.png"),
        (false, Mode::Display) => include_bytes!("../icons/tray-display.png"),
    };
    tauri::image::Image::from_bytes(bytes).expect("Bundled tray icon must be valid PNG")
}

fn tooltip(snapshot: &Snapshot) -> String {
    let text = labels(snapshot.locale);
    format!(
        "{}{}",
        text[5],
        text[match snapshot.mode {
            Mode::Off => 0,
            Mode::System => 1,
            Mode::Display => 2,
        }]
    )
}

fn publish(app: &AppHandle, snapshot: &Snapshot) {
    if let Err(error) = app.emit("state-changed", snapshot) {
        eprintln!("Cannot notify settings panel: {error}");
    }
    let app = app.clone();
    let snapshot = snapshot.clone();
    let main = app.clone();
    if let Err(error) = app.run_on_main_thread(move || {
        if let Some(tray) = main.tray_by_id("main") {
            let result = (|| -> tauri::Result<()> {
                tray.set_icon_with_as_template(
                    Some(tray_image(snapshot.mode)),
                    cfg!(target_os = "macos"),
                )?;
                tray.set_tooltip(Some(tooltip(&snapshot)))?;
                tray.set_menu(Some(menu(&main, &snapshot)?))?;
                Ok(())
            })();
            if let Err(error) = result {
                eprintln!("Cannot update tray: {error}");
            }
        }
    }) {
        eprintln!("Cannot schedule tray update: {error}");
    }
}

fn show_settings(app: &AppHandle) {
    let app = app.clone();
    let main = app.clone();
    if let Err(error) = app.run_on_main_thread(move || {
        let result = (|| -> tauri::Result<()> {
            if let Some(window) = main.get_webview_window("main") {
                window.show()?;
                window.set_focus()?;
            } else {
                let config = main
                    .config()
                    .app
                    .windows
                    .first()
                    .expect("Settings window config");
                let builder = WebviewWindowBuilder::from_config(&main, config)?;
                #[cfg(feature = "desktop-smoke")]
                let builder = builder.on_page_load(|window, payload| {
                    if smoke::enabled()
                        && payload.event() == tauri::webview::PageLoadEvent::Finished
                    {
                        if let Err(error) = window.eval(smoke::script()) {
                            eprintln!("Smoke script failed: {error}");
                        }
                    }
                });
                let window = builder.build()?;
                window.set_focus()?;
            }
            Ok(())
        })();
        if let Err(error) = result {
            eprintln!("Cannot open settings: {error}");
        }
    }) {
        eprintln!("Cannot schedule settings window: {error}");
    }
}

fn handle_tray_event(
    tray: &tauri::tray::TrayIcon,
    event: tauri::tray::TrayIconEvent,
) -> Result<(), String> {
    if let tauri::tray::TrayIconEvent::Click {
        button: tauri::tray::MouseButton::Left,
        button_state: tauri::tray::MouseButtonState::Up,
        ..
    } = event
    {
        change(tray.app_handle(), |controller| controller.toggle())?;
    }
    Ok(())
}

pub fn self_test() -> Result<(), String> {
    let mut controller = Controller::new(PowerRequest::default(), Settings::default());
    for mode in [Mode::System, Mode::Display, Mode::System, Mode::Off] {
        controller.set_mode(mode)?;
    }
    println!(
        "Caffeine {} native power self-test passed ({}/{})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    Ok(())
}

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            show_settings(app)
        }))
        .invoke_handler(tauri::generate_handler![
            get_state,
            set_mode,
            set_language,
            quit_app
        ]);
    #[cfg(feature = "desktop-smoke")]
    let builder = builder.invoke_handler(tauri::generate_handler![
        get_state,
        set_mode,
        set_language,
        quit_app,
        smoke::smoke_complete,
        smoke::smoke_reopening,
        smoke::smoke_reopen,
        smoke::smoke_tray_click,
        smoke::smoke_checkpoint
    ]);
    let app = builder
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let path = app.path().app_config_dir()?.join("settings.json");
            #[cfg(feature = "desktop-smoke")]
            let path = if smoke::enabled() {
                std::env::var_os("CAFFEINE_TEST_CONFIG_DIR")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| std::env::temp_dir().join("caffeine-desktop-smoke"))
                    .join("settings.json")
            } else {
                path
            };
            let (settings, warning) = settings::load(&path);
            let runtime = Runtime {
                controller: Controller::new(PowerRequest::default(), settings),
                settings_path: path,
                system_locale: sys_locale::get_locale().unwrap_or_else(|| "en".into()),
                warning,
                error: None,
            };
            let snapshot = runtime.snapshot();
            app.manage(Mutex::new(runtime));
            let tray_menu = menu(app.handle(), &snapshot)?;
            TrayIconBuilder::with_id("main")
                .icon(tray_image(Mode::Off))
                .icon_as_template(cfg!(target_os = "macos"))
                .tooltip(tooltip(&snapshot))
                .menu(&tray_menu)
                // Right-click keeps the native context menu; left-click toggles once on release.
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    let result = match event.id.as_ref() {
                        "off" => change(app, |c| c.set_mode(Mode::Off)).map(|_| ()),
                        "system" => change(app, |c| c.set_mode(Mode::System)).map(|_| ()),
                        "display" => change(app, |c| c.set_mode(Mode::Display)).map(|_| ()),
                        "settings" => {
                            show_settings(app);
                            Ok(())
                        }
                        "quit" => quit_app(app.clone()),
                        _ => Ok(()),
                    };
                    if let Err(error) = result {
                        eprintln!("{error}");
                        show_settings(app);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let Err(error) = handle_tray_event(tray, event) {
                        eprintln!("{error}");
                        show_settings(tray.app_handle());
                    }
                })
                .build(app)?;
            if std::env::args().any(|arg| arg == "--settings") {
                show_settings(app.handle());
            }
            #[cfg(feature = "desktop-smoke")]
            if smoke::enabled() {
                show_settings(app.handle());
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Could not start Caffeine");

    app.run(|app, event| match event {
        // Closing the last WebView destroys it, but the native tray and power requests stay alive.
        RunEvent::ExitRequested {
            code: None, api, ..
        } => api.prevent_exit(),
        RunEvent::Exit => {
            if let Some(state) = app.try_state::<Shared>() {
                // Recover a poisoned lock so cleanup still runs after an unrelated panic.
                let mut runtime = state
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                if let Err(error) = runtime.controller.power.apply(Mode::Off) {
                    eprintln!("Exit cleanup: {error}");
                }
            }
        }
        _ => {}
    });
}
