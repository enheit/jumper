use crate::app::{App, ClipboardOperation, Mode};
use crate::config::SortMode;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

fn matches_keybinding(key: &KeyEvent, binding: &str) -> bool {
    let parts: Vec<&str> = binding.split('+').collect();

    let mut has_ctrl = false;
    let mut has_shift = false;
    let mut has_alt = false;
    let mut key_char = None;

    for part in parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => has_ctrl = true,
            "shift" => has_shift = true,
            "alt" => has_alt = true,
            c if c.len() == 1 => key_char = Some(c.chars().next().unwrap()),
            _ => {}
        }
    }

    if let Some(ch) = key_char {
        if let KeyCode::Char(key_ch) = key.code {
            let modifiers_match =
                key.modifiers.contains(KeyModifiers::CONTROL) == has_ctrl &&
                key.modifiers.contains(KeyModifiers::SHIFT) == has_shift &&
                key.modifiers.contains(KeyModifiers::ALT) == has_alt;

            return key_ch == ch && modifiers_match;
        }
    }

    false
}

pub async fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    // Only handle Press events for consistency across platforms
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    // Store last key for multi-key bindings
    let current_key = format_key(&key);
    let two_key_combo = format!("{}{}", app.last_key, current_key);

    match app.mode {
        Mode::Normal => handle_normal_mode(app, key, &two_key_combo)?,
        Mode::VisualMulti => handle_visual_multi_mode(app, key)?,
        Mode::Search => handle_search_mode(app, key)?,
        Mode::SortMenu => handle_sort_menu(app, key)?,
        Mode::Create => handle_create_mode(app, key)?,
        Mode::Help => handle_help_mode(app, key)?,
        Mode::DeleteConfirm => handle_delete_confirm_mode(app, key)?,
    }

    // Update last key
    app.last_key = current_key;

    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent, two_key_combo: &str) -> Result<()> {
    // Check for history back keybinding first
    if matches_keybinding(&key, &app.config.keybindings.history_back) {
        if let Err(e) = app.go_back_in_history() {
            app.error_message = Some(format!("Error going back in history: {}", e));
        } else {
            app.error_message = None;
        }
        return Ok(());
    }

    // Handle quick jumps (two-key combinations)
    if let Some(path) = app.config.keybindings.quick_jumps.get(two_key_combo) {
        let path_buf = std::path::PathBuf::from(path);

        // Check if the path exists
        if path_buf.exists() && path_buf.is_dir() {
            // Push current location to global history before jumping
            if let Some(current_selected) = app.list_state.selected() {
                app.global_history.push(crate::app::NavigationHistory {
                    path: app.current_dir.clone(),
                    selected_index: current_selected,
                });
            }

            app.current_dir = path_buf;
            if let Err(e) = app.load_directory() {
                app.error_message = Some(format!("Error loading directory: {}", e));
            } else {
                app.list_state.select(Some(0));
                app.error_message = None;
                // Clear search when jumping to different directory
                app.clear_search();
            }
        } else {
            app.error_message = Some(format!("Path does not exist: {}", path));
        }

        app.last_key.clear();
        return Ok(());
    }

    match (key.code, key.modifiers) {
        // Quit
        (KeyCode::Char('q'), KeyModifiers::NONE) => {
            app.should_quit = true;
        }

        // Navigation
        (KeyCode::Char('j'), KeyModifiers::NONE) | (KeyCode::Down, _) => {
            app.next();
            app.error_message = None;
        }
        (KeyCode::Char('k'), KeyModifiers::NONE) | (KeyCode::Up, _) => {
            app.previous();
            app.error_message = None;
        }
        (KeyCode::Char('l'), KeyModifiers::NONE) | (KeyCode::Right, _) => {
            if let Some(path) = app.get_selected_path() {
                if path.is_dir() {
                    if let Err(e) = app.enter_directory() {
                        app.error_message = Some(format!("Error entering directory: {}", e));
                    } else {
                        app.error_message = None;
                    }
                } else {
                    // Open file with default application
                    if let Err(e) = crate::file_ops::open_file(&path) {
                        app.error_message = Some(format!("Error opening file: {}", e));
                    } else {
                        app.error_message = None;
                    }
                }
            }
        }
        (KeyCode::Char('h'), KeyModifiers::NONE) | (KeyCode::Left, _) => {
            if let Err(e) = app.go_parent() {
                app.error_message = Some(format!("Error going to parent: {}", e));
            } else {
                app.error_message = None;
            }
        }

        // Toggle hidden files
        (KeyCode::Char('.'), KeyModifiers::NONE) => {
            app.toggle_hidden()?;
        }

        // Mark toggle
        (KeyCode::Char('m'), KeyModifiers::NONE) => {
            if let Some(selected) = app.list_state.selected() {
                // Toggle mark on current file
                if let Some(pos) = app.selected_indices.iter().position(|&i| i == selected) {
                    // Already marked, unmark it
                    app.selected_indices.remove(pos);
                } else {
                    // Not marked, mark it
                    app.selected_indices.push(selected);
                }
            }
        }

        // Visual Multi mode (Shift+V)
        (KeyCode::Char('V'), KeyModifiers::SHIFT) => {
            app.mode = Mode::VisualMulti;
            if let Some(selected) = app.list_state.selected() {
                // Start with current file selected
                if !app.selected_indices.contains(&selected) {
                    app.selected_indices.push(selected);
                }
            }
        }

        // Copy (yy or y on marked files)
        (KeyCode::Char('y'), KeyModifiers::NONE) => {
            if app.last_key == "y" {
                // yy: copy current file if no marks
                if app.selected_indices.is_empty() {
                    if let Some(path) = app.get_selected_path() {
                        app.flash_copied_paths = vec![path.clone()];
                        app.clipboard = ClipboardOperation::Copy(vec![path]);
                    }
                }
                app.last_key.clear();
            } else if !app.selected_indices.is_empty() {
                // y: copy all marked files
                let paths: Vec<_> = app
                    .selected_indices
                    .iter()
                    .filter_map(|&i| app.files.get(i).map(|f| f.path.clone()))
                    .collect();
                if !paths.is_empty() {
                    app.flash_copied_paths = paths.clone();
                    app.clipboard = ClipboardOperation::Copy(paths);
                    app.selected_indices.clear();
                }
            }
        }

        // Cut (x on current or marked files)
        (KeyCode::Char('x'), KeyModifiers::NONE) => {
            if !app.selected_indices.is_empty() {
                // Cut all marked files
                let paths: Vec<_> = app
                    .selected_indices
                    .iter()
                    .filter_map(|&i| app.files.get(i).map(|f| f.path.clone()))
                    .collect();
                if !paths.is_empty() {
                    app.clipboard = ClipboardOperation::Cut(paths);
                    app.selected_indices.clear();
                }
            } else if let Some(path) = app.get_selected_path() {
                // Cut current file
                app.clipboard = ClipboardOperation::Cut(vec![path]);
            }
        }

        // Paste
        (KeyCode::Char('p'), KeyModifiers::NONE) => {
            crate::file_ops::paste(app)?;
            app.load_directory()?;
        }

        // Search
        (KeyCode::Char('/'), KeyModifiers::NONE) => {
            app.mode = Mode::Search;
            app.search_query.clear();
        }

        // Sort menu
        (KeyCode::Char('s'), KeyModifiers::NONE) => {
            app.mode = Mode::SortMenu;
        }

        // Toggle sort order
        (KeyCode::Char('o'), KeyModifiers::NONE) => {
            app.sort_ascending = !app.sort_ascending;
            app.sort_files();
        }

        // Create file/folder
        (KeyCode::Char('a'), KeyModifiers::NONE) => {
            app.mode = Mode::Create;
            app.create_input.clear();
        }

        // Delete
        (KeyCode::Char('d'), KeyModifiers::NONE) => {
            // Check if there are marked files (selected_indices)
            let paths_to_delete: Vec<_> = if !app.selected_indices.is_empty() {
                // Delete all marked files
                app.selected_indices
                    .iter()
                    .filter_map(|&i| app.files.get(i).map(|f| f.path.clone()))
                    .collect()
            } else if let Some(path) = app.get_selected_path() {
                // Delete current file
                vec![path]
            } else {
                vec![]
            };

            if !paths_to_delete.is_empty() {
                if app.config.behavior.delete_confirmation {
                    app.delete_targets = paths_to_delete;
                    app.mode = Mode::DeleteConfirm;
                } else {
                    for path in &paths_to_delete {
                        crate::file_ops::delete_path(path)?;
                    }
                    app.selected_indices.clear();
                    app.load_directory()?;
                }
            }
        }

        // Help
        (KeyCode::Char('?'), _) => {
            app.mode = Mode::Help;
        }

        // Esc - clear cut clipboard and search highlights
        (KeyCode::Esc, KeyModifiers::NONE) => {
            if matches!(app.clipboard, ClipboardOperation::Cut(_)) {
                app.clipboard = ClipboardOperation::None;
            }
            // Clear search highlights if any
            if !app.search_highlights.is_empty() {
                app.clear_search();
            }
        }

        _ => {}
    }

    Ok(())
}

fn handle_search_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.clear_search();
        }
        KeyCode::Enter => {
            app.mode = Mode::Normal;
            // Keep highlights active
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            crate::fuzzy::update_search(app);
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            crate::fuzzy::update_search(app);
        }
        _ => {}
    }

    Ok(())
}

fn handle_sort_menu(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
        }
        KeyCode::Char('n') => {
            app.sort_mode = SortMode::Name;
            app.sort_files();
            app.mode = Mode::Normal;
        }
        KeyCode::Char('s') => {
            app.sort_mode = SortMode::Size;
            app.sort_files();
            app.mode = Mode::Normal;
        }
        KeyCode::Char('m') => {
            app.sort_mode = SortMode::Modified;
            app.sort_files();
            app.mode = Mode::Normal;
        }
        _ => {}
    }

    Ok(())
}

fn handle_visual_multi_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            // Exit mode and keep marks
            app.mode = Mode::Normal;
        }
        KeyCode::Esc => {
            // Exit mode and clear all marks
            app.mode = Mode::Normal;
            app.selected_indices.clear();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.next();
            if let Some(selected) = app.list_state.selected() {
                // Add to selection if not already selected
                if !app.selected_indices.contains(&selected) {
                    app.selected_indices.push(selected);
                }
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.previous();
            if let Some(selected) = app.list_state.selected() {
                // Add to selection if not already selected
                if !app.selected_indices.contains(&selected) {
                    app.selected_indices.push(selected);
                }
            }
        }
        KeyCode::Char('m') => {
            // Deselect current file
            if let Some(selected) = app.list_state.selected() {
                if let Some(pos) = app.selected_indices.iter().position(|&i| i == selected) {
                    app.selected_indices.remove(pos);
                }
            }
        }
        KeyCode::Char('y') => {
            // Copy all selected
            let paths: Vec<_> = app
                .selected_indices
                .iter()
                .filter_map(|&i| app.files.get(i).map(|f| f.path.clone()))
                .collect();
            if !paths.is_empty() {
                app.flash_copied_paths = paths.clone();
                app.clipboard = ClipboardOperation::Copy(paths);
            }
            app.mode = Mode::Normal;
            app.selected_indices.clear();
        }
        KeyCode::Char('x') => {
            // Cut all selected
            let paths: Vec<_> = app
                .selected_indices
                .iter()
                .filter_map(|&i| app.files.get(i).map(|f| f.path.clone()))
                .collect();
            if !paths.is_empty() {
                app.clipboard = ClipboardOperation::Cut(paths);
            }
            app.mode = Mode::Normal;
            app.selected_indices.clear();
        }
        _ => {}
    }

    Ok(())
}

fn handle_create_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.create_input.clear();
        }
        KeyCode::Enter => {
            if !app.create_input.is_empty() {
                let path = app.current_dir.join(&app.create_input);

                if app.create_input.ends_with('/') {
                    // Create directory
                    crate::file_ops::create_directory(&path)?;
                } else {
                    // Create file
                    crate::file_ops::create_file(&path)?;
                }

                app.load_directory()?;
            }
            app.mode = Mode::Normal;
            app.create_input.clear();
        }
        KeyCode::Backspace => {
            app.create_input.pop();
        }
        KeyCode::Char(c) => {
            app.create_input.push(c);
        }
        _ => {}
    }

    Ok(())
}

fn handle_help_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') => {
            app.mode = Mode::Normal;
        }
        _ => {}
    }

    Ok(())
}

fn handle_delete_confirm_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            for path in &app.delete_targets {
                crate::file_ops::delete_path(path)?;
            }
            app.delete_targets.clear();
            app.selected_indices.clear();
            app.load_directory()?;
            app.mode = Mode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Enter => {
            app.delete_targets.clear();
            app.mode = Mode::Normal;
        }
        _ => {}
    }

    Ok(())
}

fn format_key(key: &KeyEvent) -> String {
    match key.code {
        KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::SHIFT) => {
            c.to_uppercase().to_string()
        }
        KeyCode::Char(c) => c.to_string(),
        _ => String::new(),
    }
}
