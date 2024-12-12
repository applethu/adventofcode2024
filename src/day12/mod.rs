use std::collections::VecDeque;

pub fn run() {
    let input = std::fs::read_to_string("src/day12/input.txt").unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut price = 0;

    let (sizes, perimeters) = find_areas(&matrix);

    for (_i, (size, perimeter)) in sizes.iter().zip(perimeters.iter()).enumerate() {
        price += size * perimeter;
    }
    println!("Price: {}", price);
}

fn find_areas(matrix: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut sizes = Vec::new();
    let mut perimeters = Vec::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if !visited[i][j] && matrix[i][j] != ' ' {
                let (size, perimeter) = flood_fill(matrix, &mut visited, i, j);
                sizes.push(size);
                perimeters.push(perimeter);
            }
        }
    }

    (sizes, perimeters)
}

fn flood_fill(matrix: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, start_i: usize, start_j: usize) -> (usize, usize) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back((start_i, start_j));
    visited[start_i][start_j] = true;

    let mut size = 0;
    let mut perimeter = 0;

    while let Some((i, j)) = queue.pop_front() {
        size += 1;
        for &(di, dj) in &directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0 && ni < matrix.len() as isize && nj >= 0 && nj < matrix[0].len() as isize {
                let ni = ni as usize;
                let nj = nj as usize;
                if !visited[ni][nj] && matrix[ni][nj] == matrix[i][j] {
                    visited[ni][nj] = true;
                    queue.push_back((ni, nj));
                } else if matrix[ni][nj] != matrix[i][j] {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (size, perimeter)
}