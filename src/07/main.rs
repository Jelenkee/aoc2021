use std::collections::HashMap;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("07.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    let crabs = vec[0]
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| u16::from_str_radix(s, 10).expect(" overlfow?"))
        .collect::<Vec<u16>>();
    one(&crabs);
    two(&crabs);
}


fn one(crabs: &Vec<u16>) {
    let min: u16 = *crabs.iter().min().unwrap();
    let max: u16 = *crabs.iter().max().unwrap();

    let mut least_fuel = u32::MAX;
    let mut least_fuel_pos = 0u16;
    for pos in min..=max {
        let mut total_fuel = 0u32;
        for crab in crabs {
            total_fuel += (*crab as i32 - pos as i32).abs() as u32;
        }
        if (total_fuel < least_fuel) {
            least_fuel = total_fuel;
            least_fuel_pos = pos;
        }
    }
    let sum: i32 = crabs.iter().map(|c| (*c as i32 - least_fuel_pos as i32).abs()).sum();
    println!("7.1: {}", sum);
}

fn two(crabs: &Vec<u16>) {
    let min: u16 = *crabs.iter().min().unwrap();
    let max: u16 = *crabs.iter().max().unwrap();

    let mut least_fuel = u32::MAX;
    let mut least_fuel_pos = 0u16;
    for pos in min..=max {
        let mut total_fuel = 0u32;
        for crab in crabs {
            total_fuel += calc_fuel((*crab as i32 - pos as i32).abs()) as u32;
        }
        if (total_fuel < least_fuel) {
            least_fuel = total_fuel;
            least_fuel_pos = pos;
        }
    }
    let sum: i32 = crabs.iter().map(|c| calc_fuel((*c as i32 - least_fuel_pos as i32).abs())).sum();
    println!("7.2: {}", sum);
}

fn calc_fuel(steps: i32) -> i32 {
    let mut cost_per_step = 1;
    let mut total_cost = 0;
    for i in 0..steps {
        total_cost += cost_per_step;
        cost_per_step += 1;
    }
    return total_cost;
}




