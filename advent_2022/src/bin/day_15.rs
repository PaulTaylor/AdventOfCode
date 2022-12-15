use humantime::format_duration;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    isize,
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

struct Sensor {
    sx: isize,
    sy: isize,
    bx: isize,
    by: isize,
}

impl Sensor {
    fn dist(&self) -> usize {
        self.sx.abs_diff(self.bx) + self.sy.abs_diff(self.by)
    }
}

struct Coverage {
    row: Vec<(isize, isize)>,
    beacon_count: isize,
}

fn parse(lines: &[String]) -> AResult<Vec<Sensor>> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(
            r##"^Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)$"##
        ).unwrap();
    }

    let mut out: Vec<Sensor> = vec![];

    for line in lines {
        let v: Vec<isize> = PATTERN
            .captures(line)
            .unwrap_or_else(|| panic!("Line doesn't match the pattern - \n{:?}", line))
            .iter()
            .skip(1)
            .map(Option::unwrap)
            .map(|c| c.as_str().parse())
            .map(Result::unwrap)
            .collect();

        if let [sx, sy, bx, by] = v[0..4] {
            out.push(Sensor { sx, sy, bx, by })
        } else {
            panic!();
        }
    }

    Ok(out)
}

fn merge(r1: &(isize, isize), r2: (isize, isize)) -> Option<(isize, isize)> {
    // Merge two ranges if they overlap - None otherwise
    let range = r1.0..=r1.1;
    if range.contains(&r2.0) || range.contains(&r2.1) {
        let s = *min(&r1.0, &r2.0);
        let e = *max(&r1.1, &r2.1);
        Some((s, e))
    } else {
        None
    }
}

fn row_coverage(sensors: &[Sensor], target_y: isize) -> Coverage {
    let mut row = Vec::new();
    let mut beacon_xs = BTreeSet::new();

    // Determine the sensor intervals for the target row
    for s in sensors.iter().filter(|l| -> bool {
        // Is this sensor in range of the target row?
        l.sy.abs_diff(target_y) < l.dist()
    }) {
        // How wide will the diamond be at the target row?
        let width = (s.dist() - s.sy.abs_diff(target_y)) as isize;
        let int = (s.sx - width, s.sx + width);
        row.push(int);

        if s.by == target_y {
            beacon_xs.insert(s.bx);
        }
    }

    // Sort the intervals into merge ready order
    row.sort_by_key(|r| (r.0, r.1));

    // Merge the intervals on this row as much as possible
    let mut merged_row = vec![];
    let mut merged = row[0];
    for r in row[1..].iter() {
        if let Some(m) = merge(&merged, *r) {
            merged = m;
        } else {
            merged_row.push(merged);
            merged = *r;
        }
    }
    merged_row.push(merged);

    Coverage {
        row: merged_row,
        beacon_count: beacon_xs.len() as isize,
    }
}

fn part_a(lines: &[String], target_y: isize) -> AResult<isize> {
    let sensors = parse(lines)?;
    let Coverage { row, beacon_count } = row_coverage(&sensors, target_y);
    assert_eq!(row.len(), 1);
    Ok((1 + (row[0].1 - row[0].0)) - beacon_count)
    // The 1+ is because the ranges are inclusive
}

fn part_b(lines: &[String], y_max: isize) -> AResult<isize> {
    // If we assume the distress beacon is inside the area delimited by the
    // sensors ranges we need to look for a gap inside one of the rows
    let sensors = parse(lines)?;

    for y in 0..=y_max {
        let row = row_coverage(&sensors, y).row;
        if row.len() > 1 {
            let x = row[1].0 - 1;
            return Ok(x * 4000000 + y);
        }
    }

    Err(anyhow::format_err!("Not found"))
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
    println!("Part A result = {}", part_a(lines.as_slice(), 2000000)?);
    println!("Part B result = {}", part_b(lines.as_slice(), 4000000)?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_small() -> AResult<()> {
        assert_eq!(
            part_a(
                &["Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_string()],
                10
            )?,
            12
        );
        Ok(())
    }

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice(), 10)?, 26);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice(), 20)?, 56000011);
        Ok(())
    }
}
