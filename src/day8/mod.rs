use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = std::fs::read_to_string("src/day8/input.txt").unwrap();
    let input2: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut distances: HashMap<(i64, i64), HashMap<char, Vec<(f64, (i64, i64))>>> = HashMap::new();
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for i in 0..input2.len() {
        for j in 0..input2[i].len() {
            for k in 0..input2.len() {
                for l in 0..input2[i].len() {
                    if i ==  k && j == l {
                        continue;
                    }
                    if input2[k][l] == '.' {
                        continue;
                    }

                    let inner_map = distances.entry((i as i64, j as i64)).or_insert_with(HashMap::new);
                    let vec = inner_map.entry(input2[k][l]).or_insert_with(Vec::new);
                    let dstnc = ((k as f64 - i as f64).powi(2) + (l as f64 - j as f64).powi(2)).sqrt();
                    vec.push((dstnc, (k as i64, l as i64)));
                }
            }
        }
    }
    for i in 0..input2.len() {
        for j in 0..input2[i].len() {
            let distancemap = distances.get(&(i as i64, j as i64)).unwrap();
            let epsilon = 1e-6;
            distancemap.iter().for_each(|(_key, value)| {
                for m in 0..value.len() {
                    for n in m+1..value.len() {
                        if (value[m].0*2.0 - value[n].0).abs() < epsilon || (value[n].0*2.0 - value[m].0).abs() < epsilon{
                            let c1 = value[m].1;
                            let c2 = value[n].1;
                            let c3 = (i as i64, j as i64);
                            let are_inline = ((c1.0 - c3.0) * (c2.1 - c3.1) - (c1.1 - c3.1) * (c2.0 - c3.0)) == 0;
                            if are_inline  {
                                antinodes.insert((i as i64, j as i64));
                                break;
                            }
                        }
                    }
                }
            });
        }
    }

    println!("ANTINODES: {}", antinodes.len());
}