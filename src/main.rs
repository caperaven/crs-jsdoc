mod args;
mod process_file;

use args::get_args;
use std::env;
use crate::args::Args;

fn main() {
    let def = get_args();

    if def.file.is_some() {
        process_file(def);
        return;
    }

    if def.folder.is_some() {
        process_folder(def, "./");
        return;
    }
}

/**
* Process a file's content line by line
* Generate a output file based on the jsdoc comments
*/
fn process_file(args_def: args::Args) {
    let file = args_def.file;

    let current_dir = env::current_dir().expect("could not get current directory");
    let file_path = current_dir.join(file.unwrap());

    process_file::read(file_path, |line| {
        println!("{}", line);
    });
}

/**
* Process the files in the folder recursively.
* For each file in the folder call process_file.
* Call this function recursively for each subfolder.
*/
fn process_folder(args_def: args::Args, path: &str) {
    todo!()
}
