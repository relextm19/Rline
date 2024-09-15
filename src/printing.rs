use crate::file_data::Data;

pub fn print_data(file_datas:&Vec<Data>, total_data:&Data){
    for data in file_datas{    
        println!("{}", data);
    }
    println!("{}", total_data);
}