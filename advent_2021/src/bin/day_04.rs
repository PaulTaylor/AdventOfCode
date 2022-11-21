use anyhow::format_err;
use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

struct Card {
    elem: [[i8; 5]; 5],
}

impl From<&[String]> for Card {
    fn from(lines: &[String]) -> Self {
        let mut elem = [[0; 5]; 5];
        for row in 0..5 {
            let nums: Vec<i8> = lines[row]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            elem[row].copy_from_slice(&nums);
        }
        Card { elem }
    }
}

impl Card {
    fn mark(&mut self, num: i8) -> bool {
        let mut changed = false;
        for row in 0..self.elem.len() {
            for col in 0..self.elem[0].len() {
                if self.elem[row][col] == num {
                    self.elem[row][col] = -1;
                    changed = true;
                }
            }
        }
        changed && self.won()
    }

    fn won(&self) -> bool {
        for row in 0..self.elem.len() {
            if self.elem[row].iter().all(|x: &i8| x < &0i8) {
                return true
            }
        }
        for col in 0..self.elem[0].len() {
            if self.elem.iter().map(|x| x[col]).all(|x| x < 0) {
                return true
            }
        }
        false  // fallback
    }

    fn unmarked_sum(&self) -> u32 {
        let mut acc = 0;
        for row in 0..self.elem.len() {
            for col in 0..self.elem[0].len() {
                let n = self.elem[row][col];
                if n > 0 {
                    acc += n as u32;
                }
            }
        }
        acc
    }
}

fn parse(lines: &[String]) -> AResult<(Vec<i8>, Vec<Card>)> {
    let draw = lines[0]
        .split(',')
        .map(|e| e.parse().expect("a positive integer"))
        .collect();

    let mut cards = vec![];
    for chunk in lines[2..].chunks(6) {
        cards.push(Card::from(&chunk[..5]));
    }
    
    Ok((draw, cards))
}

fn part_a(lines: &[String]) -> AResult<u32> {
    let (draw, mut cards) = parse(lines)?;
    for num in draw {
        for card in cards.iter_mut() {
            if card.mark(num) {
                return Ok(card.unmarked_sum() * (num as u32));
            }
        }
    }
    Err(format_err!("No winning card found :("))
}

fn part_b(lines: &[String]) -> AResult<u32> {
    let (draw, mut cards) = parse(lines)?;
    for num in draw {
        cards.iter_mut().for_each(|c| { c.mark(num); });
        if cards.len() == 1 && cards[0].won() {
            return Ok(cards[0].unmarked_sum() * (num as u32));
        } else {
            cards.retain(|c| !c.won())
        }
    }
    Err(format_err!("No winning card found :("))
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

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn test_parse() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let (draw, cards) = parse(lines.as_slice())?;
        assert_eq!(draw.len(), 27);
        assert_eq!(cards.len(), 3);
        assert_eq!(cards[0].elem[0][0], 22);
        assert_eq!(cards[2].elem[4][4], 7);
        Ok(())
    }

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 4512);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 1924);
        Ok(())
    }
}
