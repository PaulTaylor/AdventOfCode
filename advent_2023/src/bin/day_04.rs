use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug)]
struct Card {
    id: usize,
    winners: BTreeSet<usize>,
    mine: BTreeSet<usize>,
}

impl Card {
    fn n_winners(&self) -> usize {
        self.mine.intersection(&self.winners).count()
    }

    fn points(&self) -> usize {
        let win_count: u32 = self.n_winners().try_into().unwrap();
        if win_count > 0 {
            2_usize.pow(win_count - 1)
        } else {
            0
        }
    }
}

fn parse(lines: &[String]) -> AResult<Vec<Card>> {
    let pattern: Regex = Regex::new(r"Card\s+(\d+):\s+((?:\d+\s*)+)\s+\|\s+((?:\d+\s*)+)$")?;
    let mut cards = vec![];

    for l in lines {
        let caps = pattern.captures(l).unwrap();
        let (_, bits): (&str, [&str; 3]) = caps.extract();

        let id: usize = bits[0].parse().expect("An integer card id");

        let winners = bits[1]
            .split_whitespace()
            .map(|s| s.parse().expect("couldn't parse int in winner list"))
            .collect();

        let mine = bits[2]
            .split_whitespace()
            .map(|s| s.parse().expect("couldn't parse int in my list"))
            .collect();

        cards.push(Card { id, winners, mine });
    }

    Ok(cards)
}

fn part_a(lines: &[String]) -> AResult<usize> {
    let cards = parse(lines)?;
    Ok(cards.iter().map(Card::points).sum())
}

fn part_b(lines: &[String]) -> AResult<usize> {
    let cards = parse(lines)?;
    let mut copies: HashMap<usize, usize> = cards.iter().map(|c| (c.id, 1)).collect();

    for card in cards {
        let n_winners = card.n_winners();
        let count = *copies.get(&card.id).unwrap();

        for offset in 1..=n_winners {
            copies.entry(card.id + offset).and_modify(|v| *v += count);
        }
    }

    Ok(copies.values().sum())
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

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 13);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 30);
        Ok(())
    }
}
