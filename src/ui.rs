use crate::app::{App, ClipboardOperation, FileEntry, Mode};
use crate::config::SortMode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::Line,
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
    let filtered_files = app.get_filtered_files();

    let items: Vec<ListItem> = filtered_files
        .iter()
        .enumerate()
        .map(|(idx, file)| {
            let is_selected = app.selected_indices.contains(&idx);
            create_list_item(file, is_selected, app)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(
            Style::default()
                .bg(app.config.colors.selected)
                .fg(ratatui::style::Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");

    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn create_list_item<'a>(file: &'a FileEntry, is_selected: bool, app: &App) -> ListItem<'a> {
    let icon = if file.is_dir {
        ""
    } else if file.is_symlink {
        ""
    } else if file.is_executable {
        ""
    } else {
        ""
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

    let mut style = Style::default().fg(color);
    if is_selected {
        style = style.add_modifier(Modifier::REVERSED);
    }

    let size_str = if file.is_dir {
        String::from("DIR")
    } else {
        format_size(file.size)
    };

    let content = format!("{} {:<40} {:>10}", icon, file.name, size_str);
    ListItem::new(content).style(style)
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let current_path = app.current_dir.to_string_lossy().to_string();

    let footer_text = match app.mode {
        Mode::Normal => {
            let sort_info = match app.sort_mode {
                SortMode::Name => "Name",
                SortMode::Size => "Size",
                SortMode::Modified => "Modified",
            };

            let clipboard_info = match &app.clipboard {
                ClipboardOperation::Copy(paths) => format!(" | Copied: {}", paths.len()),
                ClipboardOperation::Cut(paths) => format!(" | Cut: {}", paths.len()),
                ClipboardOperation::None => String::new(),
            };

            format!("{:<width$}Sort: {}{}",
                current_path,
                sort_info,
                clipboard_info,
                width = area.width.saturating_sub(30) as usize
            )
        }
        Mode::Visual => format!("{:<width$}Visual: y=copy x=cut ESC=cancel",
            current_path,
            width = area.width.saturating_sub(35) as usize
        ),
        Mode::VisualMulti => format!("{:<width$}Visual Multi: y=copy x=cut ESC=cancel",
            current_path,
            width = area.width.saturating_sub(40) as usize
        ),
        Mode::Search => format!("Search: {}", app.search_query),
        Mode::SortMenu => format!("{:<width$}Sort: [n]ame [s]ize [m]odified ESC=cancel",
            current_path,
            width = area.width.saturating_sub(45) as usize
        ),
        Mode::Create => format!("Create (end with / for folder): {}", app.create_input),
        Mode::Help => String::from("Press ESC or ? to close help"),
    };

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default());

    frame.render_widget(footer, area);
}

fn render_help(frame: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.mode {
        Mode::Help => vec![
            Line::from(""),
            Line::from(" Jumper - Keyboard Shortcuts").bold(),
            Line::from(""),
            Line::from(" Navigation:"),
            Line::from("   j/↓     - Move down"),
            Line::from("   k/↑     - Move up"),
            Line::from("   h/←     - Go to parent directory"),
            Line::from("   l/→     - Enter directory / Open file"),
            Line::from(""),
            Line::from(" File Operations:"),
            Line::from("   a       - Create file/folder"),
            Line::from("   yy      - Copy file/folder"),
            Line::from("   x       - Cut file/folder"),
            Line::from("   p       - Paste"),
            Line::from(""),
            Line::from(" Selection:"),
            Line::from("   v       - Visual mode (single select)"),
            Line::from("   V       - Visual multi-select"),
            Line::from(""),
            Line::from(" Other:"),
            Line::from("   /       - Search (fuzzy)"),
            Line::from("   .       - Toggle hidden files"),
            Line::from("   o       - Sort menu"),
            Line::from("   ?       - Show this help"),
            Line::from("   q       - Quit"),
            Line::from(""),
            Line::from(" Press ESC to return").bold(),
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
