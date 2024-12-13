use regex::Regex;
use std::fs;

const FILE_PATH: &str = "./inputs/input13.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\r\n\r\n").collect();

    let mut part1 = 0;
    let mut part2 = 0;

    let re = Regex::new(r"[0-9]+").unwrap();

    for game in contents {
        let mut lines = game.lines();
        let mut button_a = re.find_iter(lines.next().unwrap());
        let mut button_b = re.find_iter(lines.next().unwrap());
        let mut prize = re.find_iter(lines.next().unwrap());
        let (a_x, a_y, b_x, b_y, mut p_x, mut p_y): (i64, i64, i64, i64, i64, i64) = (
            button_a.next().unwrap().as_str().parse().unwrap(),
            button_a.next().unwrap().as_str().parse().unwrap(),
            button_b.next().unwrap().as_str().parse().unwrap(),
            button_b.next().unwrap().as_str().parse().unwrap(),
            prize.next().unwrap().as_str().parse().unwrap(),
            prize.next().unwrap().as_str().parse().unwrap()
        );
        let det = a_x*b_y - b_x*a_y;

        let a = (b_y*p_x - b_x*p_y) / det;
        let b = (-a_y*p_x + a_x*p_y) / det;
        
        if a >= 0 && b >= 0 && a_x*a + b_x*b == p_x && a_y*a + b_y*b == p_y {
            part1 += 3*a + b;
        }
        p_x += 10000000000000;
        p_y += 10000000000000;
        let a = (b_y*p_x - b_x*p_y) / det;
        let b = (-a_y*p_x + a_x*p_y) / det;
        if a_x*a + b_x*b == p_x && a_y*a + b_y*b == p_y {
            part2 += 3*a + b;
        }
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}