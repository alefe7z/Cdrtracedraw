//! File access and file dialogs.
//!
//! Thin wrappers over the file system and the system file dialogs, so
//! that the open, save and import code goes through one place.
//!
//! Dialogs come in three shapes:
//! - [`Dialog::pick_file`] runs a closure with the picked path at once.
//! - [`Dialog::pick_into`] stores the picked path under a key that a
//!   dialog's own code collects with [`take_picked`] on the same frame.
//! - [`Dialog::save_file`] runs a closure with the chosen path at once.

use crate::app::App;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

thread_local! {
    /// Picked paths waiting for the dialog that asked for them.
    static PICKED: RefCell<HashMap<String, PathBuf>> = RefCell::new(HashMap::new());
}

/// Read a whole file.
pub fn read(path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
    std::fs::read(path)
}

/// Read a whole file as UTF-8 text.
pub fn read_to_string(path: impl AsRef<Path>) -> std::io::Result<String> {
    let bytes = read(path)?;
    String::from_utf8(bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
}

/// Write a whole file.
pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> std::io::Result<()> {
    std::fs::write(path, bytes)
}

/// Copy a file, creating the target folder (backups).
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<()> {
    if let Some(dir) = to.as_ref().parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::copy(from, to).map(|_| ())
}

/// True when a file exists on disk.
pub fn exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_file()
}

/// Write a file, creating its folder first (auto-backups).
pub fn write_creating(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> std::io::Result<()> {
    if let Some(dir) = path.as_ref().parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, bytes)
}

/// Delete a file.
pub fn remove(path: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::remove_file(path)
}

/// Choose a folder (`None` when cancelled).
pub fn pick_folder() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_folder()
}

/// The files directly inside a folder (not its subfolders), in no
/// particular order; empty when it cannot be read.
pub fn list_dir(path: impl AsRef<Path>) -> Vec<PathBuf> {
    match std::fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// One entry of a folder listing (the Save Drawing dialog).
#[derive(Debug, Clone, PartialEq)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    /// Bytes (files).
    pub size: u64,
    /// Seconds since 1970, when known.
    pub modified: Option<i64>,
}

/// The folders and files directly inside `dir`: folders first, then by
/// name ignoring case; hidden ones (a leading dot) left out. Empty when
/// the folder cannot be read.
pub fn list_entries(dir: impl AsRef<Path>) -> Vec<DirEntry> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<DirEntry> = read
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                return None;
            }
            let path = e.path();
            // Follow links: a link to a folder lists as a folder.
            let meta = std::fs::metadata(&path).ok();
            let is_dir = meta.as_ref().is_some_and(|m| m.is_dir());
            let size = meta.as_ref().map_or(0, |m| m.len());
            let modified = meta
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64);
            Some(DirEntry {
                name,
                path,
                is_dir,
                size,
                modified,
            })
        })
        .collect();
    out.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    out
}

/// True for a folder on disk.
pub fn is_dir(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_dir()
}

/// Make a folder.
pub fn create_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir(path)
}

/// A folder worth a shortcut in the Save Drawing dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Home,
    Desktop,
    Documents,
    Downloads,
    Pictures,
    /// A drive or the file system's root.
    Root,
}

/// The user's home folder.
pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
}

/// The shortcuts that exist on this computer: home, desktop, documents,
/// downloads and pictures (the names the system gives them, read from
/// the user's folder settings on Linux), then the drives (Windows) or
/// the root.
pub fn places() -> Vec<(Place, PathBuf)> {
    let mut out = Vec::new();
    let Some(home) = home_dir() else {
        return out;
    };
    out.push((Place::Home, home.clone()));
    let xdg = xdg_user_dirs(&home);
    for (place, key, default) in [
        (Place::Desktop, "XDG_DESKTOP_DIR", "Desktop"),
        (Place::Documents, "XDG_DOCUMENTS_DIR", "Documents"),
        (Place::Downloads, "XDG_DOWNLOAD_DIR", "Downloads"),
        (Place::Pictures, "XDG_PICTURES_DIR", "Pictures"),
    ] {
        let dir = xdg
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| home.join(default));
        if dir.is_dir() && dir != home {
            out.push((place, dir));
        }
    }
    if cfg!(windows) {
        for letter in b'C'..=b'Z' {
            let root = PathBuf::from(format!("{}:\\", letter as char));
            if root.is_dir() {
                out.push((Place::Root, root));
            }
        }
    } else {
        out.push((Place::Root, PathBuf::from("/")));
    }
    out
}

/// The `XDG_*_DIR` folders of `~/.config/user-dirs.dirs` (Linux), with
/// `$HOME` expanded.
fn xdg_user_dirs(home: &Path) -> Vec<(String, PathBuf)> {
    let Ok(text) = std::fs::read_to_string(home.join(".config/user-dirs.dirs")) else {
        return Vec::new();
    };
    parse_user_dirs(&text, home)
}

/// Read `KEY="$HOME/Folder"` lines.
pub(crate) fn parse_user_dirs(text: &str, home: &Path) -> Vec<(String, PathBuf)> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            let value = value.trim().trim_matches('"');
            let path = match value.strip_prefix("$HOME") {
                Some(rest) => home.join(rest.trim_start_matches('/')),
                None => PathBuf::from(value),
            };
            Some((key.trim().to_string(), path))
        })
        .collect()
}

/// Decode an image file (any format the image crate was built with).
pub fn open_image(path: impl AsRef<Path>) -> Result<image::DynamicImage, String> {
    let bytes = read(path).map_err(|e| e.to_string())?;
    image::load_from_memory(&bytes).map_err(|e| e.to_string())
}

/// Files dropped on the window this frame: `then` runs with each one's
/// path.
pub fn take_dropped(
    ctx: &egui::Context,
    app: &mut App,
    then: impl FnOnce(&mut App, PathBuf) + Clone + 'static,
) {
    let dropped = ctx.input(|i| i.raw.dropped_files.clone());
    for file in dropped {
        then.clone()(app, file.path().to_path_buf());
    }
}

/// The path picked for `key` by [`Dialog::pick_into`], once.
pub fn take_picked(key: &str) -> Option<PathBuf> {
    PICKED.with(|p| p.borrow_mut().remove(key))
}

/// A file dialog description.
#[derive(Debug, Clone, Default)]
pub struct Dialog {
    filters: Vec<(String, Vec<String>)>,
    file_name: Option<String>,
}

impl Dialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_filter(mut self, name: impl Into<String>, extensions: &[&str]) -> Self {
        self.filters.push((
            name.into(),
            extensions.iter().map(|e| e.to_string()).collect(),
        ));
        self
    }

    pub fn set_file_name(mut self, name: impl Into<String>) -> Self {
        self.file_name = Some(name.into());
        self
    }

    /// Pick a file and run `then` with its path.
    pub fn pick_file(self, app: &mut App, then: impl FnOnce(&mut App, PathBuf) + 'static) {
        if let Some(path) = self.native_dialog().pick_file() {
            then(app, path);
        }
    }

    /// Pick one or more files and run `then` with their paths.
    pub fn pick_files(self, app: &mut App, then: impl FnOnce(&mut App, Vec<PathBuf>) + 'static) {
        if let Some(paths) = self.native_dialog().pick_files() {
            then(app, paths);
        }
    }

    /// Pick a file and keep its path for [`take_picked`]`(key)`.
    pub fn pick_into(self, key: &str) {
        if let Some(path) = self.native_dialog().pick_file() {
            PICKED.with(|p| {
                p.borrow_mut().insert(key.to_string(), path);
            });
        }
    }

    /// Choose where to save and run `then` with that path; `then` writes
    /// with [`write`].
    pub fn save_file(self, app: &mut App, then: impl FnOnce(&mut App, PathBuf)) {
        if let Some(path) = self.save_path() {
            then(app, path);
        }
    }

    /// The path to save to, or `None` when the user cancels.
    pub fn save_path(self) -> Option<PathBuf> {
        let mut d = self.native_dialog();
        if let Some(n) = &self.file_name {
            d = d.set_file_name(n);
        }
        d.save_file()
    }

    /// The system dialog with this description's filters.
    fn native_dialog(&self) -> rfd::FileDialog {
        let mut d = rfd::FileDialog::new();
        for (name, exts) in &self.filters {
            let exts: Vec<&str> = exts.iter().map(|e| e.as_str()).collect();
            d = d.add_filter(name, &exts);
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picked_paths_are_taken_once() {
        PICKED.with(|p| {
            p.borrow_mut().insert("k".into(), PathBuf::from("/x/a.csv"));
        });
        assert_eq!(take_picked("k"), Some(PathBuf::from("/x/a.csv")));
        assert_eq!(take_picked("k"), None);
    }

    #[test]
    fn read_and_write_use_the_file_system() {
        let dir = std::env::temp_dir().join(format!("tracedraw-files-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("a.bin");
        write(&p, [1u8, 2, 3]).unwrap();
        assert_eq!(read(&p).unwrap(), vec![1, 2, 3]);
        assert!(read(dir.join("missing")).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
