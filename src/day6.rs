struct Race {
    time: u32,
    distance: u32,
}

pub fn part1(contents: &str) {
    let mut lines = contents.lines();

    let times = lines
        .next()
        .unwrap()
        .replace("  ", " ")
        .split(" ")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distances = lines
        .next()
        .unwrap()
        .replace("  ", " ")
        .split(" ")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let races = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time: *time,
            distance,
        })
        .collect::<Vec<Race>>();

    let num_ways_to_beat_record: usize = races
        .iter()
        .map(|r| {
            (2..r.time)
                .filter(|t| t * (r.time - t) > r.distance)
                .count()
        })
        .product();

    println!("Num ways to beat record: {}", num_ways_to_beat_record);
}

pub fn part2(contents: &str) {
    let mut lines = contents.lines();

    let time = lines
        .next()
        .unwrap()
        .replace("  ", " ")
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .replace("  ", " ")
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let num_ways_to_beat_record = (2..time).filter(|t| t * (time - t) > distance).count();

    println!("Num ways to beat record: {}", num_ways_to_beat_record);
}
