use humantime::format_duration;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
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

fn number_set(l: &str) -> IResult<&str, BTreeSet<usize>> {
    map(
        separated_list1(multispace1, map_res(digit1, |s: &str| s.parse::<usize>())),
        |l| l.into_iter().collect::<BTreeSet<usize>>(),
    )(l)
}

fn card_header(l: &str) -> IResult<&str, usize> {
    delimited(
        tuple((tag("Card"), multispace1)),
        map_res(digit1, |s: &str| s.parse::<usize>()),
        tuple((tag(":"), multispace1)),
    )(l)
}

fn parse_card(l: &str) -> Card {
    let (_, (id, winners, _, mine)) = tuple((
        card_header,
        number_set,
        tuple((multispace0, tag("|"), multispace0)),
        number_set,
    ))(l)
    .unwrap_or_else(|_| panic!("Can't parse {l} as a card"));

    Card { id, winners, mine }
}

fn parse(lines: &[String]) -> Vec<Card> {
    lines.iter().map(|s| parse_card(s)).collect()
}

fn part_a(lines: &[String]) -> usize {
    parse(lines).iter().map(Card::points).sum()
}

fn part_b(lines: &[String]) -> usize {
    let cards = parse(lines);
    let mut copies: HashMap<usize, usize> = cards.iter().map(|c| (c.id, 1)).collect();

    for card in cards {
        let n_winners = card.n_winners();
        let count = *copies.get(&card.id).unwrap();

        for offset in 1..=n_winners {
            copies.entry(card.id + offset).and_modify(|v| *v += count);
        }
    }

    copies.values().sum()
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

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 13);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 30);
    }
}
