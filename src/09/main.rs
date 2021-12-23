use std::collections::HashSet;
use std::collections::VecDeque;
use grid::Grid;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("09.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    one(&vec);
}


fn one(input: &Vec<String>) {
    let mut grid: Grid<u8> = Grid::new(input[0].len(), input.len());
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid[x][y] = c.to_digit(10).unwrap() as u8;
        }
    }
    let grid = grid;
    let mut low_points: Vec<(usize, usize)> = vec![];
    for x in 0..grid.size().0 {
        for y in 0..grid.size().1 {
            if get_neighbors(&grid, x, y).iter().all(|u| grid[u.0][u.1] > grid[x][y]) {
                low_points.push((x, y));
            }
        }
    }
    println!("9.1: {}", low_points.iter()
        .map(|p| grid[p.0][p.1])
        .map(|p| (p + 1) as u32)
        .sum::<u32>());

    let mut basins: Vec<usize> = vec![];

    for lp in low_points {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        set.insert(lp);
        let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
        deque.push_back(lp);
        while !deque.is_empty() {
            let p = deque.pop_front().unwrap();
            let neighbors = get_neighbors(&grid, p.0, p.1).iter()
                .filter(|pp| !deque.contains(pp))
                .filter(|pp| grid[p.0][p.1] < grid[pp.0][pp.1])
                .filter(|pp| 9 > grid[pp.0][pp.1])
                .map(|pp| *pp)
                .collect::<Vec<(usize, usize)>>();
            for n in neighbors {
                deque.push_back(n);
                set.insert(n);
            }
        }
        basins.push(set.len());
    }
    basins.sort();
    basins.reverse();
    println!("9.2: {}", (0..3).map(|i| basins[i]).fold(1, |a, b| a * b));
}

fn get_neighbors(grid: &Grid<u8>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = vec![];
    for p in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        let o = grid.get(p.0, p.1) as Option<&u8>;
        if o.is_some() {
            v.push(p);
        }
    }
    v
}





