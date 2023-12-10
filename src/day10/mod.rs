use std::{
    cmp::{max, min},
    fs,
};

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

fn get_bounding_box(pipe_loop: &Vec<(usize, usize)>) -> [(usize, usize); 2] {
    let mut bounding_box = [(usize::MAX, usize::MAX), (0, 0)];
    for position in pipe_loop {
        bounding_box[0] = (
            min(bounding_box[0].0, position.0),
            min(bounding_box[0].1, position.1),
        );
        bounding_box[1] = (
            max(bounding_box[1].0, position.0),
            max(bounding_box[1].1, position.1),
        );
    }
    bounding_box
}

fn is_connected(
    pipe_loop: &Vec<(usize, usize)>,
    cur_pos: (usize, usize),
    other_pos: (usize, usize),
) -> bool {
    if cur_pos == other_pos {
        return false;
    }
    for i in 0..pipe_loop.len() {
        if pipe_loop[i] == cur_pos {
            return pipe_loop[(i + pipe_loop.len() - 1) % pipe_loop.len()] == other_pos
                || pipe_loop[(i + 1) % pipe_loop.len()] == other_pos;
        }
    }
    false
}

fn count_enclosed_spaces(input_map: &Vec<Vec<char>>, pipe_loop: &Vec<(usize, usize)>) -> usize {
    let mut map = input_map.clone();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == '.'{
                continue;
            }
            if !pipe_loop.contains(&(row,col)){
                map[row][col] = '.'
            }
        }
    }
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
            match (map[row][col], last_wall) {
                ('|', _) => is_inside = !is_inside,
                ('L', _) => last_wall = 'L',
                ('F', _) => last_wall = 'F',
                ('7', 'L') => {
                    is_inside = !is_inside;
                    last_wall = ' '
                }
                ('7', _) => last_wall = ' ',
                ('J', 'F') => {
                    is_inside = !is_inside;
                    last_wall = ' '
                }
                ('J', _) => last_wall = ' ',
                (_, _) => (),
            }

            // let is_wall = !is_connected(pipe_loop, (row, col), last_ray_position);
            // if row == 7{
            //     println!("col: {col}, is_wall {is_wall}, pos: {:?}, last_ray_position: {:?}", (row,col), last_ray_position);
            // }
            // let is_wall = !is_connected(pipe_loop, (row,col), last_ray_position);
            // if map[row][col] == '|' || map[row][col] == 'L' || map[row][col] == 'F' {
            //     is_inside = !is_inside;
            // }
            // last_ray_position = (row, col);
        }
        println!("{:?}", map[row]);
    }

    // for row in 0..map.len() {
    //     for col in 0..map[row].len(){

    //     }
    // }
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
    // let bounding_box = get_bounding_box(&pipe_loop);
    count_enclosed_spaces(map, &pipe_loop)
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
        assert_eq!(10, sum_part_two);
    }

    #[test]
    fn day10_input_part_two() {
        let input = get_input("./src/day10/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(9858474970153, sum_part_two);
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
