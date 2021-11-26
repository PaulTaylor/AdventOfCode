import numpy as np

from .day_11 import prepare_grid, simulate, simulate_b, do_seat_b

floor_strings = ["""L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL""","""#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##""","""#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##""","""#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##""","""#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##""","""#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"""
]

floor_strings_b = ["""L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL""","""#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##""","""#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#""","""#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#""","""#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#""","""#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#""","""#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"""]

def test_simulate():
    for idx, fp_str in enumerate(floor_strings):
        if idx == len(floor_strings) - 1:
            break # last iteration would always error

        fp = prepare_grid(fp_str)
        assert len(fp) == 10
        assert len(fp) == 10

        res = simulate(fp)
        assert np.array_equal(res, prepare_grid(floor_strings[idx + 1]))

    # No do the last one again to make sure it doesn't change - per the spec
    new_res = simulate(res)
    assert np.array_equal(res, new_res)

    assert np.sum(res == "#") == 37

def test_do_seat_b():

    x = prepare_grid(""".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....""")

    print(x)

    assert x[4, 3] == 'L'
    assert do_seat_b(x, 4, 3) == 'L'


def test_simulate_b():
    for idx, fp_str in enumerate(floor_strings_b):
        if idx == len(floor_strings_b) - 1:
            break # last iteration would always error

        fp = prepare_grid(fp_str)
        assert len(fp) == 10
        assert len(fp) == 10

        res = simulate_b(fp)
        assert np.array_equal(res, prepare_grid(floor_strings_b[idx + 1]))

    # No do the last one again to make sure it doesn't change - per the spec
    new_res = simulate_b(res)
    assert np.array_equal(res, new_res)

    assert np.sum(res == "#") == 26
