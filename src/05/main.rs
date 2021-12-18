use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("05.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    one(&vec);
    two(&vec);
}

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct Vector {
    start: Point,
    end: Point,
}

fn one(input: &Vec<String>) {
    let vectors = vectors(input);
    let vectors2: Vec<&Vector> = vectors.iter().filter(|v| v.start.x == v.end.x || v.start.y == v.end.y).collect();
    let mut points: Vec<Point> = vec![];
    for vector in vectors2 {
        for p in all_point(vector) {
            points.push(p);
        }
    }
    let mut point_set = HashSet::new();
    for p in &points {
        point_set.insert(p);
    }
    let mut cc = 0;
    for p in point_set {
        let c = count(&points, &p);
        if c > 1 {
            cc += 1;
        }
    }
    println!("5.1: {}", cc);
}

fn two(input: &Vec<String>) {
    let vectors = vectors(input);
    let vectors2: Vec<&Vector> = vectors.iter().collect();
}

fn vectors(input: &Vec<String>) -> Vec<Vector> {
    return input.iter().map(|s| {
        let points: Vec<&str> = s.split("->").map(|s| s.trim()).collect();
        let coords1: Vec<&str> = points[0].split(",").collect();
        let coords2: Vec<&str> = points[1].split(",").collect();
        return Vector {
            start: Point {
                x: u16::from_str(coords1[0]).unwrap(),
                y: u16::from_str(coords1[1]).unwrap(),
            },
            end: Point {
                x: u16::from_str(coords2[0]).unwrap(),
                y: u16::from_str(coords2[1]).unwrap(),
            },
        };
    }).collect();
}

fn all_point(vector: &Vector) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();
    if vector.start.x == vector.end.x {
        let min = min(vector.start.y,vector.end.y);
        let max = max(vector.start.y,vector.end.y);
        for y in min..=max {
            set.insert(Point { x: vector.start.x, y });
        }
    } else {
        let min = min(vector.start.x,vector.end.x);
        let max = max(vector.start.x,vector.end.x);
        for x in min..=max {
            set.insert(Point { x, y: vector.start.y });
        }
    }
    return set;
}

fn count(vec: &Vec<Point>, t: &Point) -> usize {
    let mut c = 0 as usize;
    for v in vec {
        if *v == *t {
            c += 1;
        }
    }
    return c;
}



