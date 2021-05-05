use std::ffi::OsStr;
use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
    // add subdir prefix
    // add number of images per dir
}

#[derive(Debug)]
struct DirContents {
    subdirs: Vec<path::PathBuf>,
    images: Vec<path::PathBuf>,
}

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

fn read_dir(path: path::PathBuf) -> DirContents {
    let mut subdirs = Vec::new();
    let mut images = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
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
    DirContents { subdirs, images }
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", read_dir(args.path));
}
