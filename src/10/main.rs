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
    let incomplete_lines: Vec<(&String, VecDeque<char>)> = input.iter()
        .map(|s| {
            let mut stack = VecDeque::new();
            for c in s.chars() {
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
                        return (s, false, stack);
                    }
                }
            }
            return (s, true, stack);
        })
        .filter(|t| t.1)
        .map(|t| (t.0, t.2))
        .collect();

    println!("10.1: {}", score);
    let mut scores: Vec<i64> = vec![];
    for (l, stack) in incomplete_lines {
        let mut subscore = 0i64;
        stack.iter()
            .map(|cc| closings[index_of(&openings, cc)])
            .for_each(|cc| {
                subscore *= 5;
                subscore += match cc {
                    ')' => { 1 }
                    ']' => { 2 }
                    '}' => { 3 }
                    '>' => { 4 }
                    _ => { panic!("AARGH!!") }
                };
            });
        scores.push(subscore);
    }
    scores.sort();
    println!("10.2: {}", scores[scores.len() / 2]);
}

fn index_of(v: &Vec<char>, c: &char) -> usize {
    v.iter().position(|&cc| cc == *c).unwrap()
}

fn two(input: &Vec<String>) {}






