use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

/// Generate a string from the grid given a starting position, step offsets and length
fn get_string(
    lines: &[String],
    target_len: usize,
    row: usize,
    col: usize,
    row_step: isize,
    col_step: isize,
) -> String {
    let mut to_check = String::new();

    let mut r = row;
    let mut c = col;
    let mut count = 0;

    while (0..lines.len()).contains(&r) && (0..lines[0].len()).contains(&c) && count < target_len {
        to_check.push(lines[r][c..=c].chars().next().unwrap());
        r = r.wrapping_add_signed(row_step);
        c = c.wrapping_add_signed(col_step);
        count += 1;
    }

    to_check
}

fn part_a(lines: &[String]) -> usize {
    let target = "XMAS";
    let mut x_locs = 0;

    for (row, line) in lines.iter().enumerate() {
        for (col, _) in line.char_indices().filter(|&(_, ch)| ch == 'X') {
            for row_step in -1..2 {
                for col_step in -1..2 {
                    if row_step == 0 && col_step == 0 {
                        continue; // Would lead to infinite loop in get_string so skip it
                    }

                    let to_check = get_string(lines, target.len(), row, col, row_step, col_step);
                    x_locs += usize::from(to_check.starts_with(target));
                }
            }
        }
    }

    x_locs
}

fn part_b(lines: &[String]) -> usize {
    let mut x_locs = 0;
    let target = "MAS";
    let r_target = &target.chars().rev().collect::<String>();

    for (row, line) in lines.iter().enumerate().skip(1) {
        for (col, _) in line.char_indices().filter(|&(i, ch)| i > 0 && ch == 'A') {
            let ten_four_str = get_string(lines, target.len(), row - 1, col - 1, 1, 1);
            let ten_four = ten_four_str.starts_with(target) || ten_four_str.starts_with(r_target);

            let two_eight_str = get_string(lines, target.len(), row - 1, col + 1, 1, -1);
            let two_eight =
                two_eight_str.starts_with(target) || two_eight_str.starts_with(r_target);
            x_locs += usize::from(ten_four && two_eight);
        }
    }

    x_locs
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

    const TEST_INPUT: &str = "MMMSXXMASM
                              MSAMXMSMSA
                              AMXSXMAAMM
                              MSAMASMSMX
                              XMASAMXAMM
                              XXAMMXXAMA
                              SMSMSASXSS
                              SAXAMASAAA
                              MAMMMXMMMM
                              MXMXAXMASX";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 18);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 9);
    }
}
