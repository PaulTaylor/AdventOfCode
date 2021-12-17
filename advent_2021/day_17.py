"""
Advent of Code 2021 - Day 17
See: https://adventofcode.com/2021/day/17
"""

import re

from pathlib import Path

def parse_input(input_string):
    m = re.match(
        r"target area: x=([-0-9]+)..([-0-9]+), y=([-0-9]+)..([-0-9]+)",
        input_string)
    return tuple( int(x) for x in m.groups() )

def sign(x):
    if x < 0:
        return -1
    elif x > 0:
        return 1
    else:
        return 0

def run_simulation(
    target_min_x, target_max_x, target_min_y, target_max_y,
    launch_vx, launch_vy
):
    probe_x = 0
    probe_y = 0
    vx = launch_vx
    vy = launch_vy
    max_height = 0

    while probe_x <= target_max_x and probe_y >= target_min_y:
        if (probe_x >= target_min_x) and (probe_y <= target_max_y):
            return True, max_height # hit

        probe_x += vx
        probe_y += vy
        vx -= sign(vx)
        vy -= 1
        max_height = max(max_height, probe_y)

    return False, None

def part_ab(target_min_x, target_max_x, target_min_y, target_max_y):
    # Constraints:
    # vx must be high enough to reach the target before y drops below the range
    # vx must be low enough not to overshoot before y drops below the range
    y_max = 0
    solutions = 0
    for vx in range(0, target_max_x + 1):
        for vy in range(target_min_y, abs(target_min_y) + 1):
            hit, sim_y_max = run_simulation(
                target_min_x, target_max_x, target_min_y, target_max_y,
                vx, vy)
            if hit:
                solutions += 1
                y_max = max(y_max, sim_y_max)

    return y_max, solutions

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_17_a.txt'
    with open(p, "r", encoding="ascii") as f:
        instructions = parse_input(f.read())

    max_height, num_solutions = part_ab(*instructions)
    print(f"Answer for a is {max_height}.")
    print(f"Answer for b is {num_solutions}.")
