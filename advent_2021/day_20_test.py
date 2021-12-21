from pathlib import Path

import pytest
from .day_20 import parse_input, process_image, part_ab

@pytest.fixture
def puzzle_input():
    p = Path(__file__).parent / "input" / 'day_20_test.txt'
    with open(p, "r", encoding="ascii") as f:
        return parse_input(f.read())

def test_parse_input(puzzle_input):
    enh_algo, image = puzzle_input
    assert len(enh_algo) == 512
    assert image.sum() == 10

def test_process_image(puzzle_input):
    enh_algo, image = puzzle_input
    image, fill_value = process_image(enh_algo, image, "0")
    assert image.sum() == 24
    image, fill_value = process_image(enh_algo, image, fill_value)
    assert image.sum() == 35

def test_part_b(puzzle_input):
    enh_algo, image = puzzle_input
    assert part_ab(enh_algo, image)[1] == 3351
