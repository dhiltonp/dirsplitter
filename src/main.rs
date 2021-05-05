use std::ffi::OsStr;
use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,

    #[structopt(default_value = "splitter")]
    subdir_prefix: String,

    #[structopt(default_value = "2", long)]
    images_per_dir: usize
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

#[derive(Debug)]
struct DirContents {
    path: path::PathBuf,
    subdirs: Vec<path::PathBuf>,
    images: Vec<path::PathBuf>,
}

fn read_dir(path: path::PathBuf) -> DirContents {
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
    DirContents { path, subdirs, images }
}

fn split_dir(images: Vec<path::PathBuf>) {

}

/// Read a directory, recurse through it.
/// If there are < images_per_dir in that dir, leave it alone.
/// Otherwise, split the dir.
fn main() {
    let args = Cli::from_args();
    let dir_contents = read_dir(args.path);

    if dir_contents.images.len() > args.images_per_dir {
        let mut new_dir_index = 0;
        for i in 0..dir_contents.images.len() {
            if i % args.images_per_dir == 0 {
                loop {
                    let mut new_dir_path = path::PathBuf::from(&dir_contents.path);
                    let dir_name = format!("{}-{}", args.subdir_prefix, new_dir_index);
                    new_dir_path.push(dir_name);
                    new_dir_index += 1;
                    match fs::create_dir(new_dir_path) {
                        Ok(_) => break,
                        _ => continue,
                    }
                }
            }

            // fs::rename()
        }
        // split images

    }

    // for dir in dir_contents.subdirs {
    //     if dir.
    // }
    println!("{:?}", args.subdir_prefix);
    println!("{:?}", args.images_per_dir);
}
