"""
Advent of Code 2021 - Day 9
See: https://adventofcode.com/2021/day/9
"""

import math
from pathlib import Path

import numpy as np

def parse_input(input_string):
    return np.array([ list(x) for x in input_string.splitlines() ], dtype=int)

def part_a(input_array):
    low_point_values = []
    low_points = []
    for y in range(input_array.shape[0]):
        for x in range(input_array.shape[1]):
            is_lowest = True
            # Check up
            if y > 0:
                is_lowest = is_lowest and input_array[y,x] < input_array[y-1, x]
            # Check right
            if x <= input_array.shape[1] - 2:
                is_lowest = is_lowest and input_array[y,x] < input_array[y, x+1]
            # Check down
            if y <= input_array.shape[0] - 2:
                is_lowest = is_lowest and input_array[y,x] < input_array[y+1, x]
            # Check left
            if x > 0:
                is_lowest = is_lowest and input_array[y,x] < input_array[y, x-1]

            if is_lowest:
                low_point_values.append(input_array[y,x])
                low_points.append((y, x))

    return sum(map(lambda x: x + 1, low_point_values)), low_points

def part_b(a, low_points):
    # nb. points are lists of (y,x) matching the array a
    basins = []
    for lp in low_points:
        points_in_basin = { lp }
        start_size = 0

        while len(points_in_basin) > start_size:
            # while the basin is still growing - try expanding more
            start_size = len(points_in_basin)

            new_points = set()
            for y, x in points_in_basin:
                # Check up
                if y > 0 and a[y-1,x] < 9:
                    new_points.add((y-1, x))
                # Check right
                if x <= a.shape[1] - 2 and a[y,x+1] < 9:
                    new_points.add((y, x+1))
                # Check down
                if y <= a.shape[0] - 2 and a[y+1, x] < 9:
                    new_points.add((y+1, x))
                # Check left
                if x > 0 and a[y, x-1] < 9:
                    new_points.add((y, x-1))

            points_in_basin |= new_points

        basins.append(points_in_basin)

    # NB. Not checking to see if any new points are already in basins this
    # turns out not to matter in this challenge but if necessary we can
    # filter using set difference on the set of already allocated points

    # multiply the sizes of the 3 largest basins
    basin_sizes = sorted([ len(x) for x in basins ], key=lambda x: -x)
    return math.prod(basin_sizes[0:3])

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_09_a.txt'
    with open(p, "r", encoding="ascii") as f:
        input_array = parse_input(f.read())
        a_res, low_points = part_a(input_array)
        print(f"Answer for a is {a_res}.")
        print(f"Answer for b is {part_b(input_array, low_points)}.")
