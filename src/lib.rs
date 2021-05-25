use std::{sync::{Mutex, mpsc, Arc}, thread, usize};
use std::fs;
use std::ffi::OsStr;
use std::path::{PathBuf, Path};

//TODO: function to verify directories
pub fn verify_directories(origin: &PathBuf, destination: &PathBuf) {
    if !destination.is_dir() {
        fs::create_dir(destination);
    }
}

//TODO: function to check format of extensions -> remove period from beginning
pub fn format_extensions(exts: &mut Vec<String>) {
    for i in 0..exts.len() {
        if exts[i].chars().nth(0).unwrap() == '.' {
            println!("{}", exts[i]);
            let mut chars = exts[i].chars();
            chars.next();
            exts[i] = chars.as_str().to_owned();
        }
    }
}

//DONE: function to get file paths
pub fn get_file_names<'a>(origin: &'a std::path::PathBuf, extensions: &'a mut Vec<String>, paths: &'a mut Vec<PathBuf>) -> &'a mut Vec<std::path::PathBuf>{
    assert!(origin.is_dir()); //TODO: clean up error handling
    format_extensions(extensions);
    for item in fs::read_dir(origin).unwrap() {
        let file = item.unwrap();
        let path = file.path();
        if path.is_dir() {
            get_file_names(&path, extensions, paths);

        } else {
            let ext = path.extension();
            match ext {
                Some(ext) => {
                    if extensions.contains(&ext.to_str().unwrap().to_string()) {
                        paths.push(path);
                    }
                }
                None => {}
            }
        }
    }
    paths
}

fn get_file_extension(path: &std::path::PathBuf) -> &OsStr {
    path.extension().unwrap() //TODO: error handling
}

pub fn organize_file(path: &std::path::PathBuf, destination: &std::path::PathBuf, move_file: bool) -> std::io::Result<()> {
    if move_file {
        println!("Move: {}", path.display());
    } else {
        let file_name = path.file_name();

        match file_name {
            Some(file_name) => {
                //TODO: add handling if file by name already exists
                let destination = format!("{}/{}",  destination.display(), file_name.to_str().unwrap());
                fs::copy(path, &destination)?;
                println!("Copy: {:?} to {:?}", file_name, destination.to_string());
            }
            None => {}
        }
    }
    Ok(())
}