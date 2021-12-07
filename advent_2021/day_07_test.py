import pytest

from .day_07 import *

@pytest.fixture
def puzzle_input():
    return parse_input("16,1,2,0,4,2,7,1,2,14")

def test_part_a(puzzle_input):
    pos, cost = align_crabs(puzzle_input)
    assert pos == 2
    assert cost == 37

def test_part_b(puzzle_input):
    pos, cost = align_crabs_extra_cost(puzzle_input)
    assert pos == 5
    assert cost == 168
