use std::{collections::HashMap, fs};

fn find_route(
    layout: &Vec<Vec<usize>>,
    pos: (i128, i128),
    direction: (i128, i128),
    heat_loss: i128,
    straight_steps: usize,
    heat_loss_map: &mut HashMap<(usize, usize), i128>,
) {
    // println!("({}, {})", pos.0, pos.1);
    if pos.0 < 0
        || pos.0 >= layout.len() as i128
        || pos.1 < 0
        || pos.1 >= layout[pos.0 as usize].len() as i128
    {
        return;
    }

    let heat_loss = heat_loss + layout[pos.0 as usize][pos.1 as usize] as i128;
    // maybe map needs to contain direction as different angles and straight steps might be a factor
    if heat_loss_map.contains_key(&(pos.0 as usize, pos.1 as usize)) {
        let best_heat_loss_so_far = heat_loss_map[&(pos.0 as usize, pos.1 as usize)];
        if heat_loss > best_heat_loss_so_far {
            return;
        }
        if heat_loss < best_heat_loss_so_far {
            *heat_loss_map
                .get_mut(&(pos.0 as usize, pos.1 as usize))
                .unwrap() = heat_loss;
        }
    } else {
        heat_loss_map.insert((pos.0 as usize, pos.1 as usize), heat_loss);
    }

    if pos.0 as usize == layout.len() - 1 && pos.1 as usize == layout[layout.len() - 1].len() - 1 {
        return;
    }

    if straight_steps < 3 {
        let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
        find_route(
            layout,
            next_pos,
            direction,
            heat_loss,
            straight_steps + 1,
            heat_loss_map,
        );
    }

    let left_turn = (-direction.1, -direction.0);
    let next_pos = (pos.0 + left_turn.0, pos.1 + left_turn.1);
    find_route(layout, next_pos, left_turn, heat_loss, 1, heat_loss_map);

    let right_turn = (direction.1, direction.0);
    let next_pos = (pos.0 + right_turn.0, pos.1 + right_turn.1);
    find_route(layout, next_pos, right_turn, heat_loss, 1, heat_loss_map);
}

fn solve_part_one(layout: &Vec<Vec<usize>>) -> usize {
    let mut heat_loss_map: HashMap<(usize, usize), i128> = HashMap::new();
    heat_loss_map.insert((0, 0), -50);
    find_route(layout, (0, 1), (0, 1), 0, 1, &mut heat_loss_map);
    find_route(layout, (1, 0), (1, 0), 0, 1, &mut heat_loss_map);
    // for row in 0..layout.len() {
    //     for col in 0..layout[row].len() {
    //         print!("{},", heat_loss_map[&(row, col)]);
    //     }
    //     println!();
    // }
    // println!("{:?}", heat_loss_map);
    heat_loss_map[&(layout.len() - 1, layout[layout.len() - 1].len() - 1)] as usize
}

fn solve_part_two(_: &Vec<Vec<usize>>) -> usize {
    0
}

fn get_input(file: &str) -> Vec<Vec<usize>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\r\n")
        .into_iter()
        .map(|s| {
            s.to_owned()
                .chars()
                .map(|symbol| symbol.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day17/input.txt");
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
    fn day17_example_input_part_one() {
        let input = get_input("./src/day17/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(102, sum_part_one);
    }

    #[test]
    fn day17_input_part_one() {
        let input = get_input("./src/day17/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(7562, sum_part_one);
    }

    #[test]
    fn day17_example_input_part_two() {
        let input = get_input("./src/day17/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(51, sum_part_two);
    }

    #[test]
    fn day17_input_part_two() {
        let input = get_input("./src/day17/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(7793, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day17/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day17/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
