mod args;
mod process_file;

use args::get_args;
use std::env;

fn main() {
    let def = get_args();

    if def.file.is_some() {
        process_file(def);
    }
}

fn process_file(args_def: args::Args) {
    let file = args_def.file;

    let current_dir = env::current_dir().expect("could not get current directory");
    let file_path = current_dir.join(file.unwrap());

    process_file::read(file_path, |line| {
        println!("{}", line);
    });
}

