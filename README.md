# Jumper

A blazing fast terminal file manager with vim-like keybindings, built with Rust and Ratatui.

## Features

- **Vim-like Navigation**: Navigate with hjkl or arrow keys
- **Fuzzy Search**: Real-time fuzzy file searching with character-level highlighting
- **File Operations**: Copy, cut, paste, delete, and create files/directories
- **Mark System**: Mark multiple files with `m` and operate on them at once
- **Multi-Select Mode**: Visual multi-select with `Shift+V` for range selection
- **Quick Jumps**: Customizable shortcuts like `gh` (home), `gd` (downloads), `gp` (projects)
- **Navigation History**: Go back with `Ctrl+O` through your navigation history
- **Sorting**: Sort by name, size, or modified time (ascending/descending)
- **Directory Sizes**: Async calculation of directory sizes with loading indicator
- **Hidden Files**: Toggle hidden files visibility with `.`
- **Visual Feedback**: Cut files shown dimmed, copied files flash yellow
- **Customizable**: Colors, keybindings, and behaviors via TOML config
- **Fast & Async**: Built with Tokio for non-blocking operations

## Installation

### Quick Install (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/enheit/jumper/main/install.sh | bash
```

This will download and install the latest release to `~/.local/bin/jumper`.

### Arch Linux (AUR)

```bash
yay -S jumper-git
```

Or with any other AUR helper:

```bash
paru -S jumper-git
```

**After installation**, add shell integration to your shell config:

```bash
# For Bash - add to ~/.bashrc
echo 'source /usr/share/jumper/jumper.sh' >> ~/.bashrc

# For Zsh - add to ~/.zshrc
echo 'source /usr/share/jumper/jumper.sh' >> ~/.zshrc

# For Fish - add to ~/.config/fish/config.fish
echo 'source /usr/share/jumper/jumper.fish' >> ~/.config/fish/config.fish
```

Then reload your shell: `source ~/.bashrc`

### From Source

```bash
# Clone the repository
git clone https://github.com/enheit/jumper.git
cd jumper

# Install using cargo
cargo install --path .
```

**After installation**, add shell integration to your shell config:

```bash
# For Bash - add to ~/.bashrc
echo 'source ~/.local/share/jumper/shell/jumper.sh' >> ~/.bashrc

# For Zsh - add to ~/.zshrc
echo 'source ~/.local/share/jumper/shell/jumper.sh' >> ~/.zshrc

# For Fish - add to ~/.config/fish/config.fish
echo 'source ~/.local/share/jumper/shell/jumper.fish' >> ~/.config/fish/config.fish
```

Then reload your shell: `source ~/.bashrc`

### From crates.io (coming soon)

```bash
cargo install jumper
```

**After installation**, add shell integration (same as "From Source" above)

### Manual Installation

Download the latest release from [GitHub Releases](https://github.com/enheit/jumper/releases) and extract it to a directory in your PATH.

### Shell Integration (Required for Directory Navigation)

For `jumper` to change your shell's directory when you quit, source the shell integration script.

The install script automatically copies integration scripts to `~/.local/share/jumper/shell/`.

**Bash** - Add to `~/.bashrc`:
```bash
source ~/.local/share/jumper/shell/jumper.sh
```

**Zsh** - Add to `~/.zshrc`:
```zsh
source ~/.local/share/jumper/shell/jumper.sh
```

**Fish** - Add to `~/.config/fish/config.fish`:
```fish
source ~/.local/share/jumper/shell/jumper.fish
```

Then reload your shell:
```bash
source ~/.bashrc  # or ~/.zshrc
```

After setup, `jumper` will automatically change your shell's directory when you quit!

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
| `gg` | Jump to top |
| `G` | Jump to bottom |
| `Ctrl+O` | Go back in navigation history |
| `q` | Quit |
| `?` | Show help |

### File Operations
| Key | Action |
|-----|--------|
| `a` | Create new file/folder |
| `r` | Rename (cursor before extension) |
| `R` | Rename (cursor at end, with extension) |
| `yy` | Copy current file |
| `y` | Copy marked files |
| `x` | Cut current/marked files |
| `p` | Paste |
| `d` | Delete current/marked files |
| `m` | Toggle mark on current file |
| `Shift+V` | Multi-select mode |
| `ESC` | Clear marks/cut clipboard/search |

### Multi-Select Mode (Shift+V)
| Key | Action |
|-----|--------|
| `j` / `↓` | Move down and expand selection |
| `k` / `↑` | Move up and shrink selection |
| `gg` | Jump to top and select all to top |
| `G` | Jump to bottom and select all to bottom |
| `m` | Remove current file from selection |
| `y` | Copy selection and exit |
| `x` | Cut selection and exit |
| `d` | Delete selection |
| `Enter` | Exit and keep marks |
| `ESC` | Exit and clear all marks |

### Search & Sort
| Key | Action |
|-----|--------|
| `/` | Search (fuzzy) |
| `.` | Toggle hidden files |
| `s` | Open sort menu |
| `o` | Toggle sort order (ascending/descending) |

### Quick Jumps (Configurable)
| Key | Default Location |
|-----|-----------------|
| `gh` | Home directory |
| `gd` | Downloads |
| `gp` | Projects |

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
