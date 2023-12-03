use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> AResult<u32> {
    // Check all the numbers for symbols around them (this'll avoid double counting
    // a single number if it happens to have 2 symbols near it)

    // Collect all the numbers in the grid (assumes numbers are only written horizontally)
    let pattern = Regex::new("[0-9]+")?;
    let mut numbers = vec![];
    for (row_id, line) in lines.iter().enumerate() {
        let matches = pattern.captures_iter(line);
        for m in matches {
            numbers.push((row_id, m));
        }
    }

    // Check each number to find out if there's a symbol in the surrounding area
    let symbol_pattern = Regex::new(r"[^0-9\.]")?;
    let mut total = 0u32;
    for (row_id, cap) in numbers {
        let range = cap.get(0).unwrap().range();
        let value = cap.get(0).unwrap().as_str().parse::<u32>()?;

        // Check previous row
        if row_id > 0 {
            let prev = &lines[row_id - 1].as_str()
                [range.start.saturating_sub(1)..(range.end + 1).clamp(0, lines[0].len())];
            if symbol_pattern.is_match(prev) {
                // Found match on header row
                total += value;
                continue;
            }
        }

        // Check the previous character on the same row
        if symbol_pattern
            .is_match(&lines[row_id].as_str()[range.start.saturating_sub(1)..range.start])
        {
            // Found match left
            total += value;
            continue;
        }

        // Check the following character on the same row
        if symbol_pattern
            .is_match(&lines[row_id].as_str()[range.end..(range.end + 1).clamp(0, lines[0].len())])
        {
            // Found match right
            total += value;
            continue;
        }

        // Check the row below
        if row_id < lines.len() {
            let next = &lines[row_id + 1].as_str()
                [range.start.saturating_sub(1)..(range.end + 1).clamp(0, lines[0].len())];
            if symbol_pattern.is_match(next) {
                // Found match on following row
                total += value;
            }
        }
    }

    Ok(total)
}

// Takes the location of a digit within a &str and returns the entire number
// that the digit is part of (or None if this location is not a digit)
fn get_number_at(line: &str, loc: usize) -> Option<u32> {
    let value = line.chars().nth(loc).unwrap();

    if !value.is_ascii_digit() {
        return None;
    }

    // Get the digits on the left of loc by searching the line in reverse
    // and then flipping the result back to the right way around.
    let mut number_string: String = line[..loc]
        .chars()
        .rev()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    // Append the initial target char
    number_string.push(value);

    // Finally, append what's on the right of the start position
    number_string.extend(line[loc + 1..].chars().take_while(char::is_ascii_digit));

    number_string.parse().ok()
}

// Check the given row for any numbers that appear on it around the given "loc"
// Adds any detected numbers to the numbers Vec
fn check_row(line: &str, loc: usize, numbers: &mut Vec<u32>) {
    // Check the top middle first - if it's a number it must be the only one
    if let Some(n) = get_number_at(line, loc) {
        numbers.push(n);
    } else {
        // Check left and right separately if the middle is not a digit
        if loc > 0 {
            if let Some(n) = get_number_at(line, loc - 1) {
                numbers.push(n);
            }
        }

        if let Some(n) = get_number_at(line, loc + 1) {
            numbers.push(n);
        }
    }
}

fn part_b(lines: &[String]) -> AResult<u32> {
    // Do this it the opposite way around - find the *'s and then look for surrounding numbers
    // Rather than looking for the numbers and then checking the surrounds

    let pattern = Regex::new(r"\*")?;
    let mut gears = vec![];
    for (row_id, line) in lines.iter().enumerate() {
        let matches = pattern.captures_iter(line);
        for m in matches {
            gears.push((row_id, m));
        }
    }

    let mut total = 0;
    for (row_id, gear) in gears {
        let loc = gear.get(0).unwrap().start();

        let mut numbers = vec![];

        // Check above (bounds check is not necessary for the given input data)
        check_row(&lines[row_id - 1], loc, &mut numbers);

        // Check Left
        check_row(&lines[row_id], loc, &mut numbers);

        // Check Below (bounds check is not necessary for the given input data)
        check_row(&lines[row_id + 1], loc, &mut numbers);

        if numbers.len() == 2 {
            total += numbers[0] * numbers[1];
        }
    }

    Ok(total)
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
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..+114.
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 4361 + 114);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 467_835);
        Ok(())
    }
}
