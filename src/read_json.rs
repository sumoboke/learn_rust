use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Anak {
    name: String,
    age: u8,
    is_male: bool,
}

pub fn run() {
    let file_name = "data.json";

    let mut the_file = File::open(file_name).expect("Failed to open file");
    let mut content = String::new();
    the_file
        .read_to_string(&mut content)
        .expect("Failed to read file json");

    let anak: Anak = serde_json::from_str(&content).unwrap();

    let gender = {
        if anak.is_male {
            "Laki - laki".to_string()
        } else {
            "Perempuan".to_string()
        }
    };

    println!("name : {}", anak.name);
    println!("name : {}", anak.age);
    println!("name : {}", gender);
}
