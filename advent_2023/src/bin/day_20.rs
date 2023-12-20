use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module {
    // Name, Outputs, State
    FlipFlop(String, Vec<String>, bool),
    Conjunction(String, Vec<String>, HashMap<String, Signal>),
    Broadcast(String, Vec<String>),
}

struct Send(String, String, Signal);

impl Debug for Send {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -{:?}-> {}", self.0, self.2, self.1))
    }
}

fn parse(lines: &[String]) -> HashMap<String, Module> {
    let mut modules = HashMap::new();

    for line in lines {
        let mut line_split = line.split(" -> ");
        let lhs = line_split.next().unwrap();
        let rhs = line_split.next().unwrap();

        let target_names = rhs.split(", ").map(str::to_string).collect();

        if lhs == ("broadcaster") {
            let this = Module::Broadcast("broadcaster".to_string(), target_names);
            modules.insert("broadcaster".to_string(), this);
        } else if lhs.starts_with('%') {
            let name: String = lhs.chars().skip(1).collect();
            let this = Module::FlipFlop(name.to_string(), target_names, false);
            modules.insert(name, this);
        } else if lhs.starts_with('&') {
            let name: String = lhs.chars().skip(1).collect();
            let inputs = HashMap::new();
            let this = Module::Conjunction(name.to_string(), target_names, inputs);
            modules.insert(name, this);
        } else {
            panic!("Can't parse {line}");
        }
    }

    let all_modules: Vec<Module> = modules.values().cloned().collect();

    // Now update the inputs HashMap for the conjunctions
    for (name, module) in &mut modules {
        if let Module::Conjunction(_, _, inputs) = module {
            let input_names = all_modules.iter().filter_map(|m| match m {
                Module::FlipFlop(other_name, target_names, _) if target_names.contains(name) => {
                    Some(other_name.to_string())
                }
                Module::Conjunction(other_name, target_names, _) if target_names.contains(name) => {
                    Some(other_name.to_string())
                }
                Module::Broadcast(target_names, _) if target_names.contains(name) => {
                    Some("broadcast".to_string())
                }
                _ => None,
            });

            for iname in input_names {
                inputs.insert(iname.to_string(), Signal::Low);
            }
        }
    }

    modules
}

fn part_a(lines: &[String]) -> usize {
    let modules = &mut parse(lines);

    let mut pulse_count_low = 0;
    let mut pulse_count_high = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(Send(
            "button".to_string(),
            "broadcaster".to_string(),
            Signal::Low,
        ));

        while let Some(s) = queue.pop_front() {
            let Send(source, target_name, signal) = s;

            if signal == Signal::High {
                pulse_count_high += 1;
            } else {
                pulse_count_low += 1;
            }

            modules.entry(target_name.to_string()).and_modify(|target| {
                match (source, target, signal) {
                    (_, Module::FlipFlop(name, targets, on), Signal::Low) => {
                        *on = !*on;
                        if *on {
                            queue.extend(
                                targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::High)
                                }),
                            );
                        } else {
                            queue.extend(
                                targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::Low)
                                }),
                            );
                        }
                    }
                    (_, Module::FlipFlop(_, _, _), Signal::High) => (),
                    (source, Module::Conjunction(name, targets, inputs), signal) => {
                        inputs.insert(source.to_string(), signal);
                        if inputs.values().all(|s| s == &Signal::High) {
                            queue.extend(
                                targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::Low)
                                }),
                            );
                        } else {
                            queue.extend(
                                targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::High)
                                }),
                            );
                        }
                    }
                    (_, Module::Broadcast(name, target_names), Signal::Low) => {
                        queue.extend(
                            target_names
                                .iter()
                                .map(|tn| Send((*name).to_string(), tn.to_string(), Signal::Low)),
                        );
                    }
                    _ => unreachable!(),
                }
            });
        }
    }

    pulse_count_low * pulse_count_high
}

fn part_b(lines: &[String]) -> usize {
    // If you draw the digraph for the modules and connections then you'll find
    // there are 4 major components. (jp -> pg, bx -> sp, jq -> sv, nv -> qs)
    //
    // These can all modelled separately to see when what the periodicity of
    // emitting high would be for each component.
    //
    // We can then use the LCM for those periods to determine the first period when
    // the would all line up together

    const MAX: usize = 5000;
    let modules = &mut parse(lines);

    let mut push_counts = vec![];

    'outer: for (start, end) in [("jp", "pg"), ("bx", "sp"), ("jq", "sv"), ("nv", "qs")] {
        for push_count in 0..MAX {
            let mut queue = VecDeque::new();

            // Push the start node directly rather than the button by pretending to be the broadcaster
            queue.push_back(Send(
                "broadcaster".to_string(),
                start.to_string(),
                Signal::Low,
            ));

            while let Some(s) = queue.pop_front() {
                let Send(source, target_name, signal) = s;

                if target_name == end && signal == Signal::Low && push_count > 0 {
                    push_counts.push(push_count + 1);
                    continue 'outer;
                }

                modules.entry(target_name.to_string()).and_modify(|target| {
                    match (source, target, signal) {
                        (_, Module::FlipFlop(name, targets, on), Signal::Low) => {
                            *on = !*on;
                            if *on {
                                queue.extend(targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::High)
                                }));
                            } else {
                                queue.extend(targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::Low)
                                }));
                            }
                        }
                        (_, Module::FlipFlop(_, _, _), Signal::High) => (),
                        (source, Module::Conjunction(name, targets, inputs), signal) => {
                            inputs.insert(source.to_string(), signal);
                            if inputs.values().all(|s| s == &Signal::High) {
                                queue.extend(targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::Low)
                                }));
                            } else {
                                queue.extend(targets.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::High)
                                }));
                            }
                        }
                        (_, Module::Broadcast(name, target_names), Signal::Low) => {
                            queue.extend(
                                target_names.iter().map(|tn| {
                                    Send((*name).to_string(), tn.to_string(), Signal::Low)
                                }),
                            );
                        }
                        _ => unreachable!(),
                    }
                });
            }
        }
    }

    push_counts.into_iter().reduce(num::integer::lcm).unwrap()
}

#[cfg(not(tarpaulin_include))]
fn main() -> AResult<()> {
    // Figure out which exercise we are so we can load the correct input file
    let pattern = Regex::new("[0-9]+$")?;
    let name = &std::env::args().next().expect("binary name not found.");
    let ex = pattern
        .find(name)
        .expect("binary name should contain a number")
        .as_str();
    println!("Running code for Day {ex}.");

    // Load the appropriate input text
    let file = File::open(format!("./data/day_{ex}.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(lines.as_slice()));
    println!("Part B result = {}", part_b(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a";

    const TEST_INPUT_2: &str = "broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT_1.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 32_000_000);
        let lines: Vec<_> = TEST_INPUT_2.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 11_687_500);
    }

    #[test]
    fn test_b() {}
}
