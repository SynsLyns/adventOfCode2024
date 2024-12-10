use std::fs;

const FILE_PATH: &str = "./inputs/input07.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2 = 0;

    for line in contents.lines() {
        let mut data = line.trim().split(':');
        let expected_total: usize = data.next().unwrap().parse().unwrap();
        let nums: Vec<usize> = data
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // part1
        let max_op = 2usize.pow((nums.len() - 1) as u32);
        for i in 0..max_op {
            let mut total = nums[0];
            for (index, j) in nums[1..nums.len()].iter().enumerate() {
                if i & 2usize.pow(index as u32) == 0 {
                    total += j;
                } else {
                    total *= j;
                }
            }
            if total == expected_total {
                part1 += total;
                break;
            }
        }
        part2 += if has_solution(&nums[1..nums.len()], nums[0], &expected_total) {
            expected_total
        } else {
            0
        };
    }

    println!("Part 1: {part1}, Part 2: {part2}");
}

fn has_solution(nums: &[usize], current: usize, expected: &usize) -> bool {
    if nums.len() == 0 {
        return current == *expected;
    }
    if has_solution(&nums[1..nums.len()], current + nums[0], &expected)
        || has_solution(&nums[1..nums.len()], current * nums[0], &expected)
        || has_solution(&nums[1..nums.len()], concat(current, nums[0]), &expected)
    {
        return true;
    }
    return false;
}

fn concat(a: usize, b: usize) -> usize {
    a as usize * 10usize.pow(b.ilog10() + 1) + b as usize
}
