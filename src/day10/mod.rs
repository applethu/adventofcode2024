use std::collections::HashSet;

pub fn run() {
    let input = std::fs::read_to_string("src/day10/input.txt").unwrap();
    let input2 = input.lines().collect::<Vec<&str>>();
    let map: Vec<Vec<u64>> = input2
        .iter()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u64).collect())
        .collect();

    let mut trailheads = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                trailheads.push((x, y));
            }
        }
    }

    let mut all_paths = Vec::new();
    for trailhead in trailheads {
        let paths = find_paths(&map, trailhead);
        all_paths.extend(paths);
    }

    let unique_start_end_pairs: HashSet<((usize, usize), (usize, usize))> = all_paths
        .iter()
        .map(|path| (path.first().cloned().unwrap(), path.last().cloned().unwrap()))
        .collect();

    println!("Unique start and end coordinate pairs: {}", unique_start_end_pairs.len());
}

fn find_paths(map: &Vec<Vec<u64>>, start: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut paths = Vec::new();
    let mut current_path = Vec::new();
    dfs(map, start, &mut current_path, &mut paths);
    paths.into_iter().filter(|path| path.len() == 10).collect()
}

fn dfs(map: &Vec<Vec<u64>>, (x, y): (usize, usize), current_path: &mut Vec<(usize, usize)>, paths: &mut Vec<Vec<(usize, usize)>>) {
    current_path.push((x, y));
    let current_value = map[y][x];

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for &(dx, dy) in &directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0 && new_x < map[0].len() as isize && new_y >= 0 && new_y < map.len() as isize {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if map[new_y][new_x] == current_value + 1 {
                dfs(map, (new_x, new_y), current_path, paths);
            }
        }
    }

    if current_path.len() > 1 {
        paths.push(current_path.clone());
    }
    current_path.pop();
}