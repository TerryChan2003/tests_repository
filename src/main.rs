#[warn(dead_code)]
use std::io;
use std::string::String;
use std::string::ToString;

fn input<T>(text: T) -> String 
    where T: ToString
{
    println!("{}", text.to_string());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let s = input("Выпишите число: ");
    let s : u32 = s.parse().unwrap(); 
    println!("Ваше введенное число в квадрате {}", s*s);
}
