from .day_15 import *
from numba.typed import List

def test_part_a():
    test_values = [
        ([0,3,6], 436),
        ([1,3,2], 1),
        ([2,1,3], 10),
        ([1,2,3], 27),
        ([2,3,1], 78),
        ([3,2,1], 438),
        ([3,1,2], 1836)
    ]

    assert game(test_values[0][0], 10) == 0

    for sn, desired in test_values:
        assert game(sn, 2020) == desired

def test_part_b():
    test_values = [
        (List([0,3,6]), 175594),
        # ([1,3,2], 2578),
        # ([2,1,3], 3544142),
        # ([1,2,3], 261214),
        # ([2,3,1], 6895259),
        # ([3,2,1], 18),
        # ([3,1,2], 362)
    ]

    for sn, desired in test_values:
        assert numba_game(sn, 30000000) == desired

    