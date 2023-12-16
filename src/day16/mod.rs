use std::{collections::HashSet, fs, cmp::max};

fn cast_ray(
    layout: &Vec<Vec<char>>,
    pos: (i128, i128),
    direction: (i128, i128),
    visited: &mut HashSet<(i128, i128)>,
    ray_cache: &mut HashSet<((i128, i128), (i128, i128))>,
) {
    let mut current_pos = pos;
    let mut current_direction = direction;
    loop {
        if current_pos.0 < 0
            || current_pos.0 >= layout.len() as i128
            || current_pos.1 < 0
            || current_pos.1 >= layout[current_pos.0 as usize].len() as i128
        {
            return;
        }

        if ray_cache.contains(&(current_pos, current_direction)) {
            return;
        }

        visited.insert(current_pos);
        ray_cache.insert((current_pos, current_direction));

        match (
            layout[current_pos.0 as usize][current_pos.1 as usize],
            current_direction,
        ) {
            ('.', _) | ('-', (0, _)) | ('|', (_, 0)) => (),
            ('/', _) => {
                current_direction = (-current_direction.1, -current_direction.0);
            }
            ('\\', _) => {
                current_direction = (current_direction.1, current_direction.0);
            }
            ('-', (_, 0)) | ('|', (0, _)) => {
                let split_ray_direction = (-current_direction.1, -current_direction.0);
                cast_ray(
                    layout,
                    (
                        current_pos.0 + split_ray_direction.0,
                        current_pos.1 + split_ray_direction.1,
                    ),
                    split_ray_direction,
                    visited,
                    ray_cache,
                );
                current_direction = (current_direction.1, current_direction.0);
            }
            _ => panic!("invalid symbol"),
        }

        current_pos = (
            current_pos.0 + current_direction.0,
            current_pos.1 + current_direction.1,
        );
    }
}

fn solve_part_one(layout: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut ray_cache = HashSet::new();
    cast_ray(layout, (0,0), (0,1), &mut visited, &mut ray_cache);
    visited.len()
}

fn solve_part_two(layout: &Vec<Vec<char>>) -> usize {
    // println!("layout: ({},{})", layout.len(), layout[0].len());
    let mut starts: Vec<((i128, i128), (i128, i128))> = Vec::new();
    // from top and bot
    for i in 0..layout[0].len() {
        let start_pos = (0, i as i128);
        let start_direction = (1, 0);
        starts.push((start_pos, start_direction));

        let start_pos = (layout.len() as i128 - 1, i as i128);
        let start_direction = (-1, 0);
        starts.push((start_pos, start_direction));
    }

    // from left and right
    for i in 0..layout.len() {
        let start_pos = (i as i128, 0);
        let start_direction = (0, 1);
        starts.push((start_pos, start_direction));

        let start_pos = (i as i128, layout[i].len() as i128 - 1);
        let start_direction = (0, -1);
        starts.push((start_pos, start_direction));
    }

    let mut max_tiles = 0;
    for start in starts {
        // println!("start_pos: ({}, {}), start_direction: ({}, {})", start.0.0, start.0.1, start.1.0, start.1.1);
        let mut visited = HashSet::new();
        let mut ray_cache = HashSet::new();
        cast_ray(layout, start.0, start.1, &mut visited, &mut ray_cache);
        max_tiles = max(max_tiles, visited.len());
    }
    max_tiles
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned().chars().collect::<Vec<char>>())
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day16/input.txt");
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
    fn day16_example_input_part_one() {
        let input = get_input("./src/day16/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(46, sum_part_one);
    }

    #[test]
    fn day16_input_part_one() {
        let input = get_input("./src/day16/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(7562, sum_part_one);
    }

    #[test]
    fn day16_example_input_part_two() {
        let input = get_input("./src/day16/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(51, sum_part_two);
    }

    #[test]
    fn day16_input_part_two() {
        let input = get_input("./src/day16/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(7793, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day16/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day16/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
