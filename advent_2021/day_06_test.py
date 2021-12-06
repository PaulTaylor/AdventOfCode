import pytest

from .day_06 import count_fish, parse_input

@pytest.fixture
def puzzle_input():
    return "3,4,3,1,2"

def test_count_fish(puzzle_input):
    fish = parse_input(puzzle_input)
    assert count_fish(fish, 18) == 26
    assert count_fish(fish, 80) == 5934
    assert count_fish(fish, 256) == 26984457539