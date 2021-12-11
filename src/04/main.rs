#[path = "../util.rs"]
mod util;

fn main() {
    let vec: Vec<String> = util::read_file("04.txt");
    one(&vec);
    two(&vec);
}

#[derive(Debug)]
struct Boardentry {
    num: u32,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    entries: Vec<Boardentry>,
}

impl Board {
    fn push_entry(&mut self, entry: Boardentry) {
        self.entries.push(entry);
    }

    fn get_unmarked_entries(&self) -> Vec<&Boardentry> {
        return self.entries.iter().filter(|e| !e.marked).collect::<Vec<&Boardentry>>();
    }

    fn mark_entry(&mut self, num: u32) {
        self.entries.iter_mut().filter(|e| e.num == num).for_each(|e| e.marked = true);
    }

    fn has_bingo(&self) -> bool {
        let mut grid: Vec<Vec<&Boardentry>> = vec![];
        for _ in 0..5 {
            grid.push(vec![]);
        }
        let mut index = 0;
        for x in 0..5 {
            for _ in 0..5 {
                grid[x].push(&self.entries[index]);
                index += 1;
            }
        }

        for x in 0..5 {
            if (0..5).all(|y| grid[x][y].marked) {
                return true;
            }
            if (0..5).all(|y| grid[y][x].marked) {
                return true;
            }
        }

        return false;
    }
}

fn one(input: &Vec<String>) {
    let numbers_to_draw = input[0].split(",").map(|s| u32::from_str_radix(s, 10).unwrap()).collect::<Vec<u32>>();
    let mut boards: Vec<Board> = get_boards(input);
    let mut boards2: Vec<&mut Board> = boards.iter_mut().filter(|b| !b.entries.is_empty()).collect();
    for n in numbers_to_draw {
        for b in &mut boards2 {
            b.mark_entry(n);
            if b.has_bingo() {
                let sum: u32 = b.get_unmarked_entries().iter().map(|b| b.num).sum();
                println!("4.1: {}", sum * n);
                return;
            }
        }
    }
}

fn two(input: &Vec<String>) {
    let numbers_to_draw = input[0].split(",").map(|s| u32::from_str_radix(s, 10).unwrap()).collect::<Vec<u32>>();
    let mut boards: Vec<Board> = get_boards(input);
    let mut boards2: Vec<&mut Board> = boards.iter_mut().filter(|b| !b.entries.is_empty()).collect();
    let mut bingos = 0;
    let board_count = boards2.len();
    for n in numbers_to_draw {
        for b in &mut boards2 {
            let one_missing = board_count - 1 == bingos;
            let bingo = b.has_bingo();
            b.mark_entry(n);
            if !bingo && b.has_bingo() {
                bingos += 1;
            }
            if !bingo && one_missing && b.has_bingo() {
                let sum: u32 = b.get_unmarked_entries().iter().map(|b| b.num).sum();
                println!("4.2: {}", sum * n);
                return;
            }
        }
    }
}

fn get_boards(input: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    for i in 1..input.len() {
        let line = &input[i];
        if line.is_empty() {
            boards.push(Board { entries: vec![] });
        } else {
            let last_board = boards.last_mut();
            let current_board: &mut Board = last_board.unwrap();
            let row = line.split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| u32::from_str_radix(s, 10).unwrap())
                .map(|u| Boardentry { num: u, marked: false })
                .collect::<Vec<Boardentry>>();
            for e in row {
                current_board.push_entry(e);
            }
        }
    }
    return boards;
}


