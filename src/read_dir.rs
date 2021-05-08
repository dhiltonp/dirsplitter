use std::ffi::OsStr;
use std::{fs, path};

/// Returns true if `file` is a .jpg, .jpeg, .png, .bmp,
///  .webp, .gif or .tiff,... plus a bunch of raw file
///  extensions, handling case sensitivity.
fn is_image(file: &path::Path) -> bool {
    let image_types = [
        "jpg", "jpeg", "png", "bmp", "webp", "gif", "tiff", "pef", "dng", "crw", "nef", "cr2",
        "mrw", "rw2", "orf", "x3f", "arw", "kdc", "nrw", "dcr", "sr2", "raf",
    ];

    if let Some(extension) = file.extension() {
        let extension = extension.to_ascii_lowercase();
        for t in image_types.iter() {
            if extension == OsStr::new(t) {
                return true;
            }
        }
    }

    false
}

#[test]
fn test_is_image() {
    assert_eq!(is_image(&path::PathBuf::from("foo")), false);
    assert_eq!(is_image(&path::PathBuf::from("jpg")), false);
    assert_eq!(is_image(&path::PathBuf::from("blah.jpg")), true);
    assert_eq!(is_image(&path::PathBuf::from("blah.JPG")), true);
}

pub fn images(path: &path::Path) -> Vec<path::PathBuf> {
    let mut images = Vec::new();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() && is_image(&entry.path()) {
                    images.push(entry.path());
                }
            }
        }
    }
    images
}

pub fn subdirs(path: &path::Path) -> Vec<path::PathBuf> {
    let mut subdirs = Vec::new();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    subdirs.push(entry.path());
                }
            }
        }
    }
    subdirs
}
