use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<Vec<char>>> {
    let mut rows: Vec<Vec<char>> = vec![];
    for l in lines {
        let chars: Vec<char> = l.chars().collect();
        rows.push(chars);
    }
    Ok(rows)
}

fn part_a(lines: &[String]) -> AResult<u32> {
    let rows = parse(lines)?;

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for bit in 0..rows[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;

        // Count each value
        for row in &rows {
            match row[bit] {
                '0' => zero_count += 1,
                '1' => one_count += 1,
                _ => panic!("That's not binary!"),
            }
        }

        if zero_count > one_count {
            gamma.push('0');
            epsilon.push('1')
        } else {
            gamma.push('1');
            epsilon.push('0')
        }
    }

    Ok(u32::from_str_radix(&gamma, 2)? * u32::from_str_radix(&epsilon, 2)?)
}

fn part_b(lines: &[String]) -> AResult<u32> {
    let all_rows = parse(lines)?;

    // There's probably a better way to do this...
    let mut o2_rows: Vec<&Vec<_>> = all_rows.iter().collect();

    // Look for the o2 value
    for bit in 0..all_rows[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;

        // Count each value
        for row in &o2_rows {
            match row[bit] {
                '0' => zero_count += 1,
                '1' => one_count += 1,
                _ => panic!("That's not binary!"),
            }
        }

        let mcb = if zero_count <= one_count { '1' } else { '0' };
        o2_rows.retain(|row| row[bit] == mcb);

        if o2_rows.len() < 2 {
            break;
        } // Found the correct row
    }

    let o2_str: String = o2_rows[0].iter().collect();
    let o2 = u32::from_str_radix(&o2_str, 2)?;

    // Now do the co2 rows
    let mut co2_rows: Vec<&Vec<char>> = all_rows.iter().collect();
    for bit in 0..all_rows[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;

        // Count each value
        for row in &co2_rows {
            match row[bit] {
                '0' => zero_count += 1,
                '1' => one_count += 1,
                _ => panic!("That's not binary!"),
            }
        }

        let lcb = if zero_count > one_count { '1' } else { '0' };
        co2_rows.retain(|row| row[bit] == lcb);

        if co2_rows.len() < 2 {
            break;
        } // Found the correct row
    }

    let co2_str: String = co2_rows[0].iter().collect();
    let co2 = u32::from_str_radix(&co2_str, 2)?;

    Ok(o2 * co2)
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

    const TEST_INPUT: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 198);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 230);
        Ok(())
    }
}
