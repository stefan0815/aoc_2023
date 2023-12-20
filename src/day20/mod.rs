use num::Integer;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt, fs,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

struct Signal {
    from: String,
    to: String,
    high_pulse: bool, // false: low pulse
}

#[derive(Clone, PartialEq, Eq)]

struct Module {
    prefix: char,
    name: String,
    input_memory: HashMap<String, bool>, // false: low pulse
    flip: bool,
    fired: bool,
    output: Vec<String>,
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.prefix == '%' {
            return write!(
                f,
                "name: {}{}, flip: {}, fired: {}, output: {:?}",
                self.prefix, self.name, self.flip, self.fired, self.output
            );
        } else {
            let input_memory = self
                .input_memory
                .iter()
                .map(|(name, high_pulse)| (name.to_owned(), *high_pulse))
                .collect::<Vec<(String, bool)>>();
            return write!(
                f,
                "name: {}{}, fired: {}, input: {:?}, output: {:?}",
                self.prefix, self.name, self.fired, input_memory, self.output
            );
        }
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<String>, HashMap<String, Module>) {
    let mut broadcaster: Vec<String> = Vec::new();
    let mut modules: HashMap<String, Module> = HashMap::new();
    input.iter().for_each(|module_description| {
        let split: Vec<&str> = module_description.split("->").collect();
        let name = split[0].trim();
        let output: Vec<String> = split[1].trim().split(", ").map(|s| s.to_owned()).collect();
        if name == "broadcaster" {
            broadcaster = output;
            return;
        }
        let mut chars = name.chars();
        let prefix = chars.next().unwrap();
        let name = chars.as_str().to_owned();
        let module = Module {
            prefix,
            name: name.to_owned(),
            input_memory: HashMap::new(),
            flip: false,
            fired: false,
            output,
        };
        modules.insert(name, module);
    });
    broadcaster.iter().for_each(|output| {
        if modules[output].prefix == '&' {
            modules
                .get_mut(output)
                .unwrap()
                .input_memory
                .insert("broadcaster".to_owned(), false);
        }
    });

    for (_, module) in modules.clone() {
        module.output.iter().for_each(|output| {
            if modules.contains_key(output) && modules[output].prefix == '&' {
                modules
                    .get_mut(output)
                    .unwrap()
                    .input_memory
                    .insert(module.name.to_owned(), false);
            }
        });
    }

    (broadcaster, modules)
}

fn process_button_press(
    broadcaster: &Vec<String>,
    modules: &mut HashMap<String, Module>,
    module_name_of_interest: &String,
) -> ((usize, usize), HashSet<String>) {
    let mut fullfilled: HashSet<String> = HashSet::new();
    let mut num_pulses: (usize, usize) = (0, 0);
    num_pulses.0 += 1;
    for (_, module) in &mut *modules {
        module.fired = false;
    }
    let mut signals_to_process: VecDeque<Signal> = VecDeque::new();
    broadcaster.iter().for_each(|output| {
        signals_to_process.push_back(Signal {
            from: "broadcaster".to_owned(),
            to: output.to_owned(),
            high_pulse: false,
        });
    });
    while !signals_to_process.is_empty() {
        let signal = signals_to_process.pop_front().unwrap();
        // println!("signal: from: {}, to: {}, high_pulse: {}", signal.from, signal.to, signal.high_pulse);

        if signal.high_pulse {
            num_pulses.1 += 1;
        } else {
            num_pulses.0 += 1;
        }

        if !modules.contains_key(&signal.to) {
            continue;
        }
        // if !signal.high_pulse &&(signal.to == "jm" || signal.to == "jg" || signal.to == "hf" || signal.to == "rh")  {
        //     println!("Low Signal to {}", signal.to);
        // }
        let module = modules.get_mut(&signal.to).unwrap();
        module.fired = true;
        if module.prefix == '%' {
            if signal.high_pulse {
                continue;
            }
            module.flip = !module.flip;
            module.output.iter().for_each(|output| {
                let next_signal = Signal {
                    from: module.name.to_owned(),
                    to: output.to_owned(),
                    high_pulse: module.flip,
                };
                signals_to_process.push_back(next_signal);
            });
        } else {
            if module.input_memory.contains_key(&signal.from) {
                *module.input_memory.get_mut(&signal.from).unwrap() = signal.high_pulse;
            }

            let mut next_pulse: bool = false;
            for (_, high_pulse_input) in &module.input_memory {
                if !high_pulse_input {
                    next_pulse = true;
                    break;
                }
            }
            module.output.iter().for_each(|output| {
                let next_signal = Signal {
                    from: module.name.to_owned(),
                    to: output.to_owned(),
                    high_pulse: next_pulse,
                };
                signals_to_process.push_back(next_signal);
            });
        }
        if signal.to == *module_name_of_interest {
            for (memory_module_name, memory_high_pulse) in &module.input_memory {
                if *memory_high_pulse {
                    fullfilled.insert(memory_module_name.to_owned());
                }
            }
        }
    }
    (num_pulses, fullfilled)
}

fn solve_part_one(input: &Vec<String>) -> usize {
    let (broadcaster, mut modules) = parse_input(&input);
    let mut num_pulses: (usize, usize) = (0, 0);

    for _ in 0..1000 {
        let (pulses, _) = process_button_press(&broadcaster, &mut modules, &"".to_owned());
        num_pulses.0 += pulses.0;
        num_pulses.1 += pulses.1;
    }
    println!("count: {}, {}", num_pulses.0, num_pulses.1);

    num_pulses.0 * num_pulses.1
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let (broadcaster, mut modules) = parse_input(&input);
    let mut button_presses: usize = 0;

    let mut conjunction_modules: Vec<(Module, bool)> = Vec::new();
    let mut pre_goal_module: Option<String> = None;
    let mut pre_goal_input_length: Option<usize> = None;
    for (_, module) in &modules {
        if module.output.contains(&"rx".to_owned()) {
            conjunction_modules.push((module.clone(), false));
            pre_goal_module = Some(module.name.to_owned());
            pre_goal_input_length = Some(module.input_memory.len());
            break;
        }
    }
    let pre_goal_module = pre_goal_module.unwrap();
    let pre_goal_input_length = pre_goal_input_length.unwrap();
    // println!("{pre_goal_module}");
    let mut fulfilled_input: HashMap<String, usize> = HashMap::new();
    loop {
        button_presses += 1;
        let (_, fulfilled_input_modules) =
            process_button_press(&broadcaster, &mut modules, &pre_goal_module);
        for fulfilled_input_module in fulfilled_input_modules {
            fulfilled_input.insert(fulfilled_input_module, button_presses);
        }
        if fulfilled_input.len() == pre_goal_input_length {
            return fulfilled_input.iter().map(|(_, val)| *val).fold(1, |a, b| a.lcm(&b));
        }
    }
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
    let input = get_input("./src/day20/input.txt");
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
    fn day20_example_input_part_one() {
        let input = get_input("./src/day20/example_input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(32000000, sum_part_one);
    }

    #[test]
    fn day20_input_part_one() {
        let input = get_input("./src/day20/input.txt");
        let sum_part_one = solve_part_one(&input);
        assert_eq!(821985143, sum_part_one);
    }

    // #[test]
    // fn day20_example_input_part_two() {
    //     let input = get_input("./src/day20/example_input.txt");
    //     let sum_part_two = solve_part_two(&input);
    //     assert_eq!(167409079868000, sum_part_two);
    // }

    #[test]
    fn day20_input_part_two() {
        let input = get_input("./src/day20/input.txt");
        let sum_part_two = solve_part_two(&input);
        assert_eq!(240853834793347, sum_part_two);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = get_input("./src/day20/input.txt");
        b.iter(|| solve_part_one(&input))
    }

    // #[bench]
    // fn bench_part_two(b: &mut Bencher) {
    //     let input = get_input("./src/day20/input.txt");
    //     b.iter(|| solve_part_two(&input))
    // }
}
