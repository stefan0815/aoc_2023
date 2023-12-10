use std::fs;

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'S' {
                return (row, col);
            }
        }
    }
    panic!("No start found");
}

fn find_start_directions(
    map: &Vec<Vec<char>>,
    (row_start, col_start): &(usize, usize),
) -> [(usize, usize); 2] {
    let mut next_positions: Vec<(usize, usize)> = Vec::new();
    if row_start + 1 < map.len() {
        match map[row_start + 1][*col_start] {
            '|' | 'L' | 'J' => next_positions.push((row_start + 1, *col_start)),
            _ => (),
        }
    }

    if *row_start > 0 {
        match map[row_start - 1][*col_start] {
            '|' | '7' | 'F' => next_positions.push((row_start - 1, *col_start)),
            _ => (),
        }
    }

    if col_start + 1 < map[*row_start].len() {
        match map[*row_start][col_start + 1] {
            '-' | 'J' | '7' => next_positions.push((*row_start, col_start + 1)),
            _ => (),
        }
    }

    if *col_start > 0 {
        match map[*row_start][col_start - 1] {
            '-' | 'L' | 'F' => next_positions.push((*row_start, col_start - 1)),
            _ => (),
        }
    }

    if next_positions.len() == 2 {
        return [next_positions[0], next_positions[1]];
    }
    panic!("Start connections more or less not found.")
}

fn find_connecting_positions(pipe: char, (row, col): (usize, usize)) -> [(usize, usize); 2] {
    match pipe {
        '|' => return [(row - 1, col), (row + 1, col)],
        '-' => return [(row, col - 1), (row, col + 1)],
        'J' => return [(row - 1, col), (row, col - 1)],
        'L' => return [(row - 1, col), (row, col + 1)],
        '7' => return [(row + 1, col), (row, col - 1)],
        'F' => return [(row + 1, col), (row, col + 1)],
        _ => panic!("Should have been a pipe"),
    }
}

fn find_loop(map: &Vec<Vec<char>>, start: &(usize, usize)) -> (Vec<(usize, usize)>, usize) {
    let mut last_positions = [vec![*start], vec![*start]];
    let mut current_positions = find_start_directions(map, start);
    let mut steps = 1;
    loop {
        if current_positions[0] == current_positions[1] {
            last_positions[0].push(current_positions[0]);
            break;
        }
        for i in 0..2 {
            let last_position = last_positions[i].last().unwrap();
            let possible_next_positions = find_connecting_positions(
                map[current_positions[i].0][current_positions[i].1],
                current_positions[i],
            );
            let next_position = **possible_next_positions
                .iter()
                .filter(|position| *position != last_position)
                .collect::<Vec<&(usize, usize)>>()
                .first()
                .unwrap();
            last_positions[i].push(current_positions[i]);
            current_positions[i] = next_position;
        }

        steps += 1
    }

    let mut pipe_loop = last_positions[0].clone();
    pipe_loop.extend(
        last_positions[1]
            .iter()
            .skip(1)
            .rev()
            .map(|pos| *pos)
            .collect::<Vec<(usize, usize)>>(),
    );
    (pipe_loop, steps)
}

fn clean_map(input_map: &Vec<Vec<char>>, pipe_loop: &Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let mut cleaned_map = input_map.clone();
    for row in 0..cleaned_map.len() {
        for col in 0..cleaned_map[row].len() {
            if cleaned_map[row][col] == '.' {
                continue;
            }
            if !pipe_loop.contains(&(row, col)) {
                cleaned_map[row][col] = '.'
            }
        }
    }

    let start_direction_one = (
        pipe_loop[1].0 as i128 - pipe_loop[0].0 as i128,
        pipe_loop[1].1 as i128 - pipe_loop[0].1 as i128,
    );
    let start_direction_two = (
        pipe_loop.last().unwrap().0 as i128 - pipe_loop[0].0 as i128,
        pipe_loop.last().unwrap().1 as i128 - pipe_loop[0].1 as i128,
    );

    let start_symbol: char;
    match (start_direction_one, start_direction_two) {
        ((_, 0), (_, 0)) => start_symbol = '|',
        ((0, _), (0, _)) => start_symbol = '-',
        ((0, 1), (-1, 0)) | ((-1, 0), (0, 1)) => start_symbol = 'L',
        ((0, -1), (-1, 0)) | ((-1, 0), (0, -1)) => start_symbol = 'J',
        ((0, -1), (1, 0)) | ((1, 0), (0, -1)) => start_symbol = '7',
        ((0, 1), (1, 0)) | ((1, 0), (0, 1)) => start_symbol = 'F',
        _ => panic!("Start block does not really connect"),
    }

    cleaned_map[pipe_loop[0].0][pipe_loop[0].1] = start_symbol;
    cleaned_map
}

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
                ('L', '7') | ('F', 'J')=> {
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

fn solve_part_one(map: &Vec<Vec<char>>) -> usize {
    let start = find_start(map);
    find_loop(map, &start).1
}

fn solve_part_two(map: &Vec<Vec<char>>) -> usize {
    let start = find_start(map);
    let pipe_loop = find_loop(map, &start).0;
    let cleaned_map = clean_map(map, &pipe_loop);
    count_enclosed_spaces(&cleaned_map)
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let chars: Vec<Vec<char>> = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned().chars().collect::<Vec<char>>())
        .collect();
    return chars;
}

pub fn solver() {
    let input = get_input("./src/day10/input.txt");
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
    fn day10_example_input_part_one() {
        let input = get_input("./src/day10/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(4, sum_part_one);
    }

    #[test]
    fn day10_example_input_two_part_one() {
        let input = get_input("./src/day10/example_input_two.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(8, sum_part_one);
    }

    #[test]
    fn day10_input_part_one() {
        let input = get_input("./src/day10/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(6828, sum_part_one);
    }

    #[test]
    fn day10_example_input_part_two() {
        let input = get_input("./src/day10/example_input_part_two.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(4, sum_part_two);
    }

    #[test]
    fn day10_example_input_two_part_two() {
        let input = get_input("./src/day10/example_input_two_part_two.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(8, sum_part_two);
    }

    #[test]
    fn day10_example_input_three_part_two() {
        let input = get_input("./src/day10/example_input_three_part_two.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(10, sum_part_two);
    }

    #[test]
    fn day10_input_part_two() {
        let input = get_input("./src/day10/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(459, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day10/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day10/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
