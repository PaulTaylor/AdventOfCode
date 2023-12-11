use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|l| l.chars().collect()).collect()
}

fn determine_type(grid: &[Vec<char>], row: usize, col: usize) -> char {
    // | is a vertical pipe connecting north and south.
    if row > 0 && "|7F".contains(grid[row - 1][col]) && "|LJ".contains(grid[row + 1][col]) {
        '|'
    } else if col > 0 && "-LF".contains(grid[row][col - 1]) && "-J7".contains(grid[row][col + 1]) {
        // - is a horizontal pipe connecting east and west.
        '-'
    } else if row > 0 && "|7F".contains(grid[row - 1][col]) && "-J7".contains(grid[row][col + 1]) {
        // L is a 90-degree bend connecting north and east.
        'L'
    } else if row > 0
        && col > 0
        && "|7F".contains(grid[row - 1][col])
        && "-LF".contains(grid[row][col - 1])
    {
        // J is a 90-degree bend connecting north and west.
        'J'
    } else if col > 0 && "|LJ".contains(grid[row + 1][col]) && "-LF".contains(grid[row][col - 1]) {
        // 7 is a 90-degree bend connecting south and west.
        '7'
    } else if "|LJ".contains(grid[row + 1][col]) && "-J7".contains(grid[row][col + 1]) {
        // F is a 90-degree bend connecting south and east.
        'F'
    } else {
        panic!("Cannot determine underlying type");
    }
}

#[allow(clippy::match_on_vec_items)]
fn find_loop(grid: &[Vec<char>]) -> (Vec<(usize, usize)>, char) {
    let Some(start) = grid.iter().enumerate().find_map(|(ri, row)| {
        row.iter()
            .enumerate()
            .find_map(|(ci, &c)| if c == 'S' { Some((ri, ci)) } else { None })
    }) else {
        panic!("Couldn't find starting position.")
    };

    // Determine what the S should actually be
    let s_type = determine_type(grid, start.0, start.1);

    let mut pos = match s_type {
        '|' | 'F' | '7' => (start.0 + 1, start.1), // Start south in these cases
        x => panic!("No start rule defined for {x}"),
    };

    let mut path = vec![start, pos];
    let mut history: HashSet<_> = path.iter().copied().collect();

    while start != pos {
        let (p_row, p_col) = pos;

        // Determine where we go next (there will be only two options)
        let (opt1, opt2) = match grid[p_row][p_col] {
            '|' => ((p_row - 1, p_col), (p_row + 1, p_col)),
            '-' => ((p_row, p_col - 1), (p_row, p_col + 1)),
            'L' => ((p_row - 1, p_col), (p_row, p_col + 1)),
            'J' => ((p_row - 1, p_col), (p_row, p_col - 1)),
            '7' => ((p_row + 1, p_col), (p_row, p_col - 1)),
            'F' => ((p_row + 1, p_col), (p_row, p_col + 1)),
            x => panic!("No directions known for {x}"),
        };

        pos = if history.contains(&opt1) && history.contains(&opt2) {
            // We're done
            break;
        } else if history.contains(&opt1) {
            opt2
        } else {
            opt1
        };

        // Update path, history
        path.push(pos);
        history.insert(pos);
    }

    (path, s_type)
}

fn part_a(lines: &[String]) -> usize {
    let grid = parse(lines);
    let (path, _) = find_loop(&grid);
    path.len() / 2
}

fn part_b(lines: &[String]) -> usize {
    let mut grid = parse(lines);
    let (path, s_type) = find_loop(&grid);

    // Any cells that are not part of the grid can be considered ground (as they're junk)
    for (ri, row) in grid.iter_mut().enumerate() {
        for (ci, c) in row.iter_mut().enumerate() {
            if !path.contains(&(ri, ci)) {
                *c = '.';
            }
        }
    }

    // https://en.wikipedia.org/wiki/Point_in_polygon

    let l7 = Regex::new(r"L-*7").unwrap();
    let f7 = Regex::new(r"F-*7").unwrap();
    let fj = Regex::new(r"F-*J").unwrap();

    let mut inside_count = 0;
    for row in grid {
        let mut inside = false;
        let mut row_string: String = row.iter().collect();
        row_string = row_string.replace('S', &format!("{s_type}"));

        // Remove the horizontal wall sections and replace with differing | or ||
        // based on the shape formed by the pairs used in the scenario

        // Lightening bolt shape - Replace with 1 |
        row_string = l7.replace_all(&row_string, "|").to_string();
        row_string = fj.replace_all(&row_string, "|").to_string();

        // U shape - replace with 2 |
        row_string = f7.replace_all(&row_string, "||").to_string();

        for orig in row_string.chars() {
            let c = match orig {
                'S' => s_type,
                x => x,
            };

            match c {
                '|' | '7' => inside = !inside,
                '.' if inside => inside_count += 1,
                _ => (),
            }
        }
    }

    inside_count
}

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

    const SIMPLEST_INPUT: &str = ".....
    .S-7.
    .|.|.
    .L-J.
    .....";

    const LARGER_INPUT: &str = "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";

    const SIMPLE_LOOP_WITH_GAP: &str = "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";

    const SIMPLE_LOOP_NO_GAP: &str = "..........
    .S------7.
    .|F----7|.
    .||....||.
    .||....||.
    .|L-7F-J|.
    .|..||..|.
    .L--JL--J.
    ..........";

    const LARGER_LOOP: &str = ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";

    const LARGER_WITH_JUNK: &str = "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_a() {
        let lines: Vec<_> = SIMPLEST_INPUT
            .lines()
            .map(|l| l.trim().to_string())
            .collect();
        assert_eq!(part_a(lines.as_slice()), 4);

        let lines: Vec<_> = LARGER_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 8);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = SIMPLE_LOOP_WITH_GAP
            .lines()
            .map(|l| l.trim().to_string())
            .collect();
        assert_eq!(part_b(lines.as_slice()), 4);

        let lines: Vec<_> = SIMPLE_LOOP_NO_GAP
            .lines()
            .map(|l| l.trim().to_string())
            .collect();
        assert_eq!(part_b(lines.as_slice()), 4);

        let lines: Vec<_> = LARGER_LOOP.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 8);

        let lines: Vec<_> = LARGER_WITH_JUNK
            .lines()
            .map(|l| l.trim().to_string())
            .collect();
        assert_eq!(part_b(lines.as_slice()), 10);
    }
}
