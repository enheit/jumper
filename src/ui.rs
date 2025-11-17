use crate::app::{App, ClipboardOperation, FileEntry, Mode};
use crate::config::SortMode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_ui(frame: &mut Frame, app: &App) {
    // Check if we're in Help mode
    if app.mode == Mode::Help {
        render_help(frame, app, frame.area());
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // File list
            Constraint::Length(1), // Footer
        ])
        .split(frame.area());

    render_file_list(frame, app, chunks[0]);
    render_footer(frame, app, chunks[1]);
}

fn render_file_list(frame: &mut Frame, app: &App, area: Rect) {
    let current_idx = app.list_state.selected();

    // Check if directory is empty
    if app.files.is_empty() {
        let empty_text = Line::from(Span::styled(
            "empty",
            Style::default()
                .fg(ratatui::style::Color::DarkGray)
                .add_modifier(Modifier::ITALIC)
        ));
        let paragraph = ratatui::widgets::Paragraph::new(empty_text);
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app.files
        .iter()
        .enumerate()
        .map(|(idx, file)| {
            let is_cursor = current_idx == Some(idx);
            let is_selected = app.selected_paths.contains(&file.path);
            let is_highlighted = app.search_highlights.contains(&idx);
            let is_flashing = app.flash_copied_paths.contains(&file.path);
            let match_positions = app.search_match_positions.get(&idx);
            create_list_item(file, is_cursor, is_selected, is_highlighted, is_flashing, match_positions, app)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(Style::default()) // Empty style - we handle selection in create_list_item
        .highlight_symbol("");

    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn create_list_item<'a>(
    file: &'a FileEntry,
    is_cursor: bool,
    is_selected: bool,
    _is_highlighted: bool,
    is_flashing: bool,
    match_positions: Option<&Vec<usize>>,
    app: &App
) -> ListItem<'a> {
    let icon = if file.is_dir {
        ""
    } else if file.is_symlink {
        ""
    } else if file.is_executable {
        ""
    } else {
        ""
    };

    // Check if file is in cut clipboard
    let is_cut = match &app.clipboard {
        ClipboardOperation::Cut(paths) => paths.contains(&file.path),
        _ => false,
    };

    let color = if file.is_hidden {
        app.config.colors.hidden
    } else if file.is_dir {
        app.config.colors.directory
    } else if file.is_symlink {
        app.config.colors.symlink
    } else if file.is_executable {
        app.config.colors.executable
    } else {
        app.config.colors.file
    };

    let mut base_style = Style::default().fg(color);

    // Make directories bold
    if file.is_dir {
        base_style = base_style.add_modifier(Modifier::BOLD);
    }

    // Flash effect for copied files (yellow background) - takes precedence
    if is_flashing {
        base_style = base_style
            .bg(ratatui::style::Color::LightYellow)
            .fg(ratatui::style::Color::Black)
            .add_modifier(Modifier::BOLD);
    }
    // Cursor position (green background)
    else if is_cursor {
        base_style = base_style
            .bg(app.config.colors.selected)
            .fg(ratatui::style::Color::Black)
            .add_modifier(Modifier::BOLD);
    }

    // Add italic and dim for cut files (unless flashing)
    if is_cut && !is_flashing {
        base_style = base_style
            .add_modifier(Modifier::ITALIC)
            .add_modifier(Modifier::DIM);
    }

    // Marked files (reversed)
    if is_selected && !is_flashing && !is_cursor {
        base_style = base_style.add_modifier(Modifier::REVERSED);
    }

    let size_str = if file.is_dir {
        // Check if we have the calculated size for this directory
        match app.dir_sizes.get(&file.path) {
            Some(Some(size)) => format_size(*size),
            Some(None) | None => String::from("? B"), // Still calculating
        }
    } else {
        format_size(file.size)
    };

    // Build the content with character-level highlighting for search matches
    let mut spans = vec![Span::styled(format!("{} ", icon), base_style)];

    // Add trailing slash to directory names
    let display_name = if file.is_dir {
        format!("{}/", file.name)
    } else {
        file.name.clone()
    };

    // Render filename with character highlighting if there are match positions
    if let Some(positions) = match_positions {
        let chars: Vec<char> = display_name.chars().collect();

        for (char_idx, ch) in chars.iter().enumerate() {
            let mut char_style = base_style;

            // Check if this character position is a match (only for actual filename chars, not the slash)
            if char_idx < file.name.len() && positions.contains(&char_idx) {
                // Highlight matched character in yellow
                char_style = char_style.fg(ratatui::style::Color::Yellow).add_modifier(Modifier::BOLD);
            }

            spans.push(Span::styled(ch.to_string(), char_style));
        }

        // Add padding to maintain alignment
        let padding_len = 40_usize.saturating_sub(display_name.len());
        if padding_len > 0 {
            spans.push(Span::styled(" ".repeat(padding_len), base_style));
        }
    } else {
        // No search highlighting - render normally with padding
        spans.push(Span::styled(format!("{:<40}", display_name), base_style));
    }

    // Add size info
    spans.push(Span::styled(format!(" {:>10}", size_str), base_style));

    ListItem::new(Line::from(spans))
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    // Show error message if present, otherwise show current path
    let display_path = if let Some(ref error) = app.error_message {
        format!("Error: {}", error)
    } else {
        app.current_dir.to_string_lossy().to_string()
    };

    let footer_text = match app.mode {
        Mode::Normal => {
            let sort_name = match app.sort_mode {
                SortMode::Name => "Name",
                SortMode::Size => "Size",
                SortMode::Modified => "Modified",
            };
            let sort_order = if app.sort_ascending { "↑" } else { "↓" };
            let sort_info = format!("{} {}", sort_name, sort_order);

            let right_info = match &app.clipboard {
                ClipboardOperation::Copy(paths) => format!("Sort: {} | Copied: {}", sort_info, paths.len()),
                ClipboardOperation::Cut(paths) => format!("Sort: {} | Cut: {}", sort_info, paths.len()),
                ClipboardOperation::None => format!("Sort: {}", sort_info),
            };

            // Calculate padding needed between left and right sections
            let total_len = display_path.len() + right_info.len();
            let available_width = area.width as usize;

            if total_len < available_width {
                // Add padding between left and right
                let padding = available_width.saturating_sub(total_len);
                format!("{}{}{}", display_path, " ".repeat(padding), right_info)
            } else {
                // Not enough space, just show path and truncate if needed
                display_path
            }
        }
        Mode::VisualMulti => {
            let right_info = "Multi-Select: j/k=add m=remove ENTER=keep ESC=clear";
            let total_len = display_path.len() + right_info.len();
            let available_width = area.width as usize;

            if total_len < available_width {
                let padding = available_width.saturating_sub(total_len);
                format!("{}{}{}", display_path, " ".repeat(padding), right_info)
            } else {
                display_path
            }
        }
        Mode::Search => format!("Search: {}", app.search_query),
        Mode::SortMenu => {
            let right_info = "Sort: [n]ame [s]ize [m]odified ESC=cancel";
            let total_len = display_path.len() + right_info.len();
            let available_width = area.width as usize;

            if total_len < available_width {
                let padding = available_width.saturating_sub(total_len);
                format!("{}{}{}", display_path, " ".repeat(padding), right_info)
            } else {
                display_path
            }
        }
        Mode::Create => format!("Create (end with / for folder): {}", app.create_input),
        Mode::Rename => format!("Rename: {}", app.rename_input),
        Mode::Help => String::from("Press ESC or ? to close help"),
        Mode::DeleteConfirm => {
            let count = app.delete_targets.len();
            if count == 1 {
                if let Some(path) = app.delete_targets.first() {
                    format!("Delete {}? [y/N]", path.file_name().unwrap_or_default().to_string_lossy())
                } else {
                    String::from("Delete? [y/N]")
                }
            } else {
                format!("Delete {} items? [y/N]", count)
            }
        }
    };

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default());

    frame.render_widget(footer, area);

    // Set cursor position for rename and create modes
    match app.mode {
        Mode::Rename => {
            // "Rename: " is 8 characters
            let cursor_x = area.x + 8 + app.rename_cursor_pos as u16;
            let cursor_y = area.y;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
        Mode::Create => {
            // "Create (end with / for folder): " is 32 characters
            let cursor_x = area.x + 32 + app.create_input.len() as u16;
            let cursor_y = area.y;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
        _ => {}
    }
}

fn render_help(frame: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.mode {
        Mode::Help => vec![
            Line::from(""),
            Line::from("Jumper - Keyboard Shortcuts").bold(),
            Line::from(""),
            Line::from("Navigation:"),
            Line::from("  j/↓     - Move down"),
            Line::from("  k/↑     - Move up"),
            Line::from("  h/←     - Go to parent directory"),
            Line::from("  l/→     - Enter directory / Open file"),
            Line::from(""),
            Line::from("File Operations:"),
            Line::from("  a       - Create file/folder"),
            Line::from("  yy      - Copy current file"),
            Line::from("  x       - Cut current file"),
            Line::from("  p       - Paste"),
            Line::from("  d       - Delete file/folder"),
            Line::from(""),
            Line::from("Marking:"),
            Line::from("  m       - Toggle mark on current file"),
            Line::from("  y       - Copy all marked files"),
            Line::from("  x       - Cut all marked files"),
            Line::from("  d       - Delete all marked files"),
            Line::from(""),
            Line::from("Multi-Select (Shift+V):"),
            Line::from("  V       - Enter multi-select mode"),
            Line::from("  j/k     - Navigate and auto-add to selection"),
            Line::from("  m       - Remove current file from selection"),
            Line::from("  y       - Copy selection and exit"),
            Line::from("  x       - Cut selection and exit"),
            Line::from("  ENTER   - Exit and keep marks"),
            Line::from("  ESC     - Exit and clear all marks"),
            Line::from(""),
            Line::from("Other:"),
            Line::from("  /       - Search (fuzzy)"),
            Line::from("  .       - Toggle hidden files"),
            Line::from("  s       - Sort menu"),
            Line::from("  o       - Toggle sort order (↑/↓)"),
            Line::from("  ?       - Show this help"),
            Line::from("  q       - Quit"),
            Line::from(""),
            Line::from("Press ESC to return").bold(),
        ],
        _ => vec![],
    };

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default());

    frame.render_widget(help, area);
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

