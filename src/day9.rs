use std::{fs, io::empty};

const FILE_PATH: &str = "./inputs/input09.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    let orig_nums: Vec<u64> = contents.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();

    let mut position: u64 = 0;
    let mut left_idx: usize = 0;
    let mut nums = orig_nums.clone();
    let mut right_idx: usize = nums.len() - 1;
    while left_idx <= right_idx {
        if left_idx % 2 == 0 {
            while nums[left_idx] > 0 {
                part1 += position * (left_idx as u64 / 2);
                position += 1;
                nums[left_idx] -= 1;
            }
        }
        else {
            while nums[left_idx] > 0 {
                if nums[right_idx] > 0  {
                    part1 += position * (right_idx as u64 / 2);
                    nums[left_idx] -= 1;
                    nums[right_idx] -= 1;
                    position += 1;
                }
                else {
                    if right_idx - 2 <= left_idx {
                        break;
                    }
                    right_idx -= 2;
                }
            }
        }
        left_idx += 1;
    }

    let mut nums: Vec<(u64, u64)> = orig_nums.iter().enumerate().map(|(idx, x)| (*x, if idx % 2 == 0 {(idx / 2) as u64} else {0})).collect();
    let mut move_file_id = ((nums.len() - 1) / 2) as u64;
    let mut move_file_idx = nums.len() - 1;
    while move_file_id > 0 {
        if nums[move_file_idx].1 != move_file_id {
            move_file_idx -= 2;
            continue;
        }
        let mut empty_index = 1;
        while empty_index < move_file_idx && nums[empty_index].0 < nums[move_file_idx].0 {
            empty_index += 2;
        }

        if empty_index < move_file_idx {
            let move_num = nums[move_file_idx];
            nums.remove(move_file_idx);
            if nums.len() >= move_file_idx + 1 {
                nums[move_file_idx - 1].0 += nums[move_file_idx].0;
                nums.remove(move_file_idx);
            }
            nums[move_file_idx - 1].0 += move_num.0;
            nums[empty_index].0 -= move_num.0;
            nums.insert(empty_index, move_num);
            nums.insert(empty_index, (0, 0));
        }
        else {
            move_file_idx -= 2;
        }
        move_file_id -= 1;
    }

    let mut position = 0;
    for (i, (num, id)) in nums.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*num {
                part2 += position * id; 
                position += 1;
            }
        }
        else {
            position += num;
        }
    }
    println!("Part 1: {part1}, Part 2: {part2}");
}