"""
Advent of Code 2021 - Day 15
See: https://adventofcode.com/2021/day/15
"""

import networkx as nx
import numpy as np

from pathlib import Path


def parse_input(input_string):
    rows = input_string.splitlines()
    G = nx.DiGraph()
    for row, row_values in enumerate(rows):
        for col, value in enumerate(row_values):
            G.add_node((row, col), val=int(value))
            # edge values are the cost of the target.  we can do this because 
            # we only care about value when we "enter" a node
            if row+1 < len(rows):
                # Down
                target_value = int(rows[row+1][col])
                G.add_edge((row,col), (row+1, col), weight=target_value)
            if col+1 < len(rows):
                # Right
                target_value = int(rows[row][col+1])
                G.add_edge((row,col), (row, col+1), weight=target_value)
            if row-1 >= 0:
                # Up
                target_value = int(rows[row-1][col])
                G.add_edge((row,col), (row-1, col), weight=target_value)
            if col-1 >= 0:
                # Down
                target_value = int(rows[row][col-1])
                G.add_edge((row,col), (row, col-1), weight=target_value)

    return G, len(rows)

def expand_input(input_string, times_bigger):
    rows = input_string.splitlines()

    new_rows = []
    for row_delta in range(times_bigger):
        for row in rows:
            this_row = []
            for col_delta in range(times_bigger):
                row_arr = np.array(list(row), dtype=int) + col_delta + row_delta
                # Need to cleanup > 9 values
                if np.max(row_arr) > 9:
                    over_nines = np.argwhere(row_arr > 9)[:,0]
                    for idx in over_nines:
                        row_arr[idx] -= 9

                    assert np.max(row_arr) < 10, \
                        "Assumption failed - some things have passed 9 more than once"
                this_row.append(row_arr)
            new_rows.append(this_row)

    for idx in range(len(new_rows)):
        new_rows[idx] = "".join(str(c) for line in new_rows[idx] for c in line)

    return "\n".join(new_rows)

def shortest_path(G: nx.DiGraph, dim: int) -> int:
    path = nx.shortest_path(G, 
        source=(0,0), 
        target=(dim-1, dim-1), 
        weight="weight")

    path_weights = [ G[src][tgt]["weight"] for src, tgt in zip(path, path[1:]) ]
    return sum(path_weights)


if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_15_a.txt'
    with open(p, "r", encoding="ascii") as f:
        input_string = f.read()

    G, dim = parse_input(input_string)
    res = shortest_path(G, dim)
    print(f"Answer for a is {res}.")
    
    expand_input = expand_input(input_string, 5)
    G, dim = parse_input(expand_input)
    res = shortest_path(G, dim)
    print(f"Answer for b is {res}.")
