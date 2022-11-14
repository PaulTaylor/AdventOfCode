use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse(lines: &[String]) -> _ {
    return lines.iter().map(|l| (*l).parse::<usize>().unwrap());
}

fn part_a(lines: &[String]) -> usize {
    let mut parsed = parse(lines);
    let mut acc: usize = 0;
    let mut prev = parsed.next().expect(r#"more than 1 number is required"#);

    for next in parsed {
        acc += (next > prev) as usize;
        prev = next;
    }

    acc
}

fn part_b(lines: &[String]) -> usize {
    panic!("Not implemented yet.")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TestInput: &str = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

    #[test]
    fn test_a() {
        let lines: Vec<String> = TestInput.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(&lines), 7);
    }

    #[test]
    fn test_b() {
        let lines: Vec<String> = ["a", "b", "c"].into_iter().map(String::from).collect();
        assert_eq!(part_b(&lines), 5);
    }
}

fn main() -> anyhow::Result<()> {
    // Figure out which exercise we are so we can load the correct input file
    let pattern = Regex::new("[0-9]+$")?;
    let name = &std::env::args().next().unwrap();
    let ex = pattern
        .find(name)
        .expect("binary name should contain a number")
        .as_str();
    println!("Running code for Day {}.", ex);

    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    println!("Part A result = {}", part_a(&lines));
    println!("Part B result = {}", part_b(&lines));

    Ok(())
}
