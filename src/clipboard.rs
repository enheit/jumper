use anyhow::Result;
use arboard::Clipboard;
use std::path::Path;

pub fn copy_to_system_clipboard(paths: &[impl AsRef<Path>]) -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    let text = paths
        .iter()
        .map(|p| p.as_ref().to_string_lossy())
        .collect::<Vec<_>>()
        .join("\n");
    clipboard.set_text(text)?;
    Ok(())
}

pub fn get_from_system_clipboard() -> Result<String> {
    let mut clipboard = Clipboard::new()?;
    let text = clipboard.get_text()?;
    Ok(text)
}
