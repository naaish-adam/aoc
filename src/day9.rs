fn differences(sequence: &Vec<i32>) -> Vec<i32> {
    let mut differences = Vec::new();
    for i in 0..sequence.len() - 1 {
        differences.push(sequence[i + 1] - sequence[i]);
    }
    differences
}

fn extrapolate(sequence: &Vec<i32>, reverse: bool) -> i32 {
    let mut diff_sequences = Vec::new();
    let mut diffs = sequence.clone();

    while diffs.iter().any(|x| *x != 0) {
        diffs = differences(&diffs);
        diff_sequences.push(diffs.clone());
    }

    let mut next_value = 0;

    for i in (0..diff_sequences.len() - 1).rev() {
        if reverse {
            next_value = diff_sequences[i].first().unwrap() - next_value;
        } else {
            next_value += diff_sequences[i].last().unwrap();
        }
    }

    if reverse {
        next_value = sequence.first().unwrap() - next_value;
    } else {
        next_value += sequence.last().unwrap();
    }

    next_value
}

pub fn part1(contents: &str) {
    let extrapolated_sum = contents
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            extrapolate(&sequence, false)
        })
        .sum::<i32>();

    println!("Extrapolated Sum: {}", extrapolated_sum);
}

pub fn part2(contents: &str) {
    let extrapolated_sum = contents
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            extrapolate(&sequence, true)
        })
        .sum::<i32>();

    println!("Reverse Extrapolated Sum: {}", extrapolated_sum);
}
