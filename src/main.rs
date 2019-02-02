#![allow(dead_code, unused_imports)]
extern crate reqwest;
extern crate serde_json;

use serde_json::{Result, Value, from_reader, Deserializer};
use serde_json::from_str;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
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

fn get_json_data(filename: &str) -> Value
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
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
    println!("Result http: {}", t);
    let t = get_json_data("login.json");
    println!("Result reader: {}", t);
    let _t = input("Click enter for close...");
}
