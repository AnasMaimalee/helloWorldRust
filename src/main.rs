use ferris_says::say; //
use std::io::{stdout, BufWriter};

fn main() {

    println!("Hello World From Maimalee");
    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}