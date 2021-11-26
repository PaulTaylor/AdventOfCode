from .day_03 import check_slope

def test_a():
    slope = """..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#""".splitlines()

    n_trees = check_slope(slope, right=3, down=1)
    assert n_trees == 7
