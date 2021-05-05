use std::ffi::OsStr;
use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,

    #[structopt(default_value = "splitter")]
    subdir_prefix: String,

    #[structopt(default_value = "32", long)]
    images_per_dir: usize,
    // todo: add recursive mode
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

fn read_dir(path: &path::PathBuf) -> DirContents {
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

fn split_dir(dir_contents: DirContents, args: Cli) {
    if dir_contents.images.len() > args.images_per_dir {
        let mut new_dir_index = 0;
        // I do not think initializing new_dir_path is necessary, but rust says it is.
        //  Perhaps I have a flaw in my logic.
        let mut new_dir_path = path::PathBuf::from(&dir_contents.path);
        new_dir_path.push("uninitialized");
        for i in 0..dir_contents.images.len() {
            // create subdirs
            if i % args.images_per_dir == 0 {
                loop {
                    new_dir_index += 1;
                    let dir_name = format!("{}-{}", args.subdir_prefix, new_dir_index);

                    new_dir_path = path::PathBuf::from(&dir_contents.path);
                    new_dir_path.push(dir_name);
                    if new_dir_path.exists() {
                        continue;
                    }
                    match fs::create_dir(&new_dir_path) {
                        Ok(_) => break,
                        _ => panic!("Unable to create dir {:?}", new_dir_path),
                    }
                }
            }
            // move files to the new dir
            let current_path = &dir_contents.images[i];
            let file_name = match current_path.file_name() {
                Some(file_name) => file_name,
                _ => continue,
            };
            let mut new_path = path::PathBuf::from(&new_dir_path);
            new_path.push(file_name);

            match fs::rename(&current_path, &new_path) {
                Ok(_) => (),
                _ => panic!(
                    "unable to move image from {:?} to {:?}",
                    current_path, new_path
                ),
            }
        }
    }
}

/// Read a directory
/// If there are < images_per_dir in that dir, leave it alone.
/// Otherwise, split the dir.
fn main() {
    let args = Cli::from_args();
    let dir_contents = read_dir(&args.path);
    split_dir(dir_contents, args);
}
