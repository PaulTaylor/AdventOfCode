use humantime::format_duration;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::Instant;

type AResult<T> = anyhow::Result<T>;

lazy_static! {
    static ref DOUBLE_PATTERN: Regex = Regex::new("11|22|33|44|55|66|77|88|99|00").unwrap();
}

fn is_valid_a(num_str: &str) -> Option<usize> {
    // Need to check for double digits
    if DOUBLE_PATTERN.is_match(num_str) {
        let the_num = num_str.parse().unwrap();
        return Some(the_num);
    }

    None
}

fn is_valid_b(num_str: &str) -> Option<usize> {
    // Get all possible doubles
    let doubles: Vec<_> = DOUBLE_PATTERN.find_iter(num_str).collect();

    // Check there is at least 1 double that is not also a triple
    if doubles.iter().any(|d| {
        let mut triple = d.as_str().to_string();
        triple.push(triple.chars().next().unwrap());
        !num_str.contains(&triple)
    }) {
        let the_num = num_str.parse().unwrap();
        return Some(the_num);
    }

    None
}

fn part_a() -> usize {
    let start = 165_432;
    let end = 707_912;

    let DIGITS = "123456789".chars();
    let mut valid = vec![];

    for c1 in DIGITS.clone() {
        for c2 in DIGITS.clone().filter(|x| x >= &c1) {
            for c3 in DIGITS.clone().filter(|x| x >= &c2) {
                for c4 in DIGITS.clone().filter(|x| x >= &c3) {
                    for c5 in DIGITS.clone().filter(|x| x >= &c4) {
                        for c6 in DIGITS.clone().filter(|x| x >= &c5) {
                            if let Some(the_num) = is_valid_a(&format!("{c1}{c2}{c3}{c4}{c5}{c6}"))
                            {
                                if the_num > end {
                                    return valid.len();
                                }

                                if the_num >= start {
                                    valid.push(the_num);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("Nothing found")
}

fn part_b() -> usize {
    let start = 165_432;
    let end = 707_912;

    let DIGITS = "123456789".chars();
    let mut valid = vec![];

    for c1 in DIGITS.clone() {
        for c2 in DIGITS.clone().filter(|x| x >= &c1) {
            for c3 in DIGITS.clone().filter(|x| x >= &c2) {
                for c4 in DIGITS.clone().filter(|x| x >= &c3) {
                    for c5 in DIGITS.clone().filter(|x| x >= &c4) {
                        for c6 in DIGITS.clone().filter(|x| x >= &c5) {
                            if let Some(the_num) = is_valid_b(&format!("{c1}{c2}{c3}{c4}{c5}{c6}"))
                            {
                                if the_num > end {
                                    return valid.len();
                                }

                                if the_num >= start {
                                    valid.push(the_num);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("Nothing found")
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

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a());
    println!("Part B result = {}", part_b());
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}
