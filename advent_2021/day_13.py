"""
Advent of Code 2021 - Day 13
See: https://adventofcode.com/2021/day/13
"""

from pathlib import Path

import numpy as np

def parse_lines(input_string):
    dots = []
    folds = []
    for line in input_string.splitlines():
        if line and line[0] == "f":
            words = line.split()
            coord = words[-1].split("=")
            folds.append((coord[0], int(coord[1])))
        elif line:
            x, y = line.split(",")
            dots.append((int(x), int(y)))

    return dots, folds

def create_grid(dots):
    max_x = max(x for x,_ in dots) + 1
    max_y = max(y for _,y in dots) + 1

    grid = np.full((max_y, max_x), 0, dtype=np.short)
    for x, y in dots:
        grid[y, x] = 1

    return grid

def perform_fold(grid, fold):
    if fold[0] == "y":
        top_piece = grid[0:fold[1], :]
        bottom_piece = grid[fold[1]+1:, :]
        bottom_flipped = np.flipud(bottom_piece)
        grid = np.sign(top_piece + bottom_flipped)
    elif fold[0] == "x":
        left_piece = grid[:, 0:fold[1]]
        right_piece = grid[:, fold[1]+1:]
        right_flipped = np.fliplr(right_piece)
        grid = np.sign(left_piece + right_flipped)
    else:
        raise Exception("We cannot go into the third dimension!")

    return grid

def part_a(dots, folds):
    grid = create_grid(dots)
    # Only need the first fold for Part A
    grid = perform_fold(grid, folds[0])
    return np.sum(grid)

def part_b(dots, folds):
    grid = create_grid(dots)
    for fold in folds:
        grid = perform_fold(grid, fold)

    # format the grid for display
    acc = ""
    for row in grid:
        acc += ("".join(map(lambda x: "#" if x else " ", row)))
        acc += "\n"

    return acc.strip()
        
if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_13_a.txt'
    with open(p, "r", encoding="ascii") as f:
        dots, folds = parse_lines(f.read())
        print(f"Answer for a is {part_a(dots, folds)}.")
        print(f"""Answer for b is
==========================
{part_b(dots, folds)}
==========================""")
