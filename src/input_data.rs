mod get_path;
mod manipulation_json_file;
mod querry_data;
mod submit_data;
mod update_data;

use get_path::get_path;
use manipulation_json_file::{read_json, write_json};
use querry_data::querry_data;
use serde::{Deserialize, Serialize};
use std::io::{self};
use submit_data::write_data;

use crate::input_data::update_data::updating_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manusia {
    id: usize,
    name: String,
    age: u8,
    is_male: bool,
}

pub fn run() {
    let path = get_path();

    let data = read_json(path.as_str());

    match data {
        Ok(mut _data) => {
            let mut data_len = { _data.len() };

            loop {
                println!("===== Total data submited : {} ====", data_len);
                println!("Choose an Option : ");
                println!("1. Read Data");
                println!("2. Insert Data");
                println!("3. Update Data");
                println!("4. Close Program");
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
                        write_json(&_data, path.as_str());
                    }
                    //ok3
                    //update
                    Ok(3) => updating_data(&mut _data),

                    //ok4
                    //delete
                    Ok(4) => {
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
