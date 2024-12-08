use std::fs;
use regex::Regex;

const FILE_PATH: &str = "./inputs/input03.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))").unwrap();
    let re2 = Regex::new(r"[0-9]{1,3}").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut d = true;
    for m in re.find_iter(&contents) {
        if m.as_str() == "do()" {
            d = true;
            continue;
        }
        else if m.as_str() == "don't()" {
            d = false;
            continue;
        }
        let mut nums = re2.find_iter(m.as_str());
        let n1: i32 = nums.next().unwrap().as_str().parse().unwrap();
        let n2: i32 = nums.next().unwrap().as_str().parse().unwrap();
        part1 += n1 * n2;
        part2 += if d { n1 * n2 } else { 0 };
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}