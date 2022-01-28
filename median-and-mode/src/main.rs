// Problem posed on Chapter 8-3
// Link: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most
// often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    // input vector of integers
    let v = vec![1, 2, 3, -1, 2, 4, 3, 1, 6, 4, 2, 2, 10, 11];

    println!("vector: {:?}", v);

    // sort the vector
    let sorted_v = {
        let mut new_v = v.clone();
        new_v.sort();
        new_v
    };

    //println!("sorted vector: {:?}", sorted_v);

    // get median (the value at the middle)
    let median = sorted_v.get(sorted_v.len()/2);

    match median {
        None => println!("no median"),
        Some(median_number) => println!("median: {}", median_number),
    };

    // keep track of number of occurences
    let mut occurences : HashMap<i32,i32> = HashMap::new();

    for number in &v {
        let count = occurences.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut mode : Option<i32> = None;

    for (number, count) in &occurences {

        let mode_count = match mode {
            None => None,
            Some(mode_number) => occurences.get(&mode_number),
        };

        if let Some(mode_count_number) = mode_count {
            if count > mode_count_number {
                mode = Some(*number);
            }
        } else {
            mode = Some(*number);
        }
    }

    match mode {
        None => println!("no mode"),
        Some(mode_number) => println!("mode: {}", mode_number),
    };
}
