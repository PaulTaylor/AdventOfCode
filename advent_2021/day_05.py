"""
Advent of Code 2021 - Day 5
See: https://adventofcode.com/2021/day/5
"""

from pathlib import Path

import numpy as np

def parse_input(input_string):
    lines = input_string.splitlines()
    coords = [ line.split(" -> ") for line in lines ]
    segments = np.array(
        [ c1.split(",") + c2.split(",") for c1, c2 in coords],
        dtype=int
    )
    return segments, np.max(segments) + 1

def plot_lines(all_segments, grid_size, include_diagonal=False):
    grid = np.zeros((grid_size, grid_size))

    for line in all_segments:
        x1, y1, x2, y2 = line

        if x1 == x2 or y1 == y2:
            # Put co-ords in correct min->max order as numpy
            # doesn't support backwards slices in this way
            x1 = min(line[0], line[2])
            x2 = max(line[0], line[2])
            y1 = min(line[1], line[3])
            y2 = max(line[1], line[3])

            # Check if the line is horizontal/vertical and increment
            # grid appropriately
            grid[x1:x2+1, y1:y2+1] += 1
        elif include_diagonal:
            # As we can only have 45 degree lines - at each step well just
            # increment x1 and y1 by 1 until getting to x2,y2
            x_step = 1 if x2 >= x1 else -1
            y_step = 1 if y2 >= y1 else -1

            x_values = range(x1, x2 + x_step, x_step)
            y_values = range(y1, y2 + y_step, y_step)

            for x, y in zip(x_values, y_values):
                grid[x, y] += 1

    return np.sum(grid > 1)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_05_a.txt'
    with open(p, "r", encoding="ascii") as f:
        line_segments, size = parse_input(f.read())
        print(f"Answer for a is {plot_lines(line_segments, size)}.")
        print(f"Answer for b is {plot_lines(line_segments, size, include_diagonal=True)}.")
