use std::fs;

pub fn main() {
    let locations = fs::read_to_string("./src/input_1.txt").unwrap();

    let mut left_vec: Vec<i32> = vec![];
    let mut right_vec: Vec<i32> = vec![];

    let mut total: i32 = 0;

    for line in locations.lines() {
        let numbers: Vec<&str> = line.split("   ").collect();

        let left = numbers.first().unwrap().parse::<i32>().unwrap();
        let right = numbers.get(1).unwrap().parse::<i32>().unwrap();

        left_vec.push(left);
        right_vec.push(right);

        // println!("left: {left}, right: {right}");
    }

    left_vec.sort();
    right_vec.sort();

    // FIRST CHALLENGE
    // for index in 0..left_vec.len() {
    //     total += (left_vec.get(index).unwrap() - right_vec.get(index).unwrap()).abs();
    // }

    // SECOND CHALLENGE
    for index in 0..left_vec.len() {
        let left_value = left_vec.get(index).unwrap();

        let count = right_vec
            .iter()
            .filter(|number| *number == left_value)
            .count() as i32;

        // println!("{count}");

        total += left_vec.get(index).unwrap() * count;
    }

    println!("{total}");
}
