mod day9;

use std::fs;

fn main() {
    let contents =
        fs::read_to_string("inputs/day9.txt").expect("Something went wrong reading the file");

    day9::part1(&contents);
    day9::part2(&contents);
}
