use crate::file_data::{self, Data};
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::ffi::OsStr;

fn has_allowed_extension(path: &PathBuf, banned_extensions: &Vec<String>) -> bool{
    if let Some(path_extension) = path.extension(){
        for extension  in banned_extensions{
            if path_extension == OsStr::new(&extension) {
                return false;
            }
        }
    }
    
    true
}

fn is_allowed_directory(path: &PathBuf, banned_directories: &Vec<String>) -> bool{
    for directory in banned_directories{
        if path.file_name().unwrap_or_default().to_string_lossy() == *directory{
            return false;
        }
    }
    true
}

pub fn itterate_files(path: &PathBuf, datas: &mut Vec<Data>, banned_extensions: &Vec<String>, banned_directories: &Vec<String>, recursive:bool, data_sum: &mut Data){
    if path.is_dir() && is_allowed_directory(path, banned_directories){
        match fs::read_dir(path){
            Ok(entries) =>{
                let mut directory_data = Data {
                    name: path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    line_count: 0,
                    character_count: 0,
                    whitespace_count: 0,
                };
                for entry in entries{
                    match entry{
                        Ok(entry) =>{
                            if entry.path().is_file() && has_allowed_extension(&entry.path(), banned_extensions){
                                let file_data = get_file_data(&entry.path());
                                
                                //add to the directory sum
                                directory_data.character_count += file_data.character_count;
                                directory_data.line_count += file_data.line_count;
                                directory_data.whitespace_count += file_data.whitespace_count;
                                
                                datas.push(file_data);
                            } else if entry.path().is_dir() && is_allowed_directory(&entry.path(), banned_directories) && recursive{ //recursively check another directory
                                itterate_files(&entry.path(), datas, banned_extensions, banned_directories, recursive, data_sum);
                            }
                        }
                        Err(e) => {
                            println!("Error reading entry {:?}", e);
                        }
                    }
                }
                datas.push(directory_data.clone());
                *data_sum += directory_data;
            }
            Err(e) => {
                println!("Error reading directory {:?}", e)
            }
        }
    }
}

fn get_file_data(path: &PathBuf) -> Data {
    match fs::File::open(path) {
        Ok(file) => {

            let reader = io::BufReader::new(file);
            
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let mut file_data = Data {
                name,
                line_count: 0,
                character_count: 0,
                whitespace_count: 0,
            };

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        file_data.line_count += 1;
                        file_data.character_count += line.chars().count();            
                        if line.trim().is_empty() {
                            file_data.whitespace_count += 1;
                        }
                    }
                    Err(_e) => (),
                }
            }

            file_data
        }
        Err(e) => {
            println!("Error opening file: {:?}", e);
            let name = String::from("");
            // Return an empty Data in case of error
            Data {
                name,
                line_count: 0,
                character_count: 0,
                whitespace_count: 0,
            }
        }
    }
}