use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Off,
    System,
    Display,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Auto,
    En,
    Zh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub language: Language,
    pub last_active_mode: Mode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: Language::Auto,
            last_active_mode: Mode::System,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub mode: Mode,
    pub last_active_mode: Mode,
    pub language: Language,
    pub locale: &'static str,
    pub platform: &'static str,
    pub version: &'static str,
    pub revision: u64,
    pub warning: Option<String>,
    pub error: Option<String>,
}

pub fn locale(language: Language, system_locale: &str) -> &'static str {
    match language {
        Language::Zh => "zh",
        Language::En => "en",
        Language::Auto if system_locale.to_ascii_lowercase().starts_with("zh") => "zh",
        Language::Auto => "en",
    }
}

pub trait PowerBackend {
    fn apply(&mut self, mode: Mode) -> Result<(), String>;
}

pub struct Controller<P: PowerBackend> {
    pub power: P,
    pub mode: Mode,
    pub settings: Settings,
    pub revision: u64,
}

impl<P: PowerBackend> Controller<P> {
    pub fn new(power: P, mut settings: Settings) -> Self {
        if settings.last_active_mode == Mode::Off {
            settings.last_active_mode = Mode::System;
        }
        // Never silently reactivate wake prevention after a restart.
        Self {
            power,
            mode: Mode::Off,
            settings,
            revision: 0,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<(), String> {
        if self.mode == mode {
            return Ok(());
        }
        self.power.apply(mode)?;
        self.mode = mode;
        if mode != Mode::Off {
            self.settings.last_active_mode = mode;
        }
        self.revision += 1;
        Ok(())
    }

    #[cfg(any(target_os = "windows", test))]
    pub fn toggle(&mut self) -> Result<(), String> {
        self.set_mode(if self.mode == Mode::Off {
            self.settings.last_active_mode
        } else {
            Mode::Off
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakePower {
        fail: bool,
        calls: Vec<Mode>,
    }
    impl PowerBackend for FakePower {
        fn apply(&mut self, mode: Mode) -> Result<(), String> {
            self.calls.push(mode);
            if self.fail {
                Err("denied".into())
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn failed_switch_keeps_last_successful_state() {
        let mut c = Controller::new(FakePower::default(), Settings::default());
        c.set_mode(Mode::System).unwrap();
        c.power.fail = true;
        assert!(c.set_mode(Mode::Display).is_err());
        assert_eq!(c.mode, Mode::System);
        assert_eq!(c.settings.last_active_mode, Mode::System);
        assert_eq!(c.revision, 1);
        assert!(c.set_mode(Mode::Off).is_err());
        assert_eq!(c.mode, Mode::System);
    }

    #[test]
    fn toggle_restores_last_active_mode_without_duplicate_requests() {
        let mut c = Controller::new(FakePower::default(), Settings::default());
        c.set_mode(Mode::Display).unwrap();
        c.set_mode(Mode::Display).unwrap();
        c.toggle().unwrap();
        c.toggle().unwrap();
        assert_eq!(c.power.calls, vec![Mode::Display, Mode::Off, Mode::Display]);
        assert_eq!(c.mode, Mode::Display);
    }

    #[test]
    fn restart_is_off_and_invalid_last_mode_is_repaired() {
        let mut c = Controller::new(
            FakePower::default(),
            Settings {
                language: Language::Zh,
                last_active_mode: Mode::Display,
            },
        );
        assert_eq!(c.mode, Mode::Off);
        c.toggle().unwrap();
        assert_eq!(c.mode, Mode::Display);
        let c = Controller::new(
            FakePower::default(),
            Settings {
                last_active_mode: Mode::Off,
                ..Settings::default()
            },
        );
        assert_eq!(c.settings.last_active_mode, Mode::System);
    }

    #[test]
    fn locale_and_serialized_contract() {
        assert_eq!(locale(Language::Auto, "zh-Hant-TW"), "zh");
        assert_eq!(locale(Language::Auto, "de-DE"), "en");
        assert_eq!(locale(Language::En, "zh-CN"), "en");
        assert!(serde_json::from_str::<Mode>("\"invalid\"").is_err());
        assert_eq!(
            serde_json::to_string(&Mode::Display).unwrap(),
            "\"display\""
        );
        let settings: Settings = serde_json::from_str("{}").unwrap();
        assert_eq!(settings.last_active_mode, Mode::System);
        assert!(serde_json::to_value(settings)
            .unwrap()
            .get("lastActiveMode")
            .is_some());
    }
}
