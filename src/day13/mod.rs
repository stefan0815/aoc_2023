use std::fs;

fn get_row_reflection_value(mirror: &Vec<Vec<char>>) -> usize {
    for row in 1..mirror.len() {
        let mut found_mirror = true;
        if mirror[row] == mirror[row - 1] {
            let mut reflection_row = (row as i128 - 1, row as i128);
            loop {
                if reflection_row.0 < 0 || reflection_row.1 >= mirror.len() as i128 {
                    break;
                }
                if mirror[reflection_row.0 as usize] != mirror[reflection_row.1 as usize] {
                    found_mirror = false;
                    break;
                }
                reflection_row.0 -= 1;
                reflection_row.1 += 1;
            }
            if found_mirror {
                return row;
            }
        }
    }
    return 0;
}

fn flip_to_col(mirror: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut col_based_mirror: Vec<Vec<char>> = vec![vec![]; mirror[0].len()];
    for row in mirror {
        for char_index in 0..row.len() {
            col_based_mirror[char_index].push(row[char_index]);
        }
    }
    col_based_mirror
}

fn count_differences(
    reflection_one: &Vec<char>,
    reflection_two: &Vec<char>,
) -> (usize, Vec<usize>) {
    if reflection_one == reflection_two {
        return (0, vec![]);
    }
    reflection_one
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if reflection_two[i] == *val {
                return (0, vec![]);
            }
            return (1, vec![i]);
        })
        .reduce(|(diff_one, loc_one), (diff_two, loc_two)| {
            (diff_one + diff_two, [loc_one, loc_two].concat())
        })
        .unwrap()
}

fn find_reflection_row_with_smudged_mirror(mirror: &Vec<Vec<char>>) -> Option<usize> {
    for row in 1..mirror.len() {
        let mut differences = 0;
        let diff = count_differences(&mirror[row], &mirror[row - 1]).0;

        if diff <= 1 {
            let mut reflection_row = (row as i128 - 1, row as i128);
            loop {
                if reflection_row.0 < 0 || reflection_row.1 >= mirror.len() as i128 {
                    break;
                }
                differences += count_differences(
                    &mirror[reflection_row.0 as usize],
                    &mirror[reflection_row.1 as usize],
                )
                .0;

                if differences > 1 {
                    break;
                }
                reflection_row.0 -= 1;
                reflection_row.1 += 1;
            }
            if differences == 1 {
                return Some(row);
            }
        }
    }
    None
}

fn solve_part_one(mirrors: &Vec<Vec<Vec<char>>>) -> usize {
    mirrors
        .iter()
        .map(|mirror| {
            let row_value = get_row_reflection_value(mirror);
            let col_value = get_row_reflection_value(&flip_to_col(mirror));
            row_value * 100 + col_value
        })
        .sum()
}

fn solve_part_two(mirrors: &Vec<Vec<Vec<char>>>) -> usize {
    mirrors
        .iter()
        .map(|mirror| {
            let smudge_row_mirror = find_reflection_row_with_smudged_mirror(mirror);
            let smudge_col_mirror = find_reflection_row_with_smudged_mirror(&flip_to_col(mirror));
            if smudge_row_mirror.is_some() {
                return smudge_row_mirror.unwrap() * 100;
            } 
            smudge_col_mirror.unwrap()
        })
        .sum()
}

fn get_input(file: &str) -> Vec<Vec<Vec<char>>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let mirrors: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .into_iter()
        .map(|s| {
            s.to_owned()
                .lines()
                .into_iter()
                .map(|s| s.to_owned().chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();
    return mirrors;
}

pub fn solver() {
    let input = get_input("./src/day13/input.txt");
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
    fn day13_example_input_part_one() {
        let input = get_input("./src/day13/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(405, sum_part_one);
    }

    #[test]
    fn day13_input_part_one() {
        let input = get_input("./src/day13/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(29846, sum_part_one);
    }

    #[test]
    fn day13_example_input_part_two() {
        let input = get_input("./src/day13/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(400, sum_part_two);
    }

    #[test]
    fn day13_input_part_two() {
        let input = get_input("./src/day13/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(25401, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day13/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day13/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
