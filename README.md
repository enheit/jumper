# Jumper

A blazing fast terminal file manager with vim-like keybindings, built with Rust and Ratatui.

## Features

- **Vim-like Navigation**: Navigate with hjkl or arrow keys
- **Fuzzy Search**: Real-time fuzzy file searching with `/`
- **File Operations**: Copy (yy), cut (x), and paste (p) files and directories
- **Visual Selection**: Select single (v) or multiple files (Shift+V)
- **Quick Jumps**: Customizable shortcuts like `gh` (home), `gd` (downloads), `gp` (projects)
- **Sorting**: Sort by name, size, or modified time with `o`
- **Hidden Files**: Toggle hidden files visibility with `.`
- **Customizable**: Colors, keybindings, and behaviors via TOML config
- **Fast & Async**: Built with Tokio for non-blocking file operations

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/enheit/jumper.git
cd jumper

# Install using cargo
cargo install --path .
```

### From crates.io (coming soon)

```bash
cargo install jumper
```

## Usage

Simply run `jumper` in your terminal:

```bash
jumper
```

## Keybindings

### Navigation
| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Go to parent directory |
| `l` / `→` | Enter directory / Open file |
| `q` | Quit |

### File Operations
| Key | Action |
|-----|--------|
| `yy` | Copy file/directory |
| `x` | Cut file/directory |
| `p` | Paste |
| `v` | Visual mode (select single) |
| `Shift+V` | Visual multi-select mode |

### Other
| Key | Action |
|-----|--------|
| `/` | Search (fuzzy) |
| `.` | Toggle hidden files |
| `o` | Sort menu |
| `gh` | Go to home (configurable) |
| `gd` | Go to downloads (configurable) |
| `gp` | Go to projects (configurable) |

## Configuration

Jumper looks for configuration at `~/.config/jumper/config.toml`. A default config is created on first run.

### Example Configuration

```toml
[colors]
directory = "blue"
file = "white"
selected = "#00FF00"
hidden = "gray"
symlink = "cyan"
executable = "red"

[keybindings.quick_jumps]
gh = "/home/username"
gd = "/home/username/Downloads"
gp = "/home/username/Projects"
gc = "/home/username/.config"

[behavior]
show_hidden = false
default_sort = "name"  # options: "name", "size", "modified"
```

### Color Options

Colors can be specified as:
- Named colors: `"blue"`, `"red"`, `"green"`, `"yellow"`, `"cyan"`, `"magenta"`, `"white"`, `"black"`, `"gray"`, etc.
- Hex colors: `"#FF0000"`, `"#00FF00"`, etc.

## Visual Mode

### Single Select (`v`)
1. Press `v` to enter visual mode
2. Navigate with `j/k`
3. Press `y` to copy or `x` to cut
4. Press `ESC` to cancel

### Multi-Select (`Shift+V`)
1. Press `Shift+V` to enter visual multi-select mode
2. Navigate with `j/k` to select multiple items
3. Press `y` to copy all or `x` to cut all
4. Press `ESC` to cancel

## Sorting

Press `o` to open the sort menu, then:
- `n` - Sort by name
- `s` - Sort by size
- `m` - Sort by modified time
- `ESC` - Cancel

Directories are always shown above files.

## Building from Source

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Build

```bash
cargo build --release
```

The binary will be available at `target/release/jumper`.

## Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Tech Stack

- **[Ratatui](https://github.com/ratatui-org/ratatui)** - Terminal UI framework
- **[Crossterm](https://github.com/crossterm-rs/crossterm)** - Cross-platform terminal manipulation
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher)** - Fuzzy search
- **[fs_extra](https://github.com/webdesus/fs_extra)** - Extended file operations
- **[arboard](https://github.com/1Password/arboard)** - Cross-platform clipboard

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by other great terminal file managers:
- [Yazi](https://github.com/sxyazi/yazi)
- [Joshuto](https://github.com/kamiyaa/joshuto)
- [ranger](https://github.com/ranger/ranger)
