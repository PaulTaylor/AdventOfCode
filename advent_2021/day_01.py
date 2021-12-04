"""
Advent of Code 2021 - Day 1
See: https://adventofcode.com/2021/day/1
"""

from collections import deque
from pathlib import Path

def part_a(depths):
    acc = 0
    prev = depths[0]
    for val in depths[1:]:
        if val > prev:
            acc += 1

        prev = val

    return acc

def part_b(depths):
    acc = 0
    buf = deque(depths[0:3], maxlen=3)
    prev_sum = sum(buf)
    for val in depths[3:]:
        buf.append(val)
        window_sum = sum(buf)
        if window_sum > prev_sum:
            acc += 1

        prev_sum = window_sum

    return acc

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_01_a.txt'
    with open(p, "r") as f:
        values = [ int(x) for x in f.readlines() ]
        print(f"Answer for a is {part_a(values)}.")
        print(f"Answer for b is {part_b(values)}.")