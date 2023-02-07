mod args;
mod process_file;
mod markdown_generator;

use args::{get_args, Args};
use std::env;
use markdown_generator::MarkdownGenerator;

/**
* @function main
* Entry point of the application
* @example <caption>Running the application to process a file</caption>
* cargo run file src/main.rs
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

fn save_file(args_def: &Args, content: String, file: String) {
    let destination = args_def.destination.as_ref().unwrap().clone();
    let current_dir = env::current_dir().expect("could not get current directory");
    let file_path = current_dir.join(destination).join(file.replace(".js", ".md").replace(".rs", ".md"));

    println!("Saving file: {}", file_path.to_str().unwrap());

    process_file::write(file_path, content);
}
/**
* @function process_file
* Process a file's content line by line
* Generate a output file based on the jsdoc comments
* @param args_def The arguments passed to the application from the commandline
*/
fn process_file(args_def: args::Args) {
    let file = args_def.file.as_ref().unwrap().clone();

    let current_dir = env::current_dir().expect("could not get current directory");
    let file_path = current_dir.join(file.clone());
    let mut markdown_generator = MarkdownGenerator::new();

    let mut process_line = |line: String| {
        markdown_generator.process_line(line);
    };

    process_file::read(file_path, &mut process_line);

    let result = markdown_generator.generate();
    save_file(&args_def, result, file);
}

/**
* @function process_folder
* Process the files in the folder recursively.
* For each file in the folder call process_file.
* Call this function recursively for each subfolder.
* @param args_def The arguments passed to the application from the commandline
* @param path The path to the folder that will determine the output folder
*/
fn process_folder(args_def: args::Args, path: &str) {
    todo!()
}
