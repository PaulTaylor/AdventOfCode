from pathlib import Path

import pytest

from .day_04 import parse_input, part_a, part_b

@pytest.fixture
def puzzle_input():
    p = Path(__file__).parent / "input" / 'day_04_test.txt'
    with open(p, "r", encoding="ascii") as f:
        return f.read()

def test_part_a(puzzle_input):
    numbers, boards = parse_input(puzzle_input)
    assert len(numbers) == 27
    assert len(boards) == 3

    res = part_a(numbers, boards)
    assert res == 4512

def test_part_b(puzzle_input):
    numbers, boards = parse_input(puzzle_input)
    res = part_b(numbers, boards)
    assert res == 1924
