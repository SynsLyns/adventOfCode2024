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
            Some(x) => x.push(v),
            None => {
                a_list.insert(u, vec![v]);
            }
        }
        match a_list.get_mut(v) {
            Some(x) => x.push(u),
            None => {
                a_list.insert(v, vec![u]);
            }
        }
    }
    let vertices: Vec<&str> = a_list.keys().map(|&x| x).collect();
    let vertex_indices: HashMap<&str, usize> =
        vertices.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let a_list: Vec<Vec<usize>> = vertices
        .iter()
        .map(|&v| a_list[v].iter().map(|&u| vertex_indices[u]).collect())
        .collect();

    let part1 = find_cliques_3(&a_list, &vertices);
    let mut cliques: Vec<Vec<usize>> = vec![];
    bron_kerbosch(
        &mut vec![],
        &mut (0..vertices.len()).collect(),
        &mut vec![],
        &a_list,
        &mut cliques,
    );
    let max_clique = cliques.iter().max_by_key(|c| c.len()).unwrap();
    let mut part2: Vec<&str> = max_clique.iter().map(|&i| vertices[i]).collect();
    part2.sort();
    let elapsed = now.elapsed();
    println!("Took {:.2?}", elapsed);
    println!("Part 1: {part1} Part 2: {}", part2.join(","));
}

fn find_cliques_3(a_list: &Vec<Vec<usize>>, vertices: &Vec<&str>) -> usize {
    let mut return_value = 0;
    let n = vertices.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if !vertices[i].starts_with('t')
                    && !vertices[j].starts_with('t')
                    && !vertices[k].starts_with('t')
                {
                    continue;
                }
                let a = &a_list[i];
                let b = &a_list[j];
                if a.contains(&j) && a.contains(&k) && b.contains(&k) {
                    return_value += 1;
                }
            }
        }
    }
    return_value
}

fn bron_kerbosch(
    r: &mut Vec<usize>,
    p: &mut Vec<usize>,
    x: &mut Vec<usize>,
    a_list: &Vec<Vec<usize>>,
    cliques: &mut Vec<Vec<usize>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let p_clone = p.clone();
    for &v in p_clone.iter() {
        r.push(v);
        let mut new_p: Vec<usize> = p
            .iter()
            .filter(|&&u| a_list[v].contains(&u))
            .cloned()
            .collect();
        let mut new_x: Vec<usize> = x
            .iter()
            .filter(|&&u| a_list[v].contains(&u))
            .cloned()
            .collect();
        bron_kerbosch(r, &mut new_p, &mut new_x, a_list, cliques);
        r.pop();
        p.retain(|&u| u != v);
        x.push(v);
    }
}
