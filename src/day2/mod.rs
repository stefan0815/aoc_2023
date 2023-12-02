use std::{cmp::max, fs};

fn get_power_of_games(games: &Vec<(u32, (u32, u32, u32))>) -> u32 {
    let mut sum_power = 0;
    for (_, colors) in games {
        sum_power += colors.0 * colors.1 * colors.2;
    }
    return sum_power;
}

fn sum_possible_games(games: &Vec<(u32, (u32, u32, u32))>, max_colors: (u32, u32, u32)) -> u32 {
    let mut sum = 0;
    for (game_id, colors) in games {
        if colors.0 <= max_colors.0 && colors.1 <= max_colors.1 && colors.2 <= max_colors.2 {
            sum += game_id;
        }
    }
    return sum;
}

fn parse_game(input: Vec<String>) -> Vec<(u32, (u32, u32, u32))> {
    // id, red, green, blue
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
        let games_string: Vec<&str> = split[1].split(";").collect();
        let mut colors: (u32, u32, u32) = (0, 0, 0);
        for mut game in games_string {
            game = game.trim();
            let colors_string: Vec<&str> = game.split(",").collect();
            for mut color in colors_string {
                color = color.trim();
                let split_color: Vec<&str> = color.split(" ").collect();
                let color_count = split_color[0].parse::<u32>().unwrap();
                match split_color[1] {
                    "red" => colors.0 = max(colors.0, color_count),
                    "green" => colors.1 = max(colors.1, color_count),
                    "blue" => colors.2 = max(colors.2, color_count),
                    _ => panic!("color mismatch"),
                }
            }
        }
        games.push((game_id, colors));
    }

    return games;
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<String> = input
        .split("\r\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    return lines;
}

pub fn solver() {
    let input = get_input("./src/day2/input.txt");
    let games = parse_game(input);
    let sum_part_one = sum_possible_games(&games, (12, 13, 14));
    let sum_part_two = get_power_of_games(&games);
    println!("Part 1: {sum_part_one}");
    println!("Part 2: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_simple_example_part_one() {
        let input = get_input("./src/day2/example_input.txt");
        let games = parse_game(input);
        let sum = sum_possible_games(&games, (12, 13, 14));
        assert_eq!(8, sum);
    }

    #[test]
    fn day2_simple_example_part_two() {
        let input = get_input("./src/day2/example_input.txt");
        let games = parse_game(input);
        let power = get_power_of_games(&games);
        assert_eq!(2286, power);
    }

    #[test]
    fn day2_input_part_one() {
        let input = get_input("./src/day2/input.txt");
        let games = parse_game(input);
        let sum = sum_possible_games(&games, (12, 13, 14));
        assert_eq!(2268, sum);
    }

    
    #[test]
    fn day2_input_part_two() {
        let input = get_input("./src/day2/input.txt");
        let games = parse_game(input);
        let power = get_power_of_games(&games);
        assert_eq!(63542, power);
    }
}
