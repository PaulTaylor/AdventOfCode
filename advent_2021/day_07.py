"""
Advent of Code 2021 - Day 7
See: https://adventofcode.com/2021/day/7
"""

from pathlib import Path

def parse_input(input_string):
    return list(map(int, input_string.split(",")))

def sequence_sum(a):
    "returns the sum of the numbers 0->a (inclusive)"
    return ((a + 1) * a)/2

def align_crabs(initial_positions):
    min_cost = None
    min_pos  = None
    for pos in range(max(initial_positions)):
        # What will it cost to move all crabs to this position:
        cost = sum(abs(c - pos) for c in initial_positions)
        if not min_cost or (cost < min_cost):
            min_cost = cost
            min_pos = pos

    return min_pos, min_cost

def align_crabs_extra_cost(initial_positions):
    min_cost = None
    min_pos  = None
    for pos in range(max(initial_positions)):
        # What will it cost to move all crabs to this position:
        cost = sum(sequence_sum(abs(c - pos)) for c in initial_positions)
        if not min_cost or (cost < min_cost):
            min_cost = cost
            min_pos = pos

    return min_pos, int(min_cost)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_07_a.txt'
    with open(p, "r", encoding="ascii") as f:
        crab_positions = parse_input(f.read())
        print(f"Answer for a is {align_crabs(crab_positions)[1]}.")
        print(f"Answer for b is {align_crabs_extra_cost(crab_positions)[1]}.")
