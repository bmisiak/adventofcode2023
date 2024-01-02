use core::slice::SlicePattern;
use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input/day1.txt");

#[derive(Copy, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High
}
impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        if value { Pulse::High } else { Pulse:: Low }
    }
}

trait PulseReceiver {
    fn receive_pulse(&mut self, source_input_name: &str, pulse: Pulse);
}
#[derive(Default)]
struct FlipFlopModule {
    is_on: bool
}
impl PulseReceiver for FlipFlopModule {
    fn receive_pulse(&mut self, source_input_name: &str, pulse: Pulse) {

    }
}

#[derive(Default)]
struct ConjunctionModule {
    most_recent_pulse_per_input: HashMap<String, Pulse>
}

enum ModuleState {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcast,
}

struct Module {
    state: ModuleState,
    outputs: Vec<String>
}

impl Module {
    fn react_to_pulse<'a, 'inputs>(&'a mut self, inputs: &'inputs [&'inputs str], input_name: Option<&'inputs str>, pulse: Pulse) -> impl Iterator<Item = (String, String, Pulse)> + 'a where 'a: 'inputs {
        let output_pulse = match &mut self.state {
            ModuleState::FlipFlop(ref mut flipflop) => {
                if Pulse::Low == pulse {
                    flipflop.is_on = !flipflop.is_on;
                    Some(flipflop.is_on.into())
                } else {
                    None
                }
            }
            ModuleState::Conjunction(ref mut conjunction) => {
                if let Some(input_name) = input_name {
                    *conjunction.most_recent_pulse_per_input.get_mut(input_name).unwrap() = pulse;
                }
                Some(inputs
                .iter()
                .all(|&input_name| conjunction.most_recent_pulse_per_input[input_name] == Pulse::High)
                .into())
            },
            ModuleState::Broadcast => {
                Some(pulse)
            }
        };

        output_pulse.map(|output_pulse| {
            self.outputs
                .iter()
                .map(move |output_name| (input_name.clone(), output_name.clone(), output_pulse.clone()))
        })
        .into_iter()
        .flatten()
    }
}

pub fn part1() -> u32 {
    let mut modules = HashMap::new();
    let mut module_inputs = HashMap::new();
    for line in INPUT.lines() {
        let (module_pattern, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ");

        let (state, name) = if module_pattern.starts_with('%') {
            (ModuleState::FlipFlop(FlipFlopModule::default()), module_pattern.strip_prefix('%').unwrap().to_owned())
        } else if module_pattern.starts_with('&') {
            (ModuleState::Conjunction(ConjunctionModule::default()), module_pattern.strip_prefix('&').unwrap().to_owned())
        } else if module_pattern == "broadcaster" {
            (ModuleState::Broadcast, module_pattern.to_owned())
        } else {
            panic!("unrecognized module type");
        };

        let name2 = name.clone();
        for output in outputs.clone() {
            module_inputs.entry(output.to_owned()).or_insert(vec![]).push(name2.clone());
        }

        modules.insert(name.clone(), Module {
            state,
            outputs: outputs.into_iter().map(|str| str.to_string()).collect()
        });
    }

    let mut pulse_queue = modules
        .get_mut("broadcaster")
        .unwrap()
        .react_to_pulse(&[], None, Pulse::Low)
        .collect::<VecDeque<_>>();

    loop {
        let Some((sender, target, pulse)) = pulse_queue.pop_front() else { break; };
        let generated_pulses = modules.get_mut(&target).unwrap().react_to_pulse(module_inputs[&target].as_slice(), Some(&sender), pulse);
    }
    0
}

pub fn part2() -> u32 {
    0
}

#[test]
fn test_day20() {
    assert_eq!(part1(), 0);
    assert_eq!(part2(), 0);
}