use std::collections::HashMap;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("07.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    //vec[0]="16,1,2,0,4,2,7,1,2,14".to_string();
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
        let mut total_fuel=0u32;
        for crab in crabs {
            total_fuel+=(*crab as i32-pos as i32).abs() as u32;
        }
        println!("{}",total_fuel);
        if(total_fuel<least_fuel){
            least_fuel=total_fuel;
            least_fuel_pos=pos;
        }
    }
    let sum:i32=crabs.iter().map(|c|(*c as i32-least_fuel_pos as i32).abs()).sum();
    println!("7.1: {}",sum);
}

fn two(crabs: &Vec<u16>) {}





