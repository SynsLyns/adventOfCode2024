use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input25.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let contents = contents.split("\r\n\r\n");

    let now = Instant::now();

    let mut locks = vec![];
    let mut keys = vec![];
    for item in contents {
        let lines: Vec<&str> = item.lines().collect();
        let mut lines = lines[..lines.len()-1].iter();
        let mut pin_heights = [0, 0, 0, 0, 0];
        let is_lock = lines.next().unwrap().starts_with('#');

        while let Some(l) = lines.next() {
            for (i, c) in l.chars().enumerate() {
                pin_heights[i] += if c == '#' {1} else {0};
            }
        }

        match is_lock {
            true => locks.push(pin_heights),
            false => keys.push(pin_heights)
        }
    }

    let mut part1 = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut i = 0;
            while i < 5 && lock[i] + key[i] < 6 {
                i += 1;
            }
            if i == 5 {
                part1 += 1;
            }
        }
    }


    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1}");
}
