use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input19.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\r\n\r\n").collect();
    
    let patterns: HashSet<&str> = contents[0].split(", ").collect();
    let mut cache: HashMap<&str, u64> = HashMap::from([("", 1)]);

    let designs = contents[1];

    let now = Instant::now();
    let mut part1 = 0;
    let mut part2 = 0;
    for design in designs.lines() {
        let ways = design_possible_2(design, &patterns, &mut cache, patterns.iter().max_by_key(|x| x.len()).unwrap().len());
        part1 += if ways > 0 {1} else {0};
        part2 += ways;
    }

    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1}, Part 2: {part2}");
}

fn design_possible_2<'a>(design: &'a str, patterns: &HashSet<&str>, cache: &mut HashMap<&'a str, u64>, max_len: usize) -> u64 {
    if let Some(&x) = cache.get(design) {
        return x;
    }
    let mut count = 0;
    for i in 1..=design.len().min(max_len) {
        if patterns.contains(&design[..i]) {
            count += design_possible_2(&design[i..], patterns, cache, max_len);
        }
    }
    cache.insert(design, count);
    return count;
}