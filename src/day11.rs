use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "./inputs/input11.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;

    let stones: Vec<u64> = contents.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut stone_map: HashMap<u64, u64> = HashMap::new();

    for stone in &stones {
        match stone_map.get_mut(stone) {
            Some(x) => *x += 1,
            None => {stone_map.insert(*stone, 1);}
        }
    }

    for _ in 0..25 {
        stone_map = blink(stone_map);
    }

    for (_, amount) in &stone_map {
        part1 += amount;
    }

    for _ in 0..50 {
        stone_map = blink(stone_map);
    }

    for (_, amount) in &stone_map {
        part2 += amount;
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}

fn blink(stone_map: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stone_map: HashMap<u64, u64> = HashMap::new();
    for (num, amount) in stone_map {
        if num == 0 {
            match new_stone_map.get_mut(&(1 as u64)) {
                Some(x) => *x += amount,
                None => {new_stone_map.insert(1, amount);}
            }
        }
        else if (num.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
            let num_string = num.to_string();
            let first_num: u64 = num_string[..num_string.len()/2].parse().unwrap();
            let second_num: u64 = num_string[num_string.len()/2..].parse().unwrap();
            match new_stone_map.get_mut(&first_num) {
                Some(x) => *x += amount,
                None => {new_stone_map.insert(first_num, amount);}
            }
            match new_stone_map.get_mut(&second_num) {
                Some(x) => *x += amount,
                None => {new_stone_map.insert(second_num, amount);}
            }
        }
        else {
            match new_stone_map.get_mut(&(num * 2024)) {
                Some(x) => *x += amount,
                None => {new_stone_map.insert(num * 2024, amount);}
            }
        }
    }
    return new_stone_map;
}