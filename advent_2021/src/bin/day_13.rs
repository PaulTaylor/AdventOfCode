use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant, iter::repeat
};

type AResult<T> = anyhow::Result<T>;
type Instructions = (Vec<(usize, usize)>, Vec<(char, usize)>);

fn _print_grid(grid: &[Vec<char>]) {
    println!("================================");
    for (idx, row) in grid.iter().enumerate() {
        println!("{:02} {}", idx, row.iter().map(|x| if *x == '.' { ' ' } else { *x }).collect::<String>())
    }
    println!("================================");
}

fn parse(lines: &[String]) -> AResult<Instructions> {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    let point_pattern = Regex::new("^([0-9]+),([0-9]+)$")?;
    let fold_pattern = Regex::new("([xy])=([0-9]+)$")?;


    for line in lines {
        if let Some(caps) = point_pattern.captures(line) {
            points.push((
                caps.get(1).map(|v| v.as_str().parse().unwrap()).unwrap(),
                caps.get(2).map(|v| v.as_str().parse().unwrap()).unwrap(),
            ))
        } else if let Some(caps) = fold_pattern.captures(line) {
            folds.push((
                caps.get(1).unwrap().as_str().chars().next().unwrap(),
                caps.get(2).map(|v| v.as_str().parse().unwrap()).unwrap()
            ))
        }
    }

    Ok((points, folds))
}

#[allow(clippy::needless_range_loop)]  // reason: code is more readable with the loop
fn fold(grid: &mut Vec<Vec<char>>, fold_ax: char, fold_idx: usize) {
    if fold_ax == 'x' {
        // Copy values over the fold
        for col in fold_idx+1..grid[0].len() {
            let new_col_idx = grid[0].len()-col-1;
            for row in 0..grid.len() {
                grid[row][new_col_idx] = min(grid[row][col], grid[row][new_col_idx]);
            }
        }
        // Remove unneeded columns
        let num_cols = grid[0].len();
        for row in grid.iter_mut() {
            (fold_idx..num_cols).rev().for_each(|i| { row.remove(i); })
        }
    } else if fold_ax == 'y' {
        // Copy values over the fold
        for row in fold_idx+1..grid.len() {
            let new_row_idx = grid.len()-row-1;
            for col in 0..grid[0].len() {
                grid[new_row_idx][col] = min(grid[new_row_idx][col], grid[row][col])
            }
        }
        // Remove unneeded rows
        (fold_idx..grid.len()).rev().for_each(|i| { grid.remove(i); });
    } else {
        panic!("Unknown fold axis");
    }
}

fn part_a(lines: &[String]) -> AResult<u64> {
    let (points, folds) = parse(lines)?;
    let row_count = *points.iter().map(|(_,y)| y).max().unwrap() as usize;
    let col_count = *points.iter().map(|(x,_)| x).max().unwrap() as usize;

    let mut grid: Vec<Vec<char>> = (0..=row_count).map(|_| Vec::from_iter(repeat('.').take(col_count + 1))).collect();

    for (col, row) in points {
        grid[row][col] = '#';
    }

    let (fold_ax, fold_idx) = folds.first().unwrap();
    fold(&mut grid, *fold_ax, *fold_idx);

    // Count the dots
    let mut acc = 0u64;
    for row in grid {
        for col in row {
            if col == '#' {
                acc += 1;
            }
        }
    }

    Ok(acc)
}

fn part_b(lines: &[String]) -> AResult<&str> {
    let (points, folds) = parse(lines)?;
    let row_count = *points.iter().map(|(_,y)| y).max().unwrap() as usize;
    let col_count = *points.iter().map(|(x,_)| x).max().unwrap() as usize;

    let mut grid: Vec<Vec<char>> = (0..=row_count).map(|_| Vec::from_iter(repeat('.').take(col_count + 1))).collect();

    for (col, row) in points {
        grid[row][col] = '#';
    }

    for (fold_ax, fold_idx) in folds {
        fold(&mut grid, fold_ax, fold_idx);
    }

    // Count the dots
    _print_grid(&grid);

    Ok("the letters printed above")
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
    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}ms", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    #[test]
    fn test_part_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 17);
        Ok(())
    }

    #[test]
    fn test_vertical_fold() -> AResult<()> {
        let mut grid = vec![
            Vec::from_iter("#.##..#..#.".chars()),
            Vec::from_iter("#...#......".chars()),
            Vec::from_iter("......#...#".chars()),
            Vec::from_iter("#...#......".chars()),
            Vec::from_iter(".#.#..#.###".chars()),
            Vec::from_iter("...........".chars()),
            Vec::from_iter("...........".chars()),
        ];

        fold(&mut grid, 'x', 5);

        assert_eq!(grid[0].len(), 5);
        let the_string: String = grid[0].iter().collect();
        assert_eq!(the_string, "#####".to_string());
        let the_string: String = grid[1].iter().collect();
        assert_eq!(the_string, "#...#".to_string());

        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        part_b(lines.as_slice())?;
        Ok(())
    }
}
