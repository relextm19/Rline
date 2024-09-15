use std::path::PathBuf;

pub fn parse_args(path: &mut PathBuf, banned_extensions: &mut Vec<String>, banned_directories: &mut Vec<String>, recursive: &mut bool){
    let mut args = std::env::args();
    while let Some(arg) = args.next(){
        match arg.as_str(){
            "-p" => {
                if let Some(new_path) = args.next(){
                    *path = PathBuf::from(new_path);
                }
            },
            "-e" => {
                if let Some(extension) = args.next(){
                    banned_extensions.push(extension);
                }
            },
            "-d" => {
                if let Some(directory) = args.next(){
                    banned_directories.push(directory);
                    println!("{:?}", banned_directories);
                }
            },
            "-r" => {
                *recursive = true;
            },
            _ => (),
        }
    }
}