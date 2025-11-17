use crate::app::{App, ClipboardOperation};
use anyhow::Result;
use fs_extra::dir;
use std::fs;
use std::path::{Path, PathBuf};

pub fn open_file(path: &Path) -> Result<()> {
    open::that(path)?;
    Ok(())
}

pub fn paste(app: &mut App) -> Result<()> {
    let dest = &app.current_dir;

    match &app.clipboard {
        ClipboardOperation::Copy(paths) => {
            copy_items(paths, dest)?;
        }
        ClipboardOperation::Cut(paths) => {
            move_items(paths, dest)?;
            app.clipboard = ClipboardOperation::None;
        }
        ClipboardOperation::None => {}
    }

    Ok(())
}

fn get_unique_path(dest: &Path) -> PathBuf {
    // If path doesn't exist, use it as-is
    if !dest.exists() {
        return dest.to_path_buf();
    }

    // Extract components
    let parent = dest.parent().unwrap_or_else(|| Path::new("."));
    let file_name = dest.file_name().and_then(|s| s.to_str()).unwrap_or("");

    // Check if it's a directory (ends with /)
    let is_dir = dest.is_dir();

    // For files, separate stem and extension
    let (stem, extension) = if is_dir {
        (file_name.to_string(), None)
    } else {
        let stem = dest.file_stem().and_then(|s| s.to_str()).unwrap_or(file_name);
        let ext = dest.extension().and_then(|s| s.to_str());
        (stem.to_string(), ext)
    };

    // Try incrementing numbers until we find a unique name
    for i in 1..=9999 {
        let new_name = if let Some(ext) = extension {
            format!("{} ({}).{}", stem, i, ext)
        } else {
            format!("{} ({})", stem, i)
        };

        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
    }

    // Fallback: just return original (should never happen with 9999 limit)
    dest.to_path_buf()
}

fn copy_items(sources: &[std::path::PathBuf], dest: &Path) -> Result<()> {
    let dir_options = fs_extra::dir::CopyOptions::new();
    let file_options = fs_extra::file::CopyOptions::new();

    for source in sources {
        if source.is_dir() {
            let dir_name = source.file_name().unwrap();
            let initial_dest_path = dest.join(dir_name);
            // Get unique path to avoid conflicts
            let dest_path = get_unique_path(&initial_dest_path);
            dir::copy(source, &dest_path, &dir_options)?;
        } else {
            let initial_dest_path = dest.join(source.file_name().unwrap());
            // Get unique path to avoid conflicts
            let dest_path = get_unique_path(&initial_dest_path);
            fs_extra::file::copy(source, &dest_path, &file_options)?;
        }
    }

    Ok(())
}

fn move_items(sources: &[std::path::PathBuf], dest: &Path) -> Result<()> {
    let dir_options = fs_extra::dir::CopyOptions::new();
    let file_options = fs_extra::file::CopyOptions::new();

    for source in sources {
        if source.is_dir() {
            let dir_name = source.file_name().unwrap();
            let initial_dest_path = dest.join(dir_name);
            // Get unique path to avoid conflicts
            let dest_path = get_unique_path(&initial_dest_path);
            dir::move_dir(source, &dest_path, &dir_options)?;
        } else {
            let initial_dest_path = dest.join(source.file_name().unwrap());
            // Get unique path to avoid conflicts
            let dest_path = get_unique_path(&initial_dest_path);
            fs_extra::file::move_file(source, &dest_path, &file_options)?;
        }
    }

    Ok(())
}

pub fn create_file(path: &Path) -> Result<()> {
    fs::File::create(path)?;
    Ok(())
}

pub fn create_directory(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn delete_path(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}
