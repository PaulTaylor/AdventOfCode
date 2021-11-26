from collections import deque
from .day_22 import parse_input, part_a, part_b, play_game

test_input = """Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"""

def test_parse_input():
    assert parse_input(test_input) == (deque([9,2,6,3,1]), deque([5,8,4,7,10]))

def test_play_game():
    d1, d2 = parse_input(test_input)
    play_game(d1, d2)
    assert (d1, d2) == (deque(), deque([3, 2, 10, 6, 8, 5, 9, 4, 7, 1]))

def test_part_a():
    assert 306 == part_a(test_input)

def test_part_b():
    assert 291 == part_b(test_input)

def test_part_b_noinf():
    test_input = """Player 1:
43
19

Player 2:
2
29
14"""
    assert part_b(test_input)
