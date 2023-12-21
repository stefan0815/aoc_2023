use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn find_start(input: &Vec<Vec<char>>) -> (usize, usize) {
    let mut start: Option<(usize, usize)> = None;
    for row in 0..input.len() {
        let line = &input[row];
        for col in 0..line.len() {
            if line[col] == 'S' {
                start = Some((row, col));
                break;
            }
        }
        if start.is_some() {
            break;
        }
    }
    start.unwrap()
}

fn get_neighbours(input: &Vec<Vec<char>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let pos = (pos.0 as i128, pos.1 as i128);
    let possible_neighbors: Vec<(i128, i128)> = vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ];
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    possible_neighbors.iter().for_each(|pos| {
        if pos.0 < 0
            || pos.0 >= input.len() as i128
            || pos.1 < 0
            || pos.1 >= input[pos.0 as usize].len() as i128
        {
            return;
        }
        let pos = (pos.0 as usize, pos.1 as usize);
        if input[pos.0][pos.1] != '#' {
            neighbors.push(pos);
        }
    });
    neighbors
}

fn convert_pos(input_size: &(usize, usize), pos: &(i128, i128)) -> (usize, usize) {
    let mut row = pos.0;
    if pos.0 < 0 {
        let multiple = (-pos.0) as usize / input_size.0;
        row += ((multiple + 1) * input_size.0) as i128;
    }
    let row = row as usize % input_size.0;

    let mut col = pos.1;
    if pos.1 < 0 {
        let multiple = (-pos.1) as usize / input_size.1;
        col += ((multiple + 1) * input_size.1) as i128;
    }
    let col = col as usize % input_size.1;
    (row, col)
}

fn get_neighbours_part_two(input: &Vec<Vec<char>>, pos: &(i128, i128)) -> Vec<(i128, i128)> {
    let possible_neighbors: Vec<(i128, i128)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut neighbors: Vec<(i128, i128)> = Vec::new();
    possible_neighbors.iter().for_each(|delta| {
        let converted_pos = convert_pos(
            &(input.len(), input[0].len()),
            &(pos.0 + delta.0, pos.1 + delta.1),
        );
        if input[converted_pos.0][converted_pos.1] != '#' {
            neighbors.push(*delta);
        }
    });
    neighbors
}

fn solve_part_one(input: &Vec<Vec<char>>, steps: usize) -> usize {
    let start = find_start(input);
    let mut current_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut neighbours: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    current_positions.insert(start);
    let mut results : Vec<usize> = Vec::new();

    for _ in 0..steps {
        let mut next_positions: HashSet<(usize, usize)> = HashSet::new();
        current_positions.iter().for_each(|pos| {
            if !neighbours.contains_key(pos) {
                neighbours.insert(*pos, get_neighbours(input, pos));
            }
            neighbours[pos].iter().for_each(|next_pos| {
                next_positions.insert(*next_pos);
            });
        });
        current_positions = next_positions;
        results.push(current_positions.len());
    }
    println!("{:?}", results);

    current_positions.len()
}

fn solve_part_two(input: &Vec<Vec<char>>, steps: usize) -> usize {
    let start = find_start(input);
    let mut current_positions: HashSet<(i128, i128)> = HashSet::new();
    let mut neighbours_delta: HashMap<(usize, usize), Vec<(i128, i128)>> = HashMap::new();
    current_positions.insert((start.0 as i128, start.1 as i128));

    let mut results : Vec<usize> = Vec::new();
    for _ in 0..steps {
        let mut next_positions: HashSet<(i128, i128)> = HashSet::new();
        current_positions.iter().for_each(|pos| {
            let converted_pos = convert_pos(&(input.len(), input[0].len()), pos);
            if !neighbours_delta.contains_key(&converted_pos) {
                neighbours_delta.insert(converted_pos, get_neighbours_part_two(input, pos));
            }
            neighbours_delta[&converted_pos].iter().for_each(|delta| {
                next_positions.insert((pos.0 + delta.0, pos.1 + delta.1));
            });
        });
        current_positions = next_positions;
        results.push(current_positions.len());
    }
    println!("{:?}", results);
    current_positions.len()
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\r\n")
        .into_iter()
        .map(|s| s.chars().collect())
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day21/input.txt");
    let sum_part_one = solve_part_one(&input, 64);
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input, 26501365);
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn day21_example_input_part_one() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_one = solve_part_one(&input, 6);
        assert_eq!(16, sum_part_one);
    }

    #[test]
    fn day21_input_part_one() {
        let input = get_input("./src/day21/input.txt");
        let sum_part_one = solve_part_one(&input, 64);
        assert_eq!(3649, sum_part_one);
    }

    #[test]
    fn day21_input_part_one_500_steps() {
        let input = get_input("./src/day21/input.txt");
        let sum_part_one = solve_part_one(&input, 500);
        assert_eq!(3649, sum_part_one);
    }

    #[test]
    fn day21_example_input_part_two_6_steps() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_two = solve_part_two(&input, 6);
        assert_eq!(16, sum_part_two);
    }

    #[test]
    fn day21_example_input_part_two_10_steps() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_two = solve_part_two(&input, 10);
        assert_eq!(50, sum_part_two);
    }

    #[test]
    fn day21_example_input_part_two_50_steps() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_two = solve_part_two(&input, 50);
        assert_eq!(1594, sum_part_two);
    }

    #[test]
    fn day21_example_input_part_two_100_steps() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_two = solve_part_two(&input, 100);
        assert_eq!(6536, sum_part_two);
    }

    #[test]
    fn day21_example_input_part_two_500_steps() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_two = solve_part_two(&input, 500);
        assert_eq!(167004, sum_part_two);
    }

    #[test]
    fn day21_example_input_part_two_1000_steps() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_two = solve_part_two(&input, 1000);
        assert_eq!(668697, sum_part_two);
    }

    #[test]
    fn day21_example_input_part_two_5000_steps() {
        let input = get_input("./src/day21/example_input.txt");
        let sum_part_two = solve_part_two(&input, 5000);
        assert_eq!(16733044, sum_part_two);
    }

    #[test]
    fn day21_input_part_two() {
        let input = get_input("./src/day21/input.txt");
        let sum_part_two = solve_part_two(&input, 26501365);
        assert_eq!(240853834793347, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day21/input.txt");
        b.iter(|| solve_part_one(&input, 64))
    }

    // #[bench]
    // fn bench_part_two(b: &mut Bencher) {
    //     let input = get_input("./src/day21/input.txt");
    //     b.iter(|| solve_part_two(&input))
    // }
}
