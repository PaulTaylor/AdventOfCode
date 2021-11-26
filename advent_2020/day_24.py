"""
Going to use a hex grid with cube co-ordinates here.
Reference: https://www.redblobgames.com/grids/hexagons/
co-ordinates are (x,y,z)
Each direction on the hex grid is a combination of two directions on the cube grid:
- east = +x -y
-   se = -y +z
-   sw = -x +z
- west = -x +y
-   nw = +y -z
-   ne = +x -z

x + y + z must always be == 0
"""

import re

from collections import deque
from pathlib import Path
from tqdm import tqdm

moves = {
     'e': lambda x, y, z: (x+1, y-1, z),
    'se': lambda x, y, z: (x, y-1, z+1),
    'sw': lambda x, y, z: (x-1, y, z+1),
     'w': lambda x, y, z: (x-1, y+1, z),
    'nw': lambda x, y, z: (x, y+1, z-1),
    'ne': lambda x, y, z: (x+1, y, z-1)
}

def parse_input(raw):
    lines = raw.splitlines()
    return [ re.findall(r'e|se|sw|w|nw|ne', line) for line in lines ]

def do_part_a(tile_steps):
    # Create the floor
    # - indexed by co-ordinate tuple - True if black
    floor = {}

    for tile_step in tile_steps:
        tile_loc = (0, 0, 0)
        for step in tile_step:
            tile_loc = moves[step](*tile_loc)

        if floor.get(tile_loc):
            del floor[tile_loc]
        else:
            floor[tile_loc] = True

    return floor, sum(floor.values())

def do_one_day(floor):
    new_floor = floor.copy()

    # Need to generate a list of all tiles on the floor
    # 1) Calculate the bounds of the occupied floor
    min_x, max_x, min_y, max_y, min_z, max_z = [0] * 6
    for x, y, z in floor.keys():
        min_x = min(min_x, x)
        max_x = max(max_x, x)
        min_y = min(min_y, y)
        max_y = max(max_y, y)
        min_z = min(min_z, z)
        max_z = max(max_z, z)

    # 2) Generate all co-ordinates within those bounds
    to_check = deque()
    for x in range(min_x-1, max_x + 2):
        for y in range(min_y-1, max_y + 2):
            for z in range(min_z-1, max_z + 2):
                if x + y + z == 0:
                    to_check.append((x, y, z))

    # 3) Perform the update algorithm
    for loc in to_check:
        adj_blacks = 0
        for delta in moves.values():
            adj_loc = delta(*loc)
            if floor.get(adj_loc):
                adj_blacks += 1

        if floor.get(loc):
            # This loc is black
            if adj_blacks == 0 or adj_blacks > 2:
                # Any black tile with zero or more than 2 black tiles
                # immediately adjacent to it is flipped to white.
                # white is analagous to not being on the floor
                del new_floor[loc]
        else:
            # This loc is white
            if adj_blacks == 2:
                # Any white tile with exactly 2 black tiles immediately
                # adjacent to it is flipped to black.
                new_floor[loc] = True

    # Return the new floor and number of black tiles
    return new_floor, sum(new_floor.values())

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_24_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        raw = f.read()
    tile_steps = parse_input(raw)

    # Part A
    floor, num_black = do_part_a(tile_steps)
    print(num_black)

    # Part B
    for excl_day in tqdm(range(100)):
        floor, num_black = do_one_day(floor)

    print(num_black)
