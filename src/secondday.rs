use std::fs;

pub fn main() {
    let data = fs::read_to_string("./src/input_2.txt").unwrap();
    let mut all_numbers: Vec<Vec<i32>> = vec![];

    for line in data.lines() {
        let mut previous_number: Option<i32> = None;

        all_numbers.push(vec![]);

        for next_number in line.split_ascii_whitespace() {
            let this_number = next_number.parse::<i32>().unwrap();
            if let Some(previous_number) = previous_number {
                all_numbers
                    .last_mut()
                    .unwrap()
                    .push(previous_number - this_number);
            }
            previous_number = Some(this_number);
        }
    }

    // let all_numbers = all_numbers.clone().into_iter().filter(|line| {
    //     let is_positive = *line.first().unwrap() >= 0;
    //     line.into_iter().all(|number| {
    //         (*number >= 0) == is_positive && *number > -4 && *number < 4 && *number != 0
    //     })
    // });

    let all_numbers = all_numbers.clone().into_iter().filter(|line| {
        let is_positive = *line.first().unwrap() >= 0;
        line.into_iter()
            .map(|number| {
                let number = *number;
                (number >= 0) == is_positive && number > -4 && number < 4 && number != 0
            })
            .filter(|error| !*error)
            .count()
            < 2
    });

    println!("{:#?}", all_numbers.count());
}
