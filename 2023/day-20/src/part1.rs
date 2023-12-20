use std::collections::VecDeque;

use hashbrown::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (mut modules, mut input_map) = parse_modules(input);
    let mut lo_pulses = 0;
    let mut hi_pulses = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse::low("button", "broadcaster"));
        while !pulses.is_empty() {
            let pulse = pulses.pop_front().unwrap();

            if pulse.state {
                lo_pulses += 1;
            } else {
                hi_pulses += 1;
            };

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
                },
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
                }
            }
        }
    }

    lo_pulses * hi_pulses
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(32000000, process(input));
    }

    #[test]
    fn test_process_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(11687500, process(input));
    }
}
