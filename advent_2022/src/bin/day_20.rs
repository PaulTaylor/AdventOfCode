use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn mix(nums: &mut Vec<(usize, isize)>) {
    let mut temp: Vec<(usize, isize)> = nums.to_vec();

    for original_index in 0..nums.len() {
        // Where is this pair in the "temp" vec
        let (current_index, v) = temp
            .iter()
            .enumerate()
            .find_map(|(ni, &(oi, v))| {
                if original_index == oi {
                    Some((ni, v))
                } else {
                    None
                }
            })
            .unwrap();
        temp.remove(current_index);

        let mut new_index = current_index as isize + v;

        // Wrap around indexes
        new_index = new_index.rem_euclid(temp.len() as isize);

        // In the example text index 0 is displayed at the END of the text
        // but it still counts as position 0 (as len mod len = 0);
        temp.insert(new_index as usize, (original_index, v));
    }

    // Now assign the appropriate numbers back into the num vec from the temp one
    nums.copy_from_slice(&temp)
}

fn part_a(lines: &[String]) -> AResult<isize> {
    // Record each number with it's original index - so we can deal with duplicates
    let mut nums: Vec<(usize, isize)> = lines
        .iter()
        .map(|x| x.parse().unwrap())
        .enumerate()
        .collect();

    mix(&mut nums);

    let zero_pos = nums
        .iter()
        .enumerate()
        .find_map(|(i, &(_, v))| if v == 0 { Some(i) } else { None })
        .unwrap();

    let elems = [
        nums[(zero_pos + 1000) % nums.len()].1,
        nums[(zero_pos + 2000) % nums.len()].1,
        nums[(zero_pos + 3000) % nums.len()].1,
    ];

    Ok(elems.iter().sum())
}

fn part_b(lines: &[String]) -> AResult<isize> {
    // Record each number with it's original index - so we can deal with duplicates
    let mut nums: Vec<(usize, isize)> = lines
        .iter()
        .map(|x| x.parse().unwrap())
        .map(|x: isize| x * 811589153) // apply the "key"
        .enumerate()
        .collect();

    // mix 10 times
    (0..10).for_each(|_| mix(&mut nums));

    let zero_pos = nums
        .iter()
        .enumerate()
        .find_map(|(i, &(_, v))| if v == 0 { Some(i) } else { None })
        .unwrap();

    let elems = [
        nums[(zero_pos + 1000) % nums.len()].1,
        nums[(zero_pos + 2000) % nums.len()].1,
        nums[(zero_pos + 3000) % nums.len()].1,
    ];

    Ok(elems.iter().sum())
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

    const TEST_INPUT: &str = "1
    2
    -3
    3
    -2
    0
    4";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 3);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 1623178306);
        Ok(())
    }
}
