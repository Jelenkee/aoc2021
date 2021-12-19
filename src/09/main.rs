use grid::Grid;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("09.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    one(&vec);
    two(&vec);
}


fn one(input: &Vec<String>) {
    let mut grid: Grid<u8> = Grid::new(input[0].len(), input.len());
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid[x][y] = c.to_digit(10).unwrap() as u8;
        }
    }
    let mut low_points: Vec<u8> = vec![];
    for x in 0..grid.size().0 {
        for y in 0..grid.size().1 {
            let p = grid[x][y];
            if get_neighbors(&grid, x, y).iter().all(|u| *u > p) {
                low_points.push(p);
            }
        }
    }
    println!("9.1: {}", low_points.iter().map(|p| (p + 1) as u32).sum::<u32>());
}

fn get_neighbors(grid: &Grid<u8>, x: usize, y: usize) -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    for p in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        let o = grid.get(p.0, p.1) as Option<&u8>;
        if o.is_some() {
            v.push(*o.unwrap());
        }
    }
    v
}

fn two(input: &Vec<String>) {}




