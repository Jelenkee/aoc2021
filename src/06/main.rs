#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("06.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    //vec[0]="3,4,3,1,2".to_string();
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
    let mut copy = fishes.clone();
    for i in 0..80 {
        let mut new_fishes = 0_u32;
        let mut cc = copy.clone();
        for f in &mut cc {
            if *f == 0u8 {
                *f = 6;
                new_fishes += 1;
            } else {
                *f -= 1;
            }
        }
        copy = cc;
        for _ in 0..new_fishes {
            copy.push(8);
        }
    }
    println!("6.1: {}", copy.len());
}

fn two(fishes: &Vec<u8>) {
    /*let mut copy = fishes.clone();
    //println!("{}",usize::MAX);
    for i in 0..256 {
        let mut new_fishes = 0_u32;
        let mut cc = copy.clone();
        for f in &mut cc {
            if *f == 0u8 {
                *f = 6;
                new_fishes += 1;
            } else {
                *f -= 1;
            }
        }
        copy = cc;
        for _ in 0..new_fishes {
            copy.push(8);
        }
    }
    println!("6.1: {}", copy.len());*/
}




