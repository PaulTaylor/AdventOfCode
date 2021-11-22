import networkx as nx

from .day_10 import *

available_adapters = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
another_set = [28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49,
               45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3]


def test_part_a():
    assert part_a(available_adapters) == 7*5


def test_part_b():
    assert part_b(available_adapters) == 8
    assert part_b(another_set) == 19208


def test_part_b_nx():
    assert part_b_nx(available_adapters) == 8
    assert part_b_nx(another_set) == 19208


def test_find_graph_cuts():
    s_adapters = sorted(another_set)
    device_joltage = s_adapters[-1] + 3

    G = create_graph(s_adapters, device_joltage)

    test2_breaks = [
        (4, 7), (11, 14), (14, 17), (20, 23), (25, 28), (28, 31),
        (35, 38), (38, 39), (39, 42), (42, 45), (49, 52)
    ]

    r = find_graph_cuts(G)
    assert test2_breaks == r
    
    