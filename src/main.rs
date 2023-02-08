#![feature(let_chains)]

mod args;
mod process_file;
mod markdown_generator;
mod parameters;

use args::{get_args, Args};
use std::{env, fs};
use std::fs::DirEntry;
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
        process_file(&def);
        return;
    }

    if def.folder.is_some() {
        process_folder(&def, "");
        return;
    }
}

fn save_file(args_def: &Args, content: String, file: String) {
    let destination = args_def.destination.as_ref().unwrap().clone();
    let current_dir = env::current_dir().expect("could not get current directory");

    let file_name = file.replace(".js", ".md");

    let file_path = current_dir.join(destination).join(file_name.clone());

    println!("Saving file: {}", file_name);

    process_file::write(file_path, content);
}

/**
* @function process_file
* Process a file's content line by line
* Generate a output file based on the jsdoc comments
* @param args_def {args::Args} - The arguments passed to the application from the commandline
*/
fn process_file(args_def: &args::Args) {
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
* @param args_def {Args} - The arguments passed to the application from the commandline
* @param path {String} - The path to the folder that will determine the output folder
*/
fn process_folder(args_def: &args::Args, path: &str) {
    let current_dir = env::current_dir().expect("could not get current directory");
    let folder = args_def.folder.as_ref().unwrap().clone();
    let folder_path = current_dir
        .join(folder.clone())
        .join(path);

    for entry in fs::read_dir(folder_path).expect("could not read directory") {
        let entry: DirEntry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.is_dir() {
            process_folder(args_def, entry.file_name().to_str().unwrap());
        }
        else {
            let file: String;

            if path.is_empty() {
                file = format!("{}\\{}", folder.clone(), entry.file_name().to_str().unwrap());
            }
            else {
                file = format!("{}\\{}\\{}", folder.clone(), path, entry.file_name().to_str().unwrap());
            }

            let mut args_def_clone = args_def.clone();
            args_def_clone.file = Some(file);
            process_file(&args_def_clone);
        }
    }
}
