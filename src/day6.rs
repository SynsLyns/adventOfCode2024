use std::fs;

const FILE_PATH: &str = "./inputs/input06.txt";

struct Point {
    r: i32,
    c: i32,
}

impl Point {
    fn within_bounds(&self, rows: i32, cols: i32) -> bool {
        return 0 <= self.r && self.r < rows && 0 <= self.c && self.c < cols;
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut part2 = 0;

    let mut grid: Vec<(char, Vec<usize>)> = vec![];
    let rows = contents.lines().count() as i32;
    let cols = contents.lines().next().unwrap().len() as i32;

    let directions = [[-1, 0], [0, 1], [1, 0], [0, -1]];

    let mut guard_pos = Point { r: 0, c: 0 };
    let mut guard_dir = 0;

    for (i, line) in contents.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            grid.push((char, vec![]));
            if char == '^' {
                (guard_pos.r, guard_pos.c) = (i as i32, j as i32);
            }
        }
    }

    let guard_start_pos = Point { ..guard_pos };
    let orig_grid = grid.clone();

    while guard_pos.within_bounds(rows, cols) {
        grid[(guard_pos.r * cols + guard_pos.c) as usize].0 = 'c';
        let new_pos = Point {
            r: guard_pos.r + directions[guard_dir][0],
            c: guard_pos.c + directions[guard_dir][1],
        };
        if !new_pos.within_bounds(rows, cols) {
            break;
        }
        if grid[(new_pos.r * cols + new_pos.c) as usize].0 == '#' {
            guard_dir = (guard_dir + 1) % 4;
        } else {
            guard_pos = new_pos;
        }
    }

    let part1 = grid.iter().filter(|x| x.0 == 'c').count();

    let part1_grid = grid.clone();

    for i in 0..rows {
        for j in 0..cols {
            if orig_grid[(i * cols + j) as usize].0 == '#' || orig_grid[(i * cols + j) as usize].0 == '^' || part1_grid[(i * cols + j) as usize].0 == '.' {
                continue;
            }
            grid = orig_grid.clone();
            guard_pos = Point { ..guard_start_pos };
            guard_dir = 0;

            grid[(i * cols + j) as usize].0 = 'O';
            while guard_pos.within_bounds(rows, cols) {
                grid[(guard_pos.r * cols + guard_pos.c) as usize].0 = 'c';
                grid[(guard_pos.r * cols + guard_pos.c) as usize].1.push(guard_dir);
                let new_pos = Point {
                    r: guard_pos.r + directions[guard_dir][0],
                    c: guard_pos.c + directions[guard_dir][1],
                };
                if !new_pos.within_bounds(rows, cols) {
                    break;
                }
                if grid[(new_pos.r * cols + new_pos.c) as usize].0 == '#' || grid[(new_pos.r * cols + new_pos.c) as usize].0 == 'O' {
                    guard_dir = (guard_dir + 1) % 4;
                } else if grid[(new_pos.r * cols + new_pos.c) as usize].0 == 'c' && grid[(new_pos.r * cols + new_pos.c) as usize].1.contains(&guard_dir) {
                    part2 += 1;
                    break;
                } else {
                    guard_pos = new_pos;
                }
            }
        }
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}
