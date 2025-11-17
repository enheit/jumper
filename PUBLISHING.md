# Publishing Jumper

This document describes how to publish Jumper to various distribution channels.

## GitHub Releases

### Creating a Release

1. **Tag the version:**
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin v1.0.0
   ```

2. **Build release binaries:**
   ```bash
   # Linux x86_64
   cargo build --release --target x86_64-unknown-linux-gnu

   # Linux aarch64 (if you have the toolchain)
   cargo build --release --target aarch64-unknown-linux-gnu
   ```

3. **Create release on GitHub:**
   - Go to https://github.com/enheit/jumper/releases/new
   - Choose the tag you just created
   - Write release notes
   - Upload the binary from `target/release/jumper`
   - Consider creating a tarball:
     ```bash
     tar -czf jumper-v1.0.0-x86_64-linux.tar.gz -C target/release jumper
     ```

## Arch User Repository (AUR)

### Initial Setup

1. **Create AUR account:**
   - Go to https://aur.archlinux.org/register
   - Verify your email

2. **Add SSH key:**
   - Generate SSH key if you don't have one:
     ```bash
     ssh-keygen -t ed25519 -C "your.email@example.com"
     ```
   - Add public key to your AUR account
   - Add this to `~/.ssh/config`:
     ```
     Host aur.archlinux.org
       IdentityFile ~/.ssh/id_ed25519
       User aur
     ```

3. **Test SSH connection:**
   ```bash
   ssh aur@aur.archlinux.org
   ```

### Publishing Package

1. **Clone AUR repository (first time only):**
   ```bash
   git clone ssh://aur@aur.archlinux.org/jumper-git.git
   cd jumper-git
   ```

2. **Copy package files:**
   ```bash
   cp ~/jumper/aur/PKGBUILD .
   ```

3. **Update maintainer info in PKGBUILD:**
   Edit the top line with your information.

4. **Test the package:**
   ```bash
   makepkg -si
   ```

5. **Generate .SRCINFO:**
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

6. **Commit and push:**
   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Initial import"
   git push
   ```

### Updating the Package

The package will automatically track your git repository. When you:
- Push new commits
- Create new releases

Users will get updates when they rebuild. If you change dependencies or build process:

1. Update PKGBUILD
2. Test: `makepkg -si`
3. Regenerate: `makepkg --printsrcinfo > .SRCINFO`
4. Commit and push

## Crates.io (Rust Package Registry)

### Preparation

1. **Create account:**
   - Go to https://crates.io
   - Sign in with GitHub

2. **Get API token:**
   - Go to https://crates.io/settings/tokens
   - Create new token

3. **Login to cargo:**
   ```bash
   cargo login YOUR_TOKEN_HERE
   ```

### Publishing

1. **Update Cargo.toml:**
   Ensure all metadata is correct:
   ```toml
   [package]
   name = "jumper"
   version = "1.0.0"
   authors = ["Your Name <your.email@example.com>"]
   edition = "2021"
   description = "A blazing fast terminal file manager with vim-like keybindings"
   license = "MIT"
   repository = "https://github.com/enheit/jumper"
   keywords = ["cli", "file-manager", "terminal", "tui"]
   categories = ["command-line-utilities"]
   ```

2. **Test the package:**
   ```bash
   cargo package --list  # See what will be included
   cargo package         # Create the package
   ```

3. **Publish:**
   ```bash
   cargo publish
   ```

4. **Verify:**
   Check https://crates.io/crates/jumper

## Homebrew (macOS/Linux)

For Homebrew, you'll need to create a formula. This is typically done after you have stable releases.

### Creating a Formula

1. **Fork homebrew-core:**
   ```bash
   gh repo fork Homebrew/homebrew-core
   ```

2. **Create formula:**
   ```bash
   brew create https://github.com/enheit/jumper/archive/v1.0.0.tar.gz
   ```

3. **Edit the formula** in the editor that opens

4. **Test:**
   ```bash
   brew install --build-from-source jumper
   brew test jumper
   brew audit --strict jumper
   ```

5. **Submit PR to homebrew-core**

## Install Script

The `install.sh` script is already created and should be committed to your repository. It will:
- Detect OS and architecture
- Download pre-built binaries from GitHub releases
- Fall back to building from source if no binary available
- Install to `~/.local/bin`

Users can install with:
```bash
curl -sSL https://raw.githubusercontent.com/enheit/jumper/main/install.sh | bash
```

## Checklist Before Publishing

- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] README.md is up to date
- [ ] CHANGELOG.md is updated (if you have one)
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created
- [ ] GitHub release created with binaries
- [ ] AUR package updated
- [ ] Crates.io published
