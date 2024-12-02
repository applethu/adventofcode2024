pub fn run() {
    let input = std::fs::read_to_string("src/day2/input.txt").unwrap();

    let count_valid = input.lines().filter(|line| {
        let parts: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        if parts.len() < 2 {
            return true;
        }

        //return validate_part1(&parts);
        return validate_part2(&parts);
    }).count();
    println!("{}", count_valid);
}

fn is_safe(a: i32, b: i32, increasing: bool) -> bool {
    a != b && a.abs_diff(b) <= 3 && (a < b) == increasing
}

fn validate_part1(parts: &Vec<i32>) -> bool {
    let increasing = parts[0] < parts[1];
    parts.windows(2).all(|w| is_safe(w[0], w[1], increasing))
}

fn validate_part2(parts: &Vec<i32>) -> bool {
    if validate_part1(&parts) {
        return true
    }
    for i in 0..parts.len() {
        let mut temp_parts = parts.clone();
        temp_parts.remove(i);
        if validate_part1(&temp_parts) {
            return true;
        }
    }
    false
}
