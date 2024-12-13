use std::{collections::VecDeque, fs};

const FILE_PATH: &str = "./inputs/input12.txt";

#[derive(Debug, Clone, Copy)]
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

    let mut grid: Vec<char> = vec![];
    let mut visited: Vec<bool> = vec![];
    let rows = contents.lines().count() as i32;
    let cols = contents.lines().next().unwrap().len() as i32;

    for line in contents.lines() {
        for c in line.chars() {
            grid.push(c);
            visited.push(false);
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            let point = Point { r: i, c: j };
            if !visited[point.to_index(cols)] {
                let (x, y) = get_fence_pricing(point, &grid, &mut visited, rows, cols);
                part1 += x;
                part2 += y;
            }
        }
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}

fn get_fence_pricing(
    point: Point,
    grid: &Vec<char>,
    visited: &mut Vec<bool>,
    rows: i32,
    cols: i32,
) -> (u64, u64) {
    let plant = grid[point.to_index(cols)];
    visited[point.to_index(cols)] = true;

    let (mut area, mut perimeter, mut sides) = (0, 0, 0);
    let mut queue: VecDeque<Point> = VecDeque::from([point]);
    while let Some(i) = queue.pop_front() {
        area += 1;
        let adjacent = [
            Point { r: i.r + 1, c: i.c },
            Point { r: i.r - 1, c: i.c },
            Point { r: i.r, c: i.c + 1 },
            Point { r: i.r, c: i.c - 1 },
        ];
        let mut closed: Vec<usize> = vec![];
        let mut a_perimeter = 4;
        for (index, a) in adjacent.iter().enumerate() {
            if a.within_bounds(rows, cols) && grid[a.to_index(cols)] == plant {
                if !visited[a.to_index(cols)] {
                    visited[a.to_index(cols)] = true;
                    queue.push_back(*a);
                }
                a_perimeter -= 1;
                closed.push(index);
            }
        }
        sides += match a_perimeter {
            4 => 4,
            3 => 2,
            2 => {
                if (closed.contains(&0) && closed.contains(&1))
                    || (closed.contains(&2) && closed.contains(&3))
                {
                    0
                } else {
                    get_inner_corners(&closed, &adjacent, grid, plant, i, rows, cols) + 1
                }
            }
            0 | 1 => get_inner_corners(&closed, &adjacent, grid, plant, i, rows, cols),
            _ => 0,
        };
        perimeter += a_perimeter;
    }
    return (area * perimeter, area * sides);
}

fn get_inner_corners(
    closed: &Vec<usize>,
    adjacent: &[Point; 4],
    grid: &Vec<char>,
    plant: char,
    i: Point,
    rows: i32,
    cols: i32,
) -> u64 {
    let mut inner_corners = 0;
    for (j, el1) in closed.iter().enumerate() {
        for el2 in closed[j + 1..].iter() { // can be two opposite sides but then check point is equal to i (and would have value of plant in grid)
            let closed_side1 = adjacent[*el1];
            let closed_side2 = adjacent[*el2];
            let check_point = Point {
                r: closed_side1.r + closed_side2.r - i.r,
                c: closed_side1.c + closed_side2.c - i.c,
            };
            if !check_point.within_bounds(rows, cols) || grid[check_point.to_index(cols)] != plant {
                inner_corners += 1;
            }
        }
    }
    return inner_corners;
}
