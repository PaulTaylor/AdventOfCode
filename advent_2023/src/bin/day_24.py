#
# Using Python for Day 24 as the z3 interface is *much* better
#

import itertools
from pathlib import Path
from multiprocessing import Pool
from z3 import Solver, Int, Real, sat

def parse_input(instr):
    out = []
    for line in instr.splitlines():
        a, b = line.split("@")
        out.append(([float(x) for x in a.split(", ")], [float(x) for x in b.split(", ")]))
    return out

def solve_one_xy(a, b):
    [[ax,ay,az], [avx, avy, avz]] = a
    [[bx,by,bz], [bvx, bvy, bvz]] = b
    
    solver = Solver()

    ta = Real('ta')
    tb = Real('tb')
    x = Real('x')
    y = Real('y')

    solver.add(
        x == (avx * ta) + ax,
        x == (bvx * tb) + bx,
        y == (avy * ta) + ay,
        y == (bvy * tb) + by,
        ta >= 0,
        tb >= 0,
        x >= 200000000000000, x <= 400000000000000,
        y >= 200000000000000, y <= 400000000000000,
    )
    
    if solver.check() == sat:
        return 1;
    else:
        return 0;

def part_a(snow):
    with Pool() as pool:
        return sum(pool.starmap(solve_one_xy, itertools.combinations(snow, r=2)))

def part_b(snow):
    solver = Solver()

    rx, ry, rz, rvx, rvy, rvz = Int('rx'), Int('ry'), Int('rz'), Int('rvx'), Int('rvy'), Int('rvz')

    for idx, s in enumerate(snow):
        [[ax,ay,az], [avx, avy, avz]] = s

        t = Int(f"t{idx}")
        solver.add(
            (rvx * t) + rx == (avx * t) + ax, # X-coords match
            (rvy * t) + ry == (avy * t) + ay, # Y-coords match
            (rvz * t) + rz == (avz * t) + az, # Z-coords match
        )

    assert solver.check() == sat
    model = solver.model()
    return model[rx].as_long() + model[ry].as_long() + model[rz].as_long()

if __name__ == "__main__": 
    p = Path(__file__).parent/ "../.." / "data" / 'day_24.txt'
    with open(p, "r", encoding="ascii") as f:
        snow = parse_input(f.read())

    # print(f"Answer for a is {part_a(snow)}.")
    print(f"Answer for b is {part_b(snow)}.")