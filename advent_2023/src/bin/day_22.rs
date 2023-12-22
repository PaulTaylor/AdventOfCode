use humantime::format_duration;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (usize, usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Brick {
    id: usize,
    start: Coord,
    end: Coord,
}

impl Brick {
    fn all_blocks(&self) -> Vec<Coord> {
        (self.start.0..=self.end.0)
            .flat_map(|x| {
                (self.start.1..=self.end.1)
                    .flat_map(|y| {
                        (self.start.2..=self.end.2)
                            .map(|z| (x, y, z))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn parse(lines: &[String]) -> Vec<Brick> {
    let patterns = Regex::new(r"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();
    let mut bricks = vec![];
    for (idx, line) in lines.iter().enumerate() {
        let caps = patterns.captures(line).unwrap();
        let (_, bits): (_, [&str; 6]) = caps.extract();
        let nums: Vec<usize> = bits
            .into_iter()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        bricks.push(Brick {
            id: idx,
            start: (nums[0], nums[1], nums[2]),
            end: (nums[3], nums[4], nums[5]),
        });
    }
    bricks
}

fn settle(lines: &[String]) -> Vec<Brick> {
    let mut last_bricks = parse(lines);
    last_bricks.sort_by(|b1, b2| {
        let mut ret = b1.start.2.cmp(&b2.start.2);
        if ret == Ordering::Equal {
            ret = b1.start.cmp(&b2.start);
        }
        ret
    });

    let mut iterations = 0;
    loop {
        let bricks = drop(&last_bricks, &mut HashSet::new());
        if bricks == last_bricks {
            last_bricks = bricks;
            break;
        }
        last_bricks = bricks;
        iterations += 1;
    }

    println!("Iterations required to initially settle = {iterations}");
    last_bricks
}

fn drop(before: &Vec<Brick>, dropped: &mut HashSet<usize>) -> Vec<Brick> {
    let occupied: HashSet<Coord> = before.iter().flat_map(Brick::all_blocks).collect();
    let mut after = Vec::with_capacity(before.len());
    let mut moved = false;
    for brick in before {
        if moved || (brick.start.2 == 1) || (brick.end.2 == 1) {
            // This brick is on the ground as (one-of) it's z-coords is 1
            // OR we've moved another block already this turn
            after.push(*brick);
            continue;
        }

        // Which blocks to check depends on orientation
        let &Brick {
            id: _,
            start: (x1, y1, z1),
            end: (x2, y2, z2),
        } = brick;

        let (footprint_size, to_check): (usize, HashSet<Coord>) = if x1 == x2 && y1 == y2 {
            // For a Z-block we only need to check the 1 block below it
            // Z-blocks only have a 1 block x/y footprint
            (1, (1..z1).map(|z| (x1, y1, z)).collect())
        } else if x1 == x2 && z1 == z2 {
            // For a y-block we need to check the z-1's of all Y's
            (
                1 + y2 - y1,
                (y1..=y2)
                    .flat_map(|y| (1..z1).map(|z| (x1, y, z)).collect::<Vec<_>>())
                    .collect(),
            )
        } else if y1 == y2 && z1 == z2 {
            // For a x-block we need to check the z-1's of all X's
            (
                1 + x2 - x1,
                (x1..=x2)
                    .flat_map(|x| (1..z1).map(|z| (x, y1, z)).collect::<Vec<_>>())
                    .collect(),
            )
        } else {
            panic!("Found a brick that isn't a stick {brick:?}")
        };

        // The disjoint set between occupied and to_check is the set of blocks
        // that we can fall through
        let available: HashMap<usize, Vec<Coord>> =
            to_check
                .difference(&occupied)
                .fold(HashMap::new(), |mut map, &(x, y, z)| {
                    map.entry(z)
                        .and_modify(|v| v.push((x, y, z)))
                        .or_insert(vec![(x, y, z)]);

                    map
                });

        // Find the smallest appropriate row
        let mut new_z = None;
        for candidate_z in (1..brick.start.2).rev() {
            if available.get(&candidate_z).unwrap_or(&vec![]).len() == footprint_size {
                new_z = Some(candidate_z);
            } else {
                break; // We've hit a row we can't fall through
            }
        }

        if let Some(z) = new_z {
            // Can drop!
            let Brick {
                id,
                start: (x_min, y_min, z_min),
                end: (x_max, y_max, z_max),
            } = *brick;
            let z_diff = z_max - z_min;
            after.push(Brick {
                id,
                start: (x_min, y_min, z),
                end: (x_max, y_max, z + z_diff),
            });
            moved = true;
            dropped.insert(id);
        } else {
            // Can't drop this brick as it's blocked by something else - leave it where it is
            after.push(*brick);
        }
    }

    assert_eq!(before.len(), after.len());
    after
}

fn part_a(settled: &Vec<Brick>) -> usize {
    // Check if there is movement when each brick is removed in turn
    settled
        .par_iter()
        .filter_map(|b| {
            let before: Vec<_> = settled.iter().filter(|&x| x != b).copied().collect();
            assert_eq!(before.len(), settled.len() - 1);
            let after: Vec<_> = drop(&before, &mut HashSet::new());
            assert_eq!(before.len(), after.len());

            if before == after {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

fn part_b(settled: &[Brick]) -> usize {
    settled
        .par_iter()
        .progress()
        .filter_map(|b| {
            let mut dropped = HashSet::new();
            let mut before: Vec<_> = settled.iter().filter(|&x| x != b).copied().collect();
            loop {
                let after: Vec<_> = drop(&before, &mut dropped);
                if before == after {
                    return Some(dropped.len());
                }
                before = after;
            }
        })
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
    println!("Waiting for bricks to settle...");
    let settled = settle(&lines);
    println!("Part A result = {}", part_a(&settled));
    println!("Part B result = {}", part_b(&settled));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9";

    #[test]
    fn test_all_blocks() {
        let bricks = parse(&["2,2,2~2,2,2".to_string()]);
        assert_eq!(bricks[0].all_blocks(), vec![(2, 2, 2)]);

        let bricks = parse(&["0,0,10~1,0,10".to_string()]);
        assert_eq!(bricks[0].all_blocks(), vec![(0, 0, 10), (1, 0, 10)]);

        let bricks = parse(&["0,0,10~0,1,10".to_string()]);
        assert_eq!(bricks[0].all_blocks(), vec![(0, 0, 10), (0, 1, 10)]);

        let bricks = parse(&["0,1,6~2,1,6".to_string()]);
        assert_eq!(
            bricks[0].all_blocks(),
            vec![(0, 1, 6), (1, 1, 6), (2, 1, 6)]
        );
    }

    #[test]
    fn test_drop() {
        let before = parse(&["5,5,1~5,6,1".to_string()]);
        let after = drop(&before, &mut HashSet::new());
        assert_eq!(before, after);

        let before = parse(&["0,2,1~0,2,5".to_string()]);
        let after = drop(&before, &mut HashSet::new());
        assert_eq!(before, after);

        let before = parse(&["3,3,20~3,3,21".to_string()]);
        let after = drop(&before, &mut HashSet::new());
        assert_ne!(before, after);
        assert_eq!(after[0].all_blocks(), vec![(3, 3, 1), (3, 3, 2)]);

        let before = parse(&["1,3,20~3,3,20".to_string()]);
        let after = drop(&before, &mut HashSet::new());
        assert_ne!(before, after, "no change for x-block");
        assert_eq!(
            after[0].all_blocks(),
            vec![(1, 3, 1), (2, 3, 1), (3, 3, 1)],
            "wrong change for x-block"
        );

        let before = parse(&["3,1,20~3,3,20".to_string()]);
        let after = drop(&before, &mut HashSet::new());
        assert_ne!(before, after, "no change for y-block");
        assert_eq!(
            after[0].all_blocks(),
            vec![(3, 1, 1), (3, 2, 1), (3, 3, 1)],
            "wrong change for y-block"
        );

        let before = parse(&["1,1,1~3,1,1".to_string(), "1,1,10~1,1,10".to_string()]);
        let after = drop(&before, &mut HashSet::new());
        assert_ne!(before, after, "no change for blocking test");
        assert_eq!(
            after[1].all_blocks(),
            vec![(1, 1, 2)],
            "wrong change for blocking test"
        );

        let before = parse(&["1,1,20~1,1,20".to_string(), "1,1,10~1,1,10".to_string()]);
        let after = drop(&before, &mut HashSet::new());
        assert_ne!(before, after, "no change for block with gap test - step 1");
        assert_eq!(
            after[0].all_blocks(),
            vec![(1, 1, 11)],
            "wrong change for with gap test"
        );
        let before = after;
        let after = drop(&before, &mut HashSet::new());
        assert_ne!(before, after, "no change for block with gap test - step 2");
        assert_eq!(
            after[0].all_blocks(),
            vec![(1, 1, 11)],
            "wrong change for with gap test - s2"
        );
        assert_eq!(
            after[1].all_blocks(),
            vec![(1, 1, 1)],
            "wrong change for with gap test - s2"
        );
        let before = after;
        let after = drop(&before, &mut HashSet::new());
        assert_ne!(before, after, "no change for block with gap test - step 2");
        assert_eq!(
            after[0].all_blocks(),
            vec![(1, 1, 2)],
            "wrong change for with gap test - s3"
        );
        assert_eq!(
            after[1].all_blocks(),
            vec![(1, 1, 1)],
            "wrong change for with gap test - s3"
        );

        let before = after;
        let after = drop(&before, &mut HashSet::new());
        assert_eq!(before, after);
    }

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let settled = settle(&lines);
        assert_eq!(part_a(&settled), 5);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let settled = settle(&lines);
        assert_eq!(part_b(&settled), 7);
    }
}
