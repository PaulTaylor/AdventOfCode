import pytest

from .day_05 import *

@pytest.fixture
def puzzle_input():
    return """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"""

def test_plot_lines(puzzle_input):
    vent_coords, size = parse_input(puzzle_input)
    assert len(vent_coords) == 10
    assert size == 10
    
    res = plot_lines(vent_coords, size)
    assert res == 5
    
    res = plot_lines(vent_coords, size, include_diagonal=True)
    assert res == 12