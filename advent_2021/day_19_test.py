from pathlib import Path

import pytest
from .day_19 import check_alignment, parse_input, merge, part_ab


@pytest.fixture
def puzzle_input():
    p = Path(__file__).parent / "input" / 'day_19_test.txt'
    with open(p, "r", encoding="ascii") as f:
        return parse_input(f.readlines())

def test_merge(puzzle_input):
    assert len(puzzle_input) == 5
    # Per challenge text, 0 & 1 and 1 & 4 have valid alignments
    # 1 and 3 do not
    assert merge(puzzle_input[0], puzzle_input[1])
    assert not merge(puzzle_input[0], puzzle_input[2])
    assert merge(puzzle_input[1], puzzle_input[4])

    # If 0,1 merge and 1,4 merge
    # then merge(merge(0,1),4) should also work
    new_base = merge(puzzle_input[0], puzzle_input[1])
    assert new_base
    new_base = merge(new_base, puzzle_input[4])
    assert new_base
    new_base = merge(new_base, puzzle_input[2])
    assert new_base
    new_base = merge(new_base, puzzle_input[3])
    assert new_base

def test_alignment():
    pairs = [
        ((404, -588, -901), (-336, 658, 858)),
        ((528, -643, 409), (-460, 603, -452)),
        ((390, -675, -793), (-322, 571, 750)),
        ((-537, -823, -458), (605, 423, 415)),
        ((-485, -357, 347), (553, 889, -390)),
        ((-345, -311, 381), (413, 935, -424)),
        ((-661, -816, -575), (729, 430, 532)),
        ((-618, -824, -621), (686, 422, 578)),
        ((-447, -329, 318), (515, 917, -361)),
        ((544, -627, -890), (-476, 619, 847)),
        ((423, -701, 434), (-355, 545, -477)),
        ((459, -707, 401), (-391, 539, -444))
    ]
    assert not check_alignment(pairs, 0, 0, 0)
    assert check_alignment(pairs, 0, 180, 0)

def test_part_ab(puzzle_input):
    assert part_ab(puzzle_input) == (79, 3621)
