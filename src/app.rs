use crate::config::{Config, SortMode};
use anyhow::Result;
use ratatui::widgets::ListState;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_hidden: bool,
    pub is_symlink: bool,
    pub is_executable: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardOperation {
    None,
    Copy(Vec<PathBuf>),
    Cut(Vec<PathBuf>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Visual,
    VisualMulti,
    Search,
    SortMenu,
}

pub struct App {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub list_state: ListState,
    pub clipboard: ClipboardOperation,
    pub mode: Mode,
    pub show_hidden: bool,
    pub sort_mode: SortMode,
    pub search_query: String,
    pub filtered_indices: Vec<usize>,
    pub selected_indices: Vec<usize>,
    pub should_quit: bool,
    pub config: Config,
    pub last_key: String,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        let show_hidden = config.behavior.show_hidden;
        let sort_mode = config.behavior.default_sort.clone();

        let mut app = Self {
            current_dir: current_dir.clone(),
            files: Vec::new(),
            list_state: ListState::default(),
            clipboard: ClipboardOperation::None,
            mode: Mode::Normal,
            show_hidden,
            sort_mode,
            search_query: String::new(),
            filtered_indices: Vec::new(),
            selected_indices: Vec::new(),
            should_quit: false,
            config,
            last_key: String::new(),
        };

        app.load_directory()?;
        app.list_state.select(Some(0));
        Ok(app)
    }

    pub fn load_directory(&mut self) -> Result<()> {
        self.files.clear();

        // Read directory entries
        for entry in fs::read_dir(&self.current_dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let name = entry.file_name().to_string_lossy().to_string();

            let is_hidden = name.starts_with('.');
            if !self.show_hidden && is_hidden {
                continue;
            }

            let is_executable = if cfg!(unix) {
                metadata.permissions().mode() & 0o111 != 0
            } else {
                false
            };

            self.files.push(FileEntry {
                name: name.clone(),
                path: path.clone(),
                is_dir: metadata.is_dir(),
                is_hidden,
                is_symlink: metadata.is_symlink(),
                is_executable: !metadata.is_dir() && is_executable,
                size: metadata.len(),
                modified: metadata.modified().ok(),
            });
        }

        self.sort_files();
        self.update_filtered_indices();
        Ok(())
    }

    pub fn sort_files(&mut self) {
        // Separate directories and files
        let (mut dirs, mut files): (Vec<_>, Vec<_>) =
            self.files.iter().cloned().partition(|f| f.is_dir);

        // Sort each group
        match self.sort_mode {
            SortMode::Name => {
                dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }
            SortMode::Size => {
                dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                files.sort_by(|a, b| b.size.cmp(&a.size));
            }
            SortMode::Modified => {
                dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                files.sort_by(|a, b| {
                    b.modified
                        .unwrap_or(SystemTime::UNIX_EPOCH)
                        .cmp(&a.modified.unwrap_or(SystemTime::UNIX_EPOCH))
                });
            }
        }

        // Combine: directories first, then files
        self.files = dirs;
        self.files.extend(files);
    }

    pub fn update_filtered_indices(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices = (0..self.files.len()).collect();
        } else {
            // Will be implemented with fuzzy search
            self.filtered_indices = (0..self.files.len()).collect();
        }
    }

    pub fn get_filtered_files(&self) -> Vec<&FileEntry> {
        self.filtered_indices
            .iter()
            .map(|&i| &self.files[i])
            .collect()
    }

    pub fn next(&mut self) {
        let filtered_count = self.filtered_indices.len();
        if filtered_count == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= filtered_count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let filtered_count = self.filtered_indices.len();
        if filtered_count == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    filtered_count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn enter_directory(&mut self) -> Result<()> {
        if let Some(selected) = self.list_state.selected() {
            let files = self.get_filtered_files();
            if let Some(file) = files.get(selected) {
                if file.is_dir {
                    self.current_dir = file.path.clone();
                    self.load_directory()?;
                    self.list_state.select(Some(0));
                }
            }
        }
        Ok(())
    }

    pub fn go_parent(&mut self) -> Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.load_directory()?;
            self.list_state.select(Some(0));
        }
        Ok(())
    }

    pub fn toggle_hidden(&mut self) -> Result<()> {
        self.show_hidden = !self.show_hidden;
        self.load_directory()?;
        Ok(())
    }

    pub fn get_selected_path(&self) -> Option<PathBuf> {
        self.list_state.selected().and_then(|i| {
            self.get_filtered_files()
                .get(i)
                .map(|f| f.path.clone())
        })
    }
}
