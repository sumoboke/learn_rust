use std::io;

use super::Manusia;

fn print_num(_data: &[Manusia]) {
    for elm in _data {
        println!("{}. See {} data", elm.id, elm.name);
    }
}

fn get_gender(_boolean: bool) -> String {
    if _boolean == true {
        "Laki - laki".to_string()
    } else {
        "Perempuan".to_string()
    }
}

pub fn querry_data(data: &Vec<Manusia>, len: usize) {
    println!("Pick One Choice");

    loop {
        print_num(&data);

        println!("{}. See all data ", len + 1);
        println!("{}. Close querry data ", len + 2);
        println!("======= Your choice : =======");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("No input");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num == len + 1 {
                    let iter = data.into_iter();

                    for elem in iter {
                        let gender = get_gender(elem.is_male);

                        println!("============================");
                        println!("========= Data {} ==========", elem.id);

                        println!("name : {}", elem.name);
                        println!("umur : {}", elem.age);
                        println!("gender : {}", gender);

                        println!("============================");
                        println!("============================");
                        println!("============================");
                    }
                } else if num == len + 2 {
                    break;
                } else {
                    let slice_data = &data[num - 1];

                    let gender = get_gender(slice_data.is_male);

                    println!("============================");
                    println!("=========== DATA ===========");

                    println!("name : {}", slice_data.name);
                    println!("umur : {}", slice_data.age);
                    println!("gender : {}", gender);
                    println!("============================");
                    println!("============================");
                }
            }

            _ => {
                println!("please makesure it is number");
            }
        }
    }
}
