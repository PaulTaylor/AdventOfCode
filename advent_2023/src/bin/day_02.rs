use humantime::format_duration;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{self, delimited, separated_pair},
    IResult,
};
use regex::Regex;
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

// Lowest level is a colour name paired with a number
// Returns in (col, n) form
fn colour_pair(input: &str) -> IResult<&str, (&str, usize)> {
    map_res(
        separated_pair(
            digit1,
            multispace1,
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
        |res: (&str, &str)| res.0.parse::<usize>().map(|v| (res.1, v)),
    )(input)
}

// Then draws are built of comma separated color pairs
fn draw(input: &str) -> IResult<&str, Draw> {
    map(separated_list0(tag(", "), colour_pair), |pairs| {
        let mut out = Draw { red: 0, green: 0, blue: 0 };

        for pair in pairs {
            match pair {
                ("red", n) => out.red = n,
                ("green", n) => out.green = n,
                ("blue", n) => out.blue = n,
                x => panic!("Unknown colour pair {x:?}"),
            }
        }

        out
    })(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    map(
        sequence::tuple((
            delimited(
                tag("Game "),
                map_res(digit1, str::parse::<usize>),
                tag(": "),
            ),
            separated_list0(tag("; "), draw),
        )),
        |(id, draws)| Game { id, draws },
    )(input)
}

fn parse(lines: &[String]) -> Vec<Game> {
    lines
        .iter()
        .map(|l| game(l).map(|(_, g)| g))
        .map(Result::unwrap)
        .collect()
}

fn part_a(lines: &[String]) -> usize {
    parse(lines)
        .iter()
        .filter_map(|g| {
            if g.draws
                .iter()
                .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
            {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    parse(lines)
        .iter()
        .map(|game| {
            let mut minimal = Draw { red: 0, green: 0, blue: 0 };

            for draw in &game.draws {
                minimal.red = cmp::max(minimal.red, draw.red);
                minimal.green = cmp::max(minimal.green, draw.green);
                minimal.blue = cmp::max(minimal.blue, draw.blue);
            }

            minimal
        })
        .map(|draw| draw.red * draw.green * draw.blue)
        .sum()
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
    println!("Part A result = {}", part_a(lines.as_slice()));
    println!("Part B result = {}", part_b(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_color_pair() -> AResult<()> {
        assert_eq!(colour_pair("1 red")?, ("", ("red", 1)));
        Ok(())
    }

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 8);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 2286);
    }
}
