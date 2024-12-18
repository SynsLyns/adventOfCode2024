use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input17.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\r\n\r\n").collect();

    let mut registers = contents[0].lines();
    let program: Vec<i64> = contents[1].split(": ").nth(1).unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    let now = Instant::now();

    let a = registers.next().unwrap().split(": ").nth(1).unwrap().parse::<i64>().unwrap();
    let b = registers.next().unwrap().split(": ").nth(1).unwrap().parse::<i64>().unwrap();
    let c = registers.next().unwrap().split(": ").nth(1).unwrap().parse::<i64>().unwrap();

    let mut combo_op: [i64; 7] = [0, 1, 2, 3, a, b, c];
    
    let mut ip = 0;
    let mut output: Vec<i64> = vec![];
    while ip < program.len() {
        match program[ip] {
            0 => {
                combo_op[4] = combo_op[4] / 2i64.pow(combo_op[program[ip + 1] as usize] as u32);
            },
            1 => {
                combo_op[5] ^= program[ip + 1];
            },
            2 => {
                combo_op[5] = combo_op[program[ip + 1] as usize] % 8;
            },
            3 => {
                if combo_op[4] != 0 {
                    ip = program[ip + 1] as usize;
                    continue;
                }
            },
            4 => {
                combo_op[5] = combo_op[5] ^ combo_op[6];
            },
            5 => {
                output.push(combo_op[program[ip + 1] as usize] % 8);
            },
            6 => {
                combo_op[5] = combo_op[4] / 2i64.pow(combo_op[program[ip + 1] as usize] as u32);
            },
            7 => {
                combo_op[6] = combo_op[4] / 2i64.pow(combo_op[program[ip + 1] as usize] as u32);
            },
            _ => {}
        }
        ip += 2;
    }
    let elapsed = now.elapsed();
    println!("Part 1 took {:.2?}", elapsed);
    println!("Part 1: {}", output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));

    let now = Instant::now();
    let mut rev_program = program.clone();
    rev_program.reverse();
    let part2 = get_min(0, 0, &rev_program);
    let elapsed = now.elapsed();
    println!("Part 2 took {:.2?}", elapsed);
    println!("Part 2: {}", part2);
}

fn get_min(index: usize, a: i64, program: &Vec<i64>) -> i64 {
    if index >= program.len() {
        return a;
    }
    let mut min = i64::MAX;
    for i in 0..8 {
        let temp = a * 8 + i;
        let value = ((((temp % 8) ^ 5) ^ (temp >> ((temp % 8) ^ 5))) ^ 6) % 8;
        if value == program[index] {
            let new_a = get_min(index + 1, temp, program);
            if new_a < min {
                min = new_a;
            }
        }
    }
    return min;
}