import numpy as np

from .day_17 import create_starting_grid, simulation_round

test_input = """.#.
..#
###"""

def test_create_starting_grid():
    v = create_starting_grid(test_input)
    assert v is not None
    assert v.shape == (1,3,3)
    assert np.sum(v) == 5

def test_simulation_round():
    d0 = create_starting_grid(test_input)
    d1 = simulation_round(d0)
    assert np.sum(d1) == 11
    d2 = simulation_round(d1)
    assert np.sum(d2) == 21
    d3 = simulation_round(d2)
    assert np.sum(d3) == (5+10+8+10+5)

    d_n = d3
    for _ in range(3):
        d_n = simulation_round(d_n)

    assert np.sum(d_n) == 112
