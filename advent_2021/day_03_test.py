import pytest

from .day_03 import *

@pytest.fixture
def puzzle_input():
    return to_array("""00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010""")

def test_part_a(puzzle_input):
    res = part_a(puzzle_input)
    assert res == 198

def test_part_b(puzzle_input):
    res = part_b(puzzle_input)
    assert res == 230