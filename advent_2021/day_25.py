"""
Advent of Code 2021 - Day 25
See: https://adventofcode.com/2021/day/25
"""

from pathlib import Path

import numpy as np
from numba import njit

def parse_input(instr):
    lkp = { '>': 1, 'v': -1, '.': 0}
    return np.array([
        [ lkp[c] for c in line ]
        for line in instr.splitlines()
    ], dtype=np.int8)


@njit
def part_a(grid, max_rounds=-1):
    height, width = grid.shape
    temp = np.zeros_like(grid)

    rounds = 0
    while (max_rounds < 0) or (rounds < max_rounds):
        rounds += 1
        moved = False
        temp[:,:] = 0

        for ir in range(height):
            for ic in range(width):
                if grid[ir, ic] == 1:
                    if grid[ir, (ic + 1)%width] == 0:
                        temp[ir, (ic + 1)%width] = 1
                        moved=True
                    else:
                        temp[ir, ic] = 1
                elif grid[ir, ic] == -1:
                    temp[ir, ic] = -1

        grid[:,:] = 0
        for ir in range(height):
            for ic in range(width):
                if temp[ir, ic] == -1:
                    if temp[(ir+1)%height, ic] == 0:
                        grid[(ir+1)%height, ic] = -1
                        moved = True
                    else:
                        grid[ir, ic] = -1
                elif temp[ir, ic] == 1:
                    grid[ir, ic] = 1

        if not moved:
            return rounds

    if max_rounds:
        return rounds
    else:
        raise Exception("movement did not converge")


if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_25_a.txt'
    with open(p, "r", encoding="ascii") as f:
        grid = parse_input(f.read())

    print(f"Answer for a is {part_a(grid)}.")
