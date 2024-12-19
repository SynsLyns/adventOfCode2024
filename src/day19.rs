use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input19.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\r\n\r\n").collect();
    
    let patterns: HashSet<&str> = contents[0].split(", ").collect();
    let mut cache: HashMap<String, u64> = patterns.iter().map(|s| (s.to_string(), 1)).collect();

    let designs = contents[1];

    let now = Instant::now();
    let mut part1 = 0;
    let mut part2 = 0;
    for design in designs.lines() {
        let ways = design_possible_2(design, &patterns, &mut cache);
        part1 += if ways > 0 {1} else {0};
        part2 += ways;
    }

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1}, Part 2: {part2}");
}

fn design_possible_2(design: &str, patterns: &HashSet<&str>, cache: &mut HashMap<String, u64>) -> u64 {
    let mut count = 0;
    if let Some(x) = cache.get(design) {
        if patterns.contains(design) {
            count += 1;
        }
        else {
            return *x;
        }
    }
    for i in 1..design.len() {
        if patterns.contains(&design[0..i]) {
            count += design_possible_2(&design[i..], patterns, cache);
        }
    }
    cache.insert(design.to_string(), count);
    return count;
}