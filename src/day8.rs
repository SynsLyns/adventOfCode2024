use std::fs;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

const FILE_PATH: &str = "./inputs/input08.txt";

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    r: i32,
    c: i32,
}

impl Point {
    fn within_bounds(&self, rows: i32, cols: i32) -> bool {
        return 0 <= self.r && self.r < rows && 0 <= self.c && self.c < cols;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            r: self.r - other.r,
            c: self.c - other.c,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let rows = contents.lines().count() as i32;
    let cols = contents.lines().next().unwrap().len() as i32;

    for (i, line) in contents.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            match antennas.get_mut(&char) {
                Some(x) => x.push(Point {r: i as i32 , c: j as i32}),
                None => {antennas.insert(char, vec![Point {r: i as i32 , c: j as i32}]);}
            }
        }
    }

    let mut seen: HashSet<Point> = HashSet::new();
    for (_, list) in &antennas {
        for (i, el1) in list.iter().enumerate() {
            for el2 in list[i+1..].iter() {
                let dist = *el1 - *el2;
                let antinode1 = *el1 + dist;
                let antinode2 = *el2 - dist;
                if antinode1.within_bounds(rows, cols) {
                    seen.insert(antinode1);
                }
                if antinode2.within_bounds(rows, cols) {
                    seen.insert(antinode2);
                }
            }
        }
    }

    let part1 = seen.len();

    let mut seen: HashSet<Point> = HashSet::new();
    for (_, list) in &antennas {
        for (i, el1) in list.iter().enumerate() {
            for el2 in list[i+1..].iter() {
                let dist = *el1 - *el2;
                let mut current = *el1;
                while (current).within_bounds(rows, cols) {
                    seen.insert(current);
                    current = current + dist;
                }
                current = *el2;
                while (current).within_bounds(rows, cols) {
                    seen.insert(current);
                    current = current - dist;
                }
            }
        }
    }

    let part2 = seen.len();

    println!("Part 1: {part1}, Part 2: {part2}");
}