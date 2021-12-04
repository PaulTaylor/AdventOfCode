"""
Advent of Code 2021 - Day 3
See: https://adventofcode.com/2021/day/3
"""

import numpy as np

from pathlib import Path


def to_array(input_string):
    "Convert input to numpy array of numbers"
    return np.array([
        list(s) for s in input_string.splitlines()
    ], dtype=np.short)

def part_a(input_array):
    # loop over the columns
    # gamma is most common bit
    # epsilon is the least
    gamma_bits = ""
    epsilon_bits = ""
    for col_sum in np.sum(input_array, axis=0):
        if col_sum > input_array.shape[0] / 2:
            gamma_bits += "0"
            epsilon_bits += "1"
        elif col_sum < input_array.shape[0] / 2:
            gamma_bits += "1"
            epsilon_bits += "0"
        else:
            raise Exception("WTF!")

    gamma = int(gamma_bits, 2)
    epsilon = int(epsilon_bits, 2)

    return gamma * epsilon

def part_b(input_array):
    # Look for Oxgen Rating
    remaining = input_array
    for bit_position in range(input_array.shape[1]):
        bits = remaining[:, bit_position]
        criteria = 1 if np.sum(bits) >= len(bits) / 2 else 0
        indices = np.argwhere(bits == criteria)[:, 0]
        remaining = remaining[indices, :]

        if len(indices) == 1:
            break

    oxygen_rating = int("".join(map(str, remaining[0])), 2)

    # Repeat with tweaked criteria for Scrubber Rating
    remaining = input_array
    for bit_position in range(input_array.shape[1]):
        bits = remaining[:, bit_position]
        criteria = 1 if np.sum(bits) < len(bits) / 2 else 0
        indices = np.argwhere(bits == criteria)[:, 0]
        remaining = remaining[indices, :]

        if len(indices) == 1:
            break

    scrubber_rating = int("".join(map(str, remaining[0])), 2)
    
    return oxygen_rating * scrubber_rating

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_03_a.txt'
    with open(p, "r", encoding="ascii") as f:
        puzzle_input = to_array(f.read())
        print(f"Answer for a is {part_a(puzzle_input)}.")
        print(f"Answer for b is {part_b(puzzle_input)}.")
