use std::{env, process};

fn check_len(num: usize) {
    if num != 2 {
        println!("Make sure to input 1 file name");
        println!("Program Closed..");
        process::exit(0);
    }
}

fn extract_arg() -> String {
    let args: Vec<String> = env::args().collect();

    let len = { args.len() };

    check_len(len);

    args[1].to_string()
}

pub fn get_path() -> String {
    let arg = extract_arg();

    if arg.ends_with(".json") {
        arg
    } else {
        println!("Make sure to enter JSON file");
        println!("Program Closed..");
        process::exit(0);
    }
}
