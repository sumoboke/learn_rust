use std::{
    // env,
    error::Error,
    fs::File,
    io::{Read, Write},
};

use super::Manusia;

pub fn read_json(path: &str) -> Result<Vec<Manusia>, Box<dyn Error>> {
    let mut content_file = File::open(path).expect("Failed to open file");
    let mut content = String::new();

    content_file
        .read_to_string(&mut content)
        .expect("Failed to generate content");

    let json_data: Vec<Manusia> = serde_json::from_str(&content).expect("failed to write json");

    // let cwd = env::current_dir().expect("Failed to get current working directory");
    Ok(json_data)
}

pub fn write_json(arr: &[Manusia], path: &str) -> Result<(), Box<dyn Error>> {
    let json_file = serde_json::to_string(arr).expect("failed to write to Buffer");

    let mut file_write = File::create(path).expect("create file failed");
    file_write
        .write_all(json_file.as_bytes())
        .expect("overwrite file failed");

    Ok(())
}
