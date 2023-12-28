use ferris_says::say; //
use std::io::{stdout, BufWriter};
use regex::Regex;
fn main() {

    println!("Hello World From Maimalee");
    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2023-28-12"))
}
