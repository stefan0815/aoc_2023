use std::{collections::HashSet, fs};

fn get_differences(sequence: &Vec<i128>) -> Vec<i128> {
    sequence
        .windows(2)
        .map(|window| window.last().unwrap() - window.first().unwrap())
        .collect()
}

fn find_next_value(sequence: &Vec<i128>) -> i128 {
    let sequence_set: HashSet<i128> = HashSet::from_iter(sequence.iter().cloned());
    if sequence_set.len() == 1 {
        return *sequence.first().unwrap();
    }
    let next_value = sequence.last().unwrap() + find_next_value(&get_differences(sequence));
    return next_value;
}

fn find_next_value_iterative(sequence: &Vec<i128>) -> i128 {
    let mut current_sequence = sequence.clone();
    let mut next_values: Vec<i128> = vec![];
    loop {
        next_values.push(*current_sequence.last().unwrap());
        let sequence_set: HashSet<i128> = HashSet::from_iter(current_sequence.iter().cloned());
        if sequence_set.len() == 1 {
            break;
        }
        current_sequence = get_differences(&current_sequence);
    }
    return next_values.iter().sum();
}

fn parse_sequences(input: &Vec<String>) -> Vec<Vec<i128>> {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|string| string.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .collect()
}

fn solve(
    input: &Vec<String>,
    next_value: fn(&Vec<i128>) -> i128,
    sequence_func: fn(&mut Vec<i128>),
) -> i128 {
    let sequences = parse_sequences(input);
    sequences
        .iter()
        .map(|sequence| {
            let mut seq = sequence.clone();
            sequence_func(&mut seq);
            next_value(&seq)
        })
        .sum()
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
    let input = get_input("./src/day9/input.txt");
    let part_one = solve(&input, find_next_value, |_| {});
    let part_two = solve(&input, find_next_value, |s| s.reverse());
    println!("Part 1: {part_one}");
    println!("Part 2: {part_two}");

    let part_one_iterative = solve(&input, find_next_value_iterative, |_| {});
    let part_two_iterative = solve(&input, find_next_value_iterative, |s| s.reverse());
    println!("Part 1 iterative: {part_one_iterative}");
    println!("Part 2 iterative: {part_two_iterative}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn day9_example_input_part_one() {
        let input = get_input("./src/day9/example_input.txt");
        let sum_part_one = solve(&input, find_next_value, |_| {});
        assert_eq!(114, sum_part_one);
    }

    #[test]
    fn day9_input_part_one() {
        let input = get_input("./src/day9/input.txt");
        let sum_part_one = solve(&input, find_next_value, |_| {});
        assert_eq!(1798691765, sum_part_one);
    }

    #[test]
    fn day9_input_part_one_iterative() {
        let input = get_input("./src/day9/input.txt");
        let sum_part_one = solve(&input, find_next_value_iterative, |_| {});
        assert_eq!(1798691765, sum_part_one);
    }

    #[test]
    fn day9_example_input_part_two() {
        let input = get_input("./src/day9/example_input.txt");
        let sum_part_two = solve(&input, find_next_value, |s| s.reverse());
        assert_eq!(2, sum_part_two);
    }

    #[test]
    fn day9_input_part_two() {
        let input = get_input("./src/day9/input.txt");
        let sum_part_two = solve(&input, find_next_value, |s| s.reverse());
        assert_eq!(1104, sum_part_two);
    }

    #[test]
    fn day9_input_part_two_iterative() {
        let input = get_input("./src/day9/input.txt");
        let sum_part_two = solve(&input, find_next_value_iterative, |s| s.reverse());
        assert_eq!(1104, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day9/input.txt");
        b.iter(|| solve(&input, find_next_value, |_| {}))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day9/input.txt");
        b.iter(|| solve(&input, find_next_value, |s| s.reverse()))
    }

    #[bench]
    fn bench_part_one_iterative(b: &mut Bencher) {
        let input = get_input("./src/day9/input.txt");
        b.iter(|| solve(&input, find_next_value_iterative, |s| s.reverse()))
    }

    #[bench]
    fn bench_part_two_iterative(b: &mut Bencher) {
        let input = get_input("./src/day9/input.txt");
        b.iter(|| solve(&input, find_next_value_iterative, |_| {}))
    }
}
