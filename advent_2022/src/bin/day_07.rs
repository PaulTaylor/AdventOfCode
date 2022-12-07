use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn walk(lines: &[String]) -> HashMap<String, u64> {
    // First collect file locations & sizes
    let mut files: HashMap<String, u64> = HashMap::new();
    let cwd: &mut Vec<&str> = &mut Vec::new();
    for line in lines {
        match line.as_str() {
            "$ cd .." => {
                cwd.pop();
            }
            l if l.starts_with("$ cd ") => {
                cwd.push(&l[5..]);
            }
            l if l.chars().next().unwrap().is_ascii_digit() => {
                let bits: Vec<_> = l.split(' ').collect();
                let mut name: String = String::from("/");
                if !cwd.is_empty() {
                    name.push_str(cwd.join("/").as_str());
                    name.push('/');
                }
                name.push_str(bits[1]);
                files.insert(name, bits[0].parse().unwrap());
            }
            _ => { /* ignore everything else */ }
        }
    }

    // Now iterate over files to build the directory sizes
    let mut dirs: HashMap<_, _> = HashMap::new();
    for (f_key, f_size) in files {
        let path: Vec<_> = f_key.split('/').collect();
        for i in 1..path.len() {
            let d_key = path[0..i].join("/");
            dirs.entry(d_key)
                .and_modify(|d_size| *d_size += f_size)
                .or_insert(f_size);
        }
    }

    dirs
}

fn part_a(lines: &[String]) -> AResult<u64> {
    let mut dirs = walk(lines);
    dirs.retain(|_, v| *v <= 100000);
    Ok(dirs.values().sum())
}

fn part_b(lines: &[String]) -> AResult<u64> {
    let dirs = walk(lines);

    let free_space = 70000000 - dirs.get("").unwrap();
    let extra_needed = 30000000 - free_space;

    let mut dirs: Vec<_> = Vec::from_iter(dirs);
    dirs.sort_by_key(|(_, v)| *v);
    Ok(dirs.iter().find(|(_, v)| *v >= extra_needed).unwrap().1)
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

    const TEST_INPUT: &str = "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 95437);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 24933642);
        Ok(())
    }
}
