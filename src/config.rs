use anyhow::Result;
use directories::ProjectDirs;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub colors: ColorScheme,
    #[serde(default)]
    pub keybindings: KeyBindings,
    #[serde(default)]
    pub behavior: Behavior,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorScheme {
    #[serde(default = "default_directory_color")]
    pub directory: Color,
    #[serde(default = "default_file_color")]
    pub file: Color,
    #[serde(default = "default_selected_color")]
    pub selected: Color,
    #[serde(default = "default_hidden_color")]
    pub hidden: Color,
    #[serde(default = "default_symlink_color")]
    pub symlink: Color,
    #[serde(default = "default_executable_color")]
    pub executable: Color,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyBindings {
    #[serde(default)]
    pub quick_jumps: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Behavior {
    #[serde(default)]
    pub show_hidden: bool,
    #[serde(default = "default_sort")]
    pub default_sort: SortMode,
    #[serde(default = "default_delete_confirmation")]
    pub delete_confirmation: bool,
    #[serde(default = "default_flash_duration_ms")]
    pub flash_duration_ms: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortMode {
    Name,
    Size,
    Modified,
}

// Default values
fn default_directory_color() -> Color {
    Color::Blue
}

fn default_file_color() -> Color {
    Color::White
}

fn default_selected_color() -> Color {
    Color::Green
}

fn default_hidden_color() -> Color {
    Color::DarkGray
}

fn default_symlink_color() -> Color {
    Color::Cyan
}

fn default_executable_color() -> Color {
    Color::Red
}

fn default_sort() -> SortMode {
    SortMode::Name
}

fn default_delete_confirmation() -> bool {
    true
}

fn default_flash_duration_ms() -> u64 {
    150
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            directory: default_directory_color(),
            file: default_file_color(),
            selected: default_selected_color(),
            hidden: default_hidden_color(),
            symlink: default_symlink_color(),
            executable: default_executable_color(),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut quick_jumps = HashMap::new();
        if let Some(home) = dirs::home_dir() {
            quick_jumps.insert("gh".to_string(), home.to_string_lossy().to_string());
            quick_jumps.insert(
                "gd".to_string(),
                home.join("Downloads").to_string_lossy().to_string(),
            );
            quick_jumps.insert(
                "gp".to_string(),
                home.join("Projects").to_string_lossy().to_string(),
            );
        }
        Self { quick_jumps }
    }
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            show_hidden: false,
            default_sort: default_sort(),
            delete_confirmation: default_delete_confirmation(),
            flash_duration_ms: default_flash_duration_ms(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: ColorScheme::default(),
            keybindings: KeyBindings::default(),
            behavior: Behavior::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "jumper") {
            Ok(proj_dirs.config_dir().join("config.toml"))
        } else {
            anyhow::bail!("Could not determine config directory")
        }
    }
}

// Helper module for dirs (since we used directories crate, not dirs)
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}
