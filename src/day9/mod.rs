pub fn run() {
    let input = std::fs::read_to_string("src/day9/input.txt").unwrap();
    let mut data: Vec<(u64, u64)> = Vec::new(); // amount, dataindex
    let mut space: Vec<u64> = Vec::new();

    let mut result: Vec<u64> = Vec::new();

    let mut dataindex = 0;

    for (index, ch) in input.chars().enumerate() {
        if index % 2 == 0 {
            data.push((ch.to_digit(10).unwrap() as u64, dataindex));
            dataindex += 1;
        } else {
            space.push(ch.to_digit(10).unwrap() as u64);
        }
    }

    // PART 1
    // while data.len() > 0 {
    //     let current = data.remove(0);
    //     for _ in 0..current.0 {
    //         result.push(current.1);
    //     }
    //     let freespace = space.remove(0);
    //     if data.len() == 0 {
    //         break;
    //     }
    //     if freespace == 0 {
    //         continue;
    //     }
    //     let last = data.pop().unwrap();
    //     if last.0 <= freespace {
    //         data.insert(0, (last.0, last.1));
    //         let remainingspace = freespace - last.0;
    //         space.insert(0, remainingspace);
    //     } else if freespace > 0 {
    //         let overflow = last.0 - freespace;
    //         data.push((overflow, last.1));
    //         data.insert(0, (freespace, last.1));
    //         space.insert(0, 0);
    //     }
    // }

    // PART 2
    let mut handled_index = data.len() - 1;
    while handled_index > 1 {
        let current = data.remove(handled_index);
        let mut handled_space_index = 0;
        while handled_space_index < handled_index && space[handled_space_index] < current.0 {
            handled_space_index += 1;
        }
        if handled_space_index == handled_index {
            data.insert(handled_index, current);
            handled_index -= 1;
        } else {
            data.insert(handled_space_index+1, current);
            space[handled_space_index] -= current.0;
            space[handled_index-1] += current.0;
            space.insert(handled_space_index,0);
            if handled_index < space.len() -1 {
                space[handled_index] += space[handled_index+1];
                space.remove(handled_index+1);
            }
        }
    }

    while data.len() > 0 {
        let current = data.remove(0);
        for _ in 0..current.0 {
            result.push(current.1);
        }
        let freespace = space.remove(0);
        for _ in 0..freespace {
            result.push(0);
        }
    }

    let mut checksum = 0;
    for i in 0..result.len() {
        checksum += result[i] * i as u64;
    }
    println!("Checksum: {}", checksum);
}
