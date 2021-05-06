mod read_dir;

use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    dir: path::PathBuf,

    #[structopt(default_value = "splitter")]
    subdir_prefix: String,

    #[structopt(default_value = "32", long)]
    images_per_dir: usize,
    // todo: add recursive mode
}

fn split_dir(args: Cli) {
    let images = read_dir::images(&args.dir);
    if images.len() > args.images_per_dir {
        let mut split_dir_index = 0;
        // I do not think initializing new_dir_path is necessary, but rust says it is.
        //  Perhaps I have a flaw in my logic.
        let mut split_dir_path = path::PathBuf::from(&args.dir);
        split_dir_path.push("uninitialized");
        for i in 0..images.len() {
            // create subdirs
            if i % args.images_per_dir == 0 {
                loop {
                    split_dir_index += 1;
                    let dir_name = format!("{}{}", args.subdir_prefix, split_dir_index);

                    split_dir_path = path::PathBuf::from(&args.dir);
                    split_dir_path.push(dir_name);
                    if split_dir_path.exists() {
                        continue;
                    }
                    match fs::create_dir(&split_dir_path) {
                        Ok(_) => break,
                        _ => panic!("Unable to create dir {:?}", split_dir_path),
                    }
                }
            }
            // move files to the new dir
            let current_path = &images[i];
            let file_name = match current_path.file_name() {
                Some(file_name) => file_name,
                _ => continue,
            };
            let mut new_path = path::PathBuf::from(&split_dir_path);
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
    split_dir(args);
}
