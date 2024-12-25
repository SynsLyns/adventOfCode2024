use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input24.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let (inputs, gates_input) = contents.split_once("\r\n\r\n").unwrap();

    let mut wire_map: HashMap<usize, bool> = HashMap::new();

    for i in inputs.lines() {
        let (k, v) = i.split_once(": ").unwrap();
        wire_map.insert(get_hash(k), if v == "1" { true } else { false });
    }

    let mut gates_map: HashMap<usize, (usize, &str, usize)> = HashMap::new();
    for g in gates_input.lines() {
        let (i, out) = g.split_once(" -> ").unwrap();
        let mut ops = i.split_whitespace();
        let (i1, op, i2) = (
            ops.next().unwrap(),
            ops.next().unwrap(),
            ops.next().unwrap(),
        );
        gates_map.insert(get_hash(out), (get_hash(i1), op, get_hash(i2)));
    }

    let base = get_hash("z00");
    let step = get_hash("z01") - get_hash("z00");
    let step2 = get_hash("z10") - get_hash("z00");

    let now = Instant::now();

    let mut part1 = 0;
    let mut dependencies = vec![];
    for z in 0..=45 {
        let h = base + (step2 * (z / 10)) + (step * (z % 10));
        let mut temp = vec![];
        if get_value(h, &mut wire_map, &gates_map, &mut temp) {
            part1 += 2usize.pow(z as u32);
        }
        dependencies.push(temp);
    }
    
    let mut part2 = vec![];
    for (&out, &(i1, op, i2)) in gates_map.iter() {
        if op == "XOR" {
            if !(hash_to_string(out).starts_with("z") || hash_to_string(i1).starts_with("x") || hash_to_string(i1).starts_with("y")) {
                part2.push(hash_to_string(out));
            }
            else {
                for (_, &(j1, jop, j2)) in gates_map.iter() {
                    if jop == "OR" && (out == j1 || out == j2) {
                        part2.push(hash_to_string(out));
                        break;
                    }
                }
            }
        }
        else if op == "AND" {
            if hash_to_string(out).starts_with("z") {
                part2.push(hash_to_string(out));
            }
            else if !(hash_to_string(i1) == "x00" || hash_to_string(i1) == "y00") {
                for (_, &(j1, jop, j2)) in gates_map.iter() {
                    if jop != "OR" && (out == j1 || out == j2) {
                        part2.push(hash_to_string(out));
                        break;
                    }
                }
            }
        }
        else if op == "OR" {
            if hash_to_string(out).starts_with("z") && out != get_hash("z45") {
                part2.push(hash_to_string(out));
            }
        }
    }
    part2.sort();

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1} Part 2: {}", part2.join(","));
}

fn get_value(
    out: usize,
    wire_map: &mut HashMap<usize, bool>,
    gates_map: &HashMap<usize, (usize, &str, usize)>,
    dependencies: &mut Vec<(usize, usize, String, usize)>,
) -> bool {
    let (i1, op, i2) = gates_map[&out];
    let x1 = wire_map
        .get(&i1)
        .copied()
        .unwrap_or_else(|| get_value(i1, wire_map, gates_map, dependencies));
    let x2 = wire_map
        .get(&i2)
        .copied()
        .unwrap_or_else(|| get_value(i2, wire_map, gates_map, dependencies));
    let result = match op {
        "AND" => x1 & x2,
        "OR" => x1 | x2,
        "XOR" => x1 ^ x2,
        _ => false,
    };
    wire_map.insert(out, result);
    dependencies.push((i1, i2, op.to_string(), out));
    return result;
}

fn get_hash(s: &str) -> usize {
    let mut hash = 0;
    for (i, c) in s.chars().enumerate() {
        let value = match c {
            '0'..='9' => c as usize - '0' as usize,
            'a'..='z' => c as usize - 'a' as usize + 10,
            _ => 0,
        };
        hash += value * 36usize.pow(i as u32);
    }
    hash
}

fn hash_to_string(mut hash: usize) -> String {
    let mut s = String::new();
    for _ in 0..3 {
        let value = hash % 36;
        let c = if value < 10 {
            (value as u8 + b'0') as char
        } else {
            (value as u8 - 10 + b'a') as char
        };
        s.push(c);
        hash /= 36;
    }
    s
}
