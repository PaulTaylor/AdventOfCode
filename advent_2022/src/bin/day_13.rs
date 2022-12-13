use humantime::format_duration;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::{fold_many1, separated_list0},
    sequence::delimited,
    IResult,
};
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    str::FromStr,
    time::Instant,
};
use Element::*;

type AResult<T> = anyhow::Result<T>;

#[derive(Debug, PartialEq, Eq)]
enum Element {
    List(Vec<Element>),
    Num(usize),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, &other) {
            (Num(l), Num(r)) => l.cmp(r),
            (List(l), List(r)) => {
                // Calculate order of the common child items
                let common = zip(l, r)
                    .map(|(l, r)| l.cmp(r))
                    .find(|&v| v != Ordering::Equal);

                if let Some(x) = common {
                    x // One of the pairs of children are ordered - return that order
                } else {
                    // All of the children are in order - check lengths
                    l.len().cmp(&r.len())
                }
            }
            (Element::List(_), Num(r)) => self.cmp(&List(vec![Num(*r)])),
            (Num(l), Element::List(_)) => List(vec![Num(*l)]).cmp(other),
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Using nom for parsing - define the functions here

fn number(input: &str) -> IResult<&str, Element> {
    map(fold_many1(digit1, String::new, |acc, s| acc + s), |x| {
        Element::Num(usize::from_str(x.as_str()).unwrap())
    })(input)
}

fn list(input: &str) -> IResult<&str, Element> {
    map(
        delimited(tag("["), separated_list0(tag(","), element), tag("]")),
        Element::List,
    )(input)
}

fn element(input: &str) -> IResult<&str, Element> {
    alt((number, list))(input)
}

// End of nom parsing functions

fn parse(lines: &[String]) -> AResult<Vec<(Element, Element)>> {
    Ok(lines
        .chunks(3)
        .map(|chunk| (element(&chunk[0]).unwrap().1, element(&chunk[1]).unwrap().1))
        .collect())
}

fn part_a(lines: &[String]) -> AResult<usize> {
    let pairs = parse(lines)?;
    let mut acc = 0;

    for (idx, (e1, e2)) in pairs.into_iter().enumerate() {
        if e1 < e2 {
            acc += idx + 1;
        }
    }

    Ok(acc)
}

fn part_b(lines: &[String]) -> AResult<usize> {
    let pairs = parse(lines)?;

    // Create the specified divider packets
    let two = &List(vec![List(vec![Num(2)])]);
    let six = &List(vec![List(vec![Num(6)])]);

    // Create the flattened list of packets (using a sorted set)
    let mut flat: BTreeSet<_> = BTreeSet::from_iter(vec![two, six]);
    pairs.iter().for_each(|(e1, e2)| {
        flat.insert(e1);
        flat.insert(e2);
    });

    // Locate the dividers
    let i2 = flat.iter().enumerate().find(|(_, v)| *v == &two).unwrap().0 + 1;
    let i6 = flat.iter().enumerate().find(|(_, v)| *v == &six).unwrap().0 + 1;

    Ok(i2 * i6)
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

    const TEST_INPUT: &str = "[1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse() -> AResult<()> {
        assert_eq!(Num(1), element("1").unwrap().1);
        assert_eq!(List(vec![]), element("[]").unwrap().1);
        assert_eq!(List(vec![Num(1)]), element("[1]").unwrap().1);
        assert_eq!(
            List(vec![Num(1), Num(2), Num(3)]),
            element("[1,2,3]").unwrap().1
        );
        assert_eq!(
            List(vec![
                List(vec![Num(1), Num(2), Num(3)]),
                List(vec![Num(1), Num(2), Num(3)])
            ]),
            element("[[1,2,3],[1,2,3]]").unwrap().1
        );
        assert_eq!(
            List(vec![List(vec![]), List(vec![])]),
            element("[[],[]]").unwrap().1
        );
        Ok(())
    }

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 13);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 140);
        Ok(())
    }
}
