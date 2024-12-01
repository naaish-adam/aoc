use std::{ops::Range, vec};

pub fn parse_seed_and_maps(content: &str) -> (Vec<i64>, Vec<Vec<(Range<i64>, Range<i64>)>>) {
    let mut lines = content.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let maps = lines.fold(
        Vec::<Vec<(Range<i64>, Range<i64>)>>::new(),
        |mut maps, line| {
            if line.is_empty() {
                return maps;
            }

            if line.ends_with("map:") {
                maps.push(Vec::new());
            } else {
                let numbers = line
                    .split(" ")
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect::<Vec<i64>>();

                let source = numbers[1]..(numbers[1] + numbers[2]);
                let destination = numbers[0]..(numbers[0] + numbers[2]);

                maps.last_mut().unwrap().push((source, destination));
            }

            return maps;
        },
    );

    (seeds, maps)
}

pub fn part1(content: &str) {
    let (seeds, maps) = parse_seed_and_maps(content);

    let min_location_number = seeds
        .iter()
        .map(|seed| {
            let mut location = *seed;

            for map in &maps {
                for (source, destination) in map {
                    if source.contains(&location) {
                        location = destination.start + (location - source.start);

                        break;
                    }
                }
            }

            location
        })
        .min();

    if let Some(min_location_number) = min_location_number {
        println!("[Part 1] Min Location: {:?}", min_location_number);
    }
}

pub fn part2(contents: &str) {
    use std::time::Instant;
    let now = Instant::now();

    let (seeds, maps) = parse_seed_and_maps(contents);

    let mut min = i64::MAX;

    fn get_overlapping_ranges(
        loc_range: Range<i64>,
        ranges: &Vec<(Range<i64>, Range<i64>)>,
    ) -> Vec<Range<i64>> {
        for (index, (source, destination)) in ranges.iter().enumerate() {
            let loc_start = i64::max(loc_range.start, source.start).min(loc_range.end - 1);
            let loc_end = i64::min(loc_range.end, source.end);

            if source.contains(&loc_start) {
                let num1 = destination.start + (loc_start - source.start);
                let num2 = destination.start + (loc_end - source.start);

                let mut overlapping_ranges = vec![num1..num2];

                if loc_range.start < source.start {
                    let rest_pre_ranges = get_overlapping_ranges(
                        loc_range.start..source.start,
                        &ranges[(index + 1)..ranges.len()].to_vec(),
                    );

                    overlapping_ranges.extend(rest_pre_ranges);
                }

                if loc_range.end > source.end {
                    let rest_post_ranges = get_overlapping_ranges(
                        (source.end)..loc_range.end,
                        &ranges[(index + 1)..ranges.len()].to_vec(),
                    );

                    overlapping_ranges.extend(rest_post_ranges);
                }

                return overlapping_ranges;
            }
        }

        return vec![loc_range];
    }

    for (index, seed) in seeds.iter().enumerate() {
        if index % 2 != 0 {
            continue;
        }

        let mut loc_ranges = vec![*seed..(*seed + seeds[index + 1])];

        for ranges in &maps {
            let mut overlapping_ranges: Vec<Range<i64>> = vec![];

            for loc_range in loc_ranges.clone() {
                overlapping_ranges.extend(get_overlapping_ranges(loc_range.clone(), &ranges));
            }

            loc_ranges = overlapping_ranges;
        }

        let min_seed_loc_range = loc_ranges.iter().min_by_key(|r| r.start).unwrap();

        if min_seed_loc_range.start < min {
            min = min_seed_loc_range.start;
        }
    }

    println!("[Part 2] Min Location: {}", min);
}
