use std::{collections::HashMap, fs};

struct Lens {
    label: String,
    focal_length: usize,
}

fn get_hash(label: &String) -> usize {
    let mut hash = 0;
    label.chars().into_iter().for_each(|symbol| {
        let ascii = symbol as u8;
        hash += ascii as usize;
        hash = hash * 17;
        hash = hash % 256;
    });
    hash
}

fn solve_part_one(input: &Vec<String>) -> usize {
    input.iter().map(|code| get_hash(code)).sum()
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();
    input.iter().for_each(|lens_code| {
        if lens_code.contains('-') {
            let label = &lens_code
                .split('-')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()[0];
            let box_number = get_hash(&label);
            if boxes.contains_key(&box_number) {
                let lens_box = &boxes[&box_number];
                let pos = lens_box.iter().position(|lens| lens.label == *label);
                if pos.is_some() {
                    boxes.get_mut(&box_number).unwrap().remove(pos.unwrap());
                }
            }
        } else {
            let split = lens_code
                .split('=')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            let label = &split[0];
            let focal_length = split[1].parse::<usize>().unwrap();
            let new_lens = Lens {
                label: label.to_string(),
                focal_length,
            };
            let box_number = get_hash(&label);
            if boxes.contains_key(&box_number) {
                let lens_box = &boxes[&box_number];
                let pos = lens_box.iter().position(|lens| lens.label == *label);
                if pos.is_some() {
                    boxes.get_mut(&box_number).unwrap().remove(pos.unwrap());
                    boxes.get_mut(&box_number).unwrap().insert(pos.unwrap(), new_lens);
                }else{
                    boxes.get_mut(&box_number).unwrap().push(new_lens);
                }
            } else {
                boxes.insert(box_number, vec![new_lens]);
            }
        }
    });

    boxes
        .into_iter()
        .map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_pos, lens)| {
                    let focusing_power: usize =
                        (box_number + 1) * (lens_pos + 1) * lens.focal_length;
                    focusing_power
                })
                .sum::<usize>()
        })
        .sum()
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    input.split("\n").collect::<Vec<&str>>()[0]
        .trim()
        .split(",")
        .into_iter()
        .map(|s| s.to_owned())
        .collect()
}

pub fn solver() {
    let input = get_input("./src/day15/input.txt");
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
    fn day15_example_input_part_one() {
        let input = get_input("./src/day15/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(1320, sum_part_one);
    }

    #[test]
    fn day15_input_part_one() {
        let input = get_input("./src/day15/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(504036, sum_part_one);
    }

    #[test]
    fn day15_example_input_part_two() {
        let input = get_input("./src/day15/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(145, sum_part_two);
    }

    #[test]
    fn day15_input_part_two() {
        let input = get_input("./src/day15/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(295719, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day15/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day15/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
