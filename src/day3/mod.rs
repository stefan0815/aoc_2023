use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    number: u32,
    start: (usize, usize),
}

fn find_start(line: &Vec<char>, index: usize) -> usize {
    for i in (0..(index + 1)).rev() {
        if !line[i].is_numeric() {
            return i + 1;
        }
    }
    0
}

fn find_part(line: &Vec<char>, row: usize, col: usize) -> Part {
    let start = find_start(&line, col);
    let mut number_string: String = String::new();
    for i in start..line.len() {
        if !line[i].is_numeric() {
            break;
        }
        number_string.push(line[i]);
    }
    Part {
        number: number_string.parse::<u32>().unwrap(),
        start: (row, start),
    }
}

fn find_parts_around(schematic: &Vec<Vec<char>>, (y, x): (usize, usize)) -> HashSet<Part> {
    let mut parts: HashSet<Part> = HashSet::new();
    let row_range = (max(1, y) - 1)..(min(y, schematic.len() - 1) + 2);
    for row in row_range {
        let col_range = (max(1, x) - 1)..(min(x, schematic[row].len() - 1) + 2);
        for col in col_range {
            if schematic[row][col].is_numeric() {
                parts.insert(find_part(&schematic[row], row, col));
            }
        }
    }
    parts
}

fn solve_part_one(schematic: &Vec<Vec<char>>) -> u32 {
    let mut parts: HashSet<Part> = HashSet::new();
    for row in 0..schematic.len() {
        let line = &schematic[row];
        for col in 0..line.len() {
            let char = line[col];
            if !char.is_numeric() && char != '.' {
                parts.extend(find_parts_around(&schematic, (row, col)));
            }
        }
    }

    parts.iter().map(|part| part.number).sum()
}

fn solve_part_two(schematic: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    for row in 0..schematic.len() {
        let line = &schematic[row];
        for col in 0..line.len() {
            let char = line[col];
            if char == '*' {
                let parts = find_parts_around(&schematic, (row, col));
                if parts.len() == 2 {
                    let numbers: Vec<u32> = parts.iter().map(|part| part.number).collect();
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }

    sum
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<String> = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let schematic: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    return schematic;
}

pub fn solver() {
    let input = get_input("./src/day2/input.txt");
    let sum_part_one = solve_part_one(&input);
    let sum_part_two = solve_part_two(&input);
    println!("Part 1: {sum_part_one}");
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_example_input_part_one() {
        let input = get_input("./src/day3/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(4361, sum_part_one);
    }

    #[test]
    fn day3_input_part_one() {
        let input = get_input("./src/day3/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(553079, sum_part_one);
    }

    #[test]
    fn day3_example_input_part_two() {
        let input = get_input("./src/day3/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(467835, sum_part_two);
    }

    #[test]
    fn day3_input_part_two() {
        let input = get_input("./src/day3/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(84363105, sum_part_two);
    }
}
