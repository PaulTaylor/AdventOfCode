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
type Grid = HashMap<Coord, char>;

fn parse(lines: &[String]) -> (Grid, Coord, String) {
    let mut grid = HashMap::new();

    let mut line_no = 0usize;
    let mut robot = (0isize, 0isize);
    for line in lines {
        if line.is_empty() {
            break;
        }

        for (x, c) in line.char_indices() {
            match c {
                '@' => robot = (x.try_into().unwrap(), line_no.try_into().unwrap()),
                'O' | '#' | '[' | ']' => {
                    grid.insert((x.try_into().unwrap(), line_no.try_into().unwrap()), c);
                }
                _ => {}
            }
        }

        line_no += 1;
    }

    assert_ne!(robot, (0, 0), "Robot initial position was not found");

    (grid, robot, lines[line_no..].concat().trim().to_string())
}

fn r#move((x, y): Coord, (dx, dy): Coord) -> Coord {
    (x + dx, y + dy)
}

fn tick((mut grid, mut robot): (Grid, Coord), instr: char) -> (Grid, Coord) {
    let dir = match instr {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => unreachable!(),
    };

    let new_pos = r#move(robot, dir);
    match grid.get(&new_pos) {
        Some('#') => {
            // Ouch, the robot has hit a wall - noop
        }
        Some('O') => {
            let mut to_move = vec![new_pos];
            loop {
                let desired = r#move(*to_move.last().unwrap(), dir);
                match grid.get(&desired) {
                    Some('#') => {
                        // This entire move fails
                        to_move.clear();
                        break;
                    }
                    Some('O') => {
                        // Another box in the chain - check it in turn
                        to_move.push(desired);
                    }
                    None => {
                        // An empty space - the move succeeds
                        while let Some(old_box) = to_move.pop() {
                            assert_eq!(Some('O'), grid.remove(&old_box));
                            grid.insert(r#move(old_box, dir), 'O');
                        }
                        robot = new_pos;
                        break;
                    }
                    _ => unreachable!(),
                }
            }
        }
        None => {
            // Simple move - no blocking boxes
            robot = new_pos;
        }
        Some(_) => unreachable!(),
    }

    (grid, robot)
}

fn part_a(lines: &[String]) -> usize {
    let (grid, robot, instr) = parse(lines);
    instr
        .chars()
        .fold((grid, robot), tick)
        .0
        .into_iter()
        .filter_map(|((x, y), c)| match c {
            'O' => Some(y * 100 + x),
            _ => None,
        })
        .sum::<isize>()
        .try_into()
        .unwrap()
}

fn push_vertical(grid: &Grid, box_left: Coord, dir: Coord) -> Option<Vec<(Coord, Coord, char)>> {
    // Define current and desired coordinates for the box
    let (x, y) = box_left;
    let box_right = (x + 1, y);
    let next_left = r#move(box_left, dir);
    let next_right = r#move(box_right, dir);

    let add_this_move = |mut res: Vec<_>| -> Vec<_> {
        res.push((box_left, next_left, '['));
        res.push((box_right, next_right, ']'));
        res
    };

    // Take the appropriate (possibly recursive) action based on what's currently at the
    // desired coordinates
    match (grid.get(&next_left), grid.get(&next_right)) {
        (_, Some('#')) | (Some('#'), _) => {
            None // Would hit the wall
        }
        (None, None) => {
            Some(add_this_move(vec![])) // No-blockers - can move
        }
        (Some('['), Some(']')) => {
            // Theres a box flush in the way
            push_vertical(grid, next_left, dir).map(add_this_move)
        }
        (None, Some('[')) => {
            // RHS collision with the LHS in the adjacent space
            push_vertical(grid, next_right, dir).map(add_this_move)
        }
        (Some(']'), None) => {
            // LHS collision with the RHS in the adjacent space
            let adj_left = (next_left.0 - 1, next_left.1);
            push_vertical(grid, adj_left, dir).map(add_this_move)
        }
        (Some(']'), Some('[')) => {
            // Combination of the two cases above
            let adj_left = (next_left.0 - 1, next_left.1);
            let lhs_res = push_vertical(grid, adj_left, dir);
            let rhs_res = push_vertical(grid, next_right, dir);

            match (lhs_res, rhs_res) {
                (None, _) | (_, None) => None,
                (Some(lr), Some(rr)) => {
                    let res = lr.into_iter().chain(rr).collect();
                    Some(add_this_move(res))
                }
            }
        }
        _ => unreachable!(),
    }
}

fn tick_b((mut grid, mut robot): (Grid, Coord), instr: char) -> (Grid, Coord) {
    let dir = match instr {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => unreachable!(),
    };

    let new_pos = r#move(robot, dir);
    match grid.get(&new_pos) {
        None => {
            robot = new_pos; // Simple move - no blocking boxes
        }
        Some('#') => {
            // The robot has hit a wall - no op
        }
        Some('[' | ']') if dir.1 == 0 => {
            // Horizontal Moves - use the same logic as in part one
            let mut to_move = vec![(new_pos, if dir.0 == -1 { ']' } else { '[' })];
            loop {
                let desired = r#move(to_move.last().unwrap().0, dir);
                match grid.get(&desired) {
                    Some('#') => {
                        // This entire move fails
                        to_move.clear();
                        break;
                    }
                    Some(']') => {
                        to_move.push((desired, ']'));
                    }
                    Some('[') => {
                        to_move.push((desired, '['));
                    }
                    None => {
                        // An empty space - the move succeeds
                        while let Some((old_pos, v)) = to_move.pop() {
                            let new_pos = r#move(old_pos, dir);
                            assert_eq!(Some(v), grid.remove(&old_pos));
                            grid.insert(new_pos, v);
                        }
                        robot = new_pos;
                        break;
                    }
                    _ => unreachable!(),
                }
            }
        }
        Some(x) if "[]".contains(*x) && dir.1 != 0 => {
            // Vertical moves - use a recursive algorithm for this
            let box_left = if x == &'[' {
                new_pos
            } else {
                (new_pos.0 - 1, new_pos.1)
            };

            if let Some(moves) = push_vertical(&grid, box_left, dir) {
                let all_sources: BTreeSet<_> = moves.iter().map(|&(s, _, _)| s).collect();
                let all_destinations: BTreeSet<_> = moves.iter().map(|&(_, d, _)| d).collect();
                let to_clear = all_sources.difference(&all_destinations);

                grid.extend(moves.iter().map(|(_, d, v)| (d, v)));
                to_clear.for_each(|s| {
                    grid.remove(s);
                });
                robot = new_pos;
            }
        }
        Some(_) => unreachable!(),
    }

    (grid, robot)
}

fn part_b(lines: &[String]) -> usize {
    let new_lines: Vec<_> = lines
        .iter()
        .map(|line| {
            line.replace('#', "##")
                .replace('O', "[]")
                .replace('.', "..")
                .replace('@', "@.")
        })
        .collect();

    let (grid, robot, instr) = parse(&new_lines);
    let (final_grid, _) = instr.chars().fold((grid, robot), tick_b);

    final_grid
        .into_iter()
        .filter_map(|((x, y), c)| match c {
            '[' => Some((100 * y) + x),
            _ => None,
        })
        .sum::<isize>()
        .try_into()
        .unwrap()
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

    const TEST_INPUTS: &[(&str, usize, Option<usize>)] = &[
        (
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
            2028,
            None,
        ),
        (
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
            10092,
            Some(9021),
        ),
    ];

    #[test]
    fn test_a() {
        for (input, ans, _) in TEST_INPUTS {
            let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
            assert_eq!(part_a(lines.as_slice()), *ans);
        }
    }

    #[test]
    fn test_b() {
        let input = "#######
                #...#.#
                #.....#
                #..OO@#
                #..O..#
                #.....#
                #######

                <vv<<^^<<^^";
        let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 618);

        for (input, _, o) in TEST_INPUTS {
            if let Some(ans) = o {
                let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
                assert_eq!(part_b(&lines), *ans);
            }
        }
    }
}
