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
    let mut banned_directories: Vec<String> = Vec::new();
    let mut recursive = false; 
    let mut data_sum = Data{
        name: "Total".to_string(),
        line_count: 0,
        character_count: 0,
        whitespace_count: 0,
    };

    args::parse_args(&mut path, &mut banned_extensions, &mut banned_directories, &mut recursive);
    file_actions::itterate_files(&path, &mut datas, &banned_extensions, &banned_directories, recursive,  &mut data_sum);
    printing::print_data(&datas, &data_sum);
}