use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Workflow {
    tests: Vec<Test>,
    fallback: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Test {
    attribute: char,
    compare: char,
    compare_to: i128,
    next_workflow: String,
}

fn solve_part_one(
    (workflows, parts): &(HashMap<String, Workflow>, Vec<HashMap<char, i128>>),
) -> i128 {
    parts
        .iter()
        .map(|part| {
            let mut workflow_name = "in".to_owned();
            loop {
                let current_workflow = &workflows[&workflow_name];
                let mut next_workflow: Option<String> = None;
                for test in &current_workflow.tests {
                    let part_attribute = part[&test.attribute];
                    let test_passed;
                    match test.compare {
                        '<' => test_passed = part_attribute < test.compare_to,
                        '>' => test_passed = part_attribute > test.compare_to,
                        _ => panic!("Compare symbol"),
                    }
                    if test_passed {
                        next_workflow = Some(test.next_workflow.to_owned());
                        break;
                    }
                }
                if next_workflow.is_some() {
                    workflow_name = next_workflow.unwrap();
                } else {
                    workflow_name = current_workflow.fallback.to_owned();
                }
                if workflow_name == "A" {
                    return part.iter().map(|(_, value)| value).sum();
                } else if workflow_name == "R" {
                    return 0;
                }
            }
        })
        .sum()
}

fn get_combinations(accepted_parts: &HashMap<char, (i128, i128)>) -> usize {
    let mut combinations = 1;
    for (_, (lower, upper)) in accepted_parts {
        let possibilities = max(upper - lower - 1, 0) as usize;
        combinations *= possibilities;
    }
    combinations
}

fn check_workflow_with_range(
    workflows: &HashMap<String, Workflow>,
    workflow_name: &String,
    accepted_parts: &HashMap<char, (i128, i128)>,
) -> usize {
    let mut sum = 0;
    let workflow = &workflows[workflow_name];
    let mut current_accepted_parts = accepted_parts.clone();
    for test_step in 0..workflow.tests.len() {
        let mut recursive_accepted_parts = current_accepted_parts.clone();
        let test = &workflow.tests[test_step];
        let current_attribute = current_accepted_parts.get_mut(&test.attribute).unwrap();
        let recursive_attribute = recursive_accepted_parts.get_mut(&test.attribute).unwrap();

        match test.compare {
            '<' => {
                recursive_attribute.0 = min(recursive_attribute.0, test.compare_to);
                recursive_attribute.1 = min(recursive_attribute.1, test.compare_to);
                current_attribute.0 = max(current_attribute.0, test.compare_to - 1);
                current_attribute.1 = max(current_attribute.1, test.compare_to - 1);
            }
            '>' => {
                recursive_attribute.0 = max(recursive_attribute.0, test.compare_to);
                recursive_attribute.1 = max(recursive_attribute.1, test.compare_to);
                current_attribute.0 = min(current_attribute.0, test.compare_to + 1);
                current_attribute.1 = min(current_attribute.1, test.compare_to + 1);
            }
            _ => panic!("Compare symbol"),
        }
        if recursive_attribute.0 + 1 < recursive_attribute.1 {
            if test.next_workflow == "A"{
                sum += get_combinations(&recursive_accepted_parts);
            }
            else if test.next_workflow  != "R" {
                sum += check_workflow_with_range(workflows, &test.next_workflow, &recursive_accepted_parts);
            }
        }
        if !(current_attribute.0 + 1 < current_attribute.1) {
            return sum;
        }
    }
    if workflow.fallback == "A" {
        sum += get_combinations(&current_accepted_parts);
    }
    else if workflow.fallback != "R" {
        sum += check_workflow_with_range(workflows, &workflow.fallback, &current_accepted_parts)
    }
    sum
}

fn solve_part_two((workflows, _): &(HashMap<String, Workflow>, Vec<HashMap<char, i128>>)) -> usize {
    let mut accepted_parts: HashMap<char, (i128, i128)> = HashMap::new();
    accepted_parts.insert('x', (0, 4001));
    accepted_parts.insert('m', (0, 4001));
    accepted_parts.insert('a', (0, 4001));
    accepted_parts.insert('s', (0, 4001));
    check_workflow_with_range(&workflows, &"in".to_owned(), &accepted_parts)
}

fn get_input(file: &str) -> (HashMap<String, Workflow>, Vec<HashMap<char, i128>>) {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let input: Vec<Vec<String>> = input
        .split("\r\n\r\n")
        .into_iter()
        .map(|s| s.split("\r\n").map(|s| s.to_owned()).collect())
        .collect();
    let workflow_strings = &input[0];
    let part_strings = &input[1];
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<HashMap<char, i128>> = Vec::new();

    workflow_strings.iter().for_each(|workflow_string| {
        let mut chars = workflow_string.chars();
        chars.next_back();
        let workflow_string = chars.as_str();
        let split = workflow_string.split("{").collect::<Vec<&str>>();
        let workflow_name = split[0].to_owned();
        let mut tests: Vec<Test> = Vec::new();
        let test_strings = split[1].split(",").collect::<Vec<&str>>();
        for i in 0..test_strings.len() - 1 {
            let test_string = test_strings[i];
            let test_split = test_string.split(':').collect::<Vec<&str>>();
            let next_workflow = test_split[1].to_owned();
            if test_split[0].contains('<') {
                let test_split = test_split[0].split('<').collect::<Vec<&str>>();
                tests.push(Test {
                    attribute: test_split[0].chars().next().unwrap(),
                    compare: '<',
                    compare_to: test_split[1].parse::<i128>().unwrap(),
                    next_workflow,
                })
            } else {
                let test_split = test_split[0].split('>').collect::<Vec<&str>>();
                tests.push(Test {
                    attribute: test_split[0].chars().next().unwrap(),
                    compare: '>',
                    compare_to: test_split[1].parse::<i128>().unwrap(),
                    next_workflow,
                })
            }
        }
        workflows.insert(
            workflow_name,
            Workflow {
                tests,
                fallback: test_strings.last().unwrap().to_owned().to_owned(),
            },
        );
    });

    part_strings.iter().for_each(|part_string| {
        let mut chars = part_string.chars();
        chars.next();
        chars.next_back();
        let part_string = chars.as_str();
        let part_split = part_string.split(',').collect::<Vec<&str>>();
        let mut attributes: HashMap<char, i128> = HashMap::new();
        part_split.iter().for_each(|attribute_string| {
            let attribute_split = attribute_string.split('=').collect::<Vec<&str>>();
            attributes.insert(
                attribute_split[0].chars().next().unwrap(),
                attribute_split[1].parse::<i128>().unwrap(),
            );
        });
        parts.push(attributes);
    });

    (workflows, parts)
}

pub fn solver() {
    let input = get_input("./src/day19/input.txt");
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
    fn day19_example_input_part_one() {
        let input = get_input("./src/day19/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(19114, sum_part_one);
    }

    #[test]
    fn day19_input_part_one() {
        let input = get_input("./src/day19/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(342650, sum_part_one);
    }

    #[test]
    fn day19_example_input_part_two() {
        let input = get_input("./src/day19/example_input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(167409079868000, sum_part_two);
    }

    #[test]
    fn day19_input_part_two() {
        let input = get_input("./src/day19/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(201398068194715, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day19/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = get_input("./src/day19/input.txt");
        b.iter(|| solve_part_two(&input))
    }
}
