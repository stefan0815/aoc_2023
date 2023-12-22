use std::{cmp::max, collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Block {
    start: [usize; 3],
    end: [usize; 3],
}

fn parse_input(input: &Vec<String>) -> Vec<Block> {
    input
        .iter()
        .map(|line| {
            let split = line.split("~").collect::<Vec<&str>>();
            Block {
                start: split[0]
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap(),
                end: split[1]
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap(),
            }
        })
        .collect()
}

fn sort_blocks(blocks: &Vec<Block>) -> Vec<Block> {
    let mut sorted_blocks = blocks.clone();
    sorted_blocks.sort_unstable_by_key(|block| block.start[2]);
    sorted_blocks
}

fn update_height_map(height_map: &mut HashMap<(usize, usize), usize>, block: &Block) {
    for x in block.start[0]..(block.end[0] + 1) {
        for y in block.start[1]..(block.end[1] + 1) {
            height_map.insert((x, y), block.end[2]);
        }
    }
}

fn get_terrain_height(height_map: &HashMap<(usize, usize), usize>, block: &Block) -> usize {
    let mut height = 0;
    for x in block.start[0]..(block.end[0] + 1) {
        for y in block.start[1]..(block.end[1] + 1) {
            if height_map.contains_key(&(x, y)) {
                height = max(height, height_map[&(x, y)]);
            }
        }
    }
    height
}

fn settle_blocks(blocks: &Vec<Block>) -> (Vec<Block>, usize) {
    let mut settled_blocks: Vec<Block> = Vec::new();
    let mut height_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut num_fallen_blocks = 0;
    blocks.iter().for_each(|block| {
        if block.start[2] == 1 {
            update_height_map(&mut height_map, block);
            settled_blocks.push(block.clone());
            return;
        }
        let height = get_terrain_height(&height_map, block);
        let target_height = height + 1;
        if block.start[2] == target_height {
            update_height_map(&mut height_map, block);
            settled_blocks.push(block.clone());
            return;
        }
        if block.start[2] > target_height {
            let mut settled_block = block.clone();
            let fall_distance = settled_block.start[2] - target_height;
            settled_block.start[2] -= fall_distance;
            settled_block.end[2] -= fall_distance;
            update_height_map(&mut height_map, &settled_block);
            settled_blocks.push(settled_block);
            num_fallen_blocks += 1;
            return;
        }
        panic!("New Special case detected");
    });

    (settled_blocks, num_fallen_blocks)
}

fn solve_part_one(input: &Vec<String>) -> usize {
    let blocks = parse_input(&input);
    let sorted_blocks = sort_blocks(&blocks);
    let (settled_blocks, _) = settle_blocks(&sorted_blocks);
    let mut disintegrateable_blocks = 0;
    for i in 0..settled_blocks.len() {
        let mut blocks_after_disintegration = settled_blocks.clone();
        blocks_after_disintegration.remove(i);
        let (_, num_fallen_blocks) = settle_blocks(&blocks_after_disintegration);
        if num_fallen_blocks == 0 {
            disintegrateable_blocks += 1;
        }
    }

    disintegrateable_blocks
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let _ = parse_input(&input);
    let blocks = parse_input(&input);
    let sorted_blocks = sort_blocks(&blocks);
    let (settled_blocks, _) = settle_blocks(&sorted_blocks);
    let mut sum_fallen_bricks = 0;
    for i in 0..settled_blocks.len() {
        let mut blocks_after_disintegration = settled_blocks.clone();
        blocks_after_disintegration.remove(i);
        let (_, num_fallen_blocks) = settle_blocks(&blocks_after_disintegration);
        sum_fallen_bricks += num_fallen_blocks
    }

    sum_fallen_bricks
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
    let input = get_input("./src/day22/input.txt");
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
    fn day22_example_input_part_one() {
        let input = get_input("./src/day22/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(5, sum_part_one);
    }

    #[test]
    fn day22_input_part_one() {
        let input = get_input("./src/day22/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(509, sum_part_one);
    }

    #[test]
    fn day22_example_input_part_two() {
        let input = get_input("./src/day22/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(7, sum_part_two);
    }

    #[test]
    fn day22_input_part_two() {
        let input = get_input("./src/day22/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(102770, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day22/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day22/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
