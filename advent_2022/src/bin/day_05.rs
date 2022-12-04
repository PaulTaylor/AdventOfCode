use core::panic;
use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type State<'a> = Vec<Vec<char>>;
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse(lines: &[String]) -> AResult<(State, Vec<Move>)> {
    let crane_positions = (1..lines[0].len()).step_by(4);
    let mut state: State = crane_positions.clone().map(|_| Vec::new()).collect();

    let mut moves: Vec<_> = Vec::with_capacity(lines.len());
    let mut state_finished = false;
    let i_pattern = Regex::new("move ([0-9]+) from ([0-9]) to ([0-9])")?;

    for line in lines {
        if line.trim().is_empty() {
            state_finished = true;
            continue;
        }

        // instructions
        if state_finished {
            let m = i_pattern.captures(line).unwrap();
            moves.push(Move {
                quantity: m.get(1).unwrap().as_str().parse()?,
                from: m.get(2).unwrap().as_str().parse()?,
                to: m.get(3).unwrap().as_str().parse()?,
            });
            continue;
        }

        // Initial state
        let chars: Vec<char> = line.chars().collect();
        for col in crane_positions.clone() {
            let crane = col / 4;
            match chars[col] {
                'A'..='Z' => {
                    let l = state.get_mut(crane).unwrap();
                    l.insert(0, chars[col]);
                }
                '1'..='9' | ' ' => { /* ignore patterns */ }
                _ => panic!(),
            };
        }
    }

    Ok((state, moves))
}

fn _display(state: &State) {
    println!("==============================================");
    for (i, stack) in state.iter().enumerate() {
        println!("{}: {}", i + 1, stack.iter().collect::<String>())
    }
}

fn part_a(lines: &[String]) -> AResult<String> {
    let (mut state, moves) = parse(lines)?;
    for Move { quantity, from, to } in moves {
        for _ in 0..quantity {
            let temp = state.get_mut(from - 1).unwrap().pop().unwrap();
            state.get_mut(to - 1).unwrap().push(temp);
        }
    }

    Ok(state.iter().map(|l| l.last().unwrap()).collect())
}

fn part_b(lines: &[String]) -> AResult<String> {
    let (mut state, moves) = parse(lines)?;
    for Move { quantity, from, to } in moves {
        let split = state[from - 1].len() - quantity as usize;
        let temp = state.get_mut(from - 1).unwrap().split_off(split);
        state.get_mut(to - 1).unwrap().extend(temp);
    }

    Ok(state.iter().map(|l| l.last().unwrap()).collect())
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

    println!("Run took {}ms", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> [String; 9] {
        [
            "    [D]    \n".to_string(),
            "[N] [C]    \n".to_string(),
            "[Z] [M] [P]\n".to_string(),
            " 1   2   3 ".to_string(),
            "\n".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ]
    }

    #[test]
    fn test_a() -> AResult<()> {
        let the_input = test_input();
        let (state, instructions) = parse(the_input.as_slice())?;
        assert_eq!(state.len(), 3);
        assert_eq!(state[0].len(), 2);
        assert_eq!(state[1].len(), 3);
        assert_eq!(state[2].len(), 1);
        assert_eq!(instructions.len(), 4);
        assert_eq!(part_a(the_input.as_slice())?, "CMZ");
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let the_input = test_input();
        assert_eq!(part_b(the_input.as_slice())?, "MCD");
        Ok(())
    }
}
