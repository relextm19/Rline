use crate::file_data::Data;

pub fn print_data(file_datas:&Vec<Data>){
    let mut data_sum = Data{
        display_last: true,
        name: String::from("Total"),
        line_count: 0,
        character_count: 0,
        whitespace_count: 0,
    };
    let mut delayed_display: Vec<&Data> = Vec::new(); // for aesthetic reasons we display the data with display_last set to true such as directories last
    
    for data in file_datas{
        if data.display_last{
            delayed_display.push(data);
            continue;
        }    
        println!("{}", data);
        data_sum += data.clone();
    }

    for data in delayed_display{
        println!("{}", data);
        data_sum += data.clone();
    }

    println!("{}", data_sum);
}