use humantime::format_duration;
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

enum Instruction {
    Noop,
    Addx(i32),
}

use Instruction::{Addx, Noop};

fn parse(lines: &[String]) -> Vec<Instruction> {
    lines
        .iter()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .map(|x| match x.as_slice() {
            ["noop"] => Instruction::Noop,
            ["addx", ns] => Instruction::Addx(ns.parse().unwrap()),
            _ => panic!("unknown instruction {x:?}"),
        })
        .collect()
}

fn part_a(lines: &[String]) -> i32 {
    let instructions = parse(lines);
    let mut it = instructions.iter();
    let mut acc = 0;
    let mut clk = 0;
    let mut q: VecDeque<i32> = VecDeque::new();
    let mut x = 1;

    loop {
        if q.is_empty() {
            // Enque the actions for the next instruction
            match it.next() {
                None => {
                    break;
                }
                Some(Noop) => {
                    q.push_back(0);
                }
                Some(Addx(v)) => {
                    q.push_back(0);
                    q.push_back(*v);
                }
            }
        }

        // tick
        clk += 1;

        // Capture Signal strengh
        if (clk - 20) % 40 == 0 && clk < 221 {
            acc += clk * x;
        }

        // update X
        x += q.pop_front().unwrap();
    }

    acc
}

fn part_b(lines: &[String]) -> String {
    let instructions = parse(lines);
    let mut it = instructions.iter();
    let mut clk: usize = 0;
    let mut q: VecDeque<isize> = VecDeque::new();
    let mut x: isize = 1;
    let mut crt = [[' '; 40]; 6];

    loop {
        if q.is_empty() {
            // Enque the actions for the next instruction
            match it.next() {
                None => {
                    break;
                }
                Some(Noop) => {
                    q.push_back(0);
                }
                Some(Addx(v)) => {
                    q.push_back(0);
                    q.push_back((*v).try_into().unwrap());
                }
            }
        }

        // Update CRT
        let crt_row = clk / 40;
        let crt_col = clk.rem_euclid(40);
        if (x - 1..=x + 1).contains(&(crt_col.try_into().unwrap())) {
            // Sprite is overlapping the crt position - mark the crt
            crt[crt_row][crt_col] = '#';
        }

        clk += 1;
        x += q.pop_front().unwrap();
    }

    let mut lines: Vec<String> = crt.iter().map(|r| r.iter().collect()).collect();
    lines.insert(0, "\n=======================================".to_string());
    lines.push("=======================================".to_string());
    lines.join("\n")
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

    const TEST_INPUT: &str = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 13140);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let out_lines = part_b(lines.as_slice());
        assert!(out_lines.contains(
            "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     "
        ));
    }
}
