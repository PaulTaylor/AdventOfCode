"""
Advent of Code 2021 - Day 6
See: https://adventofcode.com/2021/day/6
"""

from collections import Counter
from pathlib import Path

def parse_input(input_string):
    fish = [ int(x) for x in input_string.split(",") ]
    return fish

def count_fish(fish, days):
    prev_counter = Counter(fish)
    for _ in range(days):
        new_counter = Counter()

        # For 0 day fish - reset to 6 and spawn new fish
        new_counter[6] = prev_counter[0]
        new_counter[8] = prev_counter[0]

        for age in range(1, 9):
            new_counter[age - 1] += prev_counter[age]

        prev_counter = new_counter

    return sum(prev_counter.values())

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_06_a.txt'
    with open(p, "r", encoding="ascii") as f:
        input_fish = parse_input(f.read())

    print(f"Answer for a is {count_fish(input_fish, 80)}.")
    print(f"Answer for b is {count_fish(input_fish, 256)}.")
