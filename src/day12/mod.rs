use std::{collections::VecDeque, fs};

fn get_groups(springs: &Vec<char>) -> Vec<usize> {
    let mut groups: Vec<usize> = Vec::new();
    let mut group_start = false;
    let mut current_group_size = 0;
    for i in 0..springs.len() {
        let spring = springs[i];
        match (spring, group_start) {
            ('#', _) => {
                group_start = true;
                current_group_size += 1;
            }
            ('.', true) => {
                groups.push(current_group_size);
                group_start = false;
                current_group_size = 0;
            }
            _ => (),
        }
    }
    if group_start {
        groups.push(current_group_size);
    }
    groups
}

fn get_all_arrangements(springs: &Vec<char>) -> Vec<Vec<char>> {
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
    arrangements.extend(get_all_arrangements(&springs_one));
    arrangements.extend(get_all_arrangements(&springs_two));

    arrangements
}

fn get_valid_arrangements(springs: &Vec<char>, groups: &Vec<usize>) -> Vec<Vec<char>> {
    let all_arrangements = get_all_arrangements(springs);
    let valid_groups = all_arrangements
        .iter()
        .filter(|springs| get_groups(springs) == *groups)
        .map(|chars| chars.to_owned())
        .collect::<Vec<Vec<char>>>();
    // println!("springs: {:?}", springs);
    // println!("groups: {:?}", groups);
    // println!("groups_found: {:?}", get_groups(springs));
    // println!("all_arrangements: {:?}", all_arrangements);
    // println!("num_valid_groups: {:?}", num_valid_groups);
    valid_groups
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
        .iter()
        .map(|line| {
            let split: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
            let springs: Vec<char> = split[0].chars().collect();
            let groups: Vec<usize> = split[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let mut adapted_springs = VecDeque::from(springs);
            adapted_springs.push_back('?');
            adapted_springs.push_front('?');
            // if get_valid_arrangements(&springs, &groups).len() ==  1

            let valid_arrangements = get_valid_arrangements(&Vec::from(adapted_springs), &groups);
            let num_valid_arrangements = valid_arrangements
                .iter()
                .filter(|arrangement| arrangement.first().unwrap() == arrangement.last().unwrap())
                .collect::<Vec<&Vec<char>>>()
                .len();
            let multiple = num_valid_arrangements
                * num_valid_arrangements
                * num_valid_arrangements
                * num_valid_arrangements
                * num_valid_arrangements;
            println!("{num_valid_arrangements}/{multiple}");
            multiple
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
