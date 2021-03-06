mod read_dir;

use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    dir: path::PathBuf,

    #[structopt(default_value = "splitter")]
    subdir_prefix: String,
    // todo: add recursive mode
}

fn unsplit_dir(args: Cli) {
    for subdir in read_dir::subdirs(&args.dir) {
        if let Some(dir_name) = subdir.file_name() {
            if dir_name.to_string_lossy().starts_with(&args.subdir_prefix) {
                for image in read_dir::images(&subdir) {
                    if let Some(file_name) = image.file_name() {
                        let mut new_path = path::PathBuf::from(&args.dir);
                        new_path.push(file_name);

                        // move
                        match fs::rename(&image, &new_path) {
                            Ok(_) => (),
                            _ => {
                                log::error!(
                                    "unable to move image from {:?} to {:?}",
                                    image,
                                    new_path
                                );
                                continue;
                            }
                        }
                    }
                }
                match fs::remove_dir(&subdir) {
                    Ok(_) => (),
                    _ => log::error!("unable to remove directory {:?}", &subdir),
                }
            }
        }
    }
}

/// Read a directory
/// unsplit it based on directory names.
fn main() {
    let args = Cli::from_args();
    unsplit_dir(args);
}
