use std::{cmp::min, collections::HashSet, fs};

fn get_all_wins(lines: &Vec<String>) -> Vec<usize> {
    lines
        .iter()
        .map(|line| {
            let split: Vec<String> = line
                .split(':')
                .last()
                .unwrap()
                .trim()
                .split('|')
                .map(|s| s.to_owned())
                .collect();
            let winning_numbers: HashSet<u32> = split
                .first()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let my_numbers: HashSet<u32> = split
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let number_of_wins: usize = winning_numbers
                .intersection(&my_numbers)
                .map(|n| *n)
                .collect::<Vec<u32>>()
                .len();

            number_of_wins
        })
        .collect()
}

fn solve_part_one(lines: &Vec<String>) -> usize {
    let all_wins = get_all_wins(lines);
    all_wins
        .iter()
        .map(|number_of_wins| {
            if *number_of_wins > 0 {
                let base: usize = 2;
                return base.pow((*number_of_wins - 1).try_into().unwrap());
            }
            0
        })
        .sum()
}

fn solve_part_two(lines: &Vec<String>) -> usize {
    let all_wins = get_all_wins(lines);
    let mut scratchcards: Vec<usize> = vec![1; all_wins.len()];
    for i in 0..(all_wins.len() - 1) {
        let number_of_wins = all_wins[i];
        if number_of_wins == 0 {
            continue;
        }
        let current_cards = scratchcards[i];
        for j in (i + 1)..min(i + 1 + number_of_wins , all_wins.len()) {
            scratchcards[j] += current_cards;
        }
    }
    scratchcards.iter().sum()
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<String> = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    return lines;
}

pub fn solver() {
    let input = get_input("./src/day4/input.txt");
    let sum_part_one = solve_part_one(&input);
    let sum_part_two = solve_part_two(&input);
    println!("Part 1: {sum_part_one}");
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_example_input_part_one() {
        let input = get_input("./src/day4/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(13, sum_part_one);
    }

    #[test]
    fn day4_input_part_one() {
        let input = get_input("./src/day4/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(21485, sum_part_one);
    }

    #[test]
    fn day4_example_input_part_two() {
        let input = get_input("./src/day4/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(30, sum_part_two);
    }

    #[test]
    fn day4_input_part_two() {
        let input = get_input("./src/day4/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(11024379, sum_part_two);
    }
}
