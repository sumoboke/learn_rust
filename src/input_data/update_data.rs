use std::io;

use super::Manusia;

pub fn print_num(data: &[Manusia]) {
    for elm in data {
        println!("{}. See {} data", elm.id, elm.name);
    }
}

pub fn switch_next_prev(num: usize, max: usize, len: usize) {
    if len != max {
        match num {
            0 => {
                if len >= 3 {
                    println!("99. Go to next data")
                }
            }
            _ => {
                if num + 1 < max {
                    println!("98. Go to previous data");
                    println!("99. Go to next data");
                } else {
                    println!("98. Go to previous data");
                }
            }
        }
    }
    println!("100. Go Back");
}

pub fn updating_data(data: &mut Vec<Manusia>) {
    println!("Choose data to update");

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
            Ok(num) => updating_child(num, data),
            _ => println!("Please enter the right number.."),
        }
    }
}

fn updating_child(num: usize, data: &mut Vec<Manusia>) {
    if let Some(chunk_child) = data.iter_mut().find(|x| x.id == num) {
        change_input(chunk_child);
    }
}

fn change_input(man: &mut Manusia) {
    let mut input = String::new();

    println!("Please enter your name");
    io::stdin().read_line(&mut input).unwrap();
    let input_name = input.trim().to_string();
    input.clear();

    println!("Please enter your age");
    io::stdin().read_line(&mut input).unwrap();
    let input_age = input.trim().parse::<u8>().unwrap();

    man.name = input_name;
    man.age = input_age;
}
