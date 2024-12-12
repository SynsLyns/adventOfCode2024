use std::fs;

const FILE_PATH: &str = "./inputs/input10.txt";

const DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

struct Point {
    r: i32,
    c: i32,
}

impl Point {
    fn within_bounds(&self, rows: i32, cols: i32) -> bool {
        return 0 <= self.r && self.r < rows && 0 <= self.c && self.c < cols;
    }

    fn to_index(&self, cols: i32) -> usize {
        return (self.r * cols + self.c) as usize;
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;

    let mut grid: Vec<i32> = vec![];
    let rows = contents.lines().count() as i32;
    let cols = contents.lines().next().unwrap().len() as i32;

    let mut trailheads: Vec<Point> = vec![];
    for (i, line) in contents.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            grid.push(char.to_digit(10).unwrap() as i32);
            if char == '0' {
                trailheads.push(Point {r: i as i32, c: j as i32});
            }
        }
    }
    
    for trailhead in trailheads {
        let mut seen = vec![false; grid.len()];
        part1 += get_reachable_peaks(&trailhead, &grid, &mut seen, rows, cols);
        part2 += get_reachable_peaks2(&trailhead, &grid, rows, cols);
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}

fn get_reachable_peaks(pos: &Point, grid: &Vec<i32>, seen: &mut Vec<bool>, rows: i32, cols: i32) -> i32 {
    if seen[pos.to_index(cols)] {
        return 0;
    }
    seen[pos.to_index(cols)] = true;
    if grid[pos.to_index(cols)] == 9 {
        return 1;
    }
    let mut reachable_peaks = 0;
    for dir in DIRECTIONS {
        let new_pos = Point {r: pos.r + dir[0], c: pos.c + dir[1]};
        if new_pos.within_bounds(rows, cols) && grid[new_pos.to_index(cols)] == grid[pos.to_index(cols)] + 1 {
            reachable_peaks += get_reachable_peaks(&new_pos, grid, seen, rows, cols);
        }
    }
    return reachable_peaks;
}

fn get_reachable_peaks2(pos: &Point, grid: &Vec<i32>, rows: i32, cols: i32) -> i32 {
    if grid[pos.to_index(cols)] == 9 {
        return 1;
    }
    let mut reachable_peaks = 0;
    for dir in DIRECTIONS {
        let new_pos = Point {r: pos.r + dir[0], c: pos.c + dir[1]};
        if new_pos.within_bounds(rows, cols) && grid[new_pos.to_index(cols)] == grid[pos.to_index(cols)] + 1 {
            reachable_peaks += get_reachable_peaks2(&new_pos, grid, rows, cols);
        }
    }
    return reachable_peaks;
}