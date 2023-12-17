use pathfinding::prelude::astar;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    position: (i128, i128),
    direction: (i128, i128),
    straight_steps: i128,
}

impl Pos {
    fn distance_simple(&self, other: &(i128, i128)) -> i128 {
        (self.position.0.abs_diff(other.0) + self.position.1.abs_diff(other.1)) as i128
    }

    fn successors(&self, layout: &Vec<Vec<i128>>) -> Vec<(Pos, i128)> {
        let mut next_positions: Vec<Pos> = Vec::new();
        if self.straight_steps < 3 {
            let next_pos = (
                self.position.0 + self.direction.0,
                self.position.1 + self.direction.1,
            );
            if in_bounds(layout, &next_pos) {
                let straight_pos = Pos {
                    position: next_pos,
                    direction: self.direction,
                    straight_steps: self.straight_steps + 1,
                };
                next_positions.push(straight_pos);
            }
        }

        let left_turn = (-self.direction.1, -self.direction.0);
        let next_pos = (self.position.0 + left_turn.0, self.position.1 + left_turn.1);
        if in_bounds(layout, &next_pos) {
            let left_pos = Pos {
                position: next_pos,
                direction: left_turn,
                straight_steps: 1,
            };

            next_positions.push(left_pos);
        }

        let right_turn = (self.direction.1, self.direction.0);
        let next_pos = (
            self.position.0 + right_turn.0,
            self.position.1 + right_turn.1,
        );
        if in_bounds(layout, &next_pos) {
            let right_pos = Pos {
                position: next_pos,
                direction: right_turn,
                straight_steps: 1,
            };

            next_positions.push(right_pos);
        }

        next_positions
            .iter()
            .map(|p| {
                (
                    p.clone(),
                    layout[p.position.0 as usize][p.position.1 as usize],
                )
            })
            .collect()
    }

    fn ultra_successors(&self, layout: &Vec<Vec<i128>>, goal: (i128, i128)) -> Vec<(Pos, i128)> {
        let mut next_positions: Vec<Pos> = Vec::new();
        if self.straight_steps < 10 {
            let next_pos = (
                self.position.0 + self.direction.0,
                self.position.1 + self.direction.1,
            );
            if in_bounds(layout, &next_pos) {
                let straight_pos = Pos {
                    position: next_pos,
                    direction: self.direction,
                    straight_steps: self.straight_steps + 1,
                };
                if !(next_pos.0 == goal.0 && next_pos.1 == goal.1) {
                    next_positions.push(straight_pos);
                } else if straight_pos.straight_steps >= 4 {
                    next_positions.push(straight_pos);
                }
            }
        }

        if self.straight_steps >= 4 {
            let left_turn = (-self.direction.1, -self.direction.0);
            let next_pos = (self.position.0 + left_turn.0, self.position.1 + left_turn.1);
            if in_bounds(layout, &next_pos) {
                let left_pos = Pos {
                    position: next_pos,
                    direction: left_turn,
                    straight_steps: 1,
                };
                if !(next_pos.0 == goal.0 && next_pos.1 == goal.1) {
                    next_positions.push(left_pos);
                }
            }

            let right_turn = (self.direction.1, self.direction.0);
            let next_pos = (
                self.position.0 + right_turn.0,
                self.position.1 + right_turn.1,
            );
            if in_bounds(layout, &next_pos) {
                let right_pos = Pos {
                    position: next_pos,
                    direction: right_turn,
                    straight_steps: 1,
                };
                if !(next_pos.0 == goal.0 && next_pos.1 == goal.1) {
                    next_positions.push(right_pos);
                }
            }
        }

        next_positions
            .iter()
            .map(|p| {
                (
                    p.clone(),
                    layout[p.position.0 as usize][p.position.1 as usize],
                )
            })
            .collect()
    }
}

fn in_bounds(height_map: &Vec<Vec<i128>>, position: &(i128, i128)) -> bool {
    return 0 <= position.0
        && position.0 < height_map.len() as i128
        && 0 <= position.1
        && position.1 < height_map.first().unwrap().len() as i128;
}

fn solve_part_one(layout: &Vec<Vec<i128>>) -> i128 {
    let goal: (i128, i128) = (
        (layout.len() - 1) as i128,
        (layout[layout.len() - 1].len() - 1) as i128,
    );
    let start: Pos = Pos {
        position: (0, 0),
        direction: (0, 1),
        straight_steps: 0,
    };
    let result = astar(
        &start,
        |p| p.successors(layout),
        |p| p.distance_simple(&goal) / 3,
        |p| p.position == goal,
    );
    result.unwrap().1
}

fn solve_part_two(layout: &Vec<Vec<i128>>) -> i128 {
    let goal: (i128, i128) = (
        (layout.len() - 1) as i128,
        (layout[layout.len() - 1].len() - 1) as i128,
    );
    let start: Pos = Pos {
        position: (0, 0),
        direction: (0, 1),
        straight_steps: 0,
    };
    let result = astar(
        &start,
        |p| p.ultra_successors(layout, goal),
        |p| p.distance_simple(&goal) / 3,
        |p| p.position == goal,
    );
    println!(
        "{:?}",
        result
            .clone()
            .unwrap()
            .0
            .iter()
            .map(|pos| pos.position)
            .collect::<Vec<(i128, i128)>>()
    );
    result.unwrap().1
}

fn get_input(file: &str) -> Vec<Vec<i128>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\r\n")
        .into_iter()
        .map(|s| {
            s.to_owned()
                .chars()
                .map(|symbol| symbol.to_digit(10).unwrap() as i128)
                .collect::<Vec<i128>>()
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
        assert_eq!(886, sum_part_one);
    }

    #[test]
    fn day17_example_input_part_two() {
        let input = get_input("./src/day17/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(94, sum_part_two);
    }

    #[test]
    fn day17_example_input_two_part_two() {
        let input = get_input("./src/day17/example_input_two.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(71, sum_part_two);
    }

    #[test]
    fn day17_input_part_two() {
        let input = get_input("./src/day17/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(1055, sum_part_two);
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
