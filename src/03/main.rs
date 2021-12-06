use std::collections::HashMap;

#[path = "../util.rs"]
mod util;

fn main() {
    let vec: Vec<String> = util::read_file("03.txt");
    one(&vec);
    two(&vec);
}

fn one(input: &Vec<String>) {
    let mut ones: HashMap<u8, u32> = HashMap::new();
    for s in input {
        for (i, el) in s.chars().enumerate() {
            if el == '1' {
                let option = ones.get_mut(&(i as u8));
                if option.is_none() {
                    ones.insert(i as u8, 1);
                } else {
                    *option.unwrap() += 1;
                }
            }
        }
    }
    let mut gamma_string = String::from("");
    for i in 0..ones.len() {
        let c: char = if *ones.get(&(i as u8)).unwrap() > ((input.len() / 2) as u32) { '1' } else { '0' };
        gamma_string.push(c);
    }
    let mut epsilon_string = String::from("");
    for c in gamma_string.chars() {
        if c == '1' {
            epsilon_string.push('0');
        } else {
            epsilon_string.push('1');
        }
    }
    let gamma = u32::from_str_radix(gamma_string.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(epsilon_string.as_str(), 2).unwrap();
    println!("3.1: {}", gamma * epsilon);
}

fn two(input: &Vec<String>) {}
