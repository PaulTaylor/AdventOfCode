use anyhow::anyhow;
use humantime::format_duration;
use indicatif::ParallelProgressIterator;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, newline, none_of, space0},
    combinator::{map, map_res},
    multi::{fold_many1, many_m_n, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use rayon::prelude::*;
use regex::Regex;
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug)]
struct IntervalMap {
    _name: String,
    ranges: Vec<(usize, usize, usize)>, // dest_start, source_start, length
}

impl IntervalMap {
    fn map(&self, source: usize) -> usize {
        for (dest_start, source_start, length) in &self.ranges {
            let source_end = source_start + length;
            if (*source_start..source_end).contains(&source) {
                let offset = source - source_start;
                let res = dest_start + offset;
                return res;
            }
        }

        source
    }
}

fn interval(input: &str) -> IResult<&str, (usize, usize, usize)> {
    map(
        many_m_n(
            3,
            3,
            terminated(map_res(digit1, |v: &str| v.parse::<usize>()), space0),
        ),
        |v| (v[0], v[1], v[2]),
    )(input)
}

fn name(input: &str) -> IResult<&str, String> {
    fold_many1(none_of(" "), String::new, |mut out, v| {
        out.push(v);
        out
    })(input)
}

fn map_block(input: &str) -> IResult<&str, IntervalMap> {
    map(
        tuple((
            terminated(name, tuple((tag(" map:"), multispace0))),
            separated_list1(newline, interval),
        )),
        |(bits, ranges)| IntervalMap { _name: bits, ranges },
    )(input)
}

fn block_list(input: &str) -> IResult<&str, Vec<IntervalMap>> {
    separated_list1(pair(newline, newline), map_block)(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("seeds: "),
        separated_list1(multispace1, map_res(digit1, |v: &str| v.parse::<usize>())),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<usize>, Vec<IntervalMap>)> {
    tuple((terminated(seeds, multispace0), block_list))(input)
}

#[allow(clippy::redundant_closure_for_method_calls)]
fn part_a(lines: &str) -> AResult<usize> {
    let (_, (seeds, intervals)) = parse(lines).map_err(|e| e.to_owned())?;

    let mut min = usize::MAX;
    for seed in seeds {
        // Yes, I could write this in a loop.
        let soil = intervals[0].map(seed);
        let fertilizer = intervals[1].map(soil);
        let water = intervals[2].map(fertilizer);
        let light = intervals[3].map(water);
        let temp = intervals[4].map(light);
        let humid = intervals[5].map(temp);
        let loc = intervals[6].map(humid);
        min = cmp::min(min, loc);
    }

    Ok(min)
}

//
// This is a simplistic brute-force solution utilising the excellent rayon
// library for parallel processing.  Run time is approximately 3.5 minutes
// on my desktop.
//
// I may revisit this in the future and implement the significantly more
// efficient range manipulation approach.
//
#[allow(clippy::redundant_closure_for_method_calls)]
fn part_b(lines: &str) -> AResult<usize> {
    let (_, (raw_seed_ranges, intervals)) = parse(lines).map_err(|e| e.to_owned())?;

    let seed_ranges = raw_seed_ranges.chunks(2).map(|p| p[0]..p[0] + p[1]);

    let total = seed_ranges.clone().flatten().count();
    println!("Seed count = {total}");

    let min = seed_ranges
        .flatten()
        .par_bridge()
        .progress_count(total as u64)
        .map(|seed| {
            // Yes, I could write this in a loop.
            let soil = intervals[0].map(seed);
            let fertilizer = intervals[1].map(soil);
            let water = intervals[2].map(fertilizer);
            let light = intervals[3].map(water);
            let temp = intervals[4].map(light);
            let humid = intervals[5].map(temp);
            intervals[6].map(humid)
        })
        .min();

    min.ok_or(anyhow!("Error"))
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
    let all_lines = lines.join("\n");

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(&all_lines)?);
    println!("Part B result = {}", part_b(&all_lines)?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_a() -> AResult<()> {
        assert_eq!(part_a(TEST_INPUT)?, 35);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        assert_eq!(part_b(TEST_INPUT)?, 46);
        Ok(())
    }
}
