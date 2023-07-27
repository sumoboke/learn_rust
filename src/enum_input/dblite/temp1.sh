
use std::io;

use super::Truck;

pub fn write_data(len: usize) -> Truck {
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

    let hasil = Truck {
        id: None,
        time_submit: None,
        vehicle_num,
        driver_name,
        from_location,
        to_location: todo!(),
        time_drive: todo!(),
        is_done: todo!(),
    };

    hasil
}
