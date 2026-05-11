use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use crate::{AppResult, error::AppError};

fn safe_join(root: &Path, name: &str) -> AppResult<PathBuf> {
    let out = root.join(name);
    let canon_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let parent = out.parent().unwrap_or(root);
    fs::create_dir_all(parent)?;
    let canon_parent = parent.canonicalize().unwrap_or_else(|_| parent.to_path_buf());
    if !canon_parent.starts_with(&canon_root) {
        return Err(AppError::Blocked("zip entry escapes target directory".into()));
    }
    Ok(out)
}

pub fn extract_zip_safely(zip_path: &Path, target: &Path) -> AppResult<()> {
    fs::create_dir_all(target)?;
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        if name.contains("..") || name.starts_with('/') || name.starts_with('\\') {
            return Err(AppError::Blocked("path traversal detected in archive".into()));
        }
        let out = safe_join(target, &name)?;
        if entry.name().ends_with('/') {
            fs::create_dir_all(&out)?;
        } else {
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out_file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&out)?;
            io::copy(&mut entry, &mut out_file)?;
        }
    }
    Ok(())
}

pub fn symlink_or_copy(src: &Path, dst: &Path) -> AppResult<&'static str> {
    #[cfg(unix)]
    {
        if std::os::unix::fs::symlink(src, dst).is_ok() {
            return Ok("symlink");
        }
    }
    #[cfg(windows)]
    {
        if std::os::windows::fs::symlink_dir(src, dst).is_ok() {
            return Ok("symlink");
        }
    }
    copy_dir_all(src, dst)?;
    Ok("copy")
}

fn copy_dir_all(src: &Path, dst: &Path) -> AppResult<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
