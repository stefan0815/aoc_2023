use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    number: u32,
    start: (usize, usize),
}

fn find_start(line: &Vec<char>, index: usize) -> usize {
    for i in (0..index).rev() {
        if !line[i].is_numeric() {
            return i + 1;
        }
    }
    return 0;
}

fn find_end(line: &Vec<char>, index: usize) -> usize {
    for i in index..line.len() {
        if !line[i].is_numeric() {
            return i;
        }
    }
    return line.len();
}

fn find_part(line: &Vec<char>, row: usize, col: usize) -> Part {
    let start = find_start(&line, col);
    let end = find_end(&line, col);
    let value = String::from_iter(line[start..end].iter())
        .parse::<u32>()
        .unwrap();
    Part {
        number: value,
        start: (row, start),
    }
}

fn find_parts_around(schematic: &Vec<Vec<char>>, (y, x): (usize, usize)) -> HashSet<Part> {
    let mut parts: HashSet<Part> = HashSet::new();
    let row_range = (max(0, y - 1))..(min(y + 2, schematic.len()));
    for row in row_range {
        let col_range = (max(0, x - 1))..(min(x + 2, schematic[row].len()));
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

    schematic.iter().enumerate().for_each(|(row, &ref line)| {
        line.iter()
            .enumerate()
            .filter(|(_, &char)| !char.is_numeric() && char != '.')
            .for_each(|(col, &_)| parts.extend(find_parts_around(&schematic, (row, col))))
    });

    parts.iter().map(|part| part.number).sum()
}

fn solve_part_two(schematic: &Vec<Vec<char>>) -> u32 {
    schematic
        .iter()
        .enumerate()
        .map(|(row, &ref line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &char)| char == '*')
                .map(|(col, &_)| find_parts_around(&schematic, (row, col)))
                .filter(|parts| parts.len() == 2)
                .map(|two_part_gears| {
                    two_part_gears
                        .iter()
                        .map(|part| part.number)
                        .reduce(|part1, part2| part1 * part2)
                        .unwrap()
                })
                .sum::<u32>()
        })
        .sum::<u32>()
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
    let input = get_input("./src/day3/input.txt");
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
