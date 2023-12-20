use std::collections::VecDeque;

use hashbrown::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (mut modules, mut input_map, broadcaster_idx) = parse_modules(input);
    let mut lo_pulses = 0;
    let mut hi_pulses = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse::low(usize::MAX, broadcaster_idx));
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
                            true => Pulse::high(pulse.target, *target),
                            false => Pulse::low(pulse.target, *target),
                        };
                        pulses.push_back(pulse);
                    }
                }
                (ModuleType::FlipFlop, true) => continue,
                (ModuleType::FlipFlop, false) => {
                    module.state = !module.state;
                    for target in &module.outputs {
                        let pulse = match module.state {
                            true => Pulse::high(pulse.target, *target),
                            false => Pulse::low(pulse.target, *target),
                        };
                        pulses.push_back(pulse);
                    }
                }
                (ModuleType::Conjunction, _) => {
                    let inputs = &mut input_map[pulse.target];
                    for input in inputs.iter_mut() {
                        if input.0 == pulse.origin {
                            input.1 = pulse.state;
                        }
                    }

                    for target in &module.outputs {
                        let pulse = match inputs.iter().all(|(_, b)| b == &true) {
                            false => Pulse::high(pulse.target, *target),
                            true => Pulse::low(pulse.target, *target),
                        };
                        pulses.push_back(pulse);
                    }
                }
            }
        }
    }

    lo_pulses * hi_pulses
}

fn parse_modules(input: &str) -> (Vec<Module>, Vec<Vec<(usize, bool)>>, usize) {
    let mut id_to_index: HashMap<&str, usize> = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (start, _) = line.split_once(" -> ").expect("It's got an arrow");
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
            (id, idx)
        })
        .collect();

    let module_map: Vec<Module> = input
        .lines()
        .map(|line| {
            let (start, targets) = line.split_once(" -> ").expect("It's got an arrow");
            let outputs: Vec<usize> = targets
                .split(", ")
                .map(|x| {
                    if let Some(x) = id_to_index.get(x) {
                        *x
                    } else {
                        let new_index = id_to_index.len();
                        id_to_index.insert(x, new_index);
                        new_index
                    }
                })
                .collect();
            let module_type = match &start[0..1] {
                "%" => ModuleType::FlipFlop,
                "&" => ModuleType::Conjunction,
                _ => ModuleType::Broadcast,
            };

            Module {
                module_type,
                outputs,
                state: false,
            }
        })
        .collect();

    let mut input_map: Vec<Vec<(usize, bool)>> = Vec::with_capacity(id_to_index.len());
    for _ in 0..id_to_index.len() {
        input_map.push(Vec::new());
    }

    module_map.iter().enumerate().for_each(|(idx, module)| {
        for output in module.outputs.iter() {
            let previous = &mut input_map[*output];
            previous.push((idx, false));
        }
    });
    (
        module_map,
        input_map,
        *id_to_index
            .get("broadcaster")
            .expect("contains broadcaster"),
    )
}

#[derive(Debug)]
struct Pulse {
    state: bool,
    target: usize,
    origin: usize,
}

impl Pulse {
    fn high(origin: usize, target: usize) -> Pulse {
        Pulse {
            state: true,
            target,
            origin,
        }
    }
    fn low(origin: usize, target: usize) -> Pulse {
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
struct Module {
    module_type: ModuleType,
    state: bool,
    outputs: Vec<usize>,
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
