use conv::ValueFrom;
use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (isize, isize);
type Fence = (char, isize, isize, isize);

fn parse(lines: &[String]) -> HashMap<Coord, char> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| -> Vec<_> {
            l.char_indices()
                .map(|(x, c)| {
                    (
                        (isize::value_from(x).unwrap(), isize::value_from(y).unwrap()),
                        c,
                    )
                })
                .collect()
        })
        .collect()
}

fn generate_neighbours(root: &Coord) -> Vec<Coord> {
    let &(cx, cy) = root;
    vec![(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]
}

fn generate_regions(grid: &HashMap<Coord, char>) -> Vec<(char, BTreeSet<Coord>)> {
    let mut regions = vec![];
    let mut remaining_coords: BTreeSet<Coord> = grid.keys().copied().collect();
    while !remaining_coords.is_empty() {
        let mut region = BTreeSet::new();
        let root = remaining_coords.pop_first().unwrap();
        let mut queue = vec![root];
        let v = grid.get(&root).unwrap();

        while let Some(cand) = queue.pop() {
            remaining_coords.remove(&cand);
            if !region.insert(cand) {
                continue;
            }

            for other in generate_neighbours(&cand) {
                if let Some(other_v) = grid.get(&other) {
                    if v == other_v {
                        queue.push(other);
                    }
                }
            }
        }
        regions.push((*v, region));
    }

    regions
}

fn check_corner(
    fences: &BTreeSet<Fence>,
    dir: char,
    a: isize,
    b: isize,
    start: isize,
    end: isize,
) -> bool {
    match dir {
        '-' => {
            let f1 = ('|', a, start, end);
            let f2 = ('|', b, start, end);
            fences.contains(&f1) || fences.contains(&f2)
        }
        '|' => {
            let f1 = ('-', a, start, end);
            let f2 = ('-', b, start, end);
            fences.contains(&f1) || fences.contains(&f2)
        }
        _ => unreachable!(),
    }
}

fn part_ab(lines: &[String]) -> (usize, usize) {
    let grid = parse(lines);

    // Collect the contiguous regions
    let mut regions = generate_regions(&grid);
    regions.sort();

    let mut acc_a = 0;
    let mut acc_b = 0;
    for (_ch, region) in regions {
        // Calculate the fence sections for this region (Part 1)
        let mut fences = BTreeSet::new();
        for c in &region {
            for n in generate_neighbours(c) {
                if !region.contains(&n) {
                    let mut f_raw = [*c, n];
                    f_raw.sort_unstable();

                    let f: Fence = match f_raw {
                        [(x1, y1), (x2, y2)] if x1 == x2 => ('-', x1, y1, y2),
                        [(x1, y1), (x2, y2)] if y1 == y2 => ('|', y1, x1, x2),
                        _ => unreachable!(),
                    };

                    fences.insert(f);
                }
            }
        }
        acc_a += region.len() * fences.len();

        // Calculate the straight fence sections for Part 2
        // Group the fence parts into groups of sections that are on the same axis
        let mut grouped_fences: HashMap<(char, isize, isize), BTreeSet<isize>> = HashMap::new();
        for &(dir, v, c1, c2) in &fences {
            grouped_fences
                .entry((dir, c1, c2))
                .and_modify(|s| {
                    s.insert(v);
                })
                .or_insert_with(|| [v].into_iter().collect());
        }

        // Determine how many sections there are along each fence axis
        let mut fence_count = 0;
        for ((dir, a, b), mut set) in grouped_fences {
            let mut start = set.pop_first().unwrap();
            while let Some(next) = set.pop_first() {
                // `check_corner` checks for the "mobius" case in the problem description where diagonally
                // adjacent corners can appear to be a single continuous length
                if next != start + 1 || check_corner(&fences, dir, a, b, start, next) {
                    fence_count += 1;
                }
                start = next;
            }
            fence_count += 1;
        }

        acc_b += region.len() * fence_count;
    }
    (acc_a, acc_b)
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
    let res = part_ab(lines.as_slice());
    println!("Part A result = {}", res.0);
    println!("Part B result = {}", res.1);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUTS: [(&str, Option<usize>, Option<usize>); 5] = [
        ("AAAA\nBBCD\nBBCC\nEEEC", Some(140), Some(80)),
        ("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO", Some(772), Some(436)),
        (
            "RRRRIICCFF
             RRRRIICCCF
             VVRRRCCFFF
             VVRCCCJFFF
             VVVVCJJCFE
             VVIVCCJJEE
             VVIIICJJEE
             MIIIIIJJEE
             MIIISIJEEE
             MMMISSJEEE",
            Some(1930),
            Some(1206),
        ),
        (
            "EEEEE
             EXXXX
             EEEEE
             EXXXX
             EEEEE",
            None,
            Some(236),
        ),
        (
            "AAAAAA
             AAABBA
             AAABBA
             ABBAAA
             ABBAAA
             AAAAAA",
            None,
            Some(368),
        ),
    ];

    #[test]
    fn test_a() {
        for (input, o_actual, _) in TEST_INPUTS {
            if let Some(actual) = o_actual {
                let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
                assert_eq!(part_ab(lines.as_slice()).0, actual);
            }
        }
    }

    #[test]
    fn test_b() {
        for (input, _, o_actual) in TEST_INPUTS {
            if let Some(actual) = o_actual {
                let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
                assert_eq!(part_ab(lines.as_slice()).1, actual);
            }
        }
    }
}
