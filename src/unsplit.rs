mod read_dir;

use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,

    #[structopt(default_value = "splitter")]
    subdir_prefix: String,
    // todo: add recursive mode
}

fn unsplit_dir(dir_contents: read_dir::DirContents, args: Cli) {
    let mut split_dir_index = 0;
    // I do not think initializing new_dir_path is necessary, but rust says it is.
    //  Perhaps I have a flaw in my logic.
    // Because it's in unsplit_dir, I need to look for it just in case...
    let mut split_dir_path = path::PathBuf::from(&dir_contents.path);
    split_dir_path.push("uninitialized");

    let max_missing = 50;
    let mut missing = 0;
    loop {
        if split_dir_path.exists() {
            println!("unsplitting dir");
            let split_contents = read_dir::read_dir(&split_dir_path);
            for image_path in split_contents.images {
                // generate new path
                let file_name = match image_path.file_name() {
                    Some(file_name) => file_name,
                    _ => continue,
                };
                let mut new_path = path::PathBuf::from(&dir_contents.path);
                new_path.push(file_name);

                // generate old path
                match fs::rename(&image_path, &new_path) {
                    Ok(_) => (),
                    _ => {
                        log::error!(
                            "unable to move image from {:?} to {:?}",
                            image_path,
                            new_path
                        );
                        continue;
                    }
                }
            }
            match fs::remove_dir(&split_dir_path) {
                Ok(_) => (),
                _ => log::error!("unable to remove directory {:?}", split_dir_path),
            }
        } else {
            missing += 1;
            if missing > max_missing {
                return;
            }
        }
        // generate the next directory name
        split_dir_index += 1;
        let dir_name = format!("{}-{}", args.subdir_prefix, split_dir_index);

        split_dir_path = path::PathBuf::from(&dir_contents.path);
        split_dir_path.push(dir_name);
    }
}

/// Read a directory
/// If there are < images_per_dir in that dir, leave it alone.
/// Otherwise, split the dir.
fn main() {
    let args = Cli::from_args();
    let dir_contents = read_dir::read_dir(&args.path);
    unsplit_dir(dir_contents, args);
}
