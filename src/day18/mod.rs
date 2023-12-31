use num::abs;
use std::{
    cmp::{max, min},
    fs,
};

fn count_enclosed_spaces(input_map: &Vec<Vec<char>>) -> usize {
    let mut map = input_map.clone();
    for row in 0..map.len() {
        let mut is_inside = false;
        let mut last_wall = ' ';
        for col in 0..map[row].len() {
            if map[row][col] == '0' || map[row][col] == '.' {
                if !is_inside {
                    map[row][col] = '0';
                }
                continue;
            }
            match (last_wall, map[row][col]) {
                (_, '|') => is_inside = !is_inside,
                (_, 'L') | (_, 'F') => last_wall = map[row][col],
                ('L', '7') | ('F', 'J') => {
                    is_inside = !is_inside;
                    last_wall = ' '
                }
                ('L', 'J') | ('F', '7') => last_wall = ' ',
                (_, '-') => (),
                (_, _) => panic!("weird layout {} {}", last_wall, map[row][col]),
            }
        }
        // println!("{:?}", map[row]);
    }

    map.iter()
        .map(|row| {
            row.iter()
                .filter(|tile| **tile == '.')
                .collect::<Vec<&char>>()
                .len()
        })
        .sum()
}

fn solve_part_one(instructions: &Vec<Vec<String>>) -> usize {
    let mut layout: Vec<(i128, i128)> = Vec::new();
    let mut current_pos = (0, 0);
    layout.push(current_pos);
    instructions.iter().for_each(|command| {
        let direction_string = command[0].chars().collect::<Vec<char>>()[0];
        let steps = command[1].parse::<i128>().unwrap();
        let direction: (i128, i128);
        match direction_string {
            'R' => direction = (0, 1),
            'D' => direction = (1, 0),
            'L' => direction = (0, -1),
            'U' => direction = (-1, 0),
            _ => panic!("Invalid command"),
        }
        for _ in 0..steps {
            current_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
            layout.push(current_pos);
        }
    });

    let mut bounding_box: ((i128, i128), (i128, i128)) =
        ((i128::MAX, i128::MAX), (i128::MIN, i128::MIN));
    for pos in &layout {
        bounding_box.0 = (min(bounding_box.0 .0, pos.0), min(bounding_box.0 .1, pos.1));
        bounding_box.1 = (max(bounding_box.1 .0, pos.0), max(bounding_box.1 .1, pos.1));
    }

    bounding_box.1 .0 = bounding_box.1 .0 - bounding_box.0 .0;
    bounding_box.1 .1 = bounding_box.1 .1 - bounding_box.0 .1;

    let mut map: Vec<Vec<char>> =
        vec![vec!['.'; (bounding_box.1 .1 + 1) as usize]; (bounding_box.1 .0 + 1) as usize];
    layout.pop();
    for i in 0..layout.len() {
        let last_pos = layout[(i + layout.len() - 1) % layout.len()];
        let next_pos = layout[(i + 1) % layout.len()];
        let cur_pos = layout[i];

        let next_dir = (next_pos.0 - cur_pos.0, next_pos.1 - cur_pos.1);
        let last_dir = (cur_pos.0 - last_pos.0, cur_pos.1 - last_pos.1);

        let symbol: char;
        match (last_dir, next_dir) {
            ((_, 0), (_, 0)) => symbol = '|',
            ((0, _), (0, _)) => symbol = '-',
            ((1, 0), (0, 1)) | ((0, -1), (-1, 0)) => symbol = 'L',
            ((1, 0), (0, -1)) | ((0, 1), (-1, 0)) => symbol = 'J',
            ((-1, 0), (0, -1)) | ((0, 1), (1, 0)) => symbol = '7',
            ((-1, 0), (0, 1)) | ((0, -1), (1, 0)) => symbol = 'F',
            _ => panic!("Weird layout"),
        }
        map[(cur_pos.0 - bounding_box.0 .0) as usize][(cur_pos.1 - bounding_box.0 .1) as usize] =
            symbol
    }

    layout.len() + count_enclosed_spaces(&map)
}

fn calculate_signed_area(corners: &Vec<(i128, i128)>) -> i128 {
    let mut total_area = 0;
    for i in 2..corners.len() {
        let one = corners[i];
        let two = corners[i - 1];
        let area: i128 = one.0 * two.1 - one.1 * two.0;
        total_area += area;
    }
    total_area / 2
}

fn calculate_area(commands: &Vec<((i128, i128), i128)>) -> usize {
    let mut corners: Vec<(i128, i128)> = Vec::new();
    let mut total_steps = 0;
    corners.push((0, 0));
    commands.iter().for_each(|(direction, steps)| {
        let last_corner = *corners.last().unwrap();
        let next_corner = (
            last_corner.0 + direction.0 * steps,
            last_corner.1 + direction.1 * steps,
        );
        total_steps += steps;
        corners.push(next_corner);
    });
    corners.pop();

    (total_steps as f64 * 0.5) as usize + abs(calculate_signed_area(&corners)) as usize + 1
}

#[cfg(test)]
fn solve_part_one_with_part_two_solver(instructions: &Vec<Vec<String>>) -> usize {
    let mut commands: Vec<((i128, i128), i128)> = Vec::new();

    instructions.iter().for_each(|command| {
        let direction_string = command[0].chars().collect::<Vec<char>>()[0];
        let steps = command[1].parse::<i128>().unwrap();
        let direction: (i128, i128);
        match direction_string {
            'R' => direction = (0, 1),
            'D' => direction = (1, 0),
            'L' => direction = (0, -1),
            'U' => direction = (-1, 0),
            _ => panic!("Invalid command"),
        }
        commands.push((direction, steps));
    });

    calculate_area(&commands)
}

fn solve_part_two(instructions: &Vec<Vec<String>>) -> usize {
    let mut commands: Vec<((i128, i128), i128)> = Vec::new();

    instructions.iter().for_each(|command| {
        let hex = command[2][2..(command[2].len() - 1)].to_owned();
        let direction_char = hex.chars().last().unwrap();
        let steps = i128::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();
        let direction: (i128, i128);
        match direction_char {
            '0' => direction = (0, 1),
            '1' => direction = (1, 0),
            '2' => direction = (0, -1),
            '3' => direction = (-1, 0),
            _ => panic!("Invalid command"),
        }
        commands.push((direction, steps));
    });

    calculate_area(&commands)
}

fn get_input(file: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\r\n")
        .into_iter()
        .map(|s| s.split_whitespace().map(|s| s.to_owned()).collect())
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day18/input.txt");
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
    fn day18_example_input_part_one() {
        let input = get_input("./src/day18/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(62, sum_part_one);
    }

    #[test]
    fn day18_input_part_one() {
        let input = get_input("./src/day18/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(46394, sum_part_one);
    }

    #[test]
    fn day18_example_input_part_one_with_part_two_solver() {
        let input = get_input("./src/day18/example_input.txt");
        let sum_part_one = solve_part_one_with_part_two_solver(&input);
        assert_eq!(62, sum_part_one);
    }

    #[test]
    fn day18_input_part_one_with_part_two_solver() {
        let input = get_input("./src/day18/input.txt");
        let sum_part_one = solve_part_one_with_part_two_solver(&input);
        assert_eq!(46394, sum_part_one);
    }

    #[test]
    fn day18_example_input_part_two() {
        let input = get_input("./src/day18/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(952408144115, sum_part_two);
    }

    #[test]
    fn day18_input_part_two() {
        let input = get_input("./src/day18/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(201398068194715, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day18/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day18/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
