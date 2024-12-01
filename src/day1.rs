use std::collections::HashMap;

pub fn part1(contents: &str) {
    let sum: i32 = contents
        .lines()
        .map(|line| {
            let first_digit_index = line.find(char::is_numeric).unwrap();
            let last_digit_index = line.rfind(char::is_numeric).unwrap();

            let first_digit = line.chars().nth(first_digit_index).unwrap();
            let last_digit = line.chars().nth(last_digit_index).unwrap();

            let value: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();

            value
        })
        .sum();

    println!("Calibration Sum: {}", sum);
}

pub fn part2(contents: &str) {
    let digit_name_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let sum: i32 = contents
        .lines()
        .map(|line| {
            // find the first digit in the digit_name_map give a default
            let mut fdi = line.find(char::is_numeric).unwrap_or(line.len() - 1);
            let mut ldi = line.rfind(char::is_numeric).unwrap_or(0);

            let mut first_digit = line.chars().nth(fdi).unwrap();
            let mut last_digit = line.chars().nth(ldi).unwrap();

            for (digit_name, digit_value) in digit_name_map.iter() {
                let fdn_idx = line.find(digit_name);
                let ldn_idx = line.rfind(digit_name);

                // if fdn_index exists and is less than the current first digit index
                if fdn_idx.is_some() && fdn_idx.unwrap() < fdi {
                    fdi = fdn_idx.unwrap();
                    first_digit = *digit_value;
                }

                // if ldn_index exists and is greater than the current last digit index
                if ldn_idx.is_some() && ldn_idx.unwrap() > ldi {
                    ldi = ldn_idx.unwrap();
                    last_digit = *digit_value;
                }
            }

            let value: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
            value
        })
        .sum();

    println!("Calibration Sum: {}", sum);
}
