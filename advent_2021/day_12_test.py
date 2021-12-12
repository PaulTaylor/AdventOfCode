import pytest

from .day_12 import parse_input, count_paths, count_paths_b

@pytest.fixture
def small_puzzle_input():
    return parse_input("""start-A
start-b
A-c
A-b
b-d
A-end
b-end""")

@pytest.fixture
def medium_puzzle_input():
    return parse_input("""dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc""")

@pytest.fixture
def big_puzzle_input():
    return parse_input("""fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW""")

def test_part_a(small_puzzle_input, medium_puzzle_input, big_puzzle_input):
    assert count_paths(small_puzzle_input) == 10
    assert count_paths(medium_puzzle_input) == 19
    assert count_paths(big_puzzle_input) == 226

def test_part_b(small_puzzle_input, medium_puzzle_input, big_puzzle_input):
    assert count_paths_b(small_puzzle_input) == 36
    assert count_paths_b(medium_puzzle_input) == 103
    assert count_paths_b(big_puzzle_input) == 3509
