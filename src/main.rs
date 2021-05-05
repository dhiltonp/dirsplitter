use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
    // add prefix
    // add number of images per dir
}

// struct Contents{
//     dirs: Vec<path::PathBuf>,
//     jpgs: Vec<path::PathBuf>,
// }

fn read_dirs(path: path::PathBuf) -> Vec<path::PathBuf> {
    let mut subdirs = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{:?}", entry.path());
                if entry.file_type().unwrap().is_dir() {
                    subdirs.push(entry.path());
                } else if entry.file_type().unwrap().is_file() {
                    if entry.path().ends_with("jpg") {
                        println!("image found");
                        subdirs.push(entry.path());
                    }
                } else {
                    println!("invalid data");
                }
            }
        }
    }

    // for entry in fs::read_dir(args.path)
    subdirs
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args.path);
    println!("{:?}", read_dirs(args.path));
    // fs::create_dir(args.path);
    println!("Hello, world!");
}
