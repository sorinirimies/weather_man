//! Persistent user settings — saved locations and preferred units.
//!
//! Stored as JSON at `<config-dir>/weatherman/settings.json` (e.g.
//! `~/.config/weatherman/settings.json` on Linux). All operations degrade
//! gracefully: a missing or corrupt file yields [`AppSettings::default`].

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn default_units() -> String {
    "metric".to_string()
}

/// User settings persisted across sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Preferred units: "metric", "imperial" or "standard".
    #[serde(default = "default_units")]
    pub units: String,
    /// Saved location queries (city names), in user order.
    #[serde(default)]
    pub locations: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            units: default_units(),
            locations: Vec::new(),
        }
    }
}

impl AppSettings {
    /// Add a location if it isn't already saved (case-insensitive). Returns
    /// `true` when it was newly added.
    pub fn add_location(&mut self, name: &str) -> bool {
        let name = name.trim();
        if name.is_empty() || self.locations.iter().any(|l| l.eq_ignore_ascii_case(name)) {
            return false;
        }
        self.locations.push(name.to_string());
        true
    }

    /// Remove the saved location at `index`, if present.
    pub fn remove_location(&mut self, index: usize) {
        if index < self.locations.len() {
            self.locations.remove(index);
        }
    }
}

/// Path to the settings file, if a config directory is available.
pub fn settings_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("weatherman").join("settings.json"))
}

/// Load settings from disk, returning defaults when missing or unreadable.
pub fn load_settings() -> AppSettings {
    let Some(path) = settings_path() else {
        return AppSettings::default();
    };
    match std::fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    }
}

/// Persist settings to disk, creating parent directories as needed.
pub fn save_settings(settings: &AppSettings) -> anyhow::Result<()> {
    let Some(path) = settings_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(settings)?;
    std::fs::write(&path, json)?;
    Ok(())
}
