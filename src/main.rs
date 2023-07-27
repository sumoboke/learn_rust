#![allow(dead_code)]
mod enum_input;

use enum_input::run;

#[tokio::main]
async fn main() {
    run().await;
}
