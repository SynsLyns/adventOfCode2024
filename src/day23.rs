use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILE_PATH: &str = "./inputs/input23.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    
    let now = Instant::now();
    let mut a_list: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let (u, v) = line.split_once('-').unwrap();
        match a_list.get_mut(u) {
            Some(x) => {x.push(v)},
            None => {a_list.insert(u, vec![v]);}
        }
        match a_list.get_mut(v) {
            Some(x) => {x.push(u)},
            None => {a_list.insert(v, vec![u]);}
        }
    }
    let v: Vec<&str> = a_list.keys().map(|&x| x).collect();
    let part1 = find_cliques_3(&a_list, &v);
    let max_clique = find_max_clique(0, &a_list, &v, &mut vec![]);
    let mut part2: Vec<&str> = max_clique.iter().map(|x| v[*x]).collect();
    part2.sort();
    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1} Part 2: {}", part2.join(","));
}

fn find_cliques_3(a_list: &HashMap<&str, Vec<&str>>, v: &Vec<&str>) -> usize {
    let mut return_value = 0;
    let n = v.len();
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if !v[i].starts_with('t') && !v[j].starts_with('t') && !v[k].starts_with('t') {
                    continue;
                }
                let a = &a_list[v[i]];
                let b = &a_list[v[j]];
                if a.contains(&v[j]) && a.contains(&v[k]) && b.contains(&v[k]) {
                    return_value += 1;
                }
            }
        }
    }
    return_value
}

fn find_max_clique(i: usize, a_list: &HashMap<&str, Vec<&str>>, v: &Vec<&str>, c: &mut Vec<usize>) -> Vec<usize> {
    let mut m_clique = vec![];

    for j in i..v.len() {
        c.push(j);
        if is_clique(a_list, v, c) {
            let max_clique = find_max_clique(j + 1, a_list, v, c);
            if m_clique.len() < max_clique.len() {
                m_clique = max_clique;
            }
            if m_clique.len() < c.len() {
                m_clique = c.clone();
            }
        }
        c.pop();
    }
    return m_clique;
}

fn is_clique(a_list: &HashMap<&str, Vec<&str>>, v: &Vec<&str>, c: &Vec<usize>) -> bool {
    for i in 0..c.len() {
        for j in i+1..c.len() {
            if !a_list[v[c[i]]].contains(&v[c[j]]) {
                return false;
            }
        }
    }
    return true;
}