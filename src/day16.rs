use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}, fs};
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input16.txt";
const DIRS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    cost: usize,
    r: i32,
    c: i32,
    dir: usize,
    prev: Vec<usize>
}

impl Node {
    fn to_index(&self, cols: i32) -> usize {
        return (self.r * cols + self.c) as usize;
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    
    let cols = contents.lines().next().unwrap().len() as i32;
    let mut grid = vec![];
    let mut cost: Vec<usize> = vec![];
    let mut start = (0, 0);

    for (i, line) in contents.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            grid.push(char);
            for _ in 0..DIRS.len() {
                cost.push(usize::MAX);
            }
            if char == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    let now = Instant::now();
    
    let mut pq = BinaryHeap::new();
    let mut start = Node {cost: 0, r: start.0, c: start.1, dir: 0, prev: vec![]};
    start.prev.push(start.to_index(cols));
    cost[start.to_index(cols)] = 0;
    pq.push(start);

    let mut part1 = usize::MAX;
    let mut part2 = HashSet::new();
    while let Some(u) = pq.pop() {
        if u.cost > cost[u.to_index(cols) * 4 + u.dir] {
            continue;
        }
        else {
            cost[u.to_index(cols) * 4 + u.dir] = u.cost;
        }

        if grid[u.to_index(cols)] == 'E' && u.cost <= part1 {
            part2.extend(u.prev.clone());
            part1 = u.cost;
            continue;
        }

        for (dir, (dr, dc)) in DIRS.iter().enumerate() {
            if dir == (u.dir + 2) % DIRS.len() {
                continue;
            }

            let (r, c) = (u.r + dr, u.c + dc);
            let v = (r*cols + c) as usize;
            let total_cost = if dir == u.dir {u.cost + 1} else {u.cost + 1001};

            if grid[v] != '#' {
                let mut new_prev = u.prev.clone();
                new_prev.push(v);
                pq.push(Node {cost: total_cost, r, c, dir, prev: new_prev});
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Time: {:.2?}", elapsed);
    println!("Part 1: {part1} Part 2: {}", part2.len());
}