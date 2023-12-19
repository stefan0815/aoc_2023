use num::abs;
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
                    return part.iter().map(|(_, value)| value).sum()
                }else if workflow_name == "R"{
                    return 0;
                }
            }
        })
        .sum()
}

fn solve_part_two(
    (workflows, _): &(HashMap<String, Workflow>, Vec<HashMap<char, i128>>),
) -> i128 {
    
    0
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
        assert_eq!(952408144115, sum_part_two);
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
