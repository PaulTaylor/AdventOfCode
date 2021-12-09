import pytest

from advent_2021.day_09 import parse_input

from .day_09 import *

@pytest.fixture
def puzzle_input():
    return """2199943210
3987894921
9856789892
8767896789
9899965678"""

def test_part_a(puzzle_input):
    assert part_a(parse_input(puzzle_input))[0] == 15

def test_part_b(puzzle_input):
    _, res = part_a(parse_input(puzzle_input))
    assert part_b(parse_input(puzzle_input), res) == 1134
