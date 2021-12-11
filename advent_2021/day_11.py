"""
Advent of Code 2021 - Day 11
See: https://adventofcode.com/2021/day/11
"""

from pathlib import Path
from itertools import count

import numpy as np

def parse_input(input_string):
    return np.array([
        [int(x) for x in line ]
        for line in input_string.splitlines()
    ], dtype=float) # use floats as we use nan later

def do_round(grid):
    # First, the energy level of each octopus increases by 1.
    grid += 1

    # any octopus with an energy level greater than 9 flashes
    # (loop because flashes can cause other flashes and so on)
    while np.any(grid > 9):
        flashers = np.argwhere(grid > 9)
        for flash_row, flash_col in flashers:
            impacted_slice = grid[
                max(flash_row - 1, 0):min(flash_row+2, grid.shape[0]),
                max(flash_col - 1, 0):min(flash_col+2, grid.shape[1])
            ]
            impacted_slice += 1
            # anything that has flashed shouldn't take any further part in
            # the round - will backfill nan's with zeros at the end
            grid[flash_row, flash_col] = np.nan

    # any octopus that flashed during this step (will have a nan value)
    # has its energy level set to 0.  Also add these flashes to the total
    flash_count = np.sum(np.isnan(grid))
    grid = np.nan_to_num(grid, copy=False)

    return flash_count

def part_a(grid, steps=100):
    total_flashes = 0
    for _ in range(steps):
        flash_count = do_round(grid)
        total_flashes += flash_count
    return total_flashes

def part_b(grid):
    for step in count(start=1, step=1):
        _ = do_round(grid)
        if np.all(grid == 0):
            return step

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_11_a.txt'
    with open(p, "r", encoding="ascii") as f:
        grid = parse_input(f.read())

    print(f"Answer for a is {part_a(grid)}.")

    # Add 100 to the result to account for the 100 rounds
    # performed during the call to part_a
    print(f"Answer for b is {part_b(grid) + 100}.")
