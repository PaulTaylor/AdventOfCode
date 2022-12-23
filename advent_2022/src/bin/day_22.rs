use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[allow(clippy::similar_names, clippy::no_effect)]
fn parse(lines: &[String]) -> (HashMap<(isize, isize), char>, Vec<String>) {
    let board: HashMap<_, char> = lines[..lines.len() - 2]
        .iter()
        .enumerate()
        .flat_map(|(ri, line)| {
            line.char_indices()
                .filter_map(|(ci, c)| {
                    if c == ' ' {
                        None
                    } else {
                        Some(((ri.try_into().unwrap(), ci.try_into().unwrap()), c))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let p = Regex::new("([0-9]+)|(R)|(L)").unwrap();
    let instr: Vec<String> = p
        .find_iter(lines[lines.len() - 1].as_str())
        .map(|m| m.as_str().to_string())
        .collect();

    (board, instr)
}

fn move_right(d: &str, row: isize, col: isize, board: &HashMap<(isize, isize), char>) -> isize {
    let dist: usize = d.parse().unwrap();
    let mut col = col;

    for _ in 0..dist {
        let mut nc = col + 1;
        if board.get(&(row, nc)).is_none() {
            nc = 0;
            while board.get(&(row, nc)).is_none() {
                nc += 1;
            }
        }

        if board[&(row, nc)] == '#' {
            break;
        }

        col = nc;
    }

    col
}

fn move_left(d: &str, row: isize, col: isize, board: &HashMap<(isize, isize), char>) -> isize {
    let dist: usize = d.parse().unwrap();
    let mut col = col;

    for _ in 0..dist {
        let mut nc = col - 1;
        if board.get(&(row, nc)).is_none() {
            nc = 1000; // <- would be better to not use a hardcoded value
            while board.get(&(row, nc)).is_none() {
                nc -= 1;
            }
        }

        if board[&(row, nc)] == '#' {
            break;
        }

        col = nc;
    }

    col
}

fn move_up(d: &str, row: isize, col: isize, board: &HashMap<(isize, isize), char>) -> isize {
    let dist: usize = d.parse().unwrap();
    let mut row = row;
    for _ in 0..dist {
        let mut nr = row - 1;
        if board.get(&(nr, col)).is_none() {
            nr = 1000; // <- would be better to not use a hardcoded value
            while board.get(&(nr, col)).is_none() {
                nr -= 1;
            }
        }

        if board[&(nr, col)] == '#' {
            break;
        }

        row = nr;
    }
    row
}

fn move_down(d: &str, row: isize, col: isize, board: &HashMap<(isize, isize), char>) -> isize {
    let dist: usize = d.parse().unwrap();
    let mut row = row;

    for _ in 0..dist {
        let mut nr = row + 1;
        if board.get(&(nr, col)).is_none() {
            nr = 0;
            while board.get(&(nr, col)).is_none() {
                nr += 1;
            }
        }
        if board[&(nr, col)] == '#' {
            break;
        }
        row = nr;
    }
    row
}

#[allow(clippy::match_same_arms)]
fn part_a(lines: &[String]) -> isize {
    let (board, instr) = parse(lines);

    let mut dir = '>';
    let mut row = 0;
    let mut col = board
        .keys()
        .filter_map(|&(ro, ci)| if ro == 0 { Some(ci) } else { None })
        .min()
        .unwrap();

    for ins in instr {
        match (ins.as_str(), dir) {
            // Directions
            ("R", d) => match d {
                '>' => dir = 'v',
                'v' => dir = '<',
                '<' => dir = '^',
                '^' => dir = '>',
                _ => panic!(),
            },
            ("L", d) => match d {
                '>' => dir = '^',
                '^' => dir = '<',
                '<' => dir = 'v',
                'v' => dir = '>',
                _ => panic!(),
            },
            // Movement
            (d, '>') => {
                col = move_right(d, row, col, &board);
            }
            (d, '<') => {
                col = move_left(d, row, col, &board);
            }
            (d, '^') => {
                row = move_up(d, row, col, &board);
            }
            (d, 'v') => {
                row = move_down(d, row, col, &board);
            }
            _ => panic!(),
        }
    }

    ((row + 1) * 1000)
        + ((col + 1) * 4)
        + match dir {
            '>' => 0,
            'v' => 1,
            '<' => 2,
            '^' => 3,
            _ => panic!(),
        }
}

//
// Start of part_b stuff
//

fn move_right_b(
    dist: isize,
    row: isize,
    col: isize,
    board: &HashMap<(isize, isize), char>,
) -> (isize, isize, char) {
    let mut col = col;
    for so_far in 0..dist {
        let nc = col + 1;
        if board.get(&(row, nc)).is_none() {
            // Which face are we moving off
            if row >= 150 {
                // Moving right off face 1 to move up face 3
                let nc = row - 100; // 100 = 150 - 50
                let nr = 149; // last row of face 3 is 149

                // peek to see if there's a foothold possible on the other face
                // - will steal 1 movement step
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        // blocked - return from here
                        break;
                    }
                    _ => {
                        return move_up_b(dist - so_far - 1, nr, nc, board);
                    }
                }
            } else if row >= 100 {
                // Moving right off Face 3 to start moving left across face 6
                let nr = 149 - row;
                let nc = 149;

                // peek to see if there's a foothold possible on the other face
                // - will steal 1 movement step
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        // blocked - return from here
                        break;
                    }
                    _ => {
                        return move_left_b(dist - so_far - 1, nr, nc, board);
                    }
                }
            } else if row >= 50 {
                // Moving off the rhs of face 4 onto the bottom of face 6
                let nr = 49;
                let nc = 100 + (row - 50);
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        // blocked - return from here
                        break;
                    }
                    _ => {
                        return move_up_b(dist - so_far - 1, nr, nc, board);
                    }
                }
            } else if row >= 0 {
                let nr = 100 + (49 - row);
                let nc = 99;
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        // blocked - return from here
                        break;
                    }
                    _ => {
                        return move_left_b(dist - so_far - 1, nr, nc, board);
                    }
                }
            }
            panic!("not implemented");
        }

        if board[&(row, nc)] == '#' {
            break;
        }

        col = nc;
    }

    (row, col, '>')
}

fn move_left_b(
    dist: isize,
    row: isize,
    col: isize,
    board: &HashMap<(isize, isize), char>,
) -> (isize, isize, char) {
    let mut col = col;

    for so_far in 0..dist {
        let nc = col - 1;
        if board.get(&(row, nc)).is_none() {
            if row >= 150 {
                // Moving off face 1 onto the top of face 5
                let nc = row - 100;
                let nr = 0;
                // peek it to check if blocked - will steal 1 movement step
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break; // blocked
                    }
                    _ => return move_down_b(dist - so_far - 1, nr, nc, board),
                }
            } else if row >= 100 {
                // Moving off face 2 onto the lhs of face 5
                let nr = 50 - (row - 99);
                let nc = 50;
                // peek it
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break; // blocked
                    }
                    _ => return move_right_b(dist - so_far - 1, nr, nc, board),
                }
            } else if row >= 50 {
                // Moving off left edge of face 4
                let nc = row - 50;
                let nr = 100;
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break; // blocked
                    }
                    _ => return move_down_b(dist - so_far - 1, nr, nc, board),
                }
            } else if row >= 0 {
                // Moving off left edge of face 5 onto f2
                let nr = 149 - row;
                let nc = 0;
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break; // blocked
                    }
                    _ => return move_right_b(dist - so_far - 1, nr, nc, board),
                }
            }
            panic!();
        }

        if board[&(row, nc)] == '#' {
            break;
        }

        col = nc;
    }

    (row, col, '<')
}

fn move_up_b(
    dist: isize,
    row: isize,
    col: isize,
    board: &HashMap<(isize, isize), char>,
) -> (isize, isize, char) {
    let mut row = row;
    for so_far in 0..dist {
        let nr = row - 1;
        if board.get(&(nr, col)).is_none() {
            if nr == 99 {
                // Going off the top of Face 2
                let nr = 50 + col;
                let nc = 50;
                // peek it - which steals move movement point
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break; // blocked
                    }
                    _ => return move_right_b(dist - so_far - 1, nr, nc, board),
                }
            } else if col < 100 {
                // Face 5 (100 is the first index of face 6)
                let nr = col + 100;
                let nc = 0;
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break; // blocked
                    }
                    _ => return move_right_b(dist - so_far - 1, nr, nc, board),
                }
            } else if col < 150 {
                // Face 6
                let nc = col - 100;
                let nr = 199;
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break; // blocked
                    }
                    _ => return move_up_b(dist - so_far - 1, nr, nc, board),
                }
            }

            panic!()
        }

        if board[&(nr, col)] == '#' {
            break;
        }

        row = nr;
    }

    (row, col, '^')
}

fn move_down_b(
    dist: isize,
    mut row: isize,
    mut col: isize,
    board: &HashMap<(isize, isize), char>,
) -> (isize, isize, char) {
    for so_far in 0..dist {
        let mut nr = row + 1;
        if board.get(&(nr, col)).is_none() {
            if nr >= 200 {
                // dropped off the bottom of 1 -- loop around to 6 and keep going as normal
                nr = 0;
                // peek before updating col/row
                match board.get(&(nr, col + 100)) {
                    Some(c) if *c != '#' => {
                        col += 100; // all good - can continue
                    }
                    _ => {
                        break; // blocked
                    }
                }
            } else if nr >= 150 {
                let nr = 150 + (col - 50);
                let nc = 49;
                match board.get(&(nr, nc)) {
                    Some(c) if *c != '#' => {
                        return move_left_b(dist - so_far - 1, nr, nc, board);
                    }
                    _ => {
                        break;
                    }
                }
            } else if nr >= 50 {
                // Falling of the bottom of face 6
                let nr = (col - 100) + 50;
                let nc = 99;
                match board.get(&(nr, nc)) {
                    Some(c) if *c == '#' => {
                        break;
                    }
                    _ => {
                        return move_left_b(dist - so_far - 1, nr, nc, board);
                    }
                }
            } else {
                panic!();
            }
        }
        if board[&(nr, col)] == '#' {
            break;
        }
        row = nr;
    }

    (row, col, 'v')
}

fn part_b(
    board: &HashMap<(isize, isize), char>,
    instr: Vec<String>,
    start_row: isize,
    start_col: isize,
) -> isize {
    let mut state = (start_row, start_col, '>');
    for ins in instr {
        let (row, col, dir) = state;
        state = match (ins.as_str(), dir) {
            // Directions
            ("R", d) => (
                row,
                col,
                match d {
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    '^' => '>',
                    _ => panic!(),
                },
            ),
            ("L", d) => (
                row,
                col,
                match d {
                    '>' => '^',
                    '^' => '<',
                    '<' => 'v',
                    'v' => '>',
                    _ => panic!(),
                },
            ),
            // Movement
            (d, '>') => move_right_b(d.parse().unwrap(), row, col, board),
            (d, '<') => move_left_b(d.parse().unwrap(), row, col, board),
            (d, '^') => move_up_b(d.parse().unwrap(), row, col, board),
            (d, 'v') => move_down_b(d.parse().unwrap(), row, col, board),
            _ => panic!(),
        };
    }

    let (row, col, dir) = state;
    ((row + 1) * 1000)
        + ((col + 1) * 4)
        + match dir {
            '>' => 0,
            'v' => 1,
            '<' => 2,
            '^' => 3,
            _ => panic!(),
        }
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

    let (board, instr) = parse(lines.as_slice());
    println!("Part B result = {}", part_b(&board, instr, 0, 50));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.
        
10R5L5R10L4R5L5";

    const EXTRA_TEST_INPUT: &str = "        ...#.#..
        .#...#..
        #....#..
        .....#..
    .......#
    ....#...
    ...#....
    ......#.
...#....
.....#..
.#......
......#.
        
10R5L5L10L4R5L5";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(String::from).collect();
        assert_eq!(part_a(lines.as_slice()), 6032);

        let lines: Vec<_> = EXTRA_TEST_INPUT.lines().map(String::from).collect();
        assert_eq!(part_a(lines.as_slice()), 6042);
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_b() -> AResult<()> {
        // Will do some small tests directly on the real input because the net
        // is a different shape in the test

        let file = File::open("./data/day_22.txt")?;
        let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();
        let (board, _instr) = parse(lines.as_slice());

        // faces numbered as
        //   5 6
        //   4
        // 2 3
        // 1

        // In the asserts:
        // assert_eq!(
        //     part_b(..., 150, 37), // <- these are zero indexed
        //     150_000 + 38 * 4 + 3  // <- these are one indexed
        // );

        // Face 1
        // Up
        assert_eq!(
            part_b(&board, vec!["L".to_string(), "2".to_string()], 150, 37),
            150_000 + 38 * 4 + 3
        );
        println!("===================================");
        // Right - will turn over to the bottom of 3 and start upwards
        // Normal
        assert_eq!(
            part_b(&board, vec!["1".to_string()], 150, 49),
            150_000 + 51 * 4 + 3
        );
        println!("===================================");
        // Blocked
        assert_eq!(
            part_b(&board, vec!["1".to_string()], 159, 49),
            160_000 + 50 * 4
        );
        // Down - will turn over onto face 6 and continue down
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["R".to_string(), "1".to_string()], 199, 0),
            1_000 + 101 * 4 + 1
        );
        // Down with block
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["R".to_string(), "2".to_string()], 199, 1),
            200_009
        );
        // Left - start moving down face 5
        println!("===================================");
        assert_eq!(
            part_b(
                &board,
                vec!["R".to_string(), "R".to_string(), "5".to_string()],
                150,
                0
            ),
            5000 + (51 * 4) + 1
        );
        // Left with block
        println!("===================================");
        assert_eq!(
            part_b(
                &board,
                vec!["R".to_string(), "R".to_string(), "5".to_string()],
                152,
                0
            ),
            153_006
        );

        // Face 2
        // Down and Right are as-normal as so will not be tested here
        //
        // Up - start going right across face 4
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["L".to_string(), "1".to_string()], 100, 0),
            51_000 + (51 * 4)
        );
        // Up with block
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["L".to_string(), "1".to_string()], 100, 43),
            101_000 + (44 * 4) + 3
        );
        // Left - start going right across 5 (upside down!)
        println!("===================================");
        assert_eq!(
            part_b(
                &board,
                vec!["L".to_string(), "L".to_string(), "40".to_string()],
                119,
                0
            ),
            31208,
            "f2 left clean"
        );
        // Left with block
        println!("===================================");
        assert_eq!(
            part_b(
                &board,
                vec!["L".to_string(), "L".to_string(), "40".to_string()],
                124,
                0
            ),
            26232
        );

        // Face 3
        // Up and right are "normal" moves so are not tested
        //
        // Right - turns to go left across face 6
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["40".to_string()], 149, 149),
            1_000 + (4 * 147) + 2,
            "f3 right clean"
        );
        // Right blocked
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["40".to_string()], 144, 149),
            145_000 + (4 * 150),
            "f3 right blocked"
        );
        // Down - coming in right edge of 1 proceeding left
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["R".to_string(), "40".to_string()], 149, 50),
            151_000 + (4 * 49) + 2,
            "f3 down clean"
        );

        // Face 4
        // Left - enter the top of 2 travelling down
        println!("===================================");
        assert_eq!(
            part_b(
                &board,
                vec!["R".to_string(), "R".to_string(), "1".to_string()],
                99,
                50
            ),
            101_000 + (4 * 50) + 1,
            "f4 left clean"
        );
        // Right
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["1".to_string()], 50, 99),
            50_000 + (4 * 101) + 3,
            "f4 right clean"
        );

        // Face 5
        // Left - enter lhs of 1 travelling right
        println!("===================================");
        assert_eq!(
            part_b(
                &board,
                vec!["R".to_string(), "R".to_string(), "1".to_string()],
                0,
                50
            ),
            150_004,
            "f5 left clean"
        );
        // Up - enter lh of 1 travelling right
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["L".to_string(), "1".to_string()], 0, 99),
            200_000 + 4,
            "f5 up clean"
        );

        // Face 6
        // Up
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["L".to_string(), "1".to_string()], 0, 147),
            200_000 + (48 * 4) + 3,
            "f6 up clean"
        );

        // Right
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["1".to_string()], 25, 149),
            125_402,
            "f6 right clean"
        );

        // Down
        println!("===================================");
        assert_eq!(
            part_b(&board, vec!["R".to_string(), "1".to_string()], 49, 100),
            51_000 + 400 + 2,
            "f6 down clean"
        );

        // Additional test from debugging - :'(
        assert_eq!(move_left_b(11, 144, 8, &board), (5, 52, '>'));

        Ok(())
    }
}
