use std::fs;

fn solve_part_one(input: &Vec<String>) -> usize {
    let times: Vec<usize> = input[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = input[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let races: Vec<(&usize, &usize)> = times.iter().zip(distances.iter()).collect();
    let mut result: usize = 1;
    for (time, record_distance) in races {
        let mut number_of_wins = 0;
        for i in 1..*time {
            let distance = i * (time - i);
            if distance > *record_distance {
                number_of_wins += 1;
            }
        }
        result *= number_of_wins;
    }

    result
}

fn solve_part_two(input: &Vec<String>) -> u128 {
    let time = input[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.to_owned())
        .reduce(|mut a, b| {
            a.push_str(&b);
            a
        })
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let record_distance = input[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.to_owned())
        .reduce(|mut a, b| {
            a.push_str(&b);
            a
        })
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut number_of_wins = 0;
    for i in 1..time {
        let distance = i * (time - i);
        if distance > record_distance {
            number_of_wins += 1;
        }
    }

    number_of_wins
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
    let input = get_input("./src/day6/input.txt");
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
    fn day6_example_input_part_one() {
        let input = get_input("./src/day6/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(288, sum_part_one);
    }

    #[test]
    fn day6_input_part_one() {
        let input = get_input("./src/day6/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(1159152, sum_part_one);
    }

    #[test]
    fn day6_example_input_part_two() {
        let input = get_input("./src/day6/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(71503, sum_part_two);
    }

    #[test]
    fn day6_input_part_two() {
        let input = get_input("./src/day6/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(41513103, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day6/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day6/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
