mod day1;

use std::fs;

fn main() {
    let contents =
        fs::read_to_string("inputs/day1.txt").expect("Something went wrong reading the file");

    day1::part1(&contents);
    day1::part2(&contents);
}
