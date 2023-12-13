use std::fs;

fn get_row_reflection_value(pattern: &Vec<String>) -> usize {
    for row in 1..pattern.len() {
        let mut found_mirror = true;
        if pattern[row] == pattern[row - 1] {
            let mut reflection_row = (row as i128 - 1, row as i128);
            loop {
                if reflection_row.0 < 0 || reflection_row.1 >= pattern.len() as i128 {
                    break;
                }
                if pattern[reflection_row.0 as usize] != pattern[reflection_row.1 as usize]
                {
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

fn solve_part_one(patterns: &Vec<Vec<String>>) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            let row_value = get_row_reflection_value(pattern);

            let mut col_based_pattern: Vec<String> = vec!["".to_owned(); pattern[0].len()];
            for row in pattern{
                let row_chars = row.chars().collect::<Vec<char>>();
                for char_index in 0..row_chars.len(){
                    col_based_pattern[char_index].push(row_chars[char_index]);
                }
            }

            let col_value = get_row_reflection_value(&col_based_pattern);
            row_value * 100 + col_value
        })
        .sum()
}

fn solve_part_two(_input: &Vec<Vec<String>>) -> u128 {
    0
}

fn get_input(file: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let patterns: Vec<Vec<String>> = input
        .split("\n\n")
        .into_iter()
        .map(|s| {
            s.to_owned()
                .lines()
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .collect();
    return patterns;
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
        assert_eq!(71503, sum_part_two);
    }

    #[test]
    fn day13_input_part_two() {
        let input = get_input("./src/day13/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(41513103, sum_part_two);
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
