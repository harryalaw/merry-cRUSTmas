use std::collections::VecDeque;

use hashbrown::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (mut modules, mut input_map) = parse_modules(input);
    let mut loops: Vec<(&str, usize)> = input_map
        .get(input_map.get("rx").unwrap()[0].0)
        .unwrap()
        .iter()
        .map(|(id, _)| (*id, 0))
        .collect();

    let mut values = 0;
    let mut i: usize = 0;
    loop {
        i += 1;
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse::low("button", "broadcaster"));
        while !pulses.is_empty() {
            let pulse = pulses.pop_front().unwrap();

            let target_module = modules.get_mut(pulse.target);
            if target_module.is_none() {
                continue;
            }
            let module = target_module.unwrap();

            match (module.module_type, pulse.state) {
                (ModuleType::Broadcast, _) => {
                    for target in &module.outputs {
                        let pulse = match pulse.state {
                            true => Pulse::high(pulse.target, target),
                            false => Pulse::low(pulse.target, target),
                        };
                        pulses.push_back(pulse);
                    }
                }
                (ModuleType::FlipFlop, true) => continue,
                (ModuleType::FlipFlop, false) => {
                    module.state = !module.state;
                    for target in &module.outputs {
                        let pulse = match module.state {
                            true => Pulse::high(pulse.target, target),
                            false => Pulse::low(pulse.target, target),
                        };
                        pulses.push_back(pulse);
                    }
                }
                (ModuleType::Conjunction, _) => {
                    let inputs = input_map.get(pulse.target).expect("Should have inputs");
                    let mut new_inputs = inputs.clone();

                    for input in new_inputs.iter_mut() {
                        if input.0 == pulse.origin {
                            input.1 = pulse.state;
                        }
                    }
                    for target in &module.outputs {
                        let pulse = match new_inputs.iter().all(|(_, b)| b == &true) {
                            false => Pulse::high(pulse.target, target),
                            true => Pulse::low(pulse.target, target),
                        };
                        pulses.push_back(pulse);
                    }

                    input_map.insert(pulse.target, new_inputs);
                    if pulse.state {
                        for (idx, (input, count)) in loops.iter_mut().enumerate() {
                            if &pulse.origin == input && count == &0 {
                                loops[idx] = (input, i);
                                values += 1;
                                break;
                            }
                        }
                        if values == loops.len() {
                            return lcm_arr(loops);
                        }
                    }
                }
            }
        }
    }
}

fn lcm_arr(arr: Vec<(&str, usize)>) -> usize {
    arr.iter().fold(1, |acc, curr| lcm(acc, curr.1))
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first.max(second);
    let mut min = first.min(second);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn parse_modules<'a>(
    input: &'a str,
) -> (
    HashMap<&'a str, Module<'a>>,
    HashMap<&'a str, Vec<(&str, bool)>>,
) {
    let module_map: HashMap<&'a str, Module<'a>> = input
        .lines()
        .map(|line| {
            let (start, targets) = line.split_once(" -> ").expect("It's got an arrow");
            let outputs: Vec<&str> = targets.split(", ").collect();
            let module_type = match &start[0..1] {
                "%" => ModuleType::FlipFlop,
                "&" => ModuleType::Conjunction,
                _ => ModuleType::Broadcast,
            };

            let id = if module_type == ModuleType::Broadcast {
                start
            } else {
                &start[1..]
            };

            (
                id,
                Module {
                    module_type,
                    outputs,
                    state: false,
                },
            )
        })
        .collect();

    let mut input_map: HashMap<&'a str, Vec<(&str, bool)>> = HashMap::new();
    module_map.iter().for_each(|(id, module)| {
        for output in module.outputs.iter() {
            let previous = input_map.get(output);
            if previous.is_none() {
                input_map.insert(output, vec![(id, false)]);
            } else {
                let mut new = previous.unwrap().clone();
                new.push((id, false));
                input_map.insert(output, new);
            }
        }
    });
    (module_map, input_map)
}

#[derive(Debug)]
struct Pulse<'a> {
    state: bool,
    target: &'a str,
    origin: &'a str,
}

impl Pulse<'_> {
    fn high<'a>(origin: &'a str, target: &'a str) -> Pulse<'a> {
        Pulse {
            state: true,
            target,
            origin,
        }
    }
    fn low<'a>(origin: &'a str, target: &'a str) -> Pulse<'a> {
        Pulse {
            state: false,
            target,
            origin,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug)]
struct Module<'a> {
    module_type: ModuleType,
    state: bool,
    outputs: Vec<&'a str>,
}
