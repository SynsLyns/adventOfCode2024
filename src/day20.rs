use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input20.txt";
const DIRS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();
    let mut grid = vec![];
    let mut dist_from_end = vec![];
    let mut dist_from_start = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, line) in contents.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid.push(ch);
            dist_from_end.push(usize::MAX);
            dist_from_start.push(usize::MAX);
            if ch == 'S' {
                start = (r, c);
            } else if ch == 'E' {
                end = (r, c);
            }
        }
    }

    let now = Instant::now();

    bfs(&grid, start, &mut dist_from_start, rows, cols);
    bfs(&grid, end, &mut dist_from_end, rows, cols);

    let best = dist_from_end[to_index((start.0, start.1), (0, 0), cols)];
    let mut path_idxs = vec![];
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if grid[r * cols + c] != '#' {
                path_idxs.push((r, c));
            }
        }
    }

    let part1 = get_cheats(
        &path_idxs,
        &dist_from_start,
        &dist_from_end,
        best,
        rows,
        cols,
        2,
        &grid
    );
    let part2 = get_cheats(
        &path_idxs,
        &dist_from_start,
        &dist_from_end,
        best,
        rows,
        cols,
        20,
        &grid
    );

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1} Part 2: {part2}");
}

fn to_index(point: (usize, usize), offset: (i32, i32), cols: usize) -> usize {
    return (point.0 as i32 + offset.0) as usize * cols + (point.1 as i32 + offset.1) as usize;
}

fn bfs(grid: &Vec<char>, start: (usize, usize), dist: &mut Vec<usize>, rows: usize, cols: usize) {
    let mut seen = vec![false; rows * cols];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(start.0, start.1)]);
    dist[to_index(start, (0, 0), cols)] = 0;
    seen[to_index(start, (0, 0), cols)] = true;
    while let Some(p) = queue.pop_front() {
        for dir in DIRS {
            if grid[to_index(p, dir, cols)] != '#' && !seen[to_index(p, dir, cols)] {
                seen[to_index(p, dir, cols)] = true;
                dist[to_index(p, dir, cols)] = dist[to_index(p, (0, 0), cols)] + 1;
                queue.push_back(((p.0 as i32 + dir.0) as usize, (p.1 as i32 + dir.1) as usize));
            }
        }
    }
}

fn get_cheats(
    path_idxs: &Vec<(usize, usize)>,
    dist_from_start: &Vec<usize>,
    dist_from_end: &Vec<usize>,
    best: usize,
    rows: usize,
    cols: usize,
    cheat_length_alllowed: usize,
    grid: &Vec<char>
) -> usize {
    let mut return_value = 0;
    let mut cheat_map: HashMap<usize, usize> = HashMap::new();
    for i in path_idxs {
        for r in 1..=cheat_length_alllowed {
            for j in generate_points(i, r, rows, cols, grid) {
                let i_dist_from_start = dist_from_start[to_index(*i, (0, 0), cols)];
                let j_dist_from_end = dist_from_end[to_index(j, (0, 0), cols)];
                if i_dist_from_start + j_dist_from_end + r < best {
                    let time_saved = best - (i_dist_from_start + j_dist_from_end + r);
                    match cheat_map.get_mut(&time_saved) {
                        Some(x) => {
                            *x += 1;
                        }
                        None => {
                            cheat_map.insert(time_saved, 1);
                        }
                    }
                }
            }
        }
    }

    for i in cheat_map {
        if i.0 >= 100 {
            return_value += i.1;
        }
    }
    return return_value;
}

fn generate_points(
    (x, y): &(usize, usize),
    r: usize,
    rows: usize,
    cols: usize,
    grid: &Vec<char>
) -> Vec<(usize, usize)> {
    let mut points = vec![];
    let x = *x as i32;
    let y = *y as i32;
    let r = r as i32;
    for offset in 0..r {
        let inv_offset = r - offset;
        let push_points = [
            (x + offset, y + inv_offset),
            (x + inv_offset, y - offset),
            (x - offset, y - inv_offset),
            (x - inv_offset, y + offset),
        ];
        for p in push_points {
            if 0 <= p.0 && p.0 < rows as i32 && 0 <= p.1 && p.1 < cols as i32 && grid[p.0 as usize *cols + p.1 as usize] != '#' {
                points.push((p.0 as usize, p.1 as usize));
            }
        }
    }
    return points;
}
