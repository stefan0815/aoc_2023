use std::{collections::HashMap, fs};

struct Line {
    slope: (i128, i128, i128),
    intercept: (i128, i128, i128),
}

fn solve_part_one(input: &Vec<String>, bounding_box: (f64, f64)) -> usize {
    let lines: Vec<Line> = input
        .iter()
        .map(|line_text| {
            let split = line_text.split("@").collect::<Vec<&str>>();
            let slope = split[1]
                .trim()
                .split(",")
                .map(|s| s.trim().parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            let intercept = split[0]
                .trim()
                .split(",")
                .map(|s| s.trim().parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            Line {
                slope: (slope[0], slope[1], slope[2]),
                intercept: (intercept[0], intercept[1], intercept[2]),
            }
        })
        .collect();

    let mut sum = 0;
    for i in 0..lines.len() {
        for j in (i+1)..lines.len() {
            // if i == j {
            //     continue;
            // }

            let point_one = lines[i].intercept;
            let point_two = (
                lines[i].intercept.0 + lines[i].slope.0,
                lines[i].intercept.1 + lines[i].slope.1,
                lines[i].intercept.2 + lines[i].slope.2,
            );
            let point_three = lines[j].intercept;
            let point_four = (
                lines[j].intercept.0 + lines[j].slope.0,
                lines[j].intercept.1 + lines[j].slope.1,
                lines[j].intercept.2 + lines[j].slope.2,
            );
            let divisor = (point_one.0 - point_two.0) * (point_three.1 - point_four.1)
                - (point_one.1 - point_two.1) * (point_three.0 - point_four.0);
            if divisor == 0 {
                // println!("divisor is zero, lines are parallel {i}, {j}");
                continue;
            }
            let t = ((point_one.0 - point_three.0) * (point_three.1 - point_four.1)
                - (point_one.1 - point_three.1) * (point_three.0 - point_four.0))
                as f64
                / divisor as f64;
            let u = ((point_one.0 - point_three.0) * (point_one.1 - point_two.1)
                - (point_one.1 - point_three.1) * (point_one.0 - point_two.0))
                as f64
                / divisor as f64;

            if t < 0.0 || u < 0.0 {
                // println!("Intersection in the past {i}, {j}");
                continue;
            }
            let intersection_point_one = (
                point_one.0 as f64 + t * (point_two.0 - point_one.0) as f64,
                point_one.1 as f64 + t * (point_two.1 - point_one.1) as f64,
            );

            if intersection_point_one.0 >= bounding_box.0
                && intersection_point_one.0 <= bounding_box.1
                && intersection_point_one.1 >= bounding_box.0
                && intersection_point_one.1 <= bounding_box.1
            {
                // println!("Intersect within test area: {i} {j}");
                sum += 1;
            }
            // let intersection_point_two = (
            //     point_three.0 as f64 + u * (point_four.0 - point_three.0) as f64,
            //     point_three.1 as f64 + u * (point_four.1 - point_three.1) as f64,
            // );
            // println!("intersection_point_one: {:?}, intersection_point_two: {:?}", intersection_point_one, intersection_point_two);
        }
    }
    sum
}

fn solve_part_two(input: &Vec<String>) -> usize {
    0
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day24/input.txt");
    let sum_part_one = solve_part_one(&input, (200000000000000.0, 400000000000000.0));
    println!("Part 1: {sum_part_one}");
    let sum_part_two = solve_part_two(&input);
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn day24_example_input_part_one() {
        let input = get_input("./src/day24/example_input.txt");
        let sum_part_one = solve_part_one(&input, (7.0, 27.0));
        assert_eq!(2, sum_part_one);
    }

    #[test]
    fn day24_input_part_one() {
        let input = get_input("./src/day24/input.txt");
        let sum_part_one = solve_part_one(&input, (200000000000000.0, 400000000000000.0));
        assert_eq!(11995, sum_part_one);
    }

    #[test]
    fn day24_example_input_part_two() {
        let input = get_input("./src/day24/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(154, sum_part_two);
    }

    #[test]
    fn day24_input_part_two() {
        let input = get_input("./src/day24/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(6466, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day24/input.txt");
        b.iter(|| solve_part_one(&input, (200000000000000.0, 400000000000000.0)))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day24/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
