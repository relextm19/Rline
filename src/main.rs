use file_data::Data;
use std::path::PathBuf;
use std::env;

mod file_data;
mod args;
mod file_actions;
mod printing;

fn main(){
    let mut datas: Vec<Data> = Vec::new();
    let mut path: PathBuf = env::current_dir().unwrap();
    let mut banned_extensions: Vec<String> = Vec::new();
    let mut banned_directories: Vec<String> = vec![String::from(".git")]; // default banned directories
    let mut recursive = false; 

    args::parse_args(&mut path, &mut banned_extensions, &mut banned_directories, &mut recursive);
    file_actions::itterate_files(&path, &mut datas, &banned_extensions, &banned_directories, recursive);
    printing::print_data(&datas);
}