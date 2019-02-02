#[warn(dead_code)]
extern crate reqwest;

use std::io;
use std::collections::HashMap;
fn http_get(url: &str, params: HashMap<&str, &str>) -> String
{
    let mut param = String::new();
    param.push_str(url);
    param.push_str("?");
    for (k, v) in params.iter() {
        param.push_str(k);
        param.push_str("=");
        param.push_str(v);
        param.push_str("&")
    }
    param.remove(param.len()-1);
    let mut req = reqwest::get(&param).unwrap();
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
    let mut map = HashMap::new();
    map.insert("login", "testing");
    map.insert("password", "12345");
    let t = http_get("http://httpbin.org/get", map);
    println!("Result: {}", t);
    let _t = input("Click enter for close...");
}
