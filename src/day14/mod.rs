use std::fs;

fn flip_to_col(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut col_based_platform: Vec<Vec<char>> = vec![vec![]; platform[0].len()];
    for row in platform {
        for char_index in 0..row.len() {
            col_based_platform[char_index].push(row[char_index]);
        }
    }
    col_based_platform
}

fn calculate_load(col: &Vec<char>, length: usize) -> usize {
    let num_stones = col.iter().filter(|tile| **tile == 'O').count();
    let mut load = 0;
    for i in (length-num_stones + 1)..(length+1){
        load += i;
    }
    return load;
}

fn solve_part_one(platform: &Vec<Vec<char>>) -> usize {
    let col_platform = flip_to_col(&platform);
    col_platform.iter().map(|col|{
        let mut partial_col = col.to_vec();
        let mut load = 0;
        loop {
            if !partial_col.contains(&'#'){
                load += calculate_load(&partial_col,  partial_col.len());
                break;
            }
            let square_stone_index = partial_col.iter().position(|&tile| tile == '#').unwrap();
            load += calculate_load(&partial_col[..square_stone_index].to_vec(),  partial_col.len());
            if square_stone_index + 1 >= partial_col.len(){
                break;
            }
            partial_col = partial_col[(square_stone_index + 1)..].to_vec();
        }
        load
    }).sum::<usize>()
}

fn solve_part_two(platform: &Vec<Vec<char>>) -> usize {
    0
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
    let sum_part_two = solve_part_two(&input);
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
    fn day14_example_input_part_two() {
        let input = get_input("./src/day14/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(400, sum_part_two);
    }

    #[test]
    fn day14_input_part_two() {
        let input = get_input("./src/day14/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(25401, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day14/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day14/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
