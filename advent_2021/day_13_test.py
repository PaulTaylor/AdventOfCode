import pytest
import numpy as np

from .day_13 import create_grid, parse_lines, part_a, part_b, perform_fold

@pytest.fixture
def puzzle_input():
    return parse_lines("""6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5""")

def test_part_a(puzzle_input):
    dots, folds = puzzle_input
    assert part_a(dots, folds) == 17

def test_perform_fold(puzzle_input):
    dots, folds = puzzle_input
    grid = create_grid(dots)
    grid = perform_fold(grid, folds[0])
    assert np.sum(grid) == 17
    grid = perform_fold(grid, folds[1])
    assert np.sum(grid) == 16

def test_part_b(puzzle_input):
    dots, folds = puzzle_input
    res = part_b(dots, folds)
    assert res.strip() == """#####
#   #
#   #
#   #
#####"""