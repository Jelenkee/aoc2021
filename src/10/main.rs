use std::collections::VecDeque;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("10.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    one(&vec);
    two(&vec);
}


fn one(input: &Vec<String>) {
    let openings = vec!['(', '[', '{', '<'];
    let closings = vec![')', ']', '}', '>'];

    let mut score = 0;

    for s in input {
        let mut stack = VecDeque::new();
        for (i, c) in s.chars().enumerate() {
            if openings.contains(&c) {
                stack.push_front(c);
            } else {
                if !closings.contains(&c) {
                    panic!("WTF??!");
                }
                let el = stack.pop_front().unwrap();
                if index_of(&openings, &el) != index_of(&closings, &c) {
                    score += match c {
                        ')' => { 3 }
                        ']' => { 57 }
                        '}' => { 1197 }
                        '>' => { 25137 }
                        _ => { panic!("AARGH!!") }
                    };
                    break;
                }
            }
        }
    }
    println!("10.1: {}", score);
}

fn index_of(v: &Vec<char>, c: &char) -> usize {
    v.iter().position(|&cc| cc == *c).unwrap()
}

fn two(input: &Vec<String>) {}






