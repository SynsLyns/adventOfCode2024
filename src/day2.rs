use std::fs;

const FILE_PATH: &str = "./inputs/input02.txt";

pub fn solve() {
    let reports = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    
    let mut safe_reports = reports.lines().count();
    for report in reports.lines() {
        safe_reports -= 1 - (is_report_safe(report.trim().split_whitespace().collect()) as usize);
    }

    let mut safe_reports2 = reports.lines().count();
    for report in reports.lines() {
        let mut report = report.trim().split_whitespace();
        let mut x: Vec<&str> = report.clone().collect();
        x.remove(1);
        if is_report_safe(x){
            continue;
        }
        let mut last: i32 = report.next().unwrap().parse().unwrap();
        if is_report_safe(report.clone().collect()) {
            continue;
        }
        let mut increasing = 0;
        let mut can_tolerate = 1;
        while let Some(n) = report.next() {
            let n: i32 = n.parse().unwrap();

            if increasing == 0 {
                let diff_to_last = (n - last).abs();
                if diff_to_last < 1 || diff_to_last > 3 {
                    if can_tolerate == 1 {
                        can_tolerate -= 1;
                        continue;
                    }
                    safe_reports2 -= 1;
                    break;
                }
                if n - last > 0 {
                    increasing = 1;
                }
                else {
                    increasing = -1;
                }
            }
            
            let diff_to_last = (n - last)  * increasing;
            if diff_to_last < 1 || diff_to_last > 3 {
                if can_tolerate == 1 {
                    can_tolerate -= 1;
                    continue;
                }
                safe_reports2 -= 1;
                break;
            }
            last = n;
        }
    }

    println!("Part 1: {safe_reports}, Part 2: {safe_reports2}");
}


fn is_report_safe(report: Vec<&str>) -> bool {
    let mut report = report.iter();
    let mut last: i32 = report.next().unwrap().parse().unwrap();
    let mut increasing = 0;
    while let Some(n) = report.next() {
        let n: i32 = n.parse().unwrap();

        if increasing == 0 {
            let diff_to_last = (n - last).abs();
            if diff_to_last < 1 || diff_to_last > 3 {
                return false;
            }
            if n - last > 0 {
                increasing = 1;
            }
            else {
                increasing = -1;
            }
        }
        
        let diff_to_last = (n - last)  * increasing;
        if diff_to_last < 1 || diff_to_last > 3 {
            return false;
        }
        last = n;
    }
    return true;
}