"""
Advent of Code 2021 - Day 14
See: https://adventofcode.com/2021/day/14
"""

from collections import Counter
from pathlib import Path

def parse_input(input_string):
    lines = input_string.splitlines()
    template = lines[0]

    # Grab rules and turn into a lookup dict
    rules = dict(x.split(" -> ") for x in lines[2:])
    for k in rules:
        rules[k] = k[0] + rules[k] + k[1]

    return template, rules

def part_a(template, rules):
    "Naive implementation for part a"
    current = str(template)
    for _ in range(10):
        acc = ""
        pairs = map("".join, zip(current, current[1:]))
        for pair in pairs:
            acc += rules[pair][:-1]

        # need to wrap-up by adding the final character manually
        current = acc + current[-1]

    # Now need to count the characters in the string
    counter = Counter(current)
    return max(counter.values()) - min(counter.values())


def part_b(template, rules, steps=40):
    # Use counters so we only store the pairs and the frequency
    # rather than every copy - for both speed and memory reasons
    prev_level_counter = Counter("".join(x) for x in zip(template, template[1:]))

    # for each step we need to execute iterate over the pairs from the
    # previous level, calculate the 2 new pairs that result from the
    # rules and seed a counter with those new pairs
    for _ in range(steps):
        level_counter = Counter()
        for pair, freq in prev_level_counter.items():
            res = rules[pair]
            level_counter[res[1:]] += freq
            level_counter[res[:-1]] += freq

        # Update for next iteration
        prev_level_counter = level_counter

    # Summarise the results - we only need to count the first
    # character for each pair because the second will be counted
    # as the first part of another pair (with the only exception
    # being the final character which is ...
    counter = Counter()
    for pair, freq in prev_level_counter.items():
        counter[pair[0]] += freq

    # ... added manually)
    counter[template[-1]] += 1

    # Calculate the range of values for the result
    return max(counter.values()) - min(counter.values())


if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_14_a.txt'
    with open(p, "r", encoding="ascii") as f:
        template, rules = parse_input(f.read())
        print(f"Answer for a is {part_a(template, rules)}.")
        print(f"Answer for b is {part_b(template, rules)}.")
