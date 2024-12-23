use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input22.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let now = Instant::now();
    let result: Vec<Vec<(usize, i32)>> = contents.lines().map(|x| get_next_secret(x.parse::<usize>().unwrap(), 2000)).collect();
    let part1: usize = result.iter().map(|x| x[1999].0).sum();
    let mut amounts: HashMap<usize, usize> = HashMap::new();
    for s in result {
        let mut seen: HashSet<usize> = HashSet::new();
        for slice in s.windows(4){
            if seen.contains(&get_hash(slice)) {
                continue;
            }
            let bananas = slice[3].0 % 10;
            match amounts.get_mut(&get_hash(slice)) {
                Some(x) => {*x += bananas;},
                None => {amounts.insert(get_hash(slice), bananas);}
            }
            seen.insert(get_hash(slice));
        }
    }
    let part2 = amounts.iter().max_by_key(|x| x.1).unwrap().1;
    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1} Part 2: {part2}");
}

fn get_next_secret(secret: usize, n: usize) -> Vec<(usize, i32)> {
    let mut return_value = vec![];
    let mut s = secret;
    for i in 0..n {
        s = ((s * 64) ^ s) % 16777216;
        s = ((s / 32) ^ s) % 16777216;
        s = ((s * 2048) ^ s) % 16777216;
        if i == 0 {
            return_value.push((s, (s % 10) as i32 - (secret % 10) as i32));
        }
        else {
            return_value.push((s, (s % 10) as i32 - (return_value[i-1].0 % 10) as i32));
        }
    }
    return_value
}

fn get_hash(tuple: &[(usize, i32)]) -> usize {
    return (tuple[0].1 + 9) as usize + (tuple[1].1 + 9) as usize * 19 + (tuple[2].1 + 9) as usize * 19usize.pow(2) + (tuple[3].1 + 9) as usize * 19usize.pow(3);
}