use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use regex::Regex;

const FILE_PATH: &str = "./inputs/input21.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;

    let directions: HashMap<(i32, i32), usize> =
        [((-1, 0), 0), ((1, 0), 3), ((0, -1), 2), ((0, 1), 4)]
            .iter()
            .cloned()
            .collect();

    let numeric_positions = vec![
        (3, 1),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (0, 0),
        (0, 1),
        (0, 2),
        (3, 2),
    ];

    let directional_positions = vec![(0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];

    let numeric_keypad = vec![
        vec![2, 10],
        vec![2, 4],
        vec![0, 1, 3, 5],
        vec![2, 6, 10],
        vec![1, 5, 7],
        vec![2, 4, 6, 8],
        vec![3, 5, 9],
        vec![4, 8],
        vec![5, 7, 9],
        vec![6, 8],
        vec![0, 3],
    ];

    let directional_keypad = vec![vec![1, 3], vec![0, 4], vec![3], vec![0, 2, 4], vec![1, 3]];

    let mut numeric_dist: Vec<Vec<usize>> =
        vec![vec![usize::MAX; numeric_keypad.len()]; numeric_keypad.len()];
    let mut numeric_paths: Vec<Vec<Vec<Vec<usize>>>> =
        vec![vec![vec![]; numeric_keypad.len()]; numeric_keypad.len()];

    get_all_shortest_paths(
        &mut numeric_dist,
        &mut numeric_paths,
        &numeric_positions,
        &numeric_keypad,
        &directions,
    );

    let mut directional_dist: Vec<Vec<usize>> =
        vec![vec![usize::MAX; directional_keypad.len()]; directional_keypad.len()];
    let mut directional_paths: Vec<Vec<Vec<Vec<usize>>>> =
        vec![vec![vec![]; directional_keypad.len()]; directional_keypad.len()];

    get_all_shortest_paths(
        &mut directional_dist,
        &mut directional_paths,
        &directional_positions,
        &directional_keypad,
        &directions,
    );

    for paths in [&mut numeric_paths, &mut directional_paths] {
        for i in paths.iter_mut() {
            for j in i {
                j.retain(|k| {
                    let first = k[0];
                    let mut index = 0;
                    while index < k.len() && k[index] == first {
                        index += 1;
                    }
                    while index < k.len() && k[index] != first {
                        index += 1;
                    }
                    if index < k.len() {
                        return false;
                    }
                    true
                });
                for k in j.iter_mut() {
                    k.push(1)
                }
                if j.is_empty() {
                    j.push(vec![1]);
                }
            }
        }
    }

    let now = Instant::now();

    for line in contents.lines() {
        let num: usize = re.find(line).unwrap().as_str().parse().unwrap();
        let min_seq_len = get_shortest_sequence_len(line, &numeric_paths, &directional_paths);
        part1 += num * min_seq_len.0;
        part2 += num * min_seq_len.1;
    }

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1} Part 2: {part2}");
}

fn get_shortest_sequence_len(
    code: &str,
    numeric_paths: &Vec<Vec<Vec<Vec<usize>>>>,
    directional_paths: &Vec<Vec<Vec<Vec<usize>>>>,
) -> (usize, usize) {
    let code: Vec<usize> = code
        .chars()
        .map(|x| {
            if x != 'A' {
                x.to_digit(10).unwrap() as usize
            } else {
                10
            }
        })
        .collect();
    let mut robot_pos_1 = 10;
    let mut robot_instructions_1 = vec![];

    get_next_instructions(
        &code,
        &mut robot_instructions_1,
        numeric_paths,
        &mut robot_pos_1,
    );

    let mut cache: HashMap<(&Vec<usize>, usize), usize> = HashMap::new();
    let min1 = robot_instructions_1
        .iter()
        .map(|i| recurse(i, 2, 1, &directional_paths, &mut cache))
        .min()
        .unwrap();

    let min2 = robot_instructions_1
        .iter()
        .map(|i| recurse(i, 25, 1, &directional_paths, &mut cache))
        .min()
        .unwrap();

    return (min1, min2);
}

fn recurse<'a>(
    code: &'a Vec<usize>,
    depth: usize,
    robot_pos: usize,
    paths: &'a Vec<Vec<Vec<Vec<usize>>>>,
    cache: &mut HashMap<(&'a Vec<usize>, usize), usize>,
) -> usize {
    if depth == 0 {
        return code.len();
    }
    if let Some(&x) = cache.get(&(code, depth)) {
        return x;
    }
    let mut r_pos = 1;
    let mut tot = 0;
    for &c in code {
        let mut min = usize::MAX;
        for i in &paths[r_pos][c] {
            let r = recurse(i, depth - 1, robot_pos, paths, cache);
            if r < min {
                min = r;
            }
        }
        tot += min;
        r_pos = c;
    }
    cache.insert((code, depth), tot);
    return tot;
}

fn get_next_instructions(
    code: &Vec<usize>,
    next_robot_instructions: &mut Vec<Vec<usize>>,
    paths: &Vec<Vec<Vec<Vec<usize>>>>,
    robot_pos: &mut usize,
) {
    for &c in code {
        if next_robot_instructions.is_empty() {
            *next_robot_instructions = paths[*robot_pos][c].clone();
        } else {
            *next_robot_instructions = next_robot_instructions
                .iter()
                .flat_map(|i| {
                    paths[*robot_pos][c].iter().map(|j| {
                        let mut new_path = Vec::with_capacity(i.len() + j.len());
                        new_path.extend_from_slice(i);
                        new_path.extend_from_slice(j);
                        new_path
                    })
                })
                .collect();
        }
        *robot_pos = c;
    }
}

fn get_all_shortest_paths(
    dist: &mut Vec<Vec<usize>>,
    paths: &mut Vec<Vec<Vec<Vec<usize>>>>,
    positions: &Vec<(i32, i32)>,
    keypad: &Vec<Vec<usize>>,
    directions: &HashMap<(i32, i32), usize>,
) {
    let buttons = keypad.len();
    for button in 0..buttons {
        dist[button][button] = 0;
        for &neighbour in &keypad[button] {
            dist[button][neighbour] = 1;
            let row_diff = positions[neighbour].0 - positions[button].0;
            let col_diff = positions[neighbour].1 - positions[button].1;
            let direction = directions[&(row_diff, col_diff)];
            paths[button][neighbour].push(vec![direction]);
        }
    }

    for k in 0..buttons {
        for i in 0..buttons {
            for j in 0..buttons {
                if dist[i][j] > dist[i][k] + dist[k][j]
                    && dist[k][j] != usize::MAX
                    && dist[i][k] != usize::MAX
                {
                    dist[i][j] = dist[i][k] + dist[k][j];
                    let mut new_paths = vec![];
                    for next_step in &paths[k][j] {
                        for path in &paths[i][k] {
                            let mut new_path = path.clone();
                            new_path.extend(next_step);
                            new_paths.push(new_path);
                        }
                    }
                    paths[i][j] = new_paths.clone();
                } else if dist[i][j] == dist[i][k] + dist[k][j]
                    && dist[k][j] != usize::MAX
                    && dist[i][k] != usize::MAX
                {
                    let mut new_paths = vec![];
                    for next_step in &paths[k][j] {
                        for path in &paths[i][k] {
                            let mut new_path = path.clone();
                            new_path.extend(next_step);
                            new_paths.push(new_path);
                        }
                    }
                    paths[i][j].append(&mut new_paths);
                }
            }
        }
    }
}
