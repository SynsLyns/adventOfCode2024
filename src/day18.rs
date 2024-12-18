use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input18.txt";
const DIRS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut part1 = 0;
    let mut part2 = (0, 0);

    let rows = 71;
    let cols = 71;
    let bytes = 1024;
    let mut grid = vec![vec!['.'; cols]; rows];
    let mut seen = vec![vec![false; cols]; rows];

    for (index, line) in lines.iter().enumerate() {
        if index >= bytes {
            break;
        }
        let nums: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
        grid[nums[1]][nums[0]] = '#';
    }

    let now = Instant::now();

    bfs(&mut seen, &grid, &mut part1, rows, cols);

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1:?}");

    let now = Instant::now();

    for &line in lines[bytes..].iter() {
        let nums: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
        grid[nums[1]][nums[0]] = '#';
        seen = vec![vec![false; cols]; rows];
        if !bfs(&mut seen, &grid, &mut part1, rows, cols) {
            (part2.0, part2.1) = (nums[0], nums[1]);
            break;
        }
    }
    
    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 2: {part2:?}");
}

fn within_bounds(r: i32, c: i32, rows: i32, cols: i32) -> bool {
    return 0 <= r && r < rows && 0 <= c && c < cols;
}

fn bfs(seen: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>, part1: &mut i32, rows: usize, cols: usize) -> bool {
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([(0, 0, 0)]);
    seen[0][0] = true;
    while let Some((r, c, dist)) = queue.pop_front() {
        if r == rows as i32 - 1 && c == cols as i32 - 1 {
            *part1 = dist;
            return true;
        }
        for dir in DIRS {
            if within_bounds(r + dir.0, c + dir.1, rows as i32, cols as i32)
                && grid[(r + dir.0) as usize][(c + dir.1) as usize] != '#'
                && !seen[(r + dir.0) as usize][(c + dir.1) as usize]
            {
                seen[(r + dir.0) as usize][(c + dir.1) as usize] = true;
                queue.push_back((r + dir.0, c + dir.1, dist + 1));
            }
        }
    }
    return false;
}