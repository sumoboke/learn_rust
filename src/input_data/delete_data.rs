use std::io;

use crate::input_data::update_data::{print_num, switch_next_prev};

use super::Manusia;

pub fn delete_chunk(data: &mut Vec<Manusia>) {
    println!("Choose data to delete");

    let data_len = { data.len() };
    let chunk_size = 3;
    let divided_len = (data_len + chunk_size - 1) / chunk_size;
    let mut divide_index = 0;

    loop {
        println!("\n");
        let start = divide_index * chunk_size;
        let end = (divide_index + 1) * chunk_size;
        print_num(&data[start..end.min(data_len)]);
        switch_next_prev(divide_index, divided_len, data_len);
        println!("\n");
        println!("======= Your choice : =======");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read CLI");

        match input.trim().parse::<usize>() {
            Ok(98) => {
                if divide_index == 0 {
                    println!("Please enter the right number..");
                } else {
                    divide_index -= 1;
                }
            }
            Ok(99) => {
                if divide_index >= divided_len - 1 {
                    println!("Please enter the right number..");
                } else {
                    divide_index += 1;
                }
            }
            Ok(100) => break,
            Ok(num) => {
                delete_child(num, data);
                break;
            }
            _ => println!("Please enter the right number.."),
        }
    }
}

fn delete_child(num: usize, data: &mut Vec<Manusia>) {
    if let Some(get_index) = data.iter().position(|x| x.id == num) {
        let removed_data = data.remove(get_index);

        for (i, elem) in data.into_iter().enumerate() {
            if i + 1 != elem.id {
                elem.id = i + 1;
            }
        }

        println!(
            "Success Removed data {}. {}",
            removed_data.id, removed_data.name
        )
    }
}
