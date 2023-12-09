use std::{collections::HashSet, fs};


fn get_differences(sequence: &Vec<i128>) -> Vec<i128>{
    let mut differences :Vec<i128> = Vec::new();
    for i in 1..sequence.len(){
        differences.push(sequence[i] - sequence[i-1])
    }
    differences
}

fn find_next_value(sequence: &Vec<i128>) -> i128{
    let sequence_set: HashSet<i128> = HashSet::from_iter(sequence.iter().cloned());
    if sequence_set.len() == 1 {
        return *sequence.first().unwrap();
    }
    let next_value = sequence.last().unwrap() + find_next_value(&get_differences(sequence));
    return next_value;
}

fn find_previous_value(sequence: &Vec<i128>) -> i128{
    let sequence_set: HashSet<i128> = HashSet::from_iter(sequence.iter().cloned());
    if sequence_set.len() == 1 {
        return *sequence.first().unwrap();
    }
    let previous_value = sequence.first().unwrap() - find_previous_value(&get_differences(sequence));
    return previous_value;
}

fn solve_part_one(input: &Vec<String>) -> i128 {
    let sequences: Vec<Vec<i128>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|string| string.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .collect();
    sequences.iter().map(|sequence| find_next_value(sequence)).sum()
}

fn solve_part_two(input: &Vec<String>) -> i128 {
    let sequences: Vec<Vec<i128>> = input
    .iter()
    .map(|line| {
        line.split_whitespace()
            .into_iter()
            .map(|string| string.parse::<i128>().unwrap())
            .collect::<Vec<i128>>()
    })
    .collect();
    sequences.iter().map(|sequence| find_previous_value(sequence)).sum()
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
    fn day9_example_input_part_one() {
        let input = get_input("./src/day9/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(114, sum_part_one);
    }

    #[test]
    fn day9_input_part_one() {
        let input = get_input("./src/day9/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(1798691765, sum_part_one);
    }

    #[test]
    fn day9_example_input_part_two() {
        let input = get_input("./src/day9/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(2, sum_part_two);
    }

    #[test]
    fn day9_input_part_two() {
        let input = get_input("./src/day9/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(1104, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day9/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day9/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
