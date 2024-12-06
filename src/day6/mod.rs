pub fn run() {
    let input = std::fs::read_to_string("src/day6/input.txt").unwrap();
    let input2: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum_infinite_count = 0;
    for i in 0..input2.len() {
        for j in 0..input2[i].len() {
            let mut input3 = input2.clone();
            if input3[i][j] == '.' {
                input3[i][j] = '#';
                if mainfunction(&mut input3) {
                    sum_infinite_count += 1;
                }
            }
        }
    }
    print!("SUM INFINITE COUNT: {}", sum_infinite_count);
}

fn mainfunction(mut input2: &mut Vec<Vec<char>>) -> bool {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_dir_idx = 0;

    let dimensions = (input2.len() as i32, input2[0].len() as i32);
    let mut current_pos = find_start(&input2).unwrap();
    let mut sumsteps = 0;
    while is_inside_range(dimensions, current_pos) && sumsteps < dimensions.0 * dimensions.1 {
        sumsteps += 1;
        mark_pos_at(&mut input2, current_pos.0, current_pos.1);

        let mut dirchange = 0;
        while dirchange < 4 {
            let next_pos = (
                current_pos.0 + directions[current_dir_idx].0,
                current_pos.1 + directions[current_dir_idx].1,
            );
            if is_blocked(&input2, next_pos.0, next_pos.1) {
                current_dir_idx = (current_dir_idx + 1) % 4;
                dirchange += 1;
            } else {
                current_pos = next_pos;
                break;
            }
        }
    }
    //PART 1
    //let sum = input2.iter().flatten().filter(|&&c| c == 'X').count();
    //println!("SUM: {}", sum);
    sumsteps == dimensions.0 * dimensions.1
}

fn is_blocked(input: &[Vec<char>], row: i32, col: i32) -> bool {
    let dimensions = (input.len() as i32, input[0].len() as i32);
    is_inside_range(dimensions, (row, col)) && input[row as usize][col as usize] == '#'
}

fn mark_pos_at(input: &mut [Vec<char>], row: i32, col: i32) {
    input[row as usize][col as usize] = 'X';
}

fn find_start(input: &[Vec<char>]) -> Option<(i32, i32)> {
    input.iter().enumerate().find_map(|(row, line)| {
        line.iter().position(|&c| c == '^').map(|col| (row as i32, col as i32))
    })
}

fn is_inside_range(dimensions: (i32, i32), pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < dimensions.0 && pos.1 >= 0 && pos.1 < dimensions.1
}