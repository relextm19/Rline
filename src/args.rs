use std::path::PathBuf;

pub fn parse_args(path: &mut PathBuf, banned_extensions: &mut Vec<String>, banned_directories: &mut Vec<String>, recursive: &mut bool){
    let mut args = std::env::args();
    while let Some(arg) = args.next(){
        match arg.as_str(){
            "-p" => {
                if let Some(new_path) = args.next(){
                    *path = PathBuf::from(new_path);
                    println!("p");
                }
            },
            "-e" => {
                if let Some(extension) = args.next(){
                    for extension in extension.split(","){
                        banned_extensions.push(extension.trim().to_string());
                    }
                    println!("e");
                }
            },
            "-d" => {
                if let Some(directory) = args.next(){
                    for directory in directory.split(","){
                        banned_directories.push(directory.trim().to_string());
                    }
                    println!("d");
                }
            },
            "-r" => {
                *recursive = true;
                println!("r");
            },
            _ => (),
        }
    }
    println!("{:?}, {:?}, {:?}, {:?}", path, banned_extensions, banned_directories, recursive);
}