use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn read(file_path: PathBuf, callback: fn(String)) {
    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                callback(line.unwrap());
            }
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}