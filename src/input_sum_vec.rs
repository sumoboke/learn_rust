use std::io;

fn parsed_words(_str: String) -> Option<i32> {
    match _str.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("please enter only number");
            None
        }
    }
}

pub fn run() {
    let mut arr = vec![];

    println!("===== Please enter 5 number =====");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match parsed_words(input) {
            Some(num) => arr.push(num),
            None => continue,
        };

        let length = arr.len();

        if length < 5 {
            println!("enter the number again");
            continue;
        } else {
            let even_arr = arr
                .iter()
                .filter(|&x| x % 2 == 0)
                .cloned()
                .collect::<Vec<i32>>();
            println!("even number you submited are \n");

            for num in &even_arr {
                print!("{} ,", num)
            }

            let sum_even: i32 = even_arr.into_iter().sum();
            println!("the sum are {}", sum_even);
            break;
        }
    }
}
