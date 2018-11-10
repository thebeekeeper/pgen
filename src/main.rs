#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate mustache;

use serde_json::Value;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;

fn main() {
    let mut f = File::open("test.json").expect("Failed to read test.json");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed to read file");
    let v: Value = serde_json::from_str(&contents).expect("Failed to parse json");
    println!("value: {}", v["first"]);
    let mut input = String::new();
    print!("Project name: ");
    io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("Creating {}", input);
}

/*
fn read_template(template_path: Path) -> Vec<String> {
}

fn collect_parameters(values: Value) {
    
}*/
