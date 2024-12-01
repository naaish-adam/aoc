use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

pub fn part1(content: &str) {
    let winning_num_sum = content
        .lines()
        .map(|line| {
            let (_, numbers_str) = line.split_once(':').unwrap();

            let (winning_str, yours_str) = numbers_str.split_once('|').unwrap();

            let winning = winning_str
                .split(' ')
                .filter_map(|n| n.trim().parse::<i32>().ok())
                .collect::<HashSet<i32>>();

            let your_winnings = yours_str
                .split(' ')
                .filter_map(|n| n.parse::<i32>().ok())
                .filter(|n| winning.contains(n))
                .collect::<Vec<i32>>();

            if let Some(len) = your_winnings.len().checked_sub(1) {
                2_i32.pow(len.try_into().unwrap())
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("Sum of Winning Numbers: {}", winning_num_sum);
}

pub fn part2(content: &str) {
    let mut card_map: HashMap<usize, i32> = HashMap::new();

    let total_cards = content.lines().count();

    let total_card_instances: i32 = content
        .lines()
        .enumerate()
        .map(|(card_idx, line)| {
            let copies = *card_map.get(&card_idx).unwrap_or(&0) + 1;

            card_map.insert(card_idx, copies);

            let (_, numbers_str) = line.split_once(':').unwrap();

            let (winning_str, yours_str) = numbers_str.split_once('|').unwrap();

            let winning = winning_str
                .split(' ')
                .filter_map(|n| n.trim().parse::<i32>().ok())
                .collect::<HashSet<i32>>();

            let your_winnings_count = yours_str
                .split(' ')
                .filter_map(|n| n.parse::<i32>().ok())
                .filter(|n| winning.contains(n))
                .count();

            for next_card_idx in
                (card_idx + 1)..min(card_idx + 1 + your_winnings_count, total_cards)
            {
                card_map.insert(
                    next_card_idx,
                    card_map.get(&next_card_idx).unwrap_or(&0) + copies,
                );
            }

            copies
        })
        .sum();

    println!(
        "Total no of Scratchcard Instances: {} ",
        total_card_instances
    );
}
