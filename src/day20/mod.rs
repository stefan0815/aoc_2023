use num::Integer;
use std::{
    collections::{HashMap, VecDeque},
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
    output: Vec<String>,
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.prefix == '%' {
            return write!(
                f,
                "name: {}{}, flip: {}, output: {:?}",
                self.prefix, self.name, self.flip, self.output
            );
        } else {
            let input_memory = self
                .input_memory
                .iter()
                .map(|(name, high_pulse)| (name.to_owned(), *high_pulse))
                .collect::<Vec<(String, bool)>>();
            return write!(
                f,
                "name: {}{}, input: {:?}, output: {:?}",
                self.prefix, self.name, input_memory, self.output
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
) -> (usize, usize) {
    let mut num_pulses: (usize, usize) = (0, 0);
    num_pulses.0 += 1;
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
        let module = modules.get_mut(&signal.to).unwrap();
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
    }
    num_pulses
}

fn solve_part_one(input: &Vec<String>) -> usize {
    let (broadcaster, mut modules) = parse_input(&input);
    let mut num_pulses: (usize, usize) = (0, 0);

    for _ in 0..1000 {
        let pulses = process_button_press(&broadcaster, &mut modules);
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
    let mut final_high_module_names: Vec<(String, bool)> = Vec::new();
    for (_, module) in &modules {
        if module.output.contains(&"rx".to_owned()) {
            conjunction_modules.push((module.clone(), false));
            break;
        }
    }
    let mut previous_conjunction_modules: Vec<(Module, bool)> = Vec::new();
    while !conjunction_modules.is_empty() {
        let mut next_conjunction_modules: Vec<(Module, bool)> = Vec::new();
        for (module, output_goal) in &conjunction_modules {
            if module.input_memory.is_empty() {
                if !final_high_module_names.contains(&(module.name.to_owned(), !*output_goal)) {
                    final_high_module_names.push((module.name.to_owned(), !*output_goal));
                }
            } else {
                for (input_module_name, _) in &module.input_memory {
                    next_conjunction_modules
                        .push((modules[input_module_name].clone(), !*output_goal));
                }
            }
        }
        if next_conjunction_modules.is_empty() {
            conjunction_modules = previous_conjunction_modules;
            break;
        }
        previous_conjunction_modules = conjunction_modules;
        conjunction_modules = next_conjunction_modules;
    }

    // for (conjunction_module, output_goal) in &conjunction_modules {
    //     println!(
    //         "conjunction_modules: {}, output_goal: {output_goal}",
    //         conjunction_module
    //     );
    // }
    // println!("final_high_module_names: {:?}", final_high_module_names);
    let mut module_fulfills_output: HashMap<String, usize> = HashMap::new();

    loop {
        button_presses += 1;
        process_button_press(&broadcaster, &mut modules);

        for (module, output_goal) in &conjunction_modules {
            let module = &modules[&module.name];
            if *output_goal {
                if !module
                    .input_memory
                    .iter()
                    .all(|(_, high_pulse)| *high_pulse)
                    && module
                        .input_memory
                        .iter()
                        .any(|(_, high_pulse)| *high_pulse)
                {
                    println!("Module fullfilled: {}, button_presses: {button_presses}, output_goal: {output_goal}", module);
                    module_fulfills_output.insert(module.name.to_owned(), button_presses);
                }
            } else {
                if module
                    .input_memory
                    .iter()
                    .all(|(_, high_pulse)| *high_pulse)
                {
                    println!("Module fullfilled: {}, button_presses: {button_presses}, output_goal: {output_goal}", module);
                    module_fulfills_output.insert(module.name.to_owned(), button_presses);
                }
            }
        }
        // println!("module_fulfills_output:{} / {}", module_fulfills_output.len(), conjunction_modules.len());
        if module_fulfills_output.len() == conjunction_modules.len(){
            return module_fulfills_output.iter().map(|(_, button_presses)| *button_presses).fold(1, |a, b| a.lcm(&b));
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
        assert_eq!(130303473508222, sum_part_two);
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
