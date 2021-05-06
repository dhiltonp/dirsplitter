use std::ffi::OsStr;
use std::{fs, path};

/// Returns true if the path is a .jpg, .jpeg or .png, handling case sensitivity.
fn is_image(file: path::PathBuf) -> bool {
    let image_types = ["jpg", "jpeg", "png"];

    let extension = match file.extension() {
        Some(extension) => extension.to_ascii_lowercase(),
        _ => return false,
    };

    for t in image_types.iter() {
        if extension == OsStr::new(t) {
            return true;
        }
    }

    false
}

#[test]
fn test_is_image() {
    assert!(is_image(path::PathBuf::from("foo")) == false);
    assert!(is_image(path::PathBuf::from("jpg")) == false);
    assert!(is_image(path::PathBuf::from("blah.jpg")) == true);
    assert!(is_image(path::PathBuf::from("blah.JPG")) == true);
}

pub fn images(path: &path::Path) -> Vec<path::PathBuf> {
    let mut images = Vec::new();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() && is_image(entry.path()) {
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
