use std::{cmp::max, fs};

fn get_sum_of_cube_power(games: &Vec<(u32, (u32, u32, u32))>) -> u32 {
    return games
        .iter()
        .map(|(_, (red, green, blue))| red * green * blue)
        .sum();
}

fn get_sum_of_possible_game_ids(
    games: &Vec<(u32, (u32, u32, u32))>,
    (max_red, max_green, max_blue): (u32, u32, u32),
) -> u32 {
    return games
        .iter()
        .filter(|(_, (red, green, blue))| {
            red <= &max_red && green <= &max_green && blue <= &max_blue
        })
        .map(|(game_id, _)| game_id)
        .sum();
}

fn parse_games(input: Vec<String>) -> Vec<(u32, (u32, u32, u32))> {
    // id, (red, green, blue)
    let mut games: Vec<(u32, (u32, u32, u32))> = Vec::new();
    for line in input {
        let split: Vec<&str> = line.split(":").collect();
        let game_id = split[0]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let games_string: Vec<&str> = split[1].split(";").into_iter().map(|s| s.trim()).collect();
        let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);
        for game in games_string {
            let colors_string: Vec<&str> = game.split(",").into_iter().map(|s| s.trim()).collect();
            for color in colors_string {
                let split_color: Vec<&str> = color.split(" ").collect();
                let color_count = split_color[0].parse::<u32>().unwrap();
                match split_color[1] {
                    "red" => red = max(red, color_count),
                    "green" => green = max(green, color_count),
                    "blue" => blue = max(blue, color_count),
                    _ => panic!("color mismatch"),
                }
            }
        }
        games.push((game_id, (red, green, blue)));
    }

    return games;
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<String> = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    return lines;
}

pub fn solver() {
    let input = get_input("./src/day2/input.txt");
    let games = parse_games(input);
    let sum_part_one = get_sum_of_possible_game_ids(&games, (12, 13, 14));
    let sum_part_two = get_sum_of_cube_power(&games);
    println!("Part 1: {sum_part_one}");
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_example_input_part_one() {
        let input = get_input("./src/day2/example_input.txt");
        let games = parse_games(input);
        let sum = get_sum_of_possible_game_ids(&games, (12, 13, 14));
        assert_eq!(8, sum);
    }

    #[test]
    fn day2_example_input_part_two() {
        let input = get_input("./src/day2/example_input.txt");
        let games = parse_games(input);
        let power = get_sum_of_cube_power(&games);
        assert_eq!(2286, power);
    }

    #[test]
    fn day2_input_part_one() {
        let input = get_input("./src/day2/input.txt");
        let games = parse_games(input);
        let sum = get_sum_of_possible_game_ids(&games, (12, 13, 14));
        assert_eq!(2268, sum);
    }

    #[test]
    fn day2_input_part_two() {
        let input = get_input("./src/day2/input.txt");
        let games = parse_games(input);
        let power = get_sum_of_cube_power(&games);
        assert_eq!(63542, power);
    }
}
