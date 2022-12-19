use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

type Coord = (u32, u32, u32);

#[derive(Debug, Hash, PartialEq, Eq)]
struct Face(Coord, Coord);

impl Face {
    fn new(c1: Coord, c2: Coord) -> Face {
        // Order to coords in a face so we have consistent Map keys
        Face(min(c1, c2), max(c1, c2))
    }
}

fn parse(lines: &[String]) -> AResult<Vec<Coord>> {
    Ok(lines
        .iter()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        // shift all of the coordinates by 1 so we don't have to deal with
        // bounds issues on usizes
        .map(|s| (s[0] + 1, s[1] + 1, s[2] + 1))
        .collect())
}

fn generate_faces(c: Coord) -> [Face; 6] {
    let (x, y, z) = c;
    [
        Face::new((x, y, z), (x, y, z - 1)),
        Face::new((x, y, z), (x, y, z + 1)),
        Face::new((x, y, z), (x, y - 1, z)),
        Face::new((x, y, z), (x, y + 1, z)),
        Face::new((x, y, z), (x - 1, y, z)),
        Face::new((x, y, z), (x + 1, y, z)),
    ]
}

fn part_a(lines: &[String]) -> AResult<usize> {
    let cubes = parse(lines)?;
    let mut faces = HashMap::new();

    for c in cubes.iter() {
        for face in generate_faces(*c) {
            faces.entry(face).and_modify(|v| *v += 1).or_insert(0);
        }
    }

    Ok(faces.len() - faces.values().filter(|&&v| v > 0).count())
}

fn is_outside(
    coord: Coord,
    max: Coord,
    cubes: &[Coord],
    external_blocks: &mut HashSet<Coord>,
    history: &mut HashSet<Coord>,
) -> bool {
    let (cx, cy, cz) = coord;
    let (mx, my, mz) = max;
    if cx == 0
        || cy == 0
        || cz == 0
        || cx == mx
        || cy == my
        || cz == mz
        || external_blocks.contains(&coord)
    {
        return true;
    }
    if cubes.contains(&coord) {
        return false; // we're a cube
    }

    history.insert(coord);

    // If there are no cubes to my left I'm outside
    let mut outside = !cubes
        .iter()
        .any(|&(ox, oy, oz)| ox < cx && oy == cy && oz == cz)

    || {
        !cubes
            .iter()
            .any(|&(ox, oy, oz)| ox == cx && oy < cy && oz == cz)
    }
    || {
        !cubes
            .iter()
            .any(|&(ox, oy, oz)| ox == cx && oy == cy && oz < cz)
    }

    // to the right
    || {
        !cubes
            .iter()
            .any(|&(ox, oy, oz)| ox > cx && oy == cy && oz == cz)
    }
    || {
        !cubes
            .iter()
            .any(|&(ox, oy, oz)| ox == cx && oy > cy && oz == cz)
    }
    || {
        !cubes
            .iter()
            .any(|&(ox, oy, oz)| ox == cx && oy == cy && oz > cz)
    };

    if !outside {
        // check our neighbours to see if they're outside
        outside = (!history.contains(&(cx - 1, cy, cz))
            && is_outside((cx - 1, cy, cz), max, cubes, external_blocks, history))
            || (!history.contains(&(cx + 1, cy, cz))
                && is_outside((cx + 1, cy, cz), max, cubes, external_blocks, history))
            || (!history.contains(&(cx, cy - 1, cz))
                && is_outside((cx, cy - 1, cz), max, cubes, external_blocks, history))
            || (!history.contains(&(cx, cy + 1, cz))
                && is_outside((cx, cy + 1, cz), max, cubes, external_blocks, history))
            || (!history.contains(&(cx, cy, cz - 1))
                && is_outside((cx, cy, cz - 1), max, cubes, external_blocks, history))
            || (!history.contains(&(cx, cy, cz + 1))
                && is_outside((cx, cy, cz + 1), max, cubes, external_blocks, history))
    }

    if outside {
        external_blocks.insert(coord);
    }

    outside
}

fn part_b(lines: &[String]) -> AResult<usize> {
    let mut cubes = parse(lines)?;
    cubes.sort();
    let cubes = &cubes;

    let mut external_faces = 0;
    let mut external_blocks = HashSet::new();
    let max = (
        cubes.iter().map(|x| x.0).max().unwrap() + 1,
        cubes.iter().map(|x| x.1).max().unwrap() + 1,
        cubes.iter().map(|x| x.2).max().unwrap() + 1,
    );

    // Check each cube - if it can route to 0,0,0 without colliding with
    // another cube then it's external
    for &(x, y, z) in cubes {
        // Left face
        external_faces += is_outside(
            (x - 1, y, z),
            max,
            cubes,
            &mut external_blocks,
            &mut HashSet::new(),
        ) as usize;
        external_faces += is_outside(
            (x + 1, y, z),
            max,
            cubes,
            &mut external_blocks,
            &mut HashSet::new(),
        ) as usize;
        external_faces += is_outside(
            (x, y - 1, z),
            max,
            cubes,
            &mut external_blocks,
            &mut HashSet::new(),
        ) as usize;
        external_faces += is_outside(
            (x, y + 1, z),
            max,
            cubes,
            &mut external_blocks,
            &mut HashSet::new(),
        ) as usize;
        external_faces += is_outside(
            (x, y, z - 1),
            max,
            cubes,
            &mut external_blocks,
            &mut HashSet::new(),
        ) as usize;
        external_faces += is_outside(
            (x, y, z + 1),
            max,
            cubes,
            &mut external_blocks,
            &mut HashSet::new(),
        ) as usize;
    }

    Ok(external_faces)
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

    const TEST_INPUT: &str = "2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(&lines[..2])?, 10);
        assert_eq!(part_a(lines.as_slice())?, 64);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines[..1])?, 6);
        assert_eq!(
            part_b(&lines[..2])?,
            10,
            "pair has no void - so all non-adj faces are external"
        );
        assert_eq!(
            part_b(&lines[..3])?,
            14,
            "pair has no void - so all non-adj faces are external"
        );
        assert_eq!(part_b(lines.as_slice())?, 58);
        Ok(())
    }
}
