use regex::Regex;
use std::fs;

const FILE_PATH: &str = "./inputs/input14.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let re = Regex::new(r"-?[0-9]+").unwrap();

    let (rows, cols, s) = (103, 101, 100);
    let mut robot_v: Vec<(i64, i64)> = vec![];
    let mut robot_p: Vec<(i64, i64)> = vec![];

    let (mut one, mut two, mut three, mut four) = (0, 0, 0, 0);

    for line in contents.lines() {
        let mut nums = re.find_iter(line);
        let (p_x, p_y, v_x, v_y): (i64, i64, i64, i64) = (
            nums.next().unwrap().as_str().parse().unwrap(),
            nums.next().unwrap().as_str().parse().unwrap(),
            nums.next().unwrap().as_str().parse().unwrap(),
            nums.next().unwrap().as_str().parse().unwrap(),
        );
        robot_v.push((v_x, v_y));
        robot_p.push((p_x, p_y));
        let robot_final_x = modulo(p_x + v_x * s, cols);
        let robot_final_y = modulo(p_y + v_y * s, rows);
        if robot_final_x < cols / 2 && robot_final_y < rows / 2 {
            one += 1;
        }
        else if robot_final_x > cols / 2 && robot_final_y < rows / 2 {
            two += 1;
        }
        else if robot_final_x < cols / 2 && robot_final_y > rows / 2 {
            three += 1;
        }
        else if robot_final_x > cols / 2 && robot_final_y > rows / 2 {
            four += 1
        }
    }
    let part1 = one * two * three * four;

    for i in 0..10000 {
        let mut points: Vec<(i64, i64)> = vec![];
        for ((p_x, p_y), (v_x, v_y)) in robot_p.iter().zip(robot_v.iter()) {
            let robot_final_x = modulo(p_x + v_x * i, cols);
            let robot_final_y = modulo(p_y + v_y * i, rows);
            points.push((robot_final_x, robot_final_y));
        }
        points.sort_by(|a, b| b.1.cmp(&a.1));
        points.sort_by(|a, b| a.0.cmp(&b.0));
        
        for point in &points {
            let mut matches = 0;
            for i in 1..11 {
                if points.contains(&(point.0 + i, point.1)) {
                    matches += 1;
                }
            }
            if matches >= 10 {
                for a in 0..rows {
                    for b in 0..cols {
                        if points.contains(&(b, a)) {
                            print!("x");
                        }
                        else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!("Part 2: {i}");
                break;
            }
        }
    }
    println!("Part 1: {part1}");
}


fn modulo(a: i64, b: i64) -> i64 {
    return ((a % b) + b) % b;
}