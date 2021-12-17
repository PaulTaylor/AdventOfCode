import pytest

from .day_17 import part_ab, parse_input

@pytest.fixture
def puzzle_input():
    return parse_input("target area: x=20..30, y=-10..-5")

def test_part_ab(puzzle_input):
    assert part_ab(puzzle_input) == (45, 112)
