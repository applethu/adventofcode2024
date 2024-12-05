#[allow(dead_code)]
pub fn run() {
    let input = std::fs::read_to_string("src/day5/input.txt").unwrap();

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut process_rules = true;

    input.lines().for_each(|line| {
        if line.is_empty() {
            process_rules = false;
            return;
        }
        if process_rules {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {

                if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    rules.push((a, b));
                }
            }
        } else {
            let parts: Vec<i32> = line.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            updates.push(parts);
        }
    });

    let mut sum = 0;
    let mut sumfixed = 0; // PART2

    let mut valid_updates: Vec<Vec<i32>> = Vec::new();

    updates.iter().for_each(|line| {
        if check_rules(&rules, line) {
            valid_updates.push(line.clone());
            let mid_index = line.len() / 2;
            sum += line[mid_index];
        } else {
            // PART 2
            let mut line_clone = line.clone();
            while !check_rules(&rules, line_clone.as_ref()) {
                for (a, b) in &rules {
                    let (pos_a, pos_b) = find_index(&line_clone, &a, &b);
                    if let (Some(pos_a), Some(pos_b)) = (pos_a, pos_b) {
                        if pos_a >= pos_b {
                            line_clone.swap(pos_a, pos_b);
                        }
                    }
                }
            }
            let mid_index = line_clone.len() / 2;
            sumfixed += line_clone[mid_index];
        }
    });
    println!("Sum: {}", sum);
    println!("SumFixed: {}", sumfixed);
}

#[allow(dead_code)]
fn check_rules(rules: &Vec<(i32, i32)>, line: &Vec<i32>) -> bool {
    for (a, b) in rules {
        let (pos_a, pos_b) = find_index(line, a, b);
        if let (Some(pos_a), Some(pos_b)) = (pos_a, pos_b) {
            if pos_a >= pos_b {
                return false;
            }
        }
    }
    true
}

fn find_index(line: &Vec<i32>, a: &i32, b: &i32) -> (Option<usize>, Option<usize>) {
    let pos_a = line.iter().position(|&x| x == *a);
    let pos_b = line.iter().position(|&x| x == *b);
    (pos_a, pos_b)
}



