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

fn solve_part_one(input: &Vec<Vec<char>>, steps: usize) -> usize {
    let start = find_start(input);
    let mut current_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut neighbours: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    current_positions.insert(start);

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
    }
    current_positions.len()
}

fn solve_part_two(_: &Vec<Vec<char>>) -> usize {
    0
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
    let sum_part_two = solve_part_two(&input);
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

    // #[test]
    // fn day21_example_input_part_two() {
    //     let input = get_input("./src/day21/example_input.txt");
    //     let sum_part_two = solve_part_two(&input);
    //     assert_eq!(167409079868000, sum_part_two);
    // }

    #[test]
    fn day21_input_part_two() {
        let input = get_input("./src/day21/input.txt");
        let sum_part_two = solve_part_two(&input);
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
