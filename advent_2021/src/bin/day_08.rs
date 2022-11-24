use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant, collections::{HashMap, HashSet},
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug)]
struct Key {
    top: char, ul: char, ur: char, mid: char, ll: char, lr: char, bot: char,
}

impl Key {
    fn convert(&self, s: &str) -> Pattern {
        Pattern {
            top: s.contains(self.top),
            ul: s.contains(self.ul),
            ur: s.contains(self.ur),
            mid: s.contains(self.mid),
            ll: s.contains(self.ll),
            lr: s.contains(self.lr),
            bot: s.contains(self.bot),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pattern {
    top: bool, ul: bool, ur: bool, mid: bool, ll: bool, lr: bool, bot: bool,
}

fn parse(lines: &[String]) -> AResult<Vec<(Vec<&str>, Vec<&str>)>> {
    let mut output = Vec::with_capacity(lines.len());

    for line in lines {
        let mut line_parts = line.split('|');
        let pattern_string = line_parts.next().unwrap();
        let number_string = line_parts.next().unwrap();

        let patterns = pattern_string.trim().split(' ').collect();
        let digits = number_string.trim().split(' ').collect();

        output.push((patterns, digits));
    }

    Ok(output)
}

fn part_a(lines: &[String]) -> AResult<usize> {
    let lines = parse(lines)?;
    Ok(lines
        .iter()
        .map(|(_, d)| {
            d.iter()
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum())
}

fn calculate_key(digits: Vec<&str>) -> Option<Key> {
    let mut x: HashMap<u8, HashSet<char>> = HashMap::new();
    let mut fives: Vec<HashSet<char>> = Vec::new();
    let mut sixes: Vec<HashSet<char>> = Vec::new();

    for seq in digits {
        match seq.len() {
            2 => { x.insert(1, HashSet::from_iter(seq.chars())); },
            3 => { x.insert(7, HashSet::from_iter(seq.chars())); },
            4 => { x.insert(4, HashSet::from_iter(seq.chars())); },
            5 => { fives.push(HashSet::from_iter(seq.chars())); },
            6 => { sixes.push(HashSet::from_iter(seq.chars())); },
            7 => { x.insert(8, HashSet::from_iter(seq.chars())); },
            _ => { panic!("wtf"); }
        };
    }

    let top = {
        let diff = x.get(&7)? - x.get(&1)?;
        let mut it = diff.iter();
        let v = it.next()?;
        assert!(it.next().is_none());
        *v
    };

    // There are 3 numbers with 5 segments - 2/3/5
    // and 3 numbers with 6 segments - 0/6/9

    // Taking the intersection of 2/3/5 and 4 will determine the middle
    let mid: char = {
        let mut set = x.get(&4)?.clone();
        for other in fives.iter() {
            set = &set & other;
        }

        let mut diff = set.iter();
        let v = diff.next()?;
        assert!(diff.next().is_none());
        *v
    };

    // Can determine upper-left now as 4-1-mid
    let ul = {
        let mut diff = x.get(&4)? - x.get(&1)?;
        diff.remove(&mid);
        assert_eq!(diff.len(), 1);
        diff.into_iter().next()?
    };

    // ll will only occur in one of the "fives" (the one which is 2)
    // (as will the ul - but we can ignore that as we already know what
    // it is).  Can also get the bottom from these counts (the 3-freq we
    // don't already have as top/mid).
    let (ll, bot) = {
        let counts: Vec<_> = ('a'..='g')
            .map(|c| (
                fives.iter().filter(
                    |f| f.contains(&c) && c != ul && c != top && c != mid).count(),
                 c)
            ).collect();

        let ll: char = counts.iter().find_map(
            |x| if x.0 == 1 { Some(x.1) } else { None }
        )?;

        let bot: char = counts.iter().find_map(
            |x| if x.0 == 3 { Some(x.1) } else { None }
        )?;

        (ll, bot)
    };

    // 1-6 will give me ur then lr is 1-ur
    let (ur, lr) = {
        // find 6 - only one in "sixes" with only one match with 1
        let one = x.get(&1)?;
        let six = sixes.iter().find(
            |v| v.intersection(one).count() == 1
        )?;

        let ur = (one - six).into_iter().next()?;
        let lr = (one & six).into_iter().next()?;
        (ur, lr)
    }; 

    Some(Key { top, ul, ur, mid, ll, lr, bot })
}

fn b_solver(pattern: Vec<&str>, digits: Vec<&str>) -> Option<usize> {
    let key = calculate_key(pattern)?;
    
    let mut string = String::from("");
    for digit in digits {
        let pattern = key.convert(digit);
        let n_char = match pattern {
            Pattern { top: true, ul: true, ur: true, mid: false, ll: true, lr: true, bot: true, } => '0',
            Pattern { top: false, ul: false, ur: true, mid: false, ll: false, lr: true, bot: false, } => '1',
            Pattern { top: true, ul: false, ur: true, mid: true, ll: true, lr: false, bot: true, } => '2',
            Pattern { top: true, ul: false, ur: true, mid: true, ll: false, lr: true, bot: true, } => '3',
            Pattern { top: false, ul: true, ur: true, mid: true, ll: false, lr: true, bot: false, } => '4',
            Pattern { top: true, ul: true, ur: false, mid: true, ll: false, lr: true, bot: true, } => '5',
            Pattern { top: true, ul: true, ur: false, mid: true, ll: true, lr: true, bot: true, } => '6',
            Pattern { top: true, ul: false, ur: true, mid: false, ll: false, lr: true, bot: false, } => '7',
            Pattern { top: true, ul: true, ur: true, mid: true, ll: true, lr: true, bot: true, } => '8',
            Pattern { top: true, ul: true, ur: true, mid: true, ll: false, lr: true, bot: true, } => '9',
            _ => panic!("Unknown pattern {:?}", pattern)
        };
        string.push(n_char);
    }
    string.parse().ok()
}

fn part_b(lines: &[String]) -> AResult<usize> {
    let lines = parse(lines)?;

    let mut acc = 0usize;
    for (digits, number) in lines {
        acc += b_solver(digits, number).expect("a value");
    }

    Ok(acc)
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

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 26);
        Ok(())
    }

    #[test]
    fn test_single_b() -> AResult<()> {
        let lines = vec![
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
                .to_string(),
        ];
        assert_eq!(part_b(lines.as_slice())?, 5353);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 61229);
        Ok(())
    }
}
