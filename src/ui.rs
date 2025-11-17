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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(1),    // File list
            Constraint::Length(3), // Footer/Status
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);
    render_file_list(frame, app, chunks[1]);
    render_footer(frame, app, chunks[2]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let current_path = app.current_dir.to_string_lossy().to_string();
    let header = Paragraph::new(current_path).style(Style::default().bold());
    frame.render_widget(header, area);
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
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(app.config.colors.selected)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn create_list_item<'a>(file: &'a FileEntry, is_selected: bool, app: &App) -> ListItem<'a> {
    let icon = if file.is_dir {
        " "
    } else if file.is_symlink {
        " "
    } else if file.is_executable {
        " "
    } else {
        " "
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
    let footer_content = match app.mode {
        Mode::Normal => {
            let clipboard_info = match &app.clipboard {
                ClipboardOperation::Copy(paths) => format!(" | Copied: {} items", paths.len()),
                ClipboardOperation::Cut(paths) => format!(" | Cut: {} items", paths.len()),
                ClipboardOperation::None => String::new(),
            };

            let sort_info = format!(
                "Sort: {}",
                match app.sort_mode {
                    SortMode::Name => "Name",
                    SortMode::Size => "Size",
                    SortMode::Modified => "Modified",
                }
            );

            let hidden_info = if app.show_hidden {
                "Hidden: ON"
            } else {
                "Hidden: OFF"
            };

            format!(
                " Normal | {} | {}{}",
                sort_info, hidden_info, clipboard_info
            )
        }
        Mode::Visual => String::from(" Visual | v: copy, x: cut, ESC: cancel"),
        Mode::VisualMulti => String::from(" Visual Multi | y: copy, x: cut, ESC: cancel"),
        Mode::Search => format!(" Search: {}", app.search_query),
        Mode::SortMenu => String::from(" Sort by: [n]ame, [s]ize, [m]odified, ESC: cancel"),
    };

    let help = vec![
        Line::from(footer_content),
        Line::from(" q: quit | hjkl: navigate | .: toggle hidden | /: search | o: sort | v: visual | yy: copy | x: cut | p: paste"),
    ];

    let footer = Paragraph::new(help)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());

    frame.render_widget(footer, area);
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
