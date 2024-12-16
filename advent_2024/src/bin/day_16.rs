use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (isize, isize);
type DirCoord = (isize, isize, char);

fn parse(lines: &[String]) -> (HashMap<Coord, char>, Coord, Coord) {
    let mut grid: HashMap<_, _> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| match c {
                '.' => None,
                c => Some(((x.try_into().unwrap(), y.try_into().unwrap()), c)),
            })
        })
        .collect();

    let start = grid
        .iter()
        .find_map(|(&c, &v)| if v == 'S' { Some(c) } else { None })
        .expect("No start found");
    let end = grid
        .iter()
        .find_map(|(&c, &v)| if v == 'E' { Some(c) } else { None })
        .expect("No end found");

    grid.remove(&start);
    grid.remove(&end);

    (grid, start, end)
}

fn get_visited_locations(
    end: DirCoord,
    prev: &HashMap<DirCoord, HashSet<DirCoord>>,
    seen: &mut HashSet<Coord>,
) {
    let coord = (end.0, end.1);
    seen.insert(coord);

    if let Some(prevs) = prev.get(&end) {
        for &p in prevs {
            get_visited_locations(p, prev, seen);
        }
    }
}

fn solution(lines: &[String]) -> (isize, isize) {
    let (grid, start, end) = parse(lines);

    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut queue = BTreeSet::new();

    let start_vertex = (start.0, start.1, '>');
    dist.insert(start_vertex, 0);
    queue.insert((0, start_vertex));

    for x in 0..=start.0 {
        for y in 0..=start.1 {
            for dir in "><^v".chars() {
                if !grid.contains_key(&(x, y)) && (x, y) != start {
                    queue.insert((isize::MAX, (x, y, dir)));
                }
            }
        }
    }

    while let Some((_cost, u)) = queue.pop_first() {
        let (x, y, d) = u;

        let mut neighbours = Vec::with_capacity(5);

        // Continue forward in the current direction
        let next_pos = match d {
            '^' => (x, y - 1, d),
            '>' => (x + 1, y, d),
            '<' => (x - 1, y, d),
            'v' => (x, y + 1, d),
            _ => unreachable!(),
        };
        if !grid.contains_key(&(next_pos.0, next_pos.1)) {
            neighbours.push((next_pos, 1));
        }

        // Rotations
        let rots: [_; 2] = match d {
            '^' | 'v' => [(x, y, '<'), (x, y, '>')],
            '<' | '>' => [(x, y, '^'), (x, y, 'v')],
            _ => unreachable!(),
        };
        neighbours.extend(rots.into_iter().map(|v| (v, 1000)));

        for (v, cost) in neighbours {
            let alt = dist.get(&u).unwrap() + cost;
            dist.entry(v)
                .and_modify(|dv| {
                    if alt <= *dv {
                        prev.entry(v).and_modify(|l: &mut HashSet<_>| {
                            if alt < *dv {
                                l.clear(); // If this is an improvement we wipe the previous history
                            }
                            l.insert(u);
                        });

                        *dv = alt;
                        queue.retain(|(_, x)| *x != v);
                        queue.insert((alt, v));
                    }
                })
                .or_insert_with(|| {
                    // If the previous cost was INF
                    prev.insert(v, [u].into_iter().collect());
                    queue.insert((alt, v));
                    alt
                });
        }
    }

    let (end_dir, a_result) = dist
        .into_iter()
        .filter(|&((x, y, _), _)| {
            let coord = (x, y);
            coord == end
        })
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let mut seen = HashSet::new();
    get_visited_locations(end_dir, &prev, &mut seen);
    let b_result = seen.len().try_into().unwrap();

    (a_result, b_result)
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
    let (a_res, b_res) = solution(&lines);
    println!("Part A result = {a_res}");
    println!("Part B result = {b_res}");
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUTS: [(&str, isize, isize); 2] = [
        (
            "###############
             #.......#....E#
             #.#.###.#.###.#
             #.....#.#...#.#
             #.###.#####.#.#
             #.#.#.......#.#
             #.#.#####.###.#
             #...........#.#
             ###.#.#####.#.#
             #...#.....#.#.#
             #.#.#.###.#.#.#
             #.....#...#.#.#
             #.###.#.#.#.#.#
             #S..#.....#...#
             ###############",
            7036,
            45,
        ),
        (
            "#################
             #...#...#...#..E#
             #.#.#.#.#.#.#.#.#
             #.#.#.#...#...#.#
             #.#.#.#.###.#.#.#
             #...#.#.#.....#.#
             #.#.#.#.#.#####.#
             #.#...#.#.#.....#
             #.#.#####.#.###.#
             #.#.#.......#...#
             #.#.###.#####.###
             #.#.#...#.....#.#
             #.#.#.#####.###.#
             #.#.#.........#.#
             #.#.#.#########.#
             #S#.............#
             #################",
            11048,
            64,
        ),
    ];

    #[test]
    fn test_ab() {
        for (input, ans_a, ans_b) in TEST_INPUTS {
            let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
            assert_eq!(solution(lines.as_slice()), (ans_a, ans_b));
        }
    }
}
