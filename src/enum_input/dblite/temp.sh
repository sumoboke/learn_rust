
use std::io;

use super::Manusia;

pub fn write_data(len: usize) -> Manusia {
    let mut input = String::new();

    println!("Please enter vehicle num");
    io::stdin().read_line(&mut input).unwrap();

    let vehicle_num = input.trim().to_string();
    input.clear();

    println!("Enter driver name :");
    io::stdin().read_line(&mut input).unwrap();

    let driver_name = input.trim().parse::<u8>().unwrap();
    input.clear();

    println!("Enter from location :");
    io::stdin().read_line(&mut input).unwrap();

    let from_location = input.trim().parse::<u8>().unwrap();
    input.clear();

    println!("Where the truck go :");
    io::stdin().read_line(&mut input).unwrap();

    let to_location = input.trim().parse::<u8>().unwrap();
    input.clear();

    println!("What time the truck will dispatch : ");
    io::stdin().read_line(&mut input).unwrap();

    let time_dispatch = input.trim().parse::<u8>().unwrap();
    input.clear();
    }

    let hasil = Tru {
    
    };

    hasil
}
