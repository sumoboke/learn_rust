#![allow(dead_code)]

mod create_enum;
mod dblite;
mod state_enum;

// use create_enum::create_data;
use dblite::Database;
use state_enum::Nav::*;
use std::io;

pub async fn run() {
    let data_path = "data.db";
    let data_record = Database::read_all(data_path).await.unwrap();

    let len = { data_record.len() };

    let mut nav = Home(len);
    loop {
        nav.render_layout();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("\n");
        println!("Yout type {}", &input);
        println!("\n");

        match (input.trim(), &mut nav) {
            ("1", Home(len)) => {
                Database::submit_data(&data_path).await.unwrap();
                *len += 1;
            }

            ("2", Home(_len)) => nav = Read { data: &data_record },
            ("3", Home(_len)) => nav = Update,
            ("1", Read { data: data_read }) => nav = ReadOne { data: data_read },
            ("99", ReadOne { data: data_read }) => nav = Read { data: data_read },
            (word, ReadOne { data: _ }) => nav.find_one(word),
            ("2", Read { data: _ }) => nav.list_all(),
            ("99", Read { data: _ }) => nav = Home(len),
            ("99", Update) => nav = Home(len),
            ("99", Home(_len)) => {
                println!("Program closed, goodbye");
                break;
            }
            (_, _) => {
                println!("Please enter a correct number")
            }
        }
    }
}

// pub async fn run() {
//     let data_path = "data.db";
//     let mut data_truck = Database::new("data.db").await.unwrap();
//     loop {
//         println!("1. Create");
//         println!("2. Read");
//         println!("3. Quit");
//
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//
//         match input.trim() {
//             "1" => data_truck.submit_data().await.unwrap(),
//             "2" => Database::read_all(data_path).await.unwrap(),
//             "3" => break,
//             _ => println!("please enter correct number"),
//         }
//     }
// }
