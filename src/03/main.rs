use std::collections::HashMap;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("03.txt");
    vec= vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
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

fn two(input: &Vec<String>) {
    let max_size = input.get(0).unwrap().len();
    let mut numbers1 = input.iter().map(|s| String::from(s)).collect::<Vec<String>>();
    let mut numbers2 = input.iter().map(|s| String::from(s)).collect::<Vec<String>>();
    let mut oxy = 0;
    let mut co2 = 0;
    for i in 0..max_size {
        let one_count = count(&nth_string(&numbers1, i), '1');
        let zero_count = numbers1.len() - one_count;
        if one_count >= zero_count {
            numbers1 = numbers1.iter().filter(|s| s.chars().nth(i).unwrap() == '1').map(|s| String::from(s)).collect();
        } else {
            numbers1 = numbers1.iter().filter(|s| s.chars().nth(i).unwrap() == '0').map(|s| String::from(s)).collect();
        }
        if numbers1.len() == 1 {
            oxy = u32::from_str_radix(numbers1[0].as_str(), 2).unwrap();
            break;
        }
    }
    for i in 0..max_size {
        let one_count = count(&nth_string(&numbers2, i), '1');
        let zero_count = numbers2.len() - one_count;
        if zero_count > one_count {
            numbers2 = numbers2.iter().filter(|s| s.chars().nth(i).unwrap() == '1').map(|s| String::from(s)).collect();
        } else {
            numbers2 = numbers2.iter().filter(|s| s.chars().nth(i).unwrap() == '0').map(|s| String::from(s)).collect();
        }
        if numbers2.len() == 1 {
            co2 = u32::from_str_radix(numbers2[0].as_str(), 2).unwrap();
            break;
        }
    }
    println!("3.2: {}", oxy * co2);
}

fn nth_string(input: &Vec<String>, i: usize) -> Vec<char> {
    return input.iter().map(|s| s.chars().nth(i).unwrap()).collect::<Vec<char>>();
}

fn count(input: &Vec<char>, c: char) -> usize {
    return input.iter().filter(|cc| **cc == c).count();
}

