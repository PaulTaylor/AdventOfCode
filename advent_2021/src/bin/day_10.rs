use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> AResult<u32> {
    let mut acc = 0;
    for line in lines {
        let mut stack: Vec<char> = Vec::with_capacity(line.len() / 2);
        for char in line.chars() {
            match char {
                '('|'['|'{'|'<' => { stack.push(char); },
                ')'|']'|'}'|'>' => {
                    match stack.pop().unwrap() {
                        '(' if char == ')' => (),
                        '[' if char == ']' => (),
                        '{' if char == '}' => (),
                        '<' if char == '>' => (),
                        // bracket mismatch below
                        _ if char == ')' => { acc += 3; break },
                        _ if char == ']' => { acc += 57; break },
                        _ if char == '}' => { acc += 1197; break },
                        _ if char == '>' => { acc += 25137; break },
                        // Catch-all
                        _ => panic!("wtf")
                    }
                },
                _ => panic!("Unknown character in input")
            }
        }
    }

    Ok(acc)
}

fn part_b(lines: &[String]) -> AResult<u64> {
    let mut acc = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::with_capacity(line.len() / 2);
        let mut corrupt = false;
        let mut line_acc = 0u64;

        for char in line.chars() {
            match char {
                '('|'['|'{'|'<' => { stack.push(char); },
                ')'|']'|'}'|'>' => {
                    match stack.pop().unwrap() {
                        '(' if char == ')' => (),
                        '[' if char == ']' => (),
                        '{' if char == '}' => (),
                        '<' if char == '>' => (),
                        // bracket mismatch below
                        _ if char == ')' => { corrupt = true; break },
                        _ if char == ']' => { corrupt = true; break },
                        _ if char == '}' => { corrupt = true; break },
                        _ if char == '>' => { corrupt = true; break },
                        // Catch-all
                        _ => panic!("wtf")
                    }
                },
                _ => panic!("Unknown character in input")
            }
        }

        if !corrupt {
            // Need to complete the line
            while let Some(char) = stack.pop() {
                line_acc *= 5;
                line_acc += match char {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("wtf")
                };
             }
             acc.push(line_acc);
        }
    }

    acc.sort();
    Ok(acc[acc.len() / 2])
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
    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}ms", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 26397);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 288957);
        Ok(())
    }
}
