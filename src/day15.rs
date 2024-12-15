use std::fs;

const FILE_PATH: &str = "./inputs/input15.txt";

#[derive(Debug, Clone, Copy)]
struct Point {
    r: i32,
    c: i32,
}

impl Point {
    fn to_index(&self, cols: i32) -> usize {
        return (self.r * cols + self.c) as usize;
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\r\n\r\n").collect();

    let map = contents[0];
    let moves = contents[1].replace("\r\n", "");

    let mut part1 = 0;
    let mut part2 = 0;

    let mut grid: Vec<char> = vec![];
    let mut grid2: Vec<char> = vec![];
    let rows = map.lines().count() as i32;
    let cols = map.lines().next().unwrap().len() as i32;

    let mut robot_pos = Point { r: 0, c: 0 };
    let mut robot_pos2 = Point { r: 0, c: 0 };

    for (i, line) in map.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                (robot_pos.r, robot_pos.c) = (i as i32, j as i32);
                (robot_pos2.r, robot_pos2.c) = (i as i32, (j*2) as i32);
                grid.push('.');
                grid2.push('.');
                grid2.push('.');
            }
            else {
                grid.push(c);
                match c {
                    'O' => {
                        grid2.push('[');
                        grid2.push(']');
                    },
                    _ => {
                        grid2.push(c);
                        grid2.push(c);
                    }
                }
            }
        }
    }

    for m in moves.chars() {
        let mut next_pos: Point = robot_pos;
        let mut next_pos2: Point = robot_pos2;
        let mut change = (0, 0);
        match m {
            '<' => {
                next_pos.c -= 1;
                next_pos2.c -= 1;
                change = (0, -1);
            },
            '^' => {
                next_pos.r -= 1;
                next_pos2.r -= 1;
                change = (-1, 0);
            },
            'v' => {
                next_pos.r += 1;
                next_pos2.r += 1;
                change = (1, 0);
            },
            '>' => {
                next_pos.c += 1;
                next_pos2.c += 1;
                change = (0, 1);
            },
            _ => {},
        }
        let c = grid[next_pos.to_index(cols)];
        let c2 = grid2[next_pos2.to_index(cols*2)];
        match c {
            '.' => {
                robot_pos = next_pos;
            },
            'O' => {
                let mut move_pos = Point {r: next_pos.r + change.0, c: next_pos.c + change.1};
                while grid[move_pos.to_index(cols)] == 'O' {
                    move_pos.r += change.0;
                    move_pos.c += change.1;
                }
                if grid[move_pos.to_index(cols)] == '.' {
                    grid[move_pos.to_index(cols)] = 'O';
                    grid[next_pos.to_index(cols)] = '.';
                    robot_pos = next_pos;
                }
            }
            _ => {}
        }
        match c2 {
            '.' => {
                robot_pos2 = next_pos2;
            },
            ']'|'[' => {
                if m == '<' || m == '>' {
                    let mut move_pos = Point {r: next_pos2.r + change.0, c: next_pos2.c + change.1};
                    while grid2[move_pos.to_index(cols*2)] == '[' || grid2[move_pos.to_index(cols*2)] == ']' {
                        move_pos.r += change.0;
                        move_pos.c += change.1;
                    }
                    if grid2[move_pos.to_index(cols*2)] == '.' {
                        while move_pos.c != next_pos2.c || move_pos.r != next_pos2.r {
                            let prev_pos = Point {r: move_pos.r - change.0, c: move_pos.c - change.1};
                            grid2[move_pos.to_index(cols*2)] = grid2[prev_pos.to_index(cols*2)];
                            move_pos = prev_pos;
                        }
                        grid2[next_pos2.to_index(cols*2)] = '.';
                        robot_pos2 = next_pos2;
                    }
                }
                else {
                    let suceeded = try_move(&mut grid2, &rows, &(cols*2), &change, &next_pos2);
                    if suceeded {
                        do_move(&mut grid2, &rows, &(cols*2), &change, &next_pos2);
                        robot_pos2 = next_pos2;
                    }
                }
            }
            _ => {}
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if grid[(i*cols + j) as usize] == 'O' {
                part1 += 100 * i + j;
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols*2 {
            if grid2[(i*cols*2 + j) as usize] == '[' {
                part2 += 100 * i + j;
            }
        }
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}

fn try_move(grid: &mut Vec<char>, rows: &i32, cols: &i32, dir: &(i32, i32), pos: &Point) -> bool {
    let c = grid[pos.to_index(*cols)];
    if c == '.' {
        return true;
    }
    else if c == '#' {
        return false;
    }
    let mut also_check = -1;
    if c == '[' {
        also_check = 1;
    }
    let check1 = Point {r: pos.r + dir.0, c: pos.c + dir.1};
    let check2 = Point {r: pos.r + dir.0, c: pos.c + dir.1 + also_check};
    return try_move(grid, rows, cols, dir, &check1) && try_move(grid, rows, cols, dir, &check2);
}

fn do_move(grid: &mut Vec<char>, rows: &i32, cols: &i32, dir: &(i32, i32), pos: &Point) {
    let c = grid[pos.to_index(*cols)];
    if c == '.' {
        return;
    }
    let mut also_check = -1;
    if c == '[' {
        also_check = 1;
    }
    let move1 = Point {r: pos.r + dir.0, c: pos.c + dir.1};
    let move2 = Point {r: pos.r + dir.0, c: pos.c + dir.1 + also_check};
    do_move(grid, rows, cols, dir, &move1);
    do_move(grid, rows, cols, dir, &move2);
    let other = Point {r: pos.r, c: pos.c + also_check};
    if c == '[' {
        grid[move1.to_index(*cols)] = '[';
        grid[move2.to_index(*cols)] = ']';
        
    }
    else {
        grid[move1.to_index(*cols)] = ']';
        grid[move2.to_index(*cols)] = '[';
    }
    grid[other.to_index(*cols)] = '.';
    grid[pos.to_index(*cols)] = '.';
}