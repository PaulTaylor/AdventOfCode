use humantime::format_duration;
use regex::Regex;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn solve((nums, op): (Vec<usize>, String)) -> usize {
    nums.into_iter()
        .reduce(|a, v| -> usize {
            match op.as_str() {
                "+" => a + v,
                "-" => a - v,
                "*" => a * v,
                "/" => a / v,
                _ => panic!(),
            }
        })
        .unwrap()
}

fn part_a(lines: &[String]) -> usize {
    let mut breaks: BTreeSet<_> = (0..lines[0].len()).collect();
    for line in lines {
        for (idx, char) in line.char_indices() {
            if !char.is_whitespace() {
                breaks.remove(&idx);
            }
        }
    }
    breaks.insert(0);
    breaks.insert(lines[0].len());

    let mut problems = vec![];
    let mut prob_numbers: Vec<Vec<usize>> = (0..breaks.len()).map(|_| vec![]).collect();

    for line in lines {
        for (idx, window) in Vec::from_iter(&breaks).windows(2).enumerate() {
            let &start = window.get(0).unwrap();
            let &end = window.get(1).unwrap();
            let str: String = line.chars().take(*end).skip(*start).collect();

            match str.trim() {
                "+" | "-" | "/" | "*" => {
                    problems.push((prob_numbers[idx].clone(), str.trim().to_string()));
                }
                x => {
                    let num: usize = x.trim().parse().unwrap();
                    let l = prob_numbers.get_mut(idx).unwrap();
                    l.push(num)
                }
            }
        }
    }

    problems.into_iter().map(solve).sum()
}

fn part_b(lines: &[String]) -> usize {
    let mut cols: Vec<_> = (0..lines[0].len()).map(|_| vec![]).collect();

    for line in lines {
        for (idx, c) in line.char_indices() {
            cols[idx].push(format!("{c}"));
        }
    }
    cols.reverse();
    cols.push(vec![]);

    let digits = Regex::new("(?<num>\\d+)?\\s*(?<op>[-+/*])?").unwrap();
    let mut buf = vec![];
    let mut op = String::new();
    let mut acc = 0;

    for col in cols {
        if col.concat().trim().is_empty() {
            acc += solve((buf, op));
            buf = vec![];
            op = String::new();
        } else {
            let str = col.concat();
            let caps = digits.captures(str.trim()).unwrap();

            if let Some(num_match) = caps.name("num") {
                let num: usize = num_match.as_str().parse().unwrap();
                buf.push(num);
            }

            if let Some(op_match) = caps.name("op") {
                op = op_match.as_str().to_string();
            }
        }
    }

    acc
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

    const TEST_INPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().skip(1).map(|l| l.to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 4_277_556);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().skip(1).map(|l| l.to_string()).collect();
        assert_eq!(part_b(&lines), 3_263_827);
    }
}
