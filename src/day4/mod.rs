#[allow(dead_code)]
pub fn run() {
    let input = std::fs::read_to_string("src/day4/input.txt").unwrap();
    let input2 = input.lines().collect::<Vec<&str>>();
    // PART 1
    //let searched_word = "XMAS";
    //let directions = [(0,1), (0,-1),(1,0),(-1,0), (1,1), (1,-1), (-1,1), (-1,-1)];

    // PART 2
    let searched_word = "MAS";
    let is_word_palindrome = searched_word.chars().eq(searched_word.chars().rev());
    let directions: [(i32, i32); 4]  = [(1,1), (1,-1), (-1,1), (-1,-1)];
    let middle_index = searched_word.len() / 2;
    let middle_char = searched_word.chars().nth(middle_index).unwrap();

    let rows = input2.len();
    let cols = input2[0].len();

    let mut sum = 0;

    // PART 1
    // for i in 0..rows {
    //     for j in 0..cols {
    //         for &(dx, dy) in &directions {
    //              let mut k = 0;
    //              let mut nx = i as isize;
    //              let mut ny = j as isize;
    //
    //              while k < searched_word.len() {
    //                  if nx < 0 || nx >= rows as isize || ny < 0 || ny >= cols as isize  {
    //                      break;
    //                  }
    //                  if input2[nx as usize].chars().nth(ny as usize) != searched_word.chars().nth(k){
    //                      break;
    //                  }
    //
    //                  nx += dx;
    //                  ny += dy;
    //                  k += 1;
    //              }
    //              if k == searched_word.len() {
    //                  sum += 1;
    //              }
    //         }
    //     }
    // }

    // PART 2
    for i in middle_index..rows-middle_index {
        for j in middle_index..cols-middle_index {
            if input2[i].chars().nth(j).unwrap() != middle_char {
                continue;
            }
            let mut found_dirs = 0;
            for &(dx, dy) in &directions {

                 let mut k = 0;
                 let mut nx = i as isize - dx as isize * middle_index as isize;
                 let mut ny = j as isize - dy as isize * middle_index as isize;

                 while k < searched_word.len() {
                     if nx < 0 || nx >= rows as isize || ny < 0 || ny >= cols as isize  {
                         break;
                     }
                     if input2[nx as usize].chars().nth(ny as usize) != searched_word.chars().nth(k){
                         break;
                     }

                     nx += dx as isize;
                     ny += dy as isize;
                     k += 1;
                 }
                 if k == searched_word.len() {
                     found_dirs += 1;
                 }
            }

            if (is_word_palindrome && found_dirs == 4) || (!is_word_palindrome && found_dirs == 2) {
                sum += 1;
            }
        }
    }

    println!("Sum: {}", sum);
}