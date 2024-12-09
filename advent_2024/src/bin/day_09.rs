use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Pair = (usize, usize);

fn _print_state(files: &BTreeMap<usize, usize>) -> String {
    let lfb = files.last_key_value().unwrap().0 + 10;
    let mut out = String::new();
    for i in 0..lfb {
        if let Some(fid) = files.get(&i) {
            out.push_str(&format!("{fid}"));
        } else {
            out.push('.');
        }
    }
    out
}

fn parse_blocks(lines: &[String]) -> (BTreeMap<usize, usize>, BTreeSet<usize>) {
    let mut blocks: BTreeMap<_, _> = BTreeMap::new();
    let mut free: BTreeSet<_> = BTreeSet::new();

    let mut is_file = true;
    let mut idx = 0;
    let mut file_id = 0;
    for c in lines[0].chars() {
        let v = (c as usize) - 48;

        for _ in 0..v {
            if is_file {
                blocks.insert(idx, file_id);
            } else {
                free.insert(idx);
            }
            idx += 1;
        }

        file_id += usize::from(is_file);
        is_file = !is_file;
    }

    (blocks, free)
}

fn part_a(lines: &[String]) -> usize {
    let (mut blocks, mut free_space) = parse_blocks(lines);

    loop {
        let (last_file_block, value) = blocks.pop_last().unwrap();
        let first_free_block = free_space.pop_first().unwrap_or(usize::MAX);

        if first_free_block > last_file_block {
            blocks.insert(last_file_block, value);
            return blocks
                .into_iter()
                .map(|(idx, file_id)| (idx * file_id))
                .sum();
        }

        blocks.insert(first_free_block, value);
        free_space.insert(last_file_block);
    }
}

fn parse_files(lines: &[String]) -> (BTreeMap<usize, Pair>, BTreeSet<Pair>) {
    let mut files: BTreeMap<_, _> = BTreeMap::new();
    let mut free: BTreeSet<_> = BTreeSet::new();

    let mut is_file = true;
    let mut idx = 0;
    let mut file_id = 0;
    for c in lines[0].chars() {
        let v = (c as usize) - 48;

        if is_file {
            files.insert(file_id, (idx, v));
            file_id += 1;
        } else {
            free.insert((idx, v));
        }

        idx += v;
        is_file = !is_file;
    }

    (files, free)
}

fn part_b(lines: &[String]) -> usize {
    let (input_files, mut free) = parse_files(lines);
    let mut output_blocks = BTreeMap::new();

    // Attempt to move each file exactly once in order of decreasing file ID number
    for (file_id, (file_start, file_len)) in input_files.iter().rev() {
        // Find a free space that is TO THE LEFT of the current starting position
        let gap = free
            .iter()
            .find(|(idx, free_len)| idx < file_start && free_len >= file_len);

        if let Some(gap_spec) = gap.copied() {
            // There is an appropriate gap.

            // Update the free space map
            free.remove(&gap_spec);
            let (free_start, free_len) = gap_spec;
            if *file_len < free_len {
                // Put the remaining free space back into the free space map
                let upd_free = (free_start + file_len, free_len - file_len);
                free.insert(upd_free);
            }

            // Insert the file into the block map in the new position
            for offset in 0..*file_len {
                assert!(output_blocks
                    .insert(free_start + offset, *file_id)
                    .is_none());
            }

            // Don't need to mark the original position as free as files can only move left
        } else {
            // Insert the file into the block map in the existing position
            for offset in 0..*file_len {
                assert!(output_blocks
                    .insert(file_start + offset, *file_id)
                    .is_none());
            }
        }
    }
    // Create the flat block map in order to compute the final value
    output_blocks
        .into_iter()
        .map(|(idx, file_id)| (idx * file_id))
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

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 1928);
    }

    #[test]
    fn test_b() {
        let mut lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 2858);
        // Additional test case where the disk map ends with free space
        lines[0].push('9');
        assert_eq!(part_b(&lines), 2858);
    }
}
