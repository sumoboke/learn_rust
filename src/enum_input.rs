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
    let mut nav = Home;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match (input.trim(), &mut nav) {
            ("1", Home) => Database::submit_data(&data_path).await.unwrap(),
            ("2", Home) => nav = Read,
            ("3", Home) => nav = Update,
            ("99", Read) => nav = Home,
            ("99", Update) => nav = Home,
            ("99", Home) => {
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
