import pytest
import numpy as np

from .day_25 import parse_input, part_a

@pytest.fixture
def simpler_input():
    return parse_input("""...>...
.......
......>
v.....>
......>
.......
..vvv..""")

@pytest.fixture
def complex_input():
    return parse_input("""v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>""")

def test_movement(simpler_input):
    grid = np.copy(simpler_input)
    _ = part_a(grid, max_rounds=1)
    assert np.array_equal(grid, parse_input("""..vv>..
.......
>......
v.....>
>......
.......
....v.."""))

    grid = np.copy(simpler_input)
    _ = part_a(grid, max_rounds=4)
    assert np.array_equal(grid, parse_input(""">......
..v....
..>.v..
.>.v...
...>...
.......
v......"""))

def test_part_a(complex_input):
    assert part_a(complex_input, 65) == 58
