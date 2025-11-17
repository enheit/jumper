use crate::app::{App, ClipboardOperation};
use anyhow::Result;
use fs_extra::dir;
use std::fs;
use std::path::Path;

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

fn copy_items(sources: &[std::path::PathBuf], dest: &Path) -> Result<()> {
    let dir_options = fs_extra::dir::CopyOptions::new();
    let file_options = fs_extra::file::CopyOptions::new();

    for source in sources {
        if source.is_dir() {
            let dir_name = source.file_name().unwrap();
            let dest_path = dest.join(dir_name);
            dir::copy(source, &dest_path, &dir_options)?;
        } else {
            fs_extra::file::copy(
                source,
                dest.join(source.file_name().unwrap()),
                &file_options,
            )?;
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
            let dest_path = dest.join(dir_name);
            dir::move_dir(source, &dest_path, &dir_options)?;
        } else {
            fs_extra::file::move_file(
                source,
                dest.join(source.file_name().unwrap()),
                &file_options,
            )?;
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
