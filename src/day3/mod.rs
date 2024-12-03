use regex::Regex;

#[allow(dead_code)]
pub fn run() {
    let input = std::fs::read_to_string("src/day3/input.txt").unwrap();
    // PART 1
    // let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    //PART 2
    let re = Regex::new(r"mul\((\d+),(\d+)\)|\bdo\(\)|\bdon't\(\)").unwrap();
    let mut sum = 0;
    let mut okay: bool = true;
    for cap in re.captures_iter(&input) {
        // PART 1
        // sum +=cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        if okay && cap.get(1).is_some() {
            sum +=cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        } else if cap.get(0).unwrap().as_str() == "do()" {
            okay = true;
        } else if cap.get(0).unwrap().as_str() == "don't()" {
            okay = false;
        }
    }
    println!("Sum: {}", sum);
}