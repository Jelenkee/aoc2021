use grid::Grid;

#[path = "../util.rs"]
mod util;

fn main() {
    let mut vec: Vec<String> = util::read_file("11.txt");
    vec = vec.iter().filter(|s| !s.is_empty()).map(|s| String::from(s)).collect();
    let mut grid: Grid<u8> = Grid::new(vec[0].len(), vec.len());
    for (y, row) in vec.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid[x][y] = c.to_digit(10).unwrap() as u8;
        }
    }
    one(&grid);
}


fn one(grid: &Grid<u8>) {
    println!("{:?}", grid);
    for i in 0..100 {
        for x in grid.size().0 {
            for y in grid.size().1 {
                grid[x][y]+=1;
            }
        }
    }
}








