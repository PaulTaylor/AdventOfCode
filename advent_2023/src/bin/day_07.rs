use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::{Ordering, Reverse},
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

//
// Part specific card orderings
//
const CARD_ORDER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARD_ORDER_B: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

//
// Define HandType
//

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    // Need this because Rust doesn't have Sorted Enums
    fn discriminant(self) -> u8 {
        match self {
            HandType::FiveKind => 7,
            HandType::FourKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl From<[char; 5]> for HandType {
    fn from(cards: [char; 5]) -> Self {
        let mut card_counts: HashMap<char, usize> = HashMap::new();
        for c in cards {
            card_counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut card_counts: Vec<_> = card_counts.iter().collect();
        card_counts.sort_by_key(|cc| Reverse((cc.1, cc.0)));

        match card_counts.as_slice() {
            [(_, 5)] => HandType::FiveKind,
            [(_, 4), _] => HandType::FourKind,
            [(_, 3), (_, 2)] => HandType::FullHouse,
            [(_, 3), (_, 1), (_, 1)] => HandType::ThreeKind,
            [(_, 2), (_, 2), (_, 1)] => HandType::TwoPair,
            [(_, 2), (_, 1), (_, 1), (_, 1)] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Delegate to the custom discriminant
        self.discriminant().cmp(&other.discriminant())
    }
}

//
// Define a Hand
//

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: usize,
    h_type: HandType,
}

impl Hand {
    fn best_hand_type(&self) -> HandType {
        // Work out the best hand type for this hand given the part-b joker rule
        let num_js = self.cards.iter().filter(|&&c| c == 'J').count();

        // If there is at least on Joker, but not all the cards are jokers...
        if num_js > 0 && num_js < 5 {
            // Generate possible joker replacements - but only those that might improve the hand
            // we do this my only considering joker substitutions with cards that are already in
            // the hand
            let mut candidates = vec![];

            let mut to_expand: VecDeque<_> = VecDeque::new();
            to_expand.push_front(self.cards);

            while !to_expand.is_empty() {
                let source: [char; 5] = to_expand.pop_front().unwrap();
                let unique: HashSet<_> = source.iter().filter(|&c| c != &'J').collect();

                let first_j = source.iter().position(|c| c == &'J');
                match first_j {
                    Some(idx) => {
                        let mut new_cards = source;
                        for new_card in unique {
                            new_cards[idx] = *new_card;
                            to_expand.push_back(new_cards);
                        }
                    }
                    None => candidates.push(HandType::from(source)),
                }
            }

            // Now we've generated all the candidates - return the best hand type
            *candidates
                .iter()
                .max()
                .unwrap_or_else(|| panic!("no best type for card {self:?}"))
        } else {
            // 0 or 5 jokers - the hand will not change from it's current type
            self.h_type
        }
    }
}

impl From<&String> for Hand {
    fn from(value: &String) -> Self {
        let mut bits = value.split_whitespace();
        let cards: [char; 5] = bits
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let bid = bits.next().unwrap().parse().unwrap();

        let h_type = HandType::from(cards);

        Hand { cards, bid, h_type }
    }
}

//
// Comparator Functions
//

fn cmp_a(this: &Hand, other: &Hand) -> Ordering {
    // Separate comparator function for part_a
    // IMPORTANT: This is an ascending order sort (ie. best hand last)

    let order = this.h_type.cmp(&other.h_type);
    if order == Ordering::Equal {
        return this
            .cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(x, y)| {
                let xi = CARD_ORDER.iter().rev().position(|c| c == x).unwrap();
                let yi = CARD_ORDER.iter().rev().position(|c| c == y).unwrap();
                if xi == yi {
                    None
                } else {
                    Some(xi.cmp(&yi))
                }
            })
            .unwrap();
    }
    order
}

fn cmp_b(this: &Hand, other: &Hand) -> Ordering {
    // Separate comparator function for part_b
    // IMPORTANT: This is an ascending order sort (ie. best hand last)

    let order = this.best_hand_type().cmp(&other.best_hand_type());
    if order == Ordering::Equal {
        return this
            .cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(x, y)| {
                let xi = CARD_ORDER_B.iter().rev().position(|c| c == x).unwrap();
                let yi = CARD_ORDER_B.iter().rev().position(|c| c == y).unwrap();
                if xi == yi {
                    None
                } else {
                    Some(xi.cmp(&yi))
                }
            })
            .unwrap();
    }
    order
}

fn parse(lines: &[String]) -> Vec<Hand> {
    lines.iter().map(Hand::from).collect()
}

fn part_a(lines: &[String]) -> usize {
    let mut hands = parse(lines);
    hands.sort_by(cmp_a);

    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| h.bid * (rank + 1))
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    let mut hands = parse(lines);
    hands.sort_by(cmp_b);

    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| h.bid * (rank + 1))
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
    println!("Part A result = {}", part_a(lines.as_slice()));
    println!("Part B result = {}", part_b(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 6440);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 5905);
    }
}
