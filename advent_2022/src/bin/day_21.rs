use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};
use Monkey::{Human, Number, Op};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug)]
enum Monkey<'a> {
    // Number(id, number)
    Number(&'a str, usize),
    // Op(id, l_id, op, r_id, result)
    Op(&'a str, &'a str, char, &'a str, Option<usize>),
    // Marker type for part b
    Human,
}

impl Monkey<'_> {
    fn id(&self) -> &str {
        match self {
            Number(id, _) | Op(id, _, _, _, _) => id,
            Human => "humn",
        }
    }

    // Is this monkey ready to be evaluated
    fn ready(&self, ctx: &HashMap<&str, usize>) -> bool {
        match &self {
            Op(_, l, _, r, None) => ctx.contains_key(l) && ctx.contains_key(r),
            _ => false,
        }
    }
}

fn parse(lines: &[String], humans_are_special: bool) -> Vec<Monkey> {
    lines
        .iter()
        .map(|line| -> Monkey {
            let parts: Vec<_> = line.split_whitespace().collect();
            match parts.len() {
                2 if parts[0] == "humn:" && humans_are_special => Human,
                2 => Number(&parts[0][..4], parts[1].parse().unwrap()),
                4 if parts[0] == "root:" && humans_are_special => {
                    Op(&parts[0][..4], parts[1], '=', parts[3], None)
                }
                4 => Op(
                    &parts[0][..4],
                    parts[1],
                    parts[2].chars().next().unwrap(),
                    parts[3],
                    None,
                ),
                _ => panic!("Unknown line pattern: {line}"),
            }
        })
        .collect()
}

fn observe_the_monkeys(monkeys: &mut Vec<Monkey>) {
    // Run the simulation forward as much as possible
    // (yes, I could create the DAG and do this more efficiently, but this is more than fast enough)

    // Seed the context with the "number" monkeys
    let mut ctx: HashMap<_, _> = monkeys
        .iter()
        .filter_map(|m| match m {
            &Number(id, num) => Some((id, num)),
            _ => None,
        })
        .collect();

    let mut changed = true;
    while changed {
        changed = false;

        for m in &mut *monkeys {
            let ready = m.ready(&ctx);

            match m {
                Op(id, l_id, op, r_id, res) if res.is_none() && ready => {
                    let result = match op {
                        '+' => ctx[l_id] + ctx[r_id],
                        '-' => ctx[l_id] - ctx[r_id],
                        '/' => ctx[l_id] / ctx[r_id],
                        '*' => ctx[l_id] * ctx[r_id],
                        _ => panic!("Unknown op: {op}"),
                    };

                    changed = true;
                    *res = Some(result);
                    ctx.insert(id, result);
                }
                _ => (),
            };
        }
    }
}

fn part_a(lines: &[String]) -> usize {
    let mut monkeys = parse(lines, false);
    observe_the_monkeys(&mut monkeys);

    monkeys
        .iter()
        .find_map(|m| match m {
            Op("root", _, _, _, result) => *result,
            _ => None,
        })
        .expect("Did not complete cleanly - root has no result!")
}

fn determine_value(monkey: &Monkey, target: usize, monkeys: &HashMap<&str, &Monkey>) -> usize {
    if let Op(_, l_id, op, r_id, None) = monkey {
        let lhs = monkeys.get(l_id);
        let rhs = monkeys.get(r_id);

        match (lhs, rhs) {
            (Some(Op(_, _, _, _, None)), Some(Number(_, r_res) | Op(_, _, _, _, Some(r_res)))) => {
                // lhs is unknown-op, rhs is a either number or a op with a known result
                match op {
                    '/' => determine_value(lhs.unwrap(), target * r_res, monkeys),
                    '+' => determine_value(lhs.unwrap(), target - r_res, monkeys),
                    '*' => determine_value(lhs.unwrap(), target / r_res, monkeys),
                    '-' => determine_value(lhs.unwrap(), target + r_res, monkeys),
                    _ => panic!("unknown op at location 0"),
                }
            }
            (Some(Number(_, l_res) | Op(_, _, _, _, Some(l_res))), Some(Op(_, _, _, _, None))) => {
                // lhs is a number or known-op - rhs is a unknown-op
                match op {
                    '+' => determine_value(rhs.unwrap(), target - l_res, monkeys),
                    '*' => determine_value(rhs.unwrap(), target / l_res, monkeys),
                    '-' => determine_value(rhs.unwrap(), l_res - target, monkeys),
                    _ => panic!("unknown op at location 1"),
                }
            }
            (Some(Human), Some(Number(_, r_res))) => {
                // lhs is a human and the rhs is a known value - return the target
                match op {
                    '-' => target + r_res,
                    _ => panic!("unknown op at location 2"),
                }
            }
            (Some(Number(_, l_res)), Some(Human)) => {
                // lhs is a known value and rhs is a human  - return the target
                match op {
                    '+' => target - l_res,
                    _ => panic!("unknown op at location 3"),
                }
            }
            _ => {
                panic!(r#"can't determine the value when neither side is known"#)
            }
        }
    } else {
        panic!("cannot pass non-op to this function");
    }
}

fn part_b(lines: &[String]) -> usize {
    let mut monkeys = parse(lines, true);
    observe_the_monkeys(&mut monkeys);

    // Create dial-a-monkey lookup
    let monkeys: HashMap<&str, _> = monkeys.iter().map(|m| (m.id(), m)).collect();

    // Find out which part of root is filled - and what the target value is
    // the recurse back up the tree to humn to determine its value

    if let Some((_, Op(_, l_id, _, r_id, _))) = monkeys.iter().find(|(&k, _)| k == "root") {
        if let Some(Op(_, _, _, _, Some(target))) = &monkeys.get(l_id) {
            // lhs is known - determine the rhs
            determine_value(monkeys[r_id], *target, &monkeys)
        } else if let Some(Op(_, _, _, _, Some(target))) = &monkeys.get(r_id) {
            // rhs is known - determine the lhs
            determine_value(monkeys[l_id], *target, &monkeys)
        } else {
            panic!("can't determine the value when neither monkey is known")
        }
    } else {
        panic!("No root monkey found")
    }
}

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

    const TEST_INPUT: &str = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 152);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 301);
    }
}
