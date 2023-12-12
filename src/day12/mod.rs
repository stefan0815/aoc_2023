use std::fs;
use rayon::prelude::*;

fn is_still_valid(springs: &Vec<char>, valid_groups: &Vec<usize>) -> bool {
    let mut group_start = false;
    let mut current_group_size = 0;
    let mut group_index = 0;
    for i in 0..springs.len() {
        let spring = springs[i];
        match (spring, group_start) {
            ('#', _) => {
                group_start = true;
                current_group_size += 1;
            },
            ('.', true) => {
                if group_index >= valid_groups.len() || current_group_size != valid_groups[group_index] {
                    return false;
                }
                group_start = false;
                current_group_size = 0;
                group_index += 1;
            }
            ('?', _) => {
                return true;
            }
            _ => (),
        }
    }

    if group_start && group_index < valid_groups.len() - 1 || !group_start && group_index < valid_groups.len(){
        return false;
    }

    if group_start && (group_index >= valid_groups.len() || current_group_size != valid_groups[group_index]) {
        return false;
    }
    true
}

fn get_valid_arrangements(springs: &Vec<char>, groups: &Vec<usize>) -> Vec<Vec<char>> {
    let unknown_springs: Vec<usize> = springs
        .iter()
        .enumerate()
        .filter(|(_, spring)| **spring == '?')
        .map(|(index, _)| index)
        .collect();
    if unknown_springs.len() == 0 {
        return vec![springs.to_vec()];
    }
    let mut springs_one = springs.to_vec();
    let mut springs_two = springs.to_vec();

    springs_one[*unknown_springs.first().unwrap()] = '.';
    springs_two[*unknown_springs.first().unwrap()] = '#';

    let mut arrangements: Vec<Vec<char>> = Vec::new();
    if is_still_valid(&springs_one, groups) {
        arrangements.extend(get_valid_arrangements(&springs_one, groups));
    }
    if is_still_valid(&springs_two, groups) {
        arrangements.extend(get_valid_arrangements(&springs_two, groups));
    }

    arrangements
}

fn solve_part_one(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            let split: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
            let springs: Vec<char> = split[0].chars().collect();
            let groups: Vec<usize> = split[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            get_valid_arrangements(&springs, &groups).len()
        })
        .sum()
}

fn solve_part_two(input: &Vec<String>) -> usize {
    input
        .par_iter()
        .map(|line| {
            let split: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
            let springs: Vec<char> = split[0].chars().collect();
            let groups: Vec<usize> = split[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let mut adapted_springs = springs.to_vec();
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            let mut adapted_groups = groups.to_vec();
            adapted_groups.extend(groups.to_vec());
            adapted_groups.extend(groups.to_vec());
            adapted_groups.extend(groups.to_vec());
            adapted_groups.extend(groups.to_vec());
            let num_valid_arrangements = get_valid_arrangements(&adapted_springs, &adapted_groups).len();
            // println!("num_valid_arrangements: {num_valid_arrangements}");
            num_valid_arrangements
        })
        .sum()
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let input: Vec<String> = input
        .split("\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    return input;
}

pub fn solver() {
    let input = get_input("./src/day12/input.txt");
    let sum_part_one = solve_part_one(&input);
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input);
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn day12_example_input_part_one() {
        let input = get_input("./src/day12/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(21, sum_part_one);
    }

    #[test]
    fn day12_input_part_one() {
        let input = get_input("./src/day12/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(7460, sum_part_one);
    }

    #[test]
    fn day12_example_input_part_two() {
        let input = get_input("./src/day12/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(525152, sum_part_two);
    }

    #[test]
    fn day12_input_part_two() {
        let input = get_input("./src/day12/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(0, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day12/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day12/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
