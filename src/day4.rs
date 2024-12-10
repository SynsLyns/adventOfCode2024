use std::fs;

const FILE_PATH: &str = "./inputs/input04.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut grid: Vec<char> = vec![];
    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();

    for line in contents.lines() {
        for char in line.chars() {
            grid.push(char);
        }
    }

    let directions = [[0,1],[0,-1],[1,0],[-1,0],[1,1],[1,-1],[-1,1],[-1,-1]];
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..rows {
        for j in 0..cols {
            for dir in directions {
                part1 += if is_xmas(&grid, i as i32, j as i32, rows as i32, cols as i32, dir) {1} else {0};
            }
            part2 += if is_x_mas(&grid, i as i32, j as i32, rows as i32, cols as i32) {1} else {0};
        }
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}

fn is_xmas(grid: &Vec<char>, i: i32, j: i32, rows: i32, cols: i32, direction: [i32; 2]) -> bool {
    if is_equal(&grid, i, j, rows, cols, 'X') && 
       is_equal(&grid, i + direction[0], j + direction[1], rows, cols, 'M') && 
       is_equal(&grid, i + direction[0]*2, j + direction[1]*2, rows, cols, 'A') && 
       is_equal(&grid, i + direction[0]*3, j + direction[1]*3, rows, cols, 'S')
    {
        return true;
    }
    return false;
}

fn is_x_mas(grid: &Vec<char>, i: i32, j: i32, rows: i32, cols: i32) -> bool {
    let directions = [[1,1],[-1,-1],[1,-1],[-1,1]];
    if is_equal(&grid, i, j, rows, cols, 'A') &&
       ((is_equal(&grid, i + directions[0][0], j + directions[0][1], rows, cols, 'M') && (is_equal(&grid, i + directions[1][0], j + directions[1][1], rows, cols, 'S'))) ||
        (is_equal(&grid, i + directions[0][0], j + directions[0][1], rows, cols, 'S') && (is_equal(&grid, i + directions[1][0], j + directions[1][1], rows, cols, 'M')))) &&
       ((is_equal(&grid, i + directions[2][0], j + directions[2][1], rows, cols, 'M') && (is_equal(&grid, i + directions[3][0], j + directions[3][1], rows, cols, 'S'))) ||
        (is_equal(&grid, i + directions[2][0], j + directions[2][1], rows, cols, 'S') && (is_equal(&grid, i + directions[3][0], j + directions[3][1], rows, cols, 'M'))))
    {
        return true;
    }
    return false;
}

fn is_equal(grid: &Vec<char>, i: i32, j: i32, rows: i32, cols: i32, value: char) -> bool {
    if 0 <= i && i < rows && 0 <= j && j < cols {
        return grid[(i*cols + j) as usize] == value;
    }
    return false;
}
