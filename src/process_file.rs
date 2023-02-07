use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/**
* Read a file line by line and call the callback function
* with the line as a parameter
* @param file_path The path to the file
* @param callback The callback function
*/
pub fn read<F>(file_path: PathBuf, callback: &mut F)
    where
        F: FnMut(String),
{
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

pub fn write(file_path: PathBuf, content: String) {
    let file_path_str = file_path.to_str().unwrap();
    let file_path_fixed = file_path_str.replace("/", "\\");

    ensure_directories_exist(PathBuf::from(file_path_fixed.clone())).unwrap();

    match File::create(PathBuf::from(file_path_fixed.clone())) {
        Ok(mut file) => {
            file.write_all(content.as_bytes()).unwrap();
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn ensure_directories_exist(path: PathBuf) -> Result<(), std::io::Error> {
    println!("Creating directory: {}", path.to_str().unwrap());

    let folderPath = path.parent().expect("could not get parent folder for path").to_path_buf();
    fs::create_dir_all(folderPath)?;
    Ok(())
}