#[warn(dead_code)]
use std::io;

fn input<T>(text: T) -> String 
    where T: std::string::ToString
{
    println!("{}", text.to_string());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn main() {
    println!("Hello, world!");
    let s = input("Your word: ");
    println!("{}", s)
}
