use std::{fs, cmp::min};

fn get_seed_and_maps(groups: &Vec<String>) -> (Vec<u128>, Vec<Vec<Vec<u128>>>) {
    let seeds: Vec<u128> = groups
        .first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    let maps: Vec<Vec<Vec<u128>>> = groups
        .iter()
        .skip(1)
        .map(|group| {
            group
                .split("\r\n")
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(|s| s.parse::<u128>().unwrap())
                        .into_iter()
                        .collect::<Vec<u128>>()
                })
                .collect()
        })
        .collect();
    return (seeds, maps);
}

fn get_location_and_skip(seed: u128, maps: &Vec<Vec<Vec<u128>>>) -> (u128, u128) {
    let mut value = seed;
    let mut skip = u128::MAX;
    for map in maps {
        for mapping in map {
            if (*mapping)[1] <= value && value < (*mapping)[1] + (*mapping)[2] {
                skip = min(skip, (*mapping)[1] + (*mapping)[2] - value);
                value = (*mapping)[0] + value - (*mapping)[1];
                break;
            }
        }
    }
    (value, skip)
}

fn solve_part_one(groups: &Vec<String>) -> u128 {
    let mut min_location = u128::MAX;
    let (seeds, maps) = get_seed_and_maps(groups);
    for seed in seeds {
        let (location, _) = get_location_and_skip(seed, &maps);
        if location < min_location {
            min_location = location;
        }
    }
    min_location
}

fn solve_part_two(groups: &Vec<String>) -> u128 {
    let mut min_location = u128::MAX;
    let (seed_range, maps) = get_seed_and_maps(groups);

    for i in (0..seed_range.len()).step_by(2) {
        let mut seed = seed_range[i];
        while seed < seed_range[i] + seed_range[i + 1] {
            let (location, skip) = get_location_and_skip(seed, &maps);
            if location < min_location {
                min_location = location;
            }
            seed += skip;
        }
    }
    min_location
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let groups: Vec<String> = input
        .split("\r\n\r\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    return groups;
}

pub fn solver() {
    let input = get_input("./src/day5/input.txt");
    let sum_part_one = solve_part_one(&input);
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input);
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_example_input_part_one() {
        let input = get_input("./src/day5/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(35, sum_part_one);
    }

    #[test]
    fn day5_input_part_one() {
        let input = get_input("./src/day5/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(111627841, sum_part_one);
    }

    #[test]
    fn day5_example_input_part_two() {
        let input = get_input("./src/day5/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(46, sum_part_two);
    }

    #[test]
    fn day5_input_part_two() {
        let input = get_input("./src/day5/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(69323688, sum_part_two);
    }
}
