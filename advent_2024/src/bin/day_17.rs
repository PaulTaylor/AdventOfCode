use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

fn parse(lines: &[String]) -> ([i64; 3], Vec<i64>) {
    let mut registers = [0i64; 3];
    let mut instr = vec![];

    let r_pattern = Regex::new(r"^Register (\S+): (-?\d+)$").unwrap();

    for line in lines {
        if let Some(caps) = r_pattern.captures(line) {
            let (_, bits): (_, [&str; 2]) = caps.extract();
            match bits[0].chars().next() {
                Some('A') => registers[A] = bits[1].parse().unwrap(),
                Some('B') => registers[B] = bits[1].parse().unwrap(),
                Some('C') => registers[C] = bits[1].parse().unwrap(),
                _ => unreachable!(),
            }
        }

        if line.starts_with("Program: ") {
            instr = line
                .strip_prefix("Program: ")
                .unwrap()
                .split(',')
                .flat_map(str::parse)
                .collect();
        }
    }

    (registers, instr)
}

fn combo_value(cv: i64, registers: &[i64; 3]) -> i64 {
    match cv {
        0..=3 => cv,
        4..=6 => {
            let idx: usize = (cv - 4).try_into().unwrap();
            registers[idx]
        }
        7 => panic!("Reserved"),
        _ => unreachable!(),
    }
}

fn run_code(mut registers: [i64; 3], instr: &[i64]) -> (Vec<i64>, [i64; 3]) {
    let mut out = vec![];
    let mut ip = 0;
    while ip < instr.len() {
        match &instr[ip..ip + 2] {
            [0, c] => {
                // ADV instruction
                let v = combo_value(*c, &registers);
                registers[A] /= 2_i64.pow(v.try_into().unwrap());
            }
            [1, l] => registers[B] ^= l, // BXL
            [2, c] => registers[B] = combo_value(*c, &registers) % 8, // BST
            [3, l] => {
                // JNZ
                if registers[A] != 0 {
                    ip = (*l).try_into().unwrap();
                    continue;
                }
            }
            [4, _] => registers[B] ^= registers[C], // BXC
            [5, c] => {
                let r = combo_value(*c, &registers) % 8;
                out.push(r); // OUT
            }
            [6, c] => {
                // BDV
                let v = combo_value(*c, &registers);
                registers[B] = registers[A] / 2_i64.pow(v.try_into().unwrap());
            }
            [7, c] => {
                let v = combo_value(*c, &registers);
                registers[C] = registers[A] / 2_i64.pow(v.try_into().unwrap());
            }
            _ => unreachable!(),
        }

        ip += 2;
    }

    (out, registers)
}

fn part_a(lines: &[String]) -> (String, [i64; 3]) {
    let (registers, instr) = parse(lines);
    let (outs, reg) = run_code(registers, &instr);
    (outs.iter().map(ToString::to_string).join(","), reg)
}

fn check(a: i64, depth: usize, instr: &[i64]) -> Option<i64> {
    // Check this value of a and recurse if it is a partial solution
    let (res, _) = run_code([a, 0, 0], instr);

    // Found a complete solution
    if res == instr {
        return Some(a);
    }

    // Found a partial solution
    if res == instr[instr.len() - depth..] {
        for postfix in 0..8 {
            let new_a = (a << 3) + postfix;
            if let Some(v) = check(new_a, depth + 1, instr) {
                return Some(v);
            }
        }
    }

    None
}

fn part_b(lines: &[String]) -> i64 {
    let (_, instr) = parse(lines);

    // Manually inspecting the code shows that A is shifting by 3 bits
    // each loop. So we'll run a search process where we generate each
    // output number in reverse order by extending A by 3 bits each
    // recursion

    // Hard code the starting points because the programs always end 3,0
    for a in 24..32 {
        if let Some(v) = check(a, 2, &instr) {
            return v;
        }
    }

    panic!("no result found :(")
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
    println!("Part A result = {}", part_a(lines.as_slice()).0);
    println!("Part B result = {}", part_b(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUTS: [(&str, &str, Option<[i64; 3]>); 4] = [
        ("Register C: 9\nProgram: 2,6", "", Some([0, 1, 9])),
        ("Register A: 10\nProgram: 5,0,5,1,5,4", "0,1,2", None),
        (
            "Register A: 2024\nProgram: 0,1,5,4,3,0",
            "4,2,5,6,7,7,7,7,3,1,0",
            Some([0, -1, -1]),
        ),
        (
            "Register A: 729
         Register B: 0
         Register C: 0

         Program: 0,1,5,4,3,0",
            "4,6,3,5,6,3,5,2,1,0",
            None,
        ),
    ];

    #[test]
    fn test_a() {
        for (input, output, registers) in TEST_INPUTS {
            let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
            let (out, r) = part_a(lines.as_slice());
            if !output.is_empty() {
                assert_eq!(out, output);
            }
            if let Some(r_actual) = registers {
                for (a, b) in r.into_iter().zip(r_actual) {
                    if b >= 0 {
                        assert_eq!(a, b);
                    }
                }
            }
        }
    }

    #[test]
    fn test_b() {
        let lines: Vec<String> = "Program: 0,3,5,4,3,0"
            .lines()
            .map(ToString::to_string)
            .collect();
        let res = part_b(&lines);
        assert_eq!(res, 117_440);
    }
}
