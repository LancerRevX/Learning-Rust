extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let array = {
        let mut rng = rand::thread_rng();
        let mut array = Vec::with_capacity(rng.gen_range(16, 33));
        for _ in 0..array.capacity() {
            array.push(rng.gen_range(1, 100));
        }
        array.sort();
        array
    };

    let mut sum = 0;
    let mut occurences = HashMap::new();
    let mut max: (usize, i32) = (0, 0);
    for number in &array {
        sum += number;
        let count = occurences.entry(number).or_insert(0usize);
        *count += 1;
        if *count > max.0 {
            max.0 = *count;
            max.1 = *number;
        }
    }

    println!("{:?}", array);
    println!("Mean = {}", sum / array.len() as i32); //Average value
    println!("Median = {}", match array.len() % 2 {
        0 => (array[array.len() / 2] + array[array.len() / 2 - 1]) / 2,
        _ => array[array.len() / 2]
    }); //Value in the middle position
    println!("Mode = {}", max.1) //Value that occurs most often
}