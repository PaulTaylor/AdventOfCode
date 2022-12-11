use humantime::format_duration;
use lazy_static::lazy_static;
use regex::{Captures, Regex, RegexBuilder};
use std::{collections::VecDeque, fs, path::Path, time::Instant};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug, PartialEq, Eq)]
struct Monkey<'m> {
    id: usize,
    items: VecDeque<usize>,
    op: char,
    operand: &'m str,
    test_div: usize,
    dest_t: usize,
    dest_f: usize,
    inspects: usize,
}

impl Monkey<'_> {
    fn new(c: Captures) -> Option<Monkey> {
        Some(Monkey {
            id: c.name("monkey").map(|s| s.as_str().parse().unwrap())?,
            items: c
                .name("items")?
                .as_str()
                .split(", ")
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect(),
            op: c.name("op")?.as_str().chars().next()?,
            operand: c.name("operand")?.as_str(),
            test_div: c.name("div")?.as_str().parse().unwrap(),
            dest_t: c.name("dest_t")?.as_str().parse().unwrap(),
            dest_f: c.name("dest_f")?.as_str().parse().unwrap(),
            inspects: 0,
        })
    }
}

fn parse(lines: &str) -> AResult<Vec<Monkey>> {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(
            r##"Monkey (?P<monkey>[0-9]+):
  Starting items: (?P<items>[0-9, ]+)
  Operation: new = old (?P<op>[+*]) (?P<operand>old|[0-9]+)
  Test: divisible by (?P<div>[0-9]+)
    If true: throw to monkey (?P<dest_t>[0-9]+)
    If false: throw to monkey (?P<dest_f>[0-9]+)"##
        )
        .multi_line(true)
        .build()
        .unwrap();
    }

    Ok(RE
        .captures_iter(lines)
        .map(Monkey::new)
        .map(Option::unwrap)
        .collect())
}

fn part_a(lines: &str) -> AResult<usize> {
    let mut monkeys = parse(lines)?;

    for _r in 0..20 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];

            // We need to collect the moves and apply them after the while
            // loop because the borrow checker won't let us have multiple
            // mutable references from the monkeys Vec in play at once
            let mut moves: Vec<_> = Vec::with_capacity(m.items.len());

            while let Some(mut item) = m.items.pop_front() {
                // Monkey inspects item
                m.inspects += 1;
                item = match m {
                    Monkey {
                        op: '*', operand, ..
                    } => item * operand.parse().unwrap_or(item),
                    Monkey {
                        op: '+', operand, ..
                    } => item + operand.parse().unwrap_or(item),
                    _ => panic!("Unknown op {} {}", m.op, m.operand),
                };

                // Monkey gets bored
                item /= 3;

                // apply the test and collect the results
                if item.rem_euclid(m.test_div) == 0 {
                    moves.push((m.dest_t, item));
                } else {
                    moves.push((m.dest_f, item));
                }
            }

            // Now apply the moves
            for (dest, item) in moves {
                monkeys[dest].items.push_back(item);
            }
        }
    }

    let mut counts: Vec<_> = monkeys.iter().map(|m| m.inspects).collect();
    counts.sort();
    Ok(counts.iter().rev().take(2).product())
}

fn part_b(lines: &str) -> AResult<usize> {
    let mut monkeys = parse(lines)?;

    // Worry levels can be contained within the range 0..common_factor
    // because we only need to work on the relative offset within this
    // range - not the absolute worry value, and we're only using + and *
    let common_factor = monkeys.iter().map(|m| m.test_div).product();

    for _r in 0..10000 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let mut moves: Vec<_> = Vec::with_capacity(m.items.len());

            while let Some(mut item) = m.items.pop_front() {
                // Monkey inspects item
                m.inspects += 1;
                item = match m {
                    Monkey {
                        op: '*', operand, ..
                    } => item * operand.parse().unwrap_or(item),
                    Monkey {
                        op: '+', operand, ..
                    } => item + operand.parse().unwrap_or(item),
                    _ => panic!("Unknown op {} {}", m.op, m.operand),
                };

                // Apply the worry level control
                item = item.rem_euclid(common_factor);

                // apply the test and collect the results
                if item.rem_euclid(m.test_div) == 0 {
                    moves.push((m.dest_t, item));
                } else {
                    moves.push((m.dest_f, item));
                }
            }

            // Now apply the moves
            for (dest, item) in moves {
                monkeys[dest].items.push_back(item);
            }
        }
    }

    let mut counts: Vec<_> = monkeys.iter().map(|m| m.inspects).collect();
    counts.sort();
    Ok(counts.iter().rev().take(2).product())
}

fn main() -> AResult<()> {
    // Figure out which exercise we are so we can load the correct input file
    let pattern = Regex::new("[0-9]+$")?;
    let name = &std::env::args().next().expect("binary name not found.");
    let ex = pattern
        .find(name)
        .expect("binary name should contain a number")
        .as_str();
    println!("Running code for Day {}.", ex);

    // Load the appropriate input text
    let file = format!("./data/day_{ex}.txt");
    let lines = fs::read_to_string(Path::new(file.as_str()))?;

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(&lines)?);
    println!("Part B result = {}", part_b(&lines)?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
  
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
  
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
  
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse() -> AResult<()> {
        let monkeys = parse(TEST_INPUT)?;
        assert_eq!(monkeys.len(), 4);
        assert_eq!(
            monkeys[0],
            Monkey {
                id: 0,
                items: VecDeque::from_iter(vec![79, 98]),
                op: '*',
                operand: "19",
                test_div: 23,
                dest_t: 2,
                dest_f: 3,
                inspects: 0
            }
        );
        assert_eq!(
            monkeys[2],
            Monkey {
                id: 2,
                items: VecDeque::from_iter(vec![79, 60, 97]),
                op: '*',
                operand: "old",
                test_div: 13,
                dest_t: 1,
                dest_f: 3,
                inspects: 0
            }
        );
        Ok(())
    }

    #[test]
    fn test_a() -> AResult<()> {
        assert_eq!(part_a(TEST_INPUT)?, 10605);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        assert_eq!(part_b(TEST_INPUT)?, 2713310158);
        Ok(())
    }
}
