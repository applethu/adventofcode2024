use std::sync::Arc;
use regex::Regex;

pub fn run() {
    let input = std::fs::read_to_string("src/day7/input.txt").unwrap();

    let re = Regex::new(r"(\d+):\s((?:\d+\s*)+)\n").unwrap();


    let mut tst_list = Vec::new();
    for cap in re.captures_iter(&input) {
        let result: i64 = cap[1].parse().unwrap();
        let input: Vec<i64> = cap[2].split_whitespace().map(|s| s.parse().unwrap()).collect();
        tst_list.push(Tst { result, input });
    }

    let mut sum = 0;

    for tst in tst_list {
        let combinations = generate_combinations(tst.input.len() as i64 - 1);
        for combination in combinations {
            let mut calculated = tst.input[0];
            for i in 0..combination.len() {
                calculated = combination[i](calculated, tst.input[i + 1]);
            }
            if calculated == tst.result {
                sum += tst.result;
                break;
            }
        }
    }
    println!("SUM: {}", sum);

}

fn generate_combinations(len: i64) -> Vec<Vec<Arc<dyn Fn(i64, i64) -> i64>>> {
    let mut combinations:  Vec<Vec<Arc<dyn Fn(i64, i64) -> i64>>> = Vec::new();
    let operations: Vec<Arc<dyn Fn(i64, i64) -> i64>> = vec![
        Arc::new(|x, y| format!("{}{}", x, y).parse::<i64>().unwrap()), //PART 2
        Arc::new(|x, y| x + y),
        Arc::new(|x, y| x * y),
    ];
    for i in 0..operations.len().pow(len as u32) {
        let mut combination: Vec<Arc<dyn Fn(i64, i64) -> i64>> = Vec::new();
        let mut num = i;
        for _ in 0..len {
            combination.push(Arc::clone(&operations[num % operations.len()]));
            num /= operations.len();
        }
        combinations.push(combination);
    }
    combinations
}

#[derive(Debug)]
pub struct Tst {
    pub result: i64,
    pub input: Vec<i64>,
}
