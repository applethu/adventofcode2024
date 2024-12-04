#[allow(dead_code)]
pub fn run() {
    let input = std::fs::read_to_string("src/day4/input.txt").unwrap();
    let input2 = input.lines().collect::<Vec<&str>>();
    let searched_word = "XMAS";
    let directions = [(0,1), (0,-1),(1,0),(-1,0), (1,1), (1,-1), (-1,1), (-1,-1)];

    let rows = input2.len();
    let cols = input2[0].len();

    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                 let mut k = 0;
                 let mut nx = i as isize;
                 let mut ny = j as isize;

                 while k < searched_word.len() {
                     if nx < 0 || nx >= rows as isize || ny < 0 || ny >= cols as isize  {
                         break;
                     }
                     if input2[nx as usize].chars().nth(ny as usize) != searched_word.chars().nth(k){
                         break;
                     }

                     nx += dx;
                     ny += dy;
                     k += 1;
                 }
                 if k == searched_word.len() {
                     sum += 1;
                 }
            }
        }
    }
    println!("Sum: {}", sum);
}