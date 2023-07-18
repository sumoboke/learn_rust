mod manipulation_json_file;
mod querry_data;
mod submit_data;

use manipulation_json_file::read_json;
use querry_data::querry_data;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{self, Write},
};
use submit_data::write_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manusia {
    id: usize,
    name: String,
    age: u8,
    is_male: bool,
}

pub fn write_json(_data: &[Manusia], _path: &str) {
    let data_json = serde_json::to_string(&_data).expect("Failed to serialize JSON");

    let mut file_open = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(_path)
        .expect("Open file are failed");
    file_open
        .write_all(data_json.as_bytes())
        .expect("Failed to write the file");

    println!("Success saved the data.. ")
}

pub fn run() {
    let path = "data.json";

    let data = read_json(path);

    match data {
        Ok(mut _data) => {
            let mut data_len = { _data.len() };

            loop {
                println!("===== Total data submited : {} ====", data_len);
                println!("Choose an Option : ");
                println!("1. Read Data");
                println!("2. Insert Data");
                println!("3. Close Program");
                println!("======= Your choice : =======");

                let mut input_option = String::new();
                io::stdin().read_line(&mut input_option).unwrap();

                match input_option.trim().parse::<u32>() {
                    Ok(1) => querry_data(&_data, data_len),
                    Ok(2) => {
                        let hasil_input = write_data(data_len);
                        println!("======= Your Input : =======");
                        println!("{:?}", hasil_input);
                        _data.push(hasil_input);
                        data_len = data_len + 1;
                        write_json(&_data, path);
                    }
                    //ok3
                    Ok(3) => {
                        println!("Close the program now ...");
                        break;
                    }
                    _ => {
                        println!("Please enter correct number ....")
                    } //error
                }
            }
            //end OK
        }
        Err(err) => {
            println!("Shit happen : {}", err);
        }
    };
}
