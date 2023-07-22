#![allow(dead_code)]

use std::io;

enum Nav {
    Home,
    Create,
    Read(SecondNav),
    Update(SecondNav),
}

enum SecondNav {
    Index { i: u32 },
    Pick(u32),
    Prev,
    Next,
}

use Nav::*;
use SecondNav::*;

pub fn run() {
    let mut nav = Home;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match (input.trim().parse::<u32>(), &mut nav) {
            (Ok(1), Home) => nav = Create,
            (Ok(2), Home) => nav = Read(Index { i: 0 }),
            (Ok(3), Home) => nav = Update(Index { i: 0 }),
            (Ok(4), Home) => {
                println!("Program closed, goodbye");
                break;
            }
            (Ok(98), Read(Index { i: 0 })) => nav = Read(Index { i: 0 }),
            (Ok(98), Update(Index { i: 0 })) => nav = Update(Index { i: 0 }),
            (Ok(99), Update(Index { i: 0 })) => nav = Update(Index { i: 0 }),
            (Ok(num), Read(_index)) => nav = Read(Pick(num)),
            (Ok(num), Update(_index)) => nav = Update(Pick(num)),
            (_, _) => {
                println!("Please enter a correct number")
            }
        }
    }
}
