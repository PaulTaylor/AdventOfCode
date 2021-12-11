import pytest

from .day_11 import parse_input, part_a, part_b

@pytest.fixture
def puzzle_input():
    return """5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"""

def test_part_a(puzzle_input):
    grid = parse_input(puzzle_input)
    assert(part_a(grid, steps=10)) == 204

    grid = parse_input(puzzle_input)
    assert(part_a(grid, steps=100)) == 1656

def test_part_b(puzzle_input):
    grid = parse_input(puzzle_input)
    assert(part_b(grid)) == 195
