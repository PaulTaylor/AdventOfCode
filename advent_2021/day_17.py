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

def run_simulation(target_location, launch_vx, launch_vy) -> bool:
    target_min_x, target_max_x, target_min_y, target_max_y = target_location
    probe_x = 0
    probe_y = 0
    vx = launch_vx
    vy = launch_vy
    max_height = 0

    while probe_x <= target_max_x and probe_y >= target_min_y:
        probe_x += vx
        probe_y += vy
        vx -= sign(vx)
        vy -= 1

        if probe_y > max_height:
            max_height = probe_y

        if probe_x in range(target_min_x, target_max_x + 1) \
            and probe_y in range(target_min_y, target_max_y + 1):
            # we're there!
            return True, max_height

    return False, None

def part_ab(target_location):
    _, target_max_x, target_min_y, _ = target_location
    # Just an exhaustive search
    current_best_height = 0

    # Constraints:
    # vx must be high enough to reach the target before the y drops below the range
    # vx must be low enough not to overshoot before y drops below the range

    solutions = set()
    for launch_vx in range(0, target_max_x + 1):
        for launch_vy in range(target_min_y, abs(target_min_y) + 1):
            hit, max_height = run_simulation(target_location, launch_vx, launch_vy)
            if hit:
                solutions |= { (launch_vx, launch_vy) }
                if max_height > current_best_height:
                    current_best_height = max_height

    return current_best_height, len(solutions)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_17_a.txt'
    with open(p, "r", encoding="ascii") as f:
        instructions = parse_input(f.read())

    max_height, num_solutions = part_ab(instructions)
    print(f"Answer for a is {max_height}.")
    print(f"Answer for b is {num_solutions}.")
