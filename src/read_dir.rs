use std::{path, fs};
use std::ffi::OsStr;

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

#[derive(Debug)]
pub struct DirContents {
    pub path: path::PathBuf,
    pub subdirs: Vec<path::PathBuf>,
    pub images: Vec<path::PathBuf>,
}

pub fn read_dir(path: &path::PathBuf) -> DirContents {
    let mut subdirs = Vec::new();
    let mut images = Vec::new();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                _ => continue,
            };
            if file_type.is_dir() {
                subdirs.push(entry.path());
            } else if file_type.is_file() && is_image(entry.path()) {
                images.push(entry.path());
            }
        }
    }
    DirContents {
        path: path::PathBuf::from(path),
        subdirs,
        images,
    }
}
