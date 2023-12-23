use std::{collections::HashMap, fs};

// fn print_layout_with_route(layout: &Vec<Vec<char>>, route: &Vec<(i128, i128)>){
//     let mut print_layout = layout.to_vec();
//     for tile in route {
//         print_layout[tile.0 as usize][tile.1 as usize] = '0';
//     }
//     println!("{:?}", print_layout);
// }

fn get_successors(
    layout: &Vec<Vec<char>>,
    route: &Vec<(i128, i128)>,
    pos: &(i128, i128),
) -> Vec<Vec<(i128, i128)>> {
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

fn get_successors_part_two(
    layout: &Vec<Vec<char>>,
    route: &Vec<(i128, i128)>,
    pos: &(i128, i128),
) -> Vec<(i128, i128)> {
    let possible_next_positions = vec![
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 - 1),
    ];
    let mut get_successors: Vec<(i128, i128)> = Vec::new();
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
            '.' => get_successors.push(next_position),
            _ => panic!("Unexpected Tile found {}", terrain),
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

fn step(
    layout: &Vec<Vec<char>>,
    route: &Vec<(i128, i128)>,
    pos: &(i128, i128),
    goal: &(i128, i128),
) -> usize {
    let successors = get_successors(layout, route, pos);
    let mut longest_route = 0;
    for successor in &successors {
        if successor.last().unwrap() == goal {
            return route.len() + successor.len();
        }
        let mut new_route = route.clone();
        new_route.extend(successor);
        let new_route = step(layout, &new_route, new_route.last().unwrap(), goal);
        if new_route > longest_route {
            longest_route = new_route;
        }
    }
    longest_route
}

fn step_part_two(
    crossings: &HashMap<(i128, i128), HashMap<(i128, i128), usize>>,
    route: &Vec<(i128, i128)>,
    goal: &(i128, i128),
    steps_so_far: usize
) -> usize {
    let mut longest_route = 0;
    let current_pos = route.last().unwrap();
    let mut new_route: Vec<(i128, i128)> = route.to_vec();
    for successor in &crossings[current_pos] {
        if route.contains(successor.0){
            continue;
        }
        if successor.0 == goal {
            return steps_so_far + *successor.1;
        }
        new_route.push(successor.0.clone());
        let new_route_length = step_part_two(crossings, &new_route, goal, steps_so_far + *successor.1);
        if new_route_length > longest_route {
            longest_route = new_route_length;
        }
        new_route.pop();
    }
    longest_route
}

fn solve_part_one(layout: &Vec<Vec<char>>) -> usize {
    let start: (i128, i128) = (
        0,
        layout
            .first()
            .unwrap()
            .iter()
            .position(|c| *c == '.')
            .unwrap() as i128,
    );
    let goal: (i128, i128) = (
        layout.len() as i128 - 1,
        layout
            .last()
            .unwrap()
            .iter()
            .position(|c| *c == '.')
            .unwrap() as i128,
    );
    let mut route: Vec<(i128, i128)> = Vec::new();
    route.push(start);
    let longest_route = step(layout, &route, &start, &goal);
    longest_route - 1
}

fn solve_part_two(layout: &Vec<Vec<char>>) -> usize {
    let mut clean_layout = layout.clone();
    for row in 0..layout.len() {
        for col in 0..layout[row].len() {
            if layout[row][col] != '#' {
                clean_layout[row][col] = '.'
            }
        }
    }
    let layout = clean_layout;

    let start: (i128, i128) = (
        0,
        layout
            .first()
            .unwrap()
            .iter()
            .position(|c| *c == '.')
            .unwrap() as i128,
    );
    let goal: (i128, i128) = (
        layout.len() as i128 - 1,
        layout
            .last()
            .unwrap()
            .iter()
            .position(|c| *c == '.')
            .unwrap() as i128,
    );
    let mut crossings: HashMap<(i128, i128), HashMap<(i128, i128), usize>> = HashMap::new(); // From crossing to other crossing with distance
    for row in 0..layout.len() {
        for col in 0..layout[row].len() {
            if layout[row][col] != '.' {
                continue;
            }
            let neighbours = get_successors_part_two(
                &layout,
                &vec![(row as i128, col as i128)],
                &(row as i128, col as i128),
            );
            if neighbours.len() > 2 {
                crossings.insert((row as i128, col as i128), HashMap::new());
            }
        }
    }
    crossings.insert(start, HashMap::new());
    crossings.insert(goal, HashMap::new());
    let mut mapped_crossings = crossings.clone();
    for (crossing_pos, _) in crossings {
        let neighbours = get_successors_part_two(&layout, &vec![crossing_pos], &crossing_pos);
        for neighbour in neighbours {
            let mut route: Vec<(i128, i128)> = vec![crossing_pos, neighbour];
            loop {
                let next_positions =
                    get_successors_part_two(&layout, &route, route.last().unwrap());
                if next_positions.len() != 1 {
                    panic!("Invalid path found, next_positions.len(): {}", next_positions.len());
                }
                let next_position = next_positions.first().unwrap();
                if mapped_crossings.contains_key(next_position) {
                    mapped_crossings
                        .get_mut(&crossing_pos)
                        .unwrap()
                        .insert(*next_position, route.len());
                    mapped_crossings
                        .get_mut(next_position)
                        .unwrap()
                        .insert(crossing_pos, route.len());
                    break;
                }
                route.push(*next_position);
            }
        }
    }
    let route = vec![start];
    let longest_route = step_part_two(&mapped_crossings, &route, &goal, 0);

    longest_route
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
        assert_eq!(154, sum_part_two);
    }

    #[test]
    fn day23_input_part_two() {
        let input = get_input("./src/day23/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(6466, sum_part_two);
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
