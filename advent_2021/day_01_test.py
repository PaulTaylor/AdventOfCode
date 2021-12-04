from .day_01 import *

test_input = [
    199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263
]

def test_part_a():
    res = part_a(test_input)
    assert res == 7

def test_part_b():
    res = part_b(test_input)
    assert res == 5