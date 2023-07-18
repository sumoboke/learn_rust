use std::io;

use super::Manusia;

pub fn write_data(len: usize) -> Manusia {
    let mut input = String::new();

    println!("Please enter your name");
    io::stdin().read_line(&mut input).unwrap();

    let input_name = input.trim().to_string();
    input.clear();

    println!("Please enter your age");
    io::stdin().read_line(&mut input).unwrap();

    let input_age = input.trim().parse::<u8>().unwrap();
    input.clear();

    let mut is_male_input = false;

    loop {
        println!("Enter your gender (Pick one)");
        println!("1. Male");
        println!("2. Female");
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(1) => {
                is_male_input = true;
                break;
            }
            Ok(2) => {
                break;
            }
            _ => {
                println!("Please pick one either 1 or 2")
            }
        }
    }

    let hasil = Manusia {
        id: len + 1,
        name: input_name,
        age: input_age,
        is_male: is_male_input,
    };

    hasil
}
