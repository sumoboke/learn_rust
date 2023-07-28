use super::dblite::Truck;

pub enum Nav<'a> {
    Home(usize),
    Read { data: &'a [Truck] },
    Update,
    ReadOne { data: &'a [Truck] },
}

use Nav::*;

impl Nav<'_> {
    pub fn render_layout(&self) {
        match self {
            Home(len) => self.render_home(*len),
            Read { data: _ } => self.render_read(),
            Update => println!("AT UPDATE JENK"),
            ReadOne { data: _ } => {
                self.list_names();
                println!("Press 99 to go back");
            }
        }
    }

    fn render_home(&self, len: usize) {
        println!("\n");
        println!("=========== Total order submitted : {} ===========", len);
        println!("Please choose an option : ");
        println!("1. Create Order");
        println!("2. Read data Order");
        println!("3. Update Order");
        println!("4. Delete Order");
        println!("99. Close Program");
        println!("\n");
    }

    fn render_read(&self) {
        println!("\n");
        println!("=========== Read order ===========");
        println!("Please choose an option : ");
        println!("1. See one order");
        println!("2. See all order");
        println!("99. Close Program");
        println!("\n");
    }

    fn list_names(&self) {
        if let ReadOne { data } = self {
            for (j, name) in data.iter().enumerate() {
                println!("{}. {}", j + 1, name.driver_name)
            }
        }
    }

    pub fn list_all(&self) {
        if let Read { data } = self {
            let json_obj = serde_json::to_string_pretty(&data).unwrap();
            println!("{}", json_obj)
        }
    }

    pub fn find_one(&self, name: &str) {
        if let ReadOne { data } = self {
            match data.iter().find(|o| o.driver_name == name) {
                Some(one_data) => println!("{:?}", one_data),
                _ => println!("can't find driver"),
            }
        }
    }
}
