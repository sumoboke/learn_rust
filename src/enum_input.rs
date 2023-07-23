#![allow(dead_code)]

mod create_enum;
mod dblite;
mod state_enum;

use create_enum::create_data;
use state_enum::Nav::*;
use std::io;

pub fn run() {
    


                let mut nav = Home;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match (input.trim(), &mut nav) {
            ("1", Home) => create_data(),
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
