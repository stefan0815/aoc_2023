use std::{collections::HashMap, fs};

fn flip_to_col(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut col_based_platform: Vec<Vec<char>> = vec![vec![]; platform[0].len()];
    for row in platform {
        for char_index in 0..row.len() {
            col_based_platform[char_index].push(row[char_index]);
        }
    }
    col_based_platform
}

fn calculate_load(platform: &Vec<Vec<char>>) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter().filter(|tile| **tile == 'O').count() * (platform.len() - row_index)
        })
        .sum()
}

fn reverse_platform(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut reversed_platform: Vec<Vec<char>> = platform.to_vec();
    for row in reversed_platform.iter_mut() {
        row.reverse();
    }
    reversed_platform
}

fn tilt_west(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_platform: Vec<Vec<char>> = platform.to_vec();

    for row in tilted_platform.iter_mut() {
        let mut i = 0;
        while i < row.len() {
            let square_stone_index = row[i..].iter().position(|&tile| tile == '#');
            let mut next_stone = row.len();
            if square_stone_index.is_some() {
                next_stone = i + square_stone_index.unwrap()
            }
            let round_stone_count = row[i..next_stone]
                .iter()
                .filter(|tile| **tile == 'O')
                .count();
            for j in i..next_stone {
                if j - i < round_stone_count {
                    row[j] = 'O';
                } else {
                    row[j] = '.';
                }
            }
            i = next_stone + 1;
        }
    }
    tilted_platform
}

fn tilt_north(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    flip_to_col(&tilt_west(&flip_to_col(platform)))
}

fn tilt_east(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    reverse_platform(&tilt_west(&reverse_platform(&platform)))
}

fn tilt_south(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    flip_to_col(&tilt_east(&flip_to_col(platform)))
}

fn solve_part_one(platform: &Vec<Vec<char>>) -> usize {
    calculate_load(&tilt_north(platform))
}

fn solve_part_two(platform: &Vec<Vec<char>>, cycles: i128, cached: bool) -> usize {
    let mut tilted_platform = platform.to_vec();
    let mut i = 0;
    let mut cache: HashMap<(Vec<Vec<char>>, i128), i128> = HashMap::new();
    let tilts = cycles * 4;
    let mut skipped = false;
    while i < tilts {
        let direction = i % 4;
        if cached && !skipped && cache.contains_key(&(tilted_platform.clone(), direction)) {
            let cached_index = cache[&(tilted_platform.clone(), direction)];
            let skip = i - cached_index;
            let todo = tilts - i;
            i += (todo / skip) as i128 * skip;
            skipped = true;
        }

        cache.insert((tilted_platform.clone(), direction), i);

        match direction {
            0 => tilted_platform = tilt_north(&tilted_platform),
            1 => tilted_platform = tilt_west(&tilted_platform),
            2 => tilted_platform = tilt_south(&tilted_platform),
            3 => tilted_platform = tilt_east(&tilted_platform),
            _ => panic!("Modulo is broken"),
        }
        i += 1;
    }
    calculate_load(&tilted_platform)
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\n")
        .into_iter()
        .map(|s| s.to_owned().chars().collect::<Vec<char>>())
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day14/input.txt");
    let sum_part_one = solve_part_one(&input);
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input, 1000000000, true);
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn day14_example_input_part_one() {
        let input = get_input("./src/day14/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(136, sum_part_one);
    }

    #[test]
    fn day14_input_part_one() {
        let input = get_input("./src/day14/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(108840, sum_part_one);
    }

    #[test]
    fn day14_example_input_part_two_cache_test() {
        let input = get_input("./src/day14/example_input.txt");
        let sum_part_two_cached = solve_part_two(&input, 1231, true);
        let sum_part_two = solve_part_two(&input, 1231, false);
        assert_eq!(sum_part_two, sum_part_two_cached);
    }

    #[test]
    fn day14_example_input_part_two() {
        let input = get_input("./src/day14/example_input.txt");
        let sum_part_two = solve_part_two(&input, 1000000000, true);
        assert_eq!(64, sum_part_two);
    }

    #[test]
    fn day14_input_part_two() {
        let input = get_input("./src/day14/input.txt");
        let sum_part_two = solve_part_two(&input, 1000000000, true);
        assert_eq!(103445, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day14/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day14/input.txt");
        b.iter(|| solve_part_two(&input, 1000000000, true))
    }
}
