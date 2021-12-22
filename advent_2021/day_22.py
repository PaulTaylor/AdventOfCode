"""
Advent of Code 2021 - Day 22
See: https://adventofcode.com/2021/day/22
"""
import re

from collections import deque, defaultdict, namedtuple
from pathlib import Path
from typing import Iterable, List

INPUT_PATTERN = re.compile(
    "(on|off) x=(-?[0-9]+)..(-?[0-9]+),y=(-?[0-9]+)..(-?[0-9]+),z=(-?[0-9]+)..(-?[0-9]+)")

Bounds = namedtuple('Bounds', 'min_x, max_x, min_y, max_y min_z max_z')
RebootStep = namedtuple('RebootStep', 'action bounds')

def parse_input(input_string: str) -> List[RebootStep]:
    out = []
    for line in input_string.splitlines():
        groups = INPUT_PATTERN.match(line).groups()
        out.append(RebootStep(groups[0], Bounds(*[ int(x) for x in groups[1:] ])))
    return out

def naive_solver(steps):
    "Naive implementation for limited reactor size"
    reactor = defaultdict(lambda: 0) # default reactor state is off
    for step in steps:
        for x in range(step.bounds.min_x, step.bounds.max_x+1):
            for y in range(step.bounds.min_y, step.bounds.max_y+1):
                for z in range(step.bounds.min_z, step.bounds.max_z+1):
                    reactor[(x,y,z)] = 1 if step.action == "on" else 0

    return sum(reactor.values())

def volume(b: Bounds):
    "Calculate the volume enclosed by a set of bounds"
    return ((b.max_x - b.min_x)+1) * ((b.max_y - b.min_y)+1) * ((b.max_z - b.min_z)+1)

def process_interactions(existing: Bounds, incoming: Bounds):
    """
    Resolve the interactions between the two sets of bounds
    by returning a set of bounds representing the remaining space
    in existing after incoming is removed
    (can be between 0 and 6 volumes)
    """

    # Quick check to see if there's no intersection
    if incoming.min_x > existing.max_x or \
       incoming.min_y > existing.max_y or \
       incoming.min_z > existing.max_z or \
       incoming.max_x < existing.min_x or \
       incoming.max_y < existing.min_y or \
       incoming.max_z < existing.min_z:
        return [existing]


    # for remaining space and output
    min_x = existing.min_x
    res = []

    if existing.min_x < incoming.min_x:
        res.append(existing._replace(max_x=incoming.min_x-1))
        min_x = incoming.min_x

    if existing.max_x > incoming.max_x:
        res.append(existing._replace(min_x=incoming.max_x+1))

    if existing.max_y > incoming.max_y:
        res.append(existing._replace(
            min_x=min_x,
            max_x=min(incoming.max_x, existing.max_x),
            min_y=incoming.max_y+1,
        ))

    if existing.min_y < incoming.min_y:
        res.append(existing._replace(
            min_x=min_x,
            max_x=min(incoming.max_x, existing.max_x),
            max_y=incoming.min_y-1
        ))

    if existing.min_z < incoming.min_z:
        res.append(Bounds(
            min_x=min_x,
            max_x=min(incoming.max_x, existing.max_x),
            min_y=max(incoming.min_y, existing.min_y),
            max_y=min(incoming.max_y, existing.max_y),
            min_z=existing.min_z,
            max_z=incoming.min_z-1
        ))

    if existing.max_z > incoming.max_z:
        res.append(Bounds(
            min_x=min_x,
            max_x=min(existing.max_x, incoming.max_x),
            min_y=max(incoming.min_y, existing.min_y),
            max_y=min(incoming.max_y, existing.max_y),
            min_z=incoming.max_z+1,
            max_z=existing.max_z
        ))

    return res

def faster_solver(steps: Iterable[RebootStep], limit_range=False) -> int:
    "Solve by holding references only to lit volumes and summing subsequently"

    previously_lit = deque()
    for step in steps:
        # Limiter for Part A - initial naive implementation is above
        if limit_range and any((v < -50) or (v > 50) for v in step.bounds):
            continue

        # Volumes that are still lit at the end of the round
        now_lit = deque()

        # Check each existing cube in turn to see if there's an overlap
        # with this new one that needs to be handled.
        while previously_lit:
            previous_cube = previously_lit.popleft()
            new_cubes = process_interactions(previous_cube, step.bounds)
            now_lit.extend(new_cubes)

        # Add the newly lit cube if appropriate
        if step.action == "on":
            now_lit.append(step.bounds)
        # And update for next iteration
        previously_lit = now_lit

    # Now sum the volume for all of the lit cubes
    return sum(map(volume, now_lit))

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_22_a.txt'
    with open(p, "r", encoding="ascii") as f:
        instructions = parse_input(f.read())

    print(f"Answer for a is {faster_solver(instructions, limit_range=True)}.")
    print(f"Answer for b is {faster_solver(instructions)}.")
