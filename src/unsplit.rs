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
    for subdir in dir_contents.subdirs {
        let dir_name = match subdir.file_name() {
            Some(dir_name) => dir_name,
            _ => continue,
        };
        if !dir_name.to_string_lossy().starts_with(&args.subdir_prefix) {
            continue;
        }
        let split_contents = read_dir::read_dir(&subdir);
        for image_path in split_contents.images {
            // generate new path
            let file_name = match image_path.file_name() {
                Some(file_name) => file_name,
                _ => continue,
            };
            let mut new_path = path::PathBuf::from(&dir_contents.path);
            new_path.push(file_name);

            // move
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
        match fs::remove_dir(&subdir) {
            Ok(_) => (),
            _ => log::error!("unable to remove directory {:?}", &subdir),
        }
    }
}

/// Read a directory
/// unsplit it based on directory names.
fn main() {
    let args = Cli::from_args();
    let dir_contents = read_dir::read_dir(&args.path);
    unsplit_dir(dir_contents, args);
}
