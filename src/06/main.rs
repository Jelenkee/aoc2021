use std::collections::HashMap;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("06.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    let fishes = vec[0]
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| u8::from_str_radix(s, 10).expect(" overlfow?"))
        .collect::<Vec<u8>>();
    one(&fishes);
    two(&fishes);
}


fn one(fishes: &Vec<u8>) {
    println!("6.1: {}", fishcount_after_days(fishes, 80));
}

fn two(fishes: &Vec<u8>) {
    println!("6.2: {}", fishcount_after_days(fishes, 256));
}

fn fishcount_after_days(fishes: &Vec<u8>, days: u16) -> usize {
    let mut copy = fishes.clone();
    let mut fishmap: HashMap<u8, u64> = HashMap::new();
    for f in &mut copy {
        let op = fishmap.get_mut(f);
        if op.is_some() {
            *op.unwrap() += 1;
        } else {
            fishmap.insert(*f, 1);
        }
    }
    for _ in 0..days {
        let mut new_fishes = 0_u64;
        let mut new_fishmap: HashMap<u8, u64> = HashMap::new();
        for k in fishmap.keys() {
            let value = fishmap.get(k).unwrap();
            if *k == 0u8 {
                add_to_map(&mut new_fishmap, 6, value.clone());
                new_fishes += value.clone();
            } else {
                add_to_map(&mut new_fishmap, k - 1, value.clone());
            }
        }
        if new_fishes > 0 {
            new_fishmap.insert(8, new_fishes);
        }
        fishmap = new_fishmap;
    }
    return fishmap.values().sum::<u64>() as usize;
}

fn add_to_map(map: &mut HashMap<u8, u64>, key: u8, num: u64) {
    let op = map.get_mut(&key);
    if op.is_some() {
        *op.unwrap() += num;
    } else {
        map.insert(key, num);
    }
}




