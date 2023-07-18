use serde::Serialize;
use std::{fs::File, io::Write};

#[derive(Debug, Serialize)]
struct Anak {
    name: String,
    age: usize,
    is_male: bool,
}

pub fn run() {
    let file_name = "data.json";

    let anak = Anak {
        name: "Calista".to_string(),
        age: 5,
        is_male: false,
    };

    let json_obj = serde_json::to_string(&anak).unwrap();

    let mut new_file = File::create(file_name).unwrap();

    new_file.write_all(json_obj.as_bytes()).unwrap();

    println!("json file saved!")
}
