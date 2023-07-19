use std::{
    // env,
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use super::Manusia;

fn check_file_exists(_path: &str) -> bool {
    Path::new(_path).exists()
}

fn create_new_file(_path: &str) {
    if !check_file_exists(_path) {
        let arr: &[Manusia] = &[];
        write_json(arr, _path)
    }
}

pub fn read_json(path: &str) -> Result<Vec<Manusia>, Box<dyn Error>> {
    create_new_file(path);
    let mut content_file = File::open(path).expect("Failed to open file");
    let mut content = String::new();

    content_file
        .read_to_string(&mut content)
        .expect("Failed to generate content");

    let json_data: Vec<Manusia> = serde_json::from_str(&content).expect("failed to write json");

    // let cwd = env::current_dir().expect("Failed to get current working directory");
    Ok(json_data)
}

pub fn write_json(_data: &[Manusia], _path: &str) {
    let len = { _data.len() };

    let data_json = serde_json::to_string(&_data).expect("Failed to serialize JSON");

    let mut file_open = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(_path)
        .expect("Open file are failed");
    file_open
        .write_all(data_json.as_bytes())
        .expect("Failed to write the file");

    if len == 0 {
        println!("Success create new file.. ")
    } else {
        println!("Success saved the data.. ")
    }
}
