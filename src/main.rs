mod args;
mod process_file;

use args::{get_args, Args};
use std::env;

/**
* Entry point of the application
*/
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
* @param args_def The arguments passed to the application from the commandline
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
* @param args_def The arguments passed to the application from the commandline
* @param path The path to the folder that will determine the output folder
*/
fn process_folder(args_def: args::Args, path: &str) {
    todo!()
}
