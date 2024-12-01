use std::collections::HashSet;

pub fn part1(contents: &str) {
    let schematic: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut num_keys: HashSet<String> = HashSet::new();

    for (i, row) in schematic.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() || *col == '.' {
                continue;
            } else {
                for x in -1..2 {
                    for y in -1..2 {
                        if x == 0 && y == 0 {
                            continue;
                        }

                        let x_index = i as i32 + x;
                        let y_index = j as i32 + y;

                        if x_index < 0 || y_index < 0 {
                            continue;
                        }

                        let x_index = x_index as usize;
                        let y_index = y_index as usize;

                        if x_index >= schematic.len() || y_index >= row.len() {
                            continue;
                        }

                        if !schematic[x_index][y_index].is_numeric() {
                            continue;
                        }

                        let mut left_index = y_index;
                        let mut right_index = y_index;

                        let mut left_done = false;
                        let mut right_done = false;

                        while !left_done || !right_done {
                            if !left_done
                                && left_index > 0
                                && schematic[x_index][left_index - 1].is_numeric()
                            {
                                left_index -= 1;
                            } else {
                                left_done = true;
                            }

                            if !right_done
                                && right_index < row.len() - 1
                                && schematic[x_index][right_index + 1].is_numeric()
                            {
                                right_index += 1;
                            } else {
                                right_done = true;
                            }
                        }

                        num_keys.insert(format!("{x_index},{left_index},{right_index}"));
                    }
                }
            }
        }
    }

    let part_no_sum: i32 = num_keys
        .iter()
        .map(|key| {
            let loc = key.split(",").collect::<Vec<&str>>();
            let x = loc[0].parse::<usize>().unwrap();
            let left = loc[1].parse::<usize>().unwrap();
            let right = loc[2].parse::<usize>().unwrap();

            schematic[x][left..=right]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum();

    println!("Engine Part No Sum: {}", part_no_sum);
}

pub fn part2(contents: &str) {
    let schematic: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut num_keys: HashSet<String> = HashSet::new();

    let mut gear_ratio_sum = 0;

    for (i, row) in schematic.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() || *col == '.' {
                continue;
            } else {
                let mut adjacent_numbers = Vec::new();

                for x in -1..2 {
                    for y in -1..2 {
                        if x == 0 && y == 0 {
                            continue;
                        }

                        let x_index = i as i32 + x;
                        let y_index = j as i32 + y;

                        if x_index < 0 || y_index < 0 {
                            continue;
                        }

                        let x_index = x_index as usize;
                        let y_index = y_index as usize;

                        if x_index >= schematic.len() || y_index >= row.len() {
                            continue;
                        }

                        if !schematic[x_index][y_index].is_numeric() {
                            continue;
                        }

                        let mut left_index = y_index;
                        let mut right_index = y_index;

                        let mut left_done = false;
                        let mut right_done = false;

                        while !left_done || !right_done {
                            if !left_done
                                && left_index > 0
                                && schematic[x_index][left_index - 1].is_numeric()
                            {
                                left_index -= 1;
                            } else {
                                left_done = true;
                            }

                            if !right_done
                                && right_index < row.len() - 1
                                && schematic[x_index][right_index + 1].is_numeric()
                            {
                                right_index += 1;
                            } else {
                                right_done = true;
                            }
                        }

                        let num_key = format!("{x_index},{left_index},{right_index}");

                        if num_keys.contains(&num_key) {
                            continue;
                        }

                        num_keys.insert(num_key);

                        adjacent_numbers.push(
                            schematic[x_index][left_index..=right_index]
                                .iter()
                                .collect::<String>(),
                        );
                    }
                }

                if adjacent_numbers.len() == 2 {
                    gear_ratio_sum += adjacent_numbers[0].parse::<i32>().unwrap()
                        * adjacent_numbers[1].parse::<i32>().unwrap();
                }
            }
        }
    }

    println!("Gear Ratio Sum: {}", gear_ratio_sum);
}
