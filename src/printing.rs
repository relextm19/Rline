use crate::file_data::Data;

pub fn print_data(file_datas:&Vec<Data>){
    let mut data_sum = Data{
        name: String::from("Total"),
        line_count: 0,
        character_count: 0,
        whitespace_count: 0,
    };

    for data in file_datas{    
        println!("{}", data);
        data_sum += data.clone();
    }
    println!("{}", data_sum);
}