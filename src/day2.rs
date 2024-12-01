struct Pick {
    red: i32,
    green: i32,
    blue: i32,
}

impl Pick {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: i32,
    picks: Vec<Pick>,
}

fn parse_games(contents: &str) -> Vec<Game> {
    contents
        .lines()
        .map(|line| {
            let mut game = Game {
                id: 0,
                picks: Vec::new(),
            };

            let (game_str, picks_str) = line.split_once(':').unwrap();

            game.id = game_str.split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            for pick in picks_str.split(';') {
                let pick_values = pick.split(',').collect::<Vec<&str>>();

                let mut p = Pick {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                pick_values.iter().for_each(|value| {
                    let v = value.trim_start().split(' ').collect::<Vec<&str>>();

                    match v[1] {
                        "red" => p.red = v[0].parse::<i32>().unwrap(),
                        "green" => p.green = v[0].parse::<i32>().unwrap(),
                        "blue" => p.blue = v[0].parse::<i32>().unwrap(),
                        _ => (),
                    }
                });

                game.picks.push(p);
            }

            game
        })
        .collect()
}

pub fn part1(contents: &str) {
    let sum_of_game_ids: i32 = parse_games(contents)
        .iter()
        .filter(|game| {
            !game
                .picks
                .iter()
                .any(|pick| pick.red > 12 || pick.green > 13 || pick.blue > 14)
        })
        .map(|game| game.id)
        .sum();

    println!("Sum of Game IDs: {}", sum_of_game_ids);
}

pub fn part2(contents: &str) {
    let sum: i32 = parse_games(contents)
        .iter()
        .map(|game| {
            let mut min_pick = Pick {
                red: 0,
                green: 0,
                blue: 0,
            };

            for pick in game.picks.iter() {
                if pick.red > min_pick.red {
                    min_pick.red = pick.red;
                }

                if pick.green > min_pick.green {
                    min_pick.green = pick.green;
                }

                if pick.blue > min_pick.blue {
                    min_pick.blue = pick.blue;
                }
            }

            min_pick.power()
        })
        .sum();

    println!("Sum of Min Powers: {}", sum);
}
