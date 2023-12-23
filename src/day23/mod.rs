use std::fs;

fn get_successors(layout: &Vec<Vec<char>>, route: &Vec<(i128, i128)>) -> Vec<Vec<(i128, i128)>> {
    let pos: &(i128, i128) = route.last().unwrap();
    let possible_next_positions = vec![
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 - 1),
    ];
    let mut get_successors: Vec<Vec<(i128, i128)>> = Vec::new();
    for next_position in possible_next_positions {
        let terrain = try_get_value(layout, &next_position);
        if terrain.is_none() {
            continue;
        }
        if route.contains(&next_position) {
            continue;
        }

        let terrain = terrain.unwrap();
        match terrain {
            '#' => continue,
            '.' => get_successors.push(vec![next_position]),
            '>' => {
                let slide_position = (next_position.0, next_position.1 + 1);
                if !route.contains(&slide_position) {
                    get_successors.push(vec![next_position, slide_position])
                }
            }
            'v' => {
                let slide_position = (next_position.0 + 1, next_position.1);
                if !route.contains(&slide_position) {
                    get_successors.push(vec![next_position, slide_position])
                }
            }
            '<' => {
                let slide_position = (next_position.0, next_position.1 - 1);
                if !route.contains(&slide_position) {
                    get_successors.push(vec![next_position, slide_position])
                }
            }
            '^' => {
                let slide_position = (next_position.0 - 1, next_position.1);
                if !route.contains(&slide_position) {
                    get_successors.push(vec![next_position, slide_position])
                }
            }
            _ => panic!("Unexpected Tile found"),
        }
    }
    get_successors
}

fn try_get_value(layout: &Vec<Vec<char>>, position: &(i128, i128)) -> Option<char> {
    if 0 <= position.0
        && position.0 < layout.len() as i128
        && 0 <= position.1
        && position.1 < layout.first().unwrap().len() as i128
    {
        return Some(layout[position.0 as usize][position.1 as usize]);
    }
    return None;
}

fn step(layout: &Vec<Vec<char>>, route: &Vec<(i128, i128)>) -> usize {
    let successors = get_successors(layout, route);
    let mut best_route = route.len();
    for successor in &successors {
        let mut new_route = route.clone();
        new_route.extend(successor);
        let new_route = step(layout, &new_route);
        if new_route > best_route {
            best_route = new_route;
        }
    }
    best_route
}

fn solve_part_one(layout: &Vec<Vec<char>>) -> usize {
    let start: (usize, usize) = (0, layout[0].iter().position(|c| *c == '.').unwrap());
    let route: Vec<(i128, i128)> = vec![(start.0 as i128, start.1 as i128)];
    let longest_route = step(layout, &route);
    longest_route - 1
}

fn solve_part_two(_: &Vec<Vec<char>>) -> usize {
    0
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\r\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day23/input.txt");
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
    fn day23_example_input_part_one() {
        let input = get_input("./src/day23/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(94, sum_part_one);
    }

    #[test]
    fn day23_input_part_one() {
        let input = get_input("./src/day23/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(2042, sum_part_one);
    }

    #[test]
    fn day23_example_input_part_two() {
        let input = get_input("./src/day23/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(7, sum_part_two);
    }

    #[test]
    fn day23_input_part_two() {
        let input = get_input("./src/day23/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(102770, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day23/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day23/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
