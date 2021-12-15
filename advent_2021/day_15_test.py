import pytest

from .day_15 import expand_input, parse_input, expand_input, shortest_path

@pytest.fixture
def input_string():
    return """1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"""

@pytest.fixture
def puzzle_input(input_string):
    return parse_input(input_string)

def test_part_a(puzzle_input):
    assert shortest_path(*puzzle_input) == 40

def test_part_b(input_string):
    expanded_input = expand_input(input_string, 5)

    # Check we're expanding correctly
    lines = expanded_input.splitlines()
    assert len(lines) == len(input_string.splitlines()) * 5
    assert lines[0] == "11637517422274862853338597396444961841755517295286"
    assert lines[10] == "22748628533385973964449618417555172952866628316397"            
    assert lines[-1] == "67554889357866599146897761125791887223681299833479"

    G, dim = parse_input(expanded_input)
    assert shortest_path(G, dim) == 315
