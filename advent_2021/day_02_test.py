import pytest

from .day_02 import parse_lines, part_a, part_b

@pytest.fixture
def test_tuples():
    test_input = """forward 5
down 5
forward 8
up 3
down 8
forward 2"""

    return parse_lines(test_input)

def test_part_a(test_tuples):
    res = part_a(test_tuples)
    assert res == 150

def test_part_b(test_tuples):
    res = part_b(test_tuples)
    assert res == 900
