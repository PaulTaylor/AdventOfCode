use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Write},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> HashMap<&str, HashSet<&str>> {
    lines
        .iter()
        .map(|l| {
            let (left, right) = l.split_once(':').unwrap();
            let targets = right.split_ascii_whitespace().map(|s| s.trim()).collect();
            (left, targets)
        })
        .collect()
}

fn _write_dot(adj: &HashMap<&str, HashSet<&str>>, file_name: &str) {
    let mut file = File::create(file_name).unwrap();
    file.write(b"digraph G {\n").unwrap();
    for (src, targets) in adj {
        for tgt in targets {
            file.write_fmt(format_args!("{src} -> {tgt};\n")).unwrap()
        }
    }
    file.write(b"}\n").unwrap();
}

fn part_a(lines: &[String]) -> usize {
    let adj = parse(lines);

    let mut paths = 0;
    let mut queue = VecDeque::from_iter([vec!["you"]]);
    while let Some(head) = queue.pop_front() {
        let last = head.last().unwrap();
        if last == &"out" {
            paths += 1;
        } else {
            for next in adj.get(last).unwrap() {
                let mut new_path = head.clone();
                new_path.push(next);
                queue.push_back(new_path);
            }
        }
    }
    paths
}

fn solve_path(
    adj: &HashMap<&str, HashSet<&str>>,
    start: &str,
    end: &str,
    exclude: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(res) = cache.get(start) {
        return *res;
    }

    // This turns out not to be necessary in the actual problem
    // because there are zero routes from dac to fft.
    // Leaving it in for prosperity
    if exclude == start {
        return 0;
    }

    if start == end {
        return 1;
    }

    let mut paths = 0;
    if let Some(nexts) = adj.get(start) {
        for next in nexts {
            paths += solve_path(adj, next, end, exclude, cache);
        }
    }
    cache.insert(start.to_string(), paths);
    paths
}

fn part_b(lines: &[String]) -> usize {
    let adj = parse(lines);

    let start_to_fft = solve_path(&adj, "svr", "fft", "dac", &mut HashMap::new());
    let start_to_dac = solve_path(&adj, "svr", "dac", "fft", &mut HashMap::new());

    let fft_to_dac = solve_path(&adj, "fft", "dac", "", &mut HashMap::new());
    let dac_to_fft = solve_path(&adj, "dac", "fft", "", &mut HashMap::new());

    let fft_to_out = solve_path(&adj, "fft", "out", "dac", &mut HashMap::new());
    let dac_to_out = solve_path(&adj, "dac", "out", "fft", &mut HashMap::new());

    (start_to_fft * fft_to_dac * dac_to_out) + (start_to_dac * dac_to_fft * fft_to_out)
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

    const TEST_INPUT: &str = "aaa: you hhh
                              you: bbb ccc
                              bbb: ddd eee
                              ccc: ddd eee fff
                              ddd: ggg
                              eee: out
                              fff: out
                              ggg: out
                              hhh: ccc fff iii
                              iii: out";

    const TEST_INPUT_B: &str = "svr: aaa bbb
                                aaa: fft
                                fft: ccc
                                bbb: tty
                                tty: ccc
                                ccc: ddd eee
                                ddd: hub
                                hub: fff
                                eee: dac
                                dac: fff
                                fff: ggg hhh
                                ggg: out
                                hhh: out";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 5);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT_B.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 2);
    }
}
