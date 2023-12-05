use std::{collections::HashMap, fs};

fn get_seed_and_maps(groups: &Vec<String>) -> (Vec<u32>, Vec<HashMap<u32, u32>>) {
    let seeds: Vec<u32> = groups
        .first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let maps: Vec<HashMap<u32, u32>> = groups
        .iter()
        .skip(1)
        .map(|group| {
            let mappings: Vec<Vec<u32>> = group
                .split("\r\n")
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .into_iter()
                        .collect::<Vec<u32>>()
                })
                .collect();
            let mut map: HashMap<u32, u32> = HashMap::new();
            for mapping in mappings {
                for i in 0..mapping[2] {
                    map.insert(mapping[1] + i, mapping[0] + i);
                }
            }
            map
        })
        .collect();
    return (seeds, maps);
}

fn solve_part_one(groups: &Vec<String>) -> u32 {
    let mut min_location = u32::MAX;
    let (seeds, maps) = get_seed_and_maps(groups);
    for seed in seeds {
        let mut value = seed;
        println!("{value}");
        println!("Maps length: {}", maps.len());

        for map in &maps {
            println!("{value}");

            if map.contains_key(&value) {
                value = map[&value];
            }
        }
        if value < min_location {
            min_location = value;
        }
    }
    min_location
}

fn solve_part_two(_: &Vec<String>) -> u32 {
    0
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
    let sum_part_two = solve_part_two(&input);
    println!("Part 1: {sum_part_one}");
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
        assert_eq!(21485, sum_part_one);
    }

    #[test]
    fn day5_example_input_part_two() {
        let input = get_input("./src/day5/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(30, sum_part_two);
    }

    #[test]
    fn day5_input_part_two() {
        let input = get_input("./src/day5/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(11024379, sum_part_two);
    }
}
