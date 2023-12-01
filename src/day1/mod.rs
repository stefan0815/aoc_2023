use std::fs;

fn get_first_real_number(encoded_string: &String) -> (u32, usize) {
    let encoded_chars: Vec<char> = encoded_string.chars().collect();
    for i in 0..encoded_string.len() {
        let char = encoded_chars[i];
        if char.is_numeric() {
            return (char.to_digit(10).unwrap(), i);
        }
    }
    return (0, encoded_string.len() * 2);
}

fn get_first_string_number(
    encoded_string: &String,
    numbers_as_strings: &Vec<String>,
) -> (usize, usize) {
    let mut first_index = encoded_string.len() * 2;
    let mut first_number = 0;
    for i in 0..numbers_as_strings.len() {
        let number_as_string = &numbers_as_strings[i];
        if encoded_string.contains(number_as_string.as_str()) {
            let found_index = encoded_string.find(number_as_string.as_str()).unwrap();
            if found_index < first_index {
                first_number = i + 1;
                first_index = found_index;
            }
        }
    }
    return (first_number, first_index);
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day1/input.txt").expect("Should have been able to read the file");
    let encoded_calibration_values = input.split("\r\n");
    let mut sum_part_one = 0;
    let mut sum_part_two = 0;

    for encoded_line in encoded_calibration_values {
        let mut number_part_one: String = "".to_owned();
        let mut number_part_two: String = "".to_owned();

        let encoded_string: String = encoded_line.to_string();
        let numbers_as_strings: Vec<String> = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        let (mut first_number, first_number_index) =
            get_first_string_number(&encoded_string, &numbers_as_strings);
        let (first_digit, first_digit_index) = get_first_real_number(&encoded_string);

        if first_digit_index < first_number_index {
            first_number = usize::try_from(first_digit).unwrap();
        }

        number_part_one.push_str(first_digit.to_string().as_str());
        number_part_two.push_str(first_number.to_string().as_str());

        let reversed_numbers_as_strings: Vec<String> = numbers_as_strings
            .into_iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect();

        let (mut last_number, last_number_index) = get_first_string_number(
            &encoded_string.chars().rev().collect::<String>(),
            &reversed_numbers_as_strings,
        );
        let (last_digit, last_digit_index) =
            get_first_real_number(&encoded_string.chars().rev().collect::<String>());

        if last_digit_index < last_number_index {
            last_number = usize::try_from(last_digit).unwrap();
        }

        number_part_one.push_str(last_digit.to_string().as_str());
        number_part_two.push_str(last_number.to_string().as_str());

        sum_part_one += number_part_one.parse::<i32>().unwrap();
        sum_part_two += number_part_two.parse::<i32>().unwrap();
    }

    println!("Day1:");
    println!("Sum of calibration values part one:  {sum_part_one}");
    println!("Sum of calibration values part two:  {sum_part_two}");
}
