use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Write},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Rule<'a> = (&'a str, &'a str, Op, &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::upper_case_acronyms)]
enum Op {
    AND,
    OR,
    XOR,
}

impl Op {
    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Op::AND => a & b,
            Op::OR => a | b,
            Op::XOR => a != b,
        }
    }
}

fn parse(lines: &[String]) -> (HashMap<String, bool>, VecDeque<Rule>) {
    let mut splitter = lines.split(String::is_empty);
    let initial = splitter
        .next()
        .unwrap()
        .iter()
        .map(|l| {
            let (a, b) = l.split(": ").collect_tuple().unwrap();
            (a.to_string(), b == "1")
        })
        .collect();

    let pattern = Regex::new(r"^(\w+) (AND|OR|XOR) (\w+) -> (\w+)$").unwrap();
    let mut rules = VecDeque::new();
    for line in splitter.next().unwrap() {
        let (_, [a, op, b, into]) = pattern.captures(line).unwrap().extract();
        let op = match op {
            "AND" => Op::AND,
            "OR" => Op::OR,
            "XOR" => Op::XOR,
            _ => unreachable!(),
        };

        let rule = (a, b, op, into);
        rules.push_back(rule);
    }

    (initial, rules)
}

fn find_msb(prefix: char, rules: &VecDeque<Rule>) -> usize {
    rules
        .iter()
        .filter_map(|(a, b, _, c)| {
            let q = [a, b, c].into_iter().find_map(|k| k.strip_prefix(prefix));
            if let Some(v) = q {
                v.parse().ok()
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn run(mut values: HashMap<String, bool>, rules: &VecDeque<Rule>) -> usize {
    let max_z = find_msb('z', rules);

    let mut rules = rules.clone();
    while !rules.is_empty() {
        let mut next = VecDeque::new();
        for the_rule in rules {
            let (a, b, op, into) = the_rule;

            if let (Some(&a_val), Some(&b_val)) = (values.get(a), values.get(b)) {
                let result = op.apply(a_val, b_val);
                values.insert(into.to_string(), result);
            } else {
                next.push_back(the_rule);
            }
        }
        rules = next;
    }

    let mut acc = 0usize;
    for z in (0..=max_z).rev() {
        let &v = values.get(format!("z{z:02}").as_str()).unwrap_or(&false);
        acc = (acc * 2) + usize::from(v);
    }
    acc
}

fn part_a(lines: &[String]) -> usize {
    let (values, rules) = parse(lines);
    run(values, &rules)
}

fn write_dot(file_name: &str, rules: &VecDeque<Rule>) -> AResult<()> {
    let mut f = File::create(format!("./output/{file_name}.dot"))?;
    f.write_fmt(format_args!("digraph G {{\nrankdir=LR;\nranksep=1.0;\n"))?;
    for &(a, b, op, t) in rules {
        let fc = match op {
            Op::AND => "red",
            Op::OR => "green",
            Op::XOR => "blue",
        };

        f.write_fmt(format_args!(
            "{a}_{op:?}_{b} [label=\"{op:?}\", shape=box, color={fc}];\n"
        ))?;

        f.write_fmt(format_args!("{a} -> {a}_{op:?}_{b};\n"))?;
        f.write_fmt(format_args!("{b} -> {a}_{op:?}_{b};\n"))?;
        f.write_fmt(format_args!("{a}_{op:?}_{b} -> {t};\n"))?;
    }
    f.write_fmt(format_args!("}}\n"))?;
    Ok(())
}

fn detect_issues(rules: &VecDeque<Rule>, max_x: usize) -> Vec<usize> {
    // Do 1+1 in all positions to check which carry bits are out of position.
    // This'll help point us towards the location in the call graph which need
    // to be examined for issues

    let mut issues = vec![];
    for bit in 0..max_x {
        let mut values = HashMap::new();

        for xy_bit in 0..=max_x {
            let xn = format!("x{xy_bit:02}");
            let yn = format!("y{xy_bit:02}");

            values.insert(xn, bit == xy_bit);
            values.insert(yn, bit == xy_bit);
        }

        let actual = 2 << bit;
        let output = run(values, rules);
        if output != actual {
            issues.push(bit);
        }
    }
    issues
}

fn part_b(lines: &[String]) -> String {
    // Solve this by plotting the call graph and manual inspection
    // broken.dot/fixed.dot will be produced in the output folder
    let (_, rules) = parse(lines);
    let max_x = find_msb('x', &rules);

    write_dot("broken", &rules).expect("Couldn't write the broken.dot file");

    let issues = detect_issues(&rules, max_x);
    println!("Detected issues with bits: {issues:?}");

    // Now manually inspect the graph to determine the appropriate swaps
    // entering them into the array below

    let replacements: HashMap<&str, &str> = [
        ("z10", "mkk"),
        ("z14", "qbw"),
        ("wjb", "cvp"),
        ("z34", "wcb"),
    ]
    .into_iter()
    .flat_map(|(a, b)| [(a, b), (b, a)])
    .collect();

    println!("Applying {} replacements", replacements.len() / 2);
    let new_rules: VecDeque<_> = rules
        .iter()
        .map(|&r @ (a, b, o, t)| {
            if let Some(new) = replacements.get(t) {
                (a, b, o, *new)
            } else {
                r
            }
        })
        .collect();

    write_dot("fixed", &rules).expect("Couldn't write the fixed.dot file");

    let issues = detect_issues(&new_rules, max_x);
    assert!(issues.is_empty(), "Remaining issues at bits: {issues:?}");

    replacements.values().sorted().join(",")
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

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 2024);
    }

    const TEST_INPUT: &str = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
}
