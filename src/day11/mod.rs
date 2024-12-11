use std::collections::HashMap;
use num_integer::Integer;

pub fn run() {
    let input = std::fs::read_to_string("src/day11/input.txt").unwrap();
    let stones: Vec<u64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut counts = HashMap::new();
    for &stone in &stones {
        *counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 { //PART 1: for _ in 0..25
        let mut next_counts = HashMap::new();
        for (num, count) in &counts {
            if *num == 0 {
                *next_counts.entry(1).or_insert(0) += count;
            } else {
                let len = (*num as f64).log10().floor() as u64 + 1;
                if len % 2 == 0 {
                    let divisor = 10u64.pow((len / 2) as u32);
                    let (num1, num2) = num.div_rem(&divisor);
                    *next_counts.entry(num1).or_insert(0) += count;
                    *next_counts.entry(num2).or_insert(0) += count;

                } else {
                    let new_num = num * 2024;
                    *next_counts.entry(new_num).or_insert(0) += count;
                }
            }
        }
        counts = next_counts;
    }
    println!("Number of elements: {}", counts.values().sum::<u64>());
}