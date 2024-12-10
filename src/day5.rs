use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "./inputs/input05.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;

    let mut reading_rules = true;
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        if line == "" {
            reading_rules = false;
            continue;
        }
        if reading_rules {
            let mut line = line.trim().split('|');
            let k = line.next().unwrap();
            match rules.get_mut(k) {
                Some(x) => x.push(line.next().unwrap()),
                None => {rules.insert(k, vec![line.next().unwrap()]);}
            }
            
        }
        else {
            let updates: Vec<&str> = line.split(',').collect();
            let mut new_updates = updates.clone();
            new_updates.sort_by(|a, b| {
                match rules.get(a) {
                    Some(x) => if x.contains(b) {return Ordering::Less},
                    None => {}
                }
                match rules.get(b) {
                    Some(x) => if x.contains(a) {return Ordering::Greater},
                    None => {}
                }
                return Ordering::Equal;
            });
            part1 += if updates.iter().zip(new_updates.iter()).all(|(a,b)| a == b) {updates[updates.len()/2].parse().unwrap()} else {0};
            part2 += if updates.iter().zip(new_updates.iter()).all(|(a,b)| a == b) {0} else {new_updates[new_updates.len()/2].parse().unwrap()};
        }
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}
