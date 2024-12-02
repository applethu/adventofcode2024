
#[allow(dead_code)]
pub(crate) fn run() {
    let input = std::fs::read_to_string("src/day1/input.txt").unwrap();

    let (mut left_values, mut right_values) = (Vec::new(), Vec::new());

    input.lines()
        .filter_map(|line| {
            let parts: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            if parts.len() == 2 {
                Some((parts[0], parts[1]))
            } else {
                None
            }
        })
        .for_each(|(left, right)| {
            left_values.push(left);
            right_values.push(right);
        });

    left_values.sort();
    right_values.sort();

    //let number: u32 = sum_part1(&mut left_values, right_values);
    let number: u32 = sum_part2(&mut left_values, &right_values);
    println!("{}", number);
}

#[allow(dead_code)]
fn sum_part1(left_values: &mut Vec<i32>, right_values: &Vec<i32>) -> u32 {
    left_values.iter()
        .zip(right_values.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum()
}

#[allow(dead_code)]
fn sum_part2(left_values: &mut Vec<i32>, right_values: &Vec<i32>) -> u32 {
    left_values.iter()
        .map(|&left| right_values.iter().filter(|&&right| right == left).count() as u32 * left as u32)
        .sum()
}