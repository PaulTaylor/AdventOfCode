use humantime::format_duration;
use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::map,
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

lazy_static! {
    static ref RULE_PATTERN: Regex = Regex::new(r"(\w+)([<>])(\d+)").unwrap();
}

#[derive(Debug, Clone)]
struct Workflow(String, Vec<Instruction>);

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    LT(String, usize, String),
    GT(String, usize, String),
    JMP(String),
}

#[derive(Debug)]
struct Item {
    fields: HashMap<String, usize>,
}

//
// Nom Parsing Functions
//

fn parse_rule(input: &str) -> IResult<&str, Instruction> {
    map_res(
        separated_pair(
            tuple((alpha1::<&str, _>, alt((tag("<"), tag(">"))), digit1)),
            tag(":"),
            alpha1,
        ),
        |((field, op, val_str), target)| {
            val_str.parse().map(|val| match op {
                "<" => Instruction::LT(field.to_string(), val, target.to_string()),
                ">" => Instruction::GT(field.to_string(), val, target.to_string()),
                _ => unreachable!(),
            })
        },
    )(input)
}

fn parse_jmp(input: &str) -> IResult<&str, Instruction> {
    map(alpha1, |s: &str| Instruction::JMP(s.to_string()))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    map(
        tuple((
            alpha1,
            delimited(
                tag("{"),
                separated_list1(tag(","), alt((parse_rule, parse_jmp))),
                tag("}"),
            ),
        )),
        |(name, instrs)| Workflow(name.to_string(), instrs),
    )(input)
}

fn parse_workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
    separated_list1(newline, parse_workflow)(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    map(
        delimited(
            tag("{"),
            separated_list1(
                tag(","),
                separated_pair(
                    alpha1,
                    tag("="),
                    map_res(digit1, |s: &str| s.parse::<usize>()),
                ),
            ),
            tag("}"),
        ),
        |input| {
            let mut fields = HashMap::new();
            for (n, v) in input {
                fields.insert(n.to_string(), v);
            }
            Item { fields }
        },
    )(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(newline, parse_item)(input)
}

//
// End of nom parsing functions
//

fn parse(lines: &[String]) -> (HashMap<String, Workflow>, Vec<Item>) {
    let mut s = lines.split(String::is_empty);
    let wf_lines = s.next().unwrap().join("\n");
    let (_, wf_list) = parse_workflows(&wf_lines).unwrap();
    let mut workflows = HashMap::new();
    for wf in wf_list {
        workflows.insert(wf.0.clone(), wf);
    }

    let i_lines = s.next().unwrap().join("\n");
    let (_, items) = parse_items(&i_lines).unwrap();

    (workflows, items)
}

fn execute(workflows: &HashMap<String, Workflow>, wf_name: &str, item: &Item) -> bool {
    if wf_name == "A" {
        return true;
    } else if wf_name == "R" {
        return false;
    }

    if let Some(Workflow(_, instructions)) = workflows.get(wf_name) {
        for instr in instructions {
            match instr {
                Instruction::LT(f, v, t) => {
                    if let Some(curr) = item.fields.get(f) {
                        if curr < v {
                            return execute(workflows, t, item);
                        }
                    }
                }
                Instruction::GT(f, v, t) => {
                    if let Some(curr) = item.fields.get(f) {
                        if curr > v {
                            return execute(workflows, t, item);
                        }
                    }
                }
                Instruction::JMP(t) => return execute(workflows, t, item),
            }
        }
    }
    false
}

fn part_a(lines: &[String]) -> usize {
    let (workflows, items) = parse(lines);
    let mut acc = 0;
    for item in items {
        if execute(&workflows, "in", &item) {
            acc += item.fields.values().sum::<usize>();
        }
    }

    acc
}

fn combinations(ranges: &HashMap<String, (usize, usize)>) -> usize {
    let (x_min, x_max) = ranges["x"];
    let (m_min, m_max) = ranges["m"];
    let (a_min, a_max) = ranges["a"];
    let (s_min, s_max) = ranges["s"];
    (1 + x_max - x_min) * (1 + m_max - m_min) * (1 + a_max - a_min) * (1 + s_max - s_min)
}

fn count_combinations(
    workflows: &HashMap<String, Workflow>,
    wf_name: &str,
    ranges: HashMap<String, (usize, usize)>,
) -> usize {
    if wf_name == "R" {
        return 0;
    }

    if wf_name == "A" {
        return combinations(&ranges);
    }

    let mut n_comb = 0;
    let wf = workflows[wf_name].clone();
    let mut remaining = ranges;

    for instr in wf.1 {
        n_comb += match instr {
            Instruction::LT(f, v, t) => {
                let c_range = remaining[&f];

                // The New Ranges (n_ranges) are those which pass the condition
                let mut n_ranges = remaining.clone();
                n_ranges.insert(f.clone(), (c_range.0, v - 1));

                // Update the remaining ranges with what's left in preparation for the next loop
                remaining.insert(f, (v, c_range.1));

                // Finally, count the combinations for the selected branch over n_ranges
                count_combinations(workflows, &t, n_ranges)
            }
            Instruction::GT(f, v, t) => {
                let c_range = remaining[&f];

                // New Ranges
                let mut n_ranges = remaining.clone();
                n_ranges.insert(f.clone(), (v + 1, c_range.1));

                // Update the remaining ranges
                remaining.insert(f, (c_range.0, v));
                count_combinations(workflows, &t, n_ranges)
            }
            Instruction::JMP(t) => count_combinations(workflows, &t, remaining.clone()),
        }
    }

    n_comb
}

fn part_b(lines: &[String]) -> usize {
    // Lets walk the workflows with ranges...
    let (workflows, _) = parse(lines);
    let mut ranges = HashMap::new();
    for c in "xmas".chars() {
        ranges.insert(format!("{c}"), (1, 4000));
    }
    count_combinations(&workflows, "in", ranges)
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

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}
    
    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 19114);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 167_409_079_868_000);
    }
}
