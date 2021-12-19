use std::collections::HashMap;
use std::time::SystemTime;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("08.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    one(&vec);
    two(&vec);
}


fn one(input: &Vec<String>) {
    let mut ones = 0;
    let mut fours = 0;
    let mut sevens = 0;
    let mut eights = 0;
    for s in input {
        let words: Vec<&str> = s.split("|")
            .last().unwrap()
            .split_whitespace()
            .map(|s| s.trim())
            .collect();
        for w in words {
            match w.len() {
                2 => { ones += 1 }
                3 => { sevens += 1 }
                4 => { fours += 1 }
                7 => { eights += 1 }
                _ => {}
            }
        }
    }
    println!("8.1: {}", ones + fours + sevens + eights);
}

fn two(input: &Vec<String>) {}




