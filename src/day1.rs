use std::fs;

const FILE_PATH: &str = "./inputs/input01.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    for line in contents.lines() {
        let mut row = line.split_whitespace();
        list1.push(row.next().unwrap().parse().unwrap());
        list2.push(row.next().unwrap().parse().unwrap());
    }

    list1.sort();
    list2.sort();
    let mut total = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        let distance = (a-b).abs();
        total += distance;
    }

    let (mut prev, mut count, mut total2, mut list2index) = (-1, 0, 0, 0);
    for i in list1 {
        if i == prev {
            total2 += count * i;
            continue;
        }
        count = 0;
        prev = i;
        while list2index < list2.len() && list2[list2index] <= i {
            if list2[list2index] == i {
                count += 1;
            }
            list2index += 1;
        }
        total2 += count * i;
    }

    println!("Part 1: {total}, Part 2: {total2}")
}
