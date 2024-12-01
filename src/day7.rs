use std::{cmp::Reverse, collections::HashSet};

fn parse_hands(contents: &str, joker: bool) -> Vec<(Vec<u32>, i32)> {
    contents
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(a, b)| {
                    (
                        a.chars()
                            .map(|ch| match ch {
                                'A' => 14,
                                'K' => 13,
                                'Q' => 12,
                                'J' => match joker {
                                    true => 1,
                                    false => 11,
                                },
                                'T' => 10,
                                _ => ch.to_digit(10).unwrap(),
                            })
                            .collect::<Vec<_>>(),
                        b.parse::<i32>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn find_hand_type(hand: &Vec<u32>, joker: bool) -> u32 {
    let mut counts = hand
        .iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|&card| (*card, hand.iter().filter(|c| *c == card).count()))
        .collect::<Vec<_>>();

    if joker {
        counts.sort_by_key(|(label, count)| Reverse((*count, *label)));

        if let Some(joker) = counts.iter().find(|(card, _)| *card == 1) {
            let mut count_clone = counts
                .iter()
                .filter(|(card, _)| *card != 1)
                .map(|(card, count)| (*card, *count))
                .collect::<Vec<_>>();

            if count_clone.len() > 0 {
                count_clone[0].1 += joker.1;
            } else {
                count_clone.push((13, joker.1));
            }

            counts = count_clone;
        }
    } else {
        counts.sort_by(|(_, a), (_, b)| b.cmp(a));
    }

    match &counts[..] {
        &[(_, 5), ..] => 7,                                 // Five of a kind
        &[(_, 4), ..] => 6,                                 // Four of a kind
        &[(_, 3), (_, 2), ..] => 5,                         // Full house
        &[(_, 3), ..] => 4,                                 // Three of a kind
        &[(_, 2), (_, 2), ..] | &[.., (_, 2), (_, 2)] => 3, // Two Pair
        &[(_, 2), ..] => 2,                                 // One Pair
        _ => 1,                                             // High card
    }
}

fn calculate_winnings(hands: &Vec<(Vec<u32>, i32)>, joker: bool) -> i32 {
    let mut sorted_hands = hands.clone();

    sorted_hands.sort_by(|(a, _), (b, _)| {
        let a_type = find_hand_type(a, joker);
        let b_type = find_hand_type(b, joker);

        if a_type == b_type {
            a.iter()
                .zip(b.iter())
                .find(|(a, b)| a != b)
                .map(|(a, b)| a.cmp(b))
                .unwrap()
        } else {
            a_type.cmp(&b_type)
        }
    });

    sorted_hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bet))| *bet * (rank as i32 + 1))
        .sum::<i32>()
}

pub fn part1(contents: &str) {
    let hands = parse_hands(contents, false);
    let winnings = calculate_winnings(&hands, false);

    println!("[Part 1] Winnings: {}", winnings);
}

pub fn part2(contents: &str) {
    let hands = parse_hands(contents, true);
    let winnings = calculate_winnings(&hands, true);

    println!("[Part 2] Winnings: {}", winnings);
}
