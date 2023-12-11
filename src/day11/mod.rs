use std::{
    cmp::{max, min},
    fs,
};

use num::abs;

fn find_empty_space(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    for row in 0..universe.len() {
        if !universe[row].contains(&'#') {
            empty_rows.push(row);
        }
    }

    for col in 0..universe[0].len() {
        let mut column_contains_universe = false;
        for row in 0..universe.len() {
            if universe[row][col] == '#' {
                column_contains_universe = true;
                break;
            }
        }
        if column_contains_universe {
            continue;
        }
        empty_cols.push(col);
    }
    (empty_rows, empty_cols)
}

fn expand_universe(universe: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_universe = universe.clone();
    let mut inserted_rows = 0;
    for row in 0..universe.len() {
        if !universe[row].contains(&'#') {
            expanded_universe.insert(row + inserted_rows, universe[row].clone());
            inserted_rows += 1;
        }
    }

    let mut inserted_cols = 0;
    for col in 0..universe[0].len() {
        let mut column_contains_universe = false;
        for row in 0..universe.len() {
            if universe[row][col] == '#' {
                column_contains_universe = true;
                break;
            }
        }
        if column_contains_universe {
            continue;
        }
        for row in 0..expanded_universe.len() {
            expanded_universe[row].insert(col + inserted_cols, '.');
        }
        inserted_cols += 1;
    }

    expanded_universe
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<(i128, i128)> {
    let mut galaxies: Vec<(i128, i128)> = Vec::new();
    for row in 0..universe.len() {
        universe[row]
            .iter()
            .enumerate()
            .filter(|(_, symbol)| **symbol == '#')
            .for_each(|(col, _)| galaxies.push((row as i128, col as i128)));
    }
    galaxies
}

fn get_distance_between_galaxies(
    start_galaxy: (i128, i128),
    end_galaxy: (i128, i128),
    (empty_rows, empty_cols): (&Vec<usize>, &Vec<usize>),
    factor: i128,
) -> usize {
    let start_row = min(start_galaxy.0, end_galaxy.0) as usize;
    let end_row = max(start_galaxy.0, end_galaxy.0) as usize;
    let mut empty_rows_inbetween = 0;
    for &row in empty_rows {
        if start_row < row && row < end_row {
            empty_rows_inbetween += 1
        }
        if row > end_row {
            break;
        }
    }

    let start_col = min(start_galaxy.1, end_galaxy.1) as usize;
    let end_col = max(start_galaxy.1, end_galaxy.1) as usize;
    let mut empty_cols_inbetween = 0;
    for &col in empty_cols {
        if start_col < col && col < end_col {
            empty_cols_inbetween += 1
        }
        if col > end_col {
            break;
        }
    }
    let distance_without_empty_space =
        abs(start_galaxy.0 - end_galaxy.0) + abs(start_galaxy.1 - end_galaxy.1);
    (distance_without_empty_space + (empty_rows_inbetween + empty_cols_inbetween) * (factor - 1)) as usize
}

fn solve_part_one(universe: &Vec<Vec<char>>) -> usize {
    let expanded_universe = expand_universe(universe);
    let galaxies = find_galaxies(&expanded_universe);
    let mut combined_distance = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let distance = (abs(galaxies[i].0 - galaxies[j].0) + abs(galaxies[i].1 - galaxies[j].1)) as usize;
            combined_distance +=distance;
        }
    }
    combined_distance
}

fn solve_part_two(universe: &Vec<Vec<char>>, factor: i128) -> usize {
    let galaxies = find_galaxies(&universe);
    let (empty_rows, empty_cols) = find_empty_space(universe);
    let mut combined_distance = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let distance = get_distance_between_galaxies(
                galaxies[i],
                galaxies[j],
                (&empty_rows, &empty_cols),
                factor,
            );
            combined_distance += distance;
        }
    }
    combined_distance
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let groups: Vec<Vec<char>> = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned().chars().collect::<Vec<char>>())
        .collect();
    return groups;
}

pub fn solver() {
    let input = get_input("./src/day11/input.txt");
    let sum_part_one = solve_part_one(&input);
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input, 1000000);
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn day11_example_input_part_one() {
        let input = get_input("./src/day11/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(374, sum_part_one);
    }

    #[test]
    fn day11_input_part_one() {
        let input = get_input("./src/day11/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(9723824, sum_part_one);
    }

    #[test]
    fn day11_example_input_part_two() {
        let input = get_input("./src/day11/example_input.txt");
        let sum_part_two = solve_part_two(&input, 2);
        assert_eq!(374, sum_part_two);
    }

    #[test]
    fn day11_example_input_part_two_10() {
        let input = get_input("./src/day11/example_input.txt");
        let sum_part_two = solve_part_two(&input, 10);
        assert_eq!(1030, sum_part_two);
    }

    #[test]
    fn day11_example_input_part_two_100() {
        let input = get_input("./src/day11/example_input.txt");
        let sum_part_two = solve_part_two(&input, 100);
        assert_eq!(8410, sum_part_two);
    }

    #[test]
    fn day11_input_part_two() {
        let input = get_input("./src/day11/input.txt");
        let sum_part_two = solve_part_two(&input, 1000000);
        assert_eq!(731244261352, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day11/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day11/input.txt");
        b.iter(|| solve_part_two(&input, 1000000))
    }
}
