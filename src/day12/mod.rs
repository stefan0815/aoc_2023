use rayon::prelude::*;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    sync::Mutex,
};

fn get_num_valid_arrangements(
    springs: &[char],
    groups: &[usize],
    possible_failures: usize,
    necessary_failures: usize,
) -> usize {
    if springs.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        return 0;
    }

    if groups.is_empty(){
        if springs.iter().any(|spring| *spring == '#'){
            return 0;
        }
        return 1;
    }

    if springs.len() < necessary_failures + groups.len() - 1 {
        return 0;
    }


    if possible_failures < necessary_failures {
        return 0;
    }

    match springs[0] {
        '.' => return get_num_valid_arrangements(&springs[1..], &groups, possible_failures, necessary_failures),
        '?' => {
            return get_num_valid_arrangements(&springs[1..], &groups, possible_failures - 1, necessary_failures)
                + get_num_valid_arrangements(
                    &[['#'].as_slice(), &springs[1..]].concat(),
                    &groups,
                    possible_failures,
                    necessary_failures
                )
        }
        '#' => {
            if groups.is_empty() {
                return 0;
            }
            if springs.len() < groups[0] || springs[..groups[0]].iter().any(|spring| *spring == '.')
            {
                return 0;
            }

            if springs.len() > groups[0] {
                if springs[groups[0]] == '#' {
                    return 0;
                }
                let mut new_possible_failures = possible_failures;
                if springs[groups[0]] == '?' {
                    new_possible_failures -= 1;
                }
                return get_num_valid_arrangements(
                    &springs[(groups[0] + 1)..],
                    &groups[1..],
                    new_possible_failures,
                    necessary_failures - groups[0]
                );
            }

            return get_num_valid_arrangements(&springs[groups[0]..], &groups[1..], 0, necessary_failures - groups[0]);
        }
        _ => panic!("Illegal symbol"),
    }
}

fn solve_part_one(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            let split: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
            let springs: Vec<char> = split[0].chars().collect();
            let groups: Vec<usize> = split[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let possible_failures = springs.iter().filter(|spring| **spring != '.').count();
            let necessary_failures:usize = groups.iter().sum::<usize>();

            get_num_valid_arrangements(&springs, &groups, possible_failures, necessary_failures)
        })
        .sum()
}

// fn solve_part_two_without_cache(input: &Vec<String>) -> usize {
//     input
//         .par_iter()
//         .map(|line| {
//             let split: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
//             let springs: Vec<char> = split[0].chars().collect();
//             let groups: Vec<usize> = split[1]
//                 .split(',')
//                 .map(|s| s.parse::<usize>().unwrap())
//                 .collect();
//             let mut adapted_springs = springs.to_vec();
//             adapted_springs.push('?');
//             adapted_springs.extend(springs.to_vec());
//             adapted_springs.push('?');
//             adapted_springs.extend(springs.to_vec());
//             adapted_springs.push('?');
//             adapted_springs.extend(springs.to_vec());
//             adapted_springs.push('?');
//             adapted_springs.extend(springs.to_vec());
//             let mut adapted_groups = groups.to_vec();
//             adapted_groups.extend(groups.to_vec());
//             adapted_groups.extend(groups.to_vec());
//             adapted_groups.extend(groups.to_vec());
//             adapted_groups.extend(groups.to_vec());

//             let possible_failures = adapted_springs
//                 .iter()
//                 .filter(|spring| **spring != '.')
//                 .count();
//             let necessary_failures:usize = adapted_groups.iter().sum::<usize>();
            
//             let num_valid_arrangements =
//                 get_num_valid_arrangements(&adapted_springs, &adapted_groups, possible_failures, necessary_failures);
//             num_valid_arrangements
//         })
//         .sum()
// }

fn solve_part_two(input: &Vec<String>, file_path: &str) -> usize {
    let previous_results =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let previous_results: Vec<String> = previous_results
        .split("\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let file = Mutex::new(file);

    input
        .par_iter()
        .map(|line| {
            let split: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
            let springs: Vec<char> = split[0].chars().collect();
            let groups: Vec<usize> = split[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let result_name = format!("{:?}|{:?}", springs, groups);
            for previous_result in &previous_results {
                if previous_result.starts_with(&result_name) {
                    let loaded_value = previous_result
                        .split('|')
                        .collect::<Vec<&str>>()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    return loaded_value;
                }
            }

            let mut adapted_springs = springs.to_vec();
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            adapted_springs.push('?');
            adapted_springs.extend(springs.to_vec());
            let mut adapted_groups = groups.to_vec();
            adapted_groups.extend(groups.to_vec());
            adapted_groups.extend(groups.to_vec());
            adapted_groups.extend(groups.to_vec());
            adapted_groups.extend(groups.to_vec());

            let possible_failures = adapted_springs
                .iter()
                .filter(|spring| **spring != '.')
                .count();
            let necessary_failures:usize = adapted_groups.iter().sum::<usize>();
            
            let num_valid_arrangements =
                get_num_valid_arrangements(&adapted_springs, &adapted_groups, possible_failures, necessary_failures);
            let output = format!("{:?}|{:?}|{num_valid_arrangements}\n", springs, groups);
            file.lock().unwrap().write_all(output.as_bytes()).unwrap();
            num_valid_arrangements
        })
        .sum()
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let input: Vec<String> = input
        .split("\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    return input;
}

pub fn solver() {
    let input = get_input("./src/day12/input.txt");
    let sum_part_one = solve_part_one(&input);
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input, "./src/day12/part_two_output.txt");
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn day12_example_input_part_one() {
        let input = get_input("./src/day12/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(21, sum_part_one);
    }

    #[test]
    fn day12_input_part_one() {
        let input = get_input("./src/day12/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(7460, sum_part_one);
    }

    #[test]
    fn day12_example_input_part_two() {
        let input = get_input("./src/day12/example_input.txt");
        let sum_part_two = solve_part_two(&input, "./src/day12/part_two_example_output.txt");
        assert_eq!(525152, sum_part_two);
    }

    #[test]
    fn day12_input_part_two() {
        let input = get_input("./src/day12/input.txt");
        let sum_part_two = solve_part_two(&input, "./src/day12/part_two_output.txt");
        assert_eq!(0, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day12/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    // #[bench]
    // fn bench_part_two(b: &mut Bencher) {
    //     let input = get_input("./src/day12/example_input.txt");
    //     b.iter(|| solve_part_two_without_cache(&input))
    // }
}
