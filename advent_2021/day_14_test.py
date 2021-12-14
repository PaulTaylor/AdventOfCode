import pytest

from .day_14 import parse_input, part_a, part_b

@pytest.fixture
def puzzle_input():
    return parse_input("""NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C""")

def test_part_a(puzzle_input):
    template, rules = puzzle_input
    assert part_a(template, rules) == 1588

def test_part_b(puzzle_input):
    template, rules = puzzle_input
    assert part_b(template, rules, 10) == 1588
    assert part_b(template, rules, 40) == 2188189693529
