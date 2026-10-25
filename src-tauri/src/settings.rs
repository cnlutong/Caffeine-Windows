use crate::model::Settings;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

pub fn load(path: &Path) -> (Settings, Option<String>) {
    match fs::read(path) {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(settings) => (settings, None),
            Err(error) => (
                Settings::default(),
                Some(format!("Invalid settings: {error}")),
            ),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => (Settings::default(), None),
        Err(error) => (
            Settings::default(),
            Some(format!("Cannot read settings: {error}")),
        ),
    }
}

pub fn save(path: &Path, settings: &Settings) -> Result<(), String> {
    let parent = path.parent().ok_or("Missing settings directory")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let temporary: PathBuf = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(settings).map_err(|e| e.to_string())?;
    let result = (|| {
        let mut file = fs::File::create(&temporary)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        drop(file);
        // On Windows std::fs::rename replaces an existing destination via MoveFileExW.
        fs::rename(&temporary, path)
    })();
    if let Err(error) = result {
        let _ = fs::remove_file(temporary);
        return Err(format!("Cannot save settings: {error}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Language, Mode};

    #[test]
    fn corrupt_settings_fall_back_without_losing_the_file() {
        let path =
            std::env::temp_dir().join(format!("caffeine-corrupt-{}.json", std::process::id()));
        fs::write(&path, b"{broken").unwrap();
        let (settings, warning) = load(&path);
        assert_eq!(settings.last_active_mode, Mode::System);
        assert!(warning.is_some());
        assert_eq!(fs::read(&path).unwrap(), b"{broken");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn saving_replaces_an_existing_file() {
        let path = std::env::temp_dir().join(format!("caffeine-save-{}.json", std::process::id()));
        save(&path, &Settings::default()).unwrap();
        save(
            &path,
            &Settings {
                language: Language::Zh,
                last_active_mode: Mode::Display,
            },
        )
        .unwrap();
        let (settings, warning) = load(&path);
        assert_eq!(settings.language, Language::Zh);
        assert_eq!(settings.last_active_mode, Mode::Display);
        assert!(warning.is_none());
        fs::remove_file(path).unwrap();
    }
}
