use std::{fs, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

// struct Contents{
//     dirs: Vec<path::PathBuf>,
//     jpgs: Vec<path::PathBuf>,
// }

fn read_dirs(path: path::PathBuf) -> Vec<path::PathBuf> {
    let mut subdirs = Vec::new();
    subdirs.push(path);
    subdirs
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args.path);
    println!("{:?}", read_dirs(args.path));
    // fs::create_dir(args.path);
    // for entry in fs::read_dir(args.path) {
    //     // let path = entry.path();
    //     println!("{:?}", entry);
    // }
    println!("Hello, world!");
}
