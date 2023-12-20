use num::Integer;
use std::{
    collections::{HashMap, VecDeque},
    fs,
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

fn solve_part_one(input: &Vec<String>) -> usize {
    let (broadcaster, mut modules) = parse_input(&input);
    let mut num_pulses: (usize, usize) = (0, 0);

    for _ in 0..1000 {
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
    }
    println!("count: {}, {}", num_pulses.0, num_pulses.1);

    num_pulses.0 * num_pulses.1
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let (broadcaster, mut modules) = parse_input(&input);
    let mut button_presses: usize = 0;

    let mut conjunction_modules: Vec<Module> = Vec::new();
    let mut final_high_module_names: Vec<String> = Vec::new();
    for (_, module) in &modules {
        // println!("module.output: {:?}", module.output);
        if module.output.contains(&"rx".to_owned()) {
            conjunction_modules.push(module.clone());
            break;
        }
    }

    while !conjunction_modules.is_empty() {
        // println!(
        //     "conjunction_modules: {:?}",
        //     conjunction_modules
        //         .iter()
        //         .map(|module| module.name.to_owned())
        //         .collect::<Vec<String>>()
        // );
        let mut next_conjunction_modules: Vec<Module> = Vec::new();
        for module in &conjunction_modules {
            if module.input_memory.is_empty() {
                if !final_high_module_names.contains(&module.name){
                    final_high_module_names.push(module.name.to_owned());
                }
            } else {
                for (input_module_name, _) in &module.input_memory {
                    next_conjunction_modules.push(modules[input_module_name].clone());
                }
            }
        }
        conjunction_modules = next_conjunction_modules;
    }

    println!("final_high_module_names: {:?}", final_high_module_names);
    let mut module_high_for_the_first_time: HashMap<String, usize> =  HashMap::new();

    loop {
        button_presses += 1;
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

            if signal.to == "rx" {
                if !signal.high_pulse {
                    return button_presses;
                }
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
        for module_name in &final_high_module_names {
            if modules[module_name].flip && !module_high_for_the_first_time.contains_key(module_name) {
                module_high_for_the_first_time.insert(module_name.to_owned(), button_presses);
            }
        }
        println!("module_high_for_the_first_time: {:?}", module_high_for_the_first_time);
        if module_high_for_the_first_time.len() == final_high_module_names.len() {
            return module_high_for_the_first_time.iter().map(|(_, val)| *val).fold(1, |a, b| a.lcm(&b))
        }
        // for (name, module) in &modules {
        //     if module.output.contains(&"rx".to_owned()) {
        //         if module.input_memory.iter().any(|(_, high_pulse)| *high_pulse) {
        //             print!("Buttons pressed: {button_presses}, Module: {name}, Input Memory: [");
        //             module
        //                 .input_memory
        //                 .iter()
        //                 .for_each(|(input_module, high_pulse)| {
        //                     print!("{input_module}:{high_pulse}, ");
        //                 });
        //             println!("]");
        //         }
        //     }
        // }
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
