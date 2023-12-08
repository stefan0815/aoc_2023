use std::{collections::HashMap, fs};
use num::Integer;

fn parse_commands(input: &Vec<String>) -> Vec<usize> {
    let commands: Vec<usize> = input[0]
        .chars()
        .map(|char| match char {
            'L' => 0,
            'R' => 1,
            _ => panic!("Command does not match."),
        })
        .collect();
    commands
}

fn parse_map(input: &Vec<String>) -> HashMap<String, [String; 2]> {
    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    input.iter().skip(2).for_each(|line| {
        let split: Vec<String> = line.split(" = (").map(|s| s.trim().to_owned()).collect();
        let from = split[0].clone();
        let to: Vec<String> = (split[1][..(split[1].len() - 1)])
            .to_owned()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        map.insert(from, [to[0].clone(), to[1].clone()]);
    });
    map
}

fn get_number_of_moves(
    commands: &Vec<usize>,
    map: &HashMap<String, [String; 2]>,
    start_position: &String,
    goal_position_ends_with: &String,
) -> usize {
    let mut moves = 0;
    let mut position: &String = start_position;
    while !position.ends_with(goal_position_ends_with) {
        let command = commands[moves % commands.len()];
        position = &map[position][command];
        moves += 1;
    }
    moves
}

fn solve_part_one(input: &Vec<String>) -> usize {
    let commands = parse_commands(input);
    let map: HashMap<String, [String; 2]> = parse_map(input);

    get_number_of_moves(&commands, &map, &"AAA".to_owned(), &"ZZZ".to_owned())
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let commands = parse_commands(input);
    let map: HashMap<String, [String; 2]> = parse_map(input);

    let positions: Vec<&String> = map
        .keys()
        .filter(|position| position.ends_with('A'))
        .collect();

    positions
        .iter()
        .map(|position| get_number_of_moves(&commands, &map, *position, &"Z".to_owned()))
        .fold(1, |a, b| a.lcm(&b))
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
    let input = get_input("./src/day8/input.txt");
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
    fn day8_example_input_part_one() {
        let input = get_input("./src/day8/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(2, sum_part_one);
    }

    #[test]
    fn day8_example_input_two_part_one() {
        let input = get_input("./src/day8/example_input_two.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(6, sum_part_one);
    }

    #[test]
    fn day8_input_part_one() {
        let input = get_input("./src/day8/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(11567, sum_part_one);
    }

    #[test]
    fn day8_example_input_part_two() {
        let input = get_input("./src/day8/example_input_part_two.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(6, sum_part_two);
    }

    #[test]
    fn day8_input_part_two() {
        let input = get_input("./src/day8/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(9858474970153, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day8/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day8/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
