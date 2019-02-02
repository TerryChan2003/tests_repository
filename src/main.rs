#[warn(dead_code)]
extern crate reqwest;

use std::io;
use std::string::String;
use std::string::ToString;
fn http_get<T>(url: T) -> String
    where T: reqwest::IntoUrl
{
    let mut req = reqwest::get(url).unwrap();
    req.text().unwrap()
}

fn input<T>(text: T) -> String 
    where T: ToString
{
    println!("{}", text.to_string());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let t = http_get("http://httpbin.org/get");
    println!("Result: {}", t);
    let _t = input("Click enter for close...");
}
