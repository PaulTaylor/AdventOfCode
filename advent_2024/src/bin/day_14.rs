use humantime::format_duration;
use image::{ImageBuffer, Rgb};
use indicatif::*;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Robot = (usize, isize, isize, isize, isize);

fn parse(lines: &[String]) -> AResult<HashMap<(isize, isize), Vec<Robot>>> {
    let pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut out = HashMap::new();
    for (id, line) in lines.iter().enumerate() {
        let caps = pattern.captures(line).unwrap();
        let (_, [px, py, vx, vy]): (_, [&str; 4]) = caps.extract();
        let r: Robot = (id, px.parse()?, py.parse()?, vx.parse()?, vy.parse()?);

        out.entry((r.1, r.2))
            .and_modify(|l: &mut Vec<_>| l.push(r))
            .or_insert(vec![r]);
    }
    Ok(out)
}

fn tick(r: Robot, width: isize, height: isize) -> Robot {
    let (id, x, y, vx, vy) = r;
    (
        id,
        (x + vx).rem_euclid(width),
        (y + vy).rem_euclid(height),
        vx,
        vy,
    )
}

fn solution(lines: &[String], width: isize, height: isize, max_iter: i16) -> AResult<usize> {
    let mut states = parse(lines)?;

    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut a = 0;

    for clock in (1..=max_iter).progress() {
        // Update the state
        let mut new_state = HashMap::new();
        for &old_r in states.values().flatten() {
            let r = tick(old_r, width, height);
            new_state
                .entry((r.1, r.2))
                .and_modify(|l: &mut Vec<_>| l.push(r))
                .or_insert(vec![r]);
        }
        states = new_state;

        // Check if there are large blocks of robots near each other to reduce
        // the number of output images and do the quadrant counts for Part One
        let mut quads = [0; 4];
        let mut contiguous_counts: Vec<_> = (0..width).map(|_| 0).collect();
        for (&(x, y), l) in &states {
            if states.contains_key(&(x, y - 1)) {
                let ux: usize = x.try_into()?;
                contiguous_counts[ux] += 1;
            }

            // Calculate the quadrant values
            match (x, y) {
                (x, y) if x < mid_x && y < mid_y => quads[0] += l.len(),
                (x, y) if x > mid_x && y < mid_y => quads[1] += l.len(),
                (x, y) if x < mid_x && y > mid_y => quads[2] += l.len(),
                (x, y) if x > mid_x && y > mid_y => quads[3] += l.len(),
                _ => (),
            };
        }

        // Grab the Part One result as it goes past
        if clock == 100 {
            a = quads.iter().product::<usize>();
        }

        let most_contiguous = *contiguous_counts.iter().max().unwrap();
        if most_contiguous > 15 {
            let mut buffer: image::RgbImage =
                ImageBuffer::new(width.try_into()?, height.try_into()?);
            for (x, y, pixel) in buffer.enumerate_pixels_mut() {
                if states.contains_key(&(x.try_into()?, y.try_into()?)) {
                    *pixel = Rgb([255, 255, 255]);
                } else {
                    *pixel = Rgb([0, 0, 0]);
                };
            }
            buffer
                .save(format!("./output/image-{clock:06}.png"))
                .unwrap();
        }
    }

    println!("Part A result = {a}\nPart B result => Go and inspect the output in ./output");
    Ok(a)
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
    solution(lines.as_slice(), 101, 103, 10_000)?;
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
                              p=6,3 v=-1,-3
                              p=10,3 v=-1,2
                              p=2,0 v=2,-1
                              p=0,0 v=1,3
                              p=3,0 v=-2,-2
                              p=7,6 v=-1,-3
                              p=3,0 v=-1,-2
                              p=9,3 v=2,3
                              p=7,3 v=-1,2
                              p=2,4 v=2,-3
                              p=9,5 v=-3,-3";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solution(lines.as_slice(), 11, 7, 100).unwrap(), 12);
    }
}
