use std::io;

use super::Truck;

pub fn write_data() -> Truck {
    let mut input = String::new();

    println!("Please enter vehicle num");
    io::stdin().read_line(&mut input).unwrap();

    let vehicle_num = input.trim().to_string();
    input.clear();

    println!("Enter driver name :");
    io::stdin().read_line(&mut input).unwrap();

    let driver_name = input.trim().to_string();
    input.clear();

    println!("Enter from location :");
    io::stdin().read_line(&mut input).unwrap();

    let from_location = input.trim().to_string();
    input.clear();

    println!("Where the truck go :");
    io::stdin().read_line(&mut input).unwrap();

    let to_location = input.trim().to_string();
    input.clear();

    println!("What time the truck will dispatch : ");
    io::stdin().read_line(&mut input).unwrap();

    let time_dispatch = input.trim().parse::<i64>().unwrap();
    input.clear();

    let hasil = Truck {
        id: None,
        time_submit: None,
        vehicle_num,
        driver_name,
        from_location,
        to_location,
        time_drive: time_dispatch,
        is_done: None,
    };
    hasil
}
