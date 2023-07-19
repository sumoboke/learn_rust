use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    println!("My path is {}.", args[0]);

    for (i, arg) in args.iter().skip(1).enumerate() {
        println!("Argument of {}: {}", i + 1, arg);
    }
}
