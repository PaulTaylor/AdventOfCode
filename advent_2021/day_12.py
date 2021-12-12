"""
Advent of Code 2021 - Day XX
See: https://adventofcode.com/2021/day/XX
"""

from collections import deque, Counter
from pathlib import Path

import networkx as nx

def parse_input(input_string):
    return [ x.split("-") for x in input_string.splitlines() ]

def count_paths(input_edges):
    G = nx.Graph()
    for start, end in input_edges:
        G.add_edge(start, end)

    path_count = 0
    q = deque()
    q.append(["start"])
    while len(q) > 0:
        current = q.popleft()
        last_node = current[-1]

        possible_nexts = G.edges(last_node)
        for _, poss_end in possible_nexts:
            if poss_end == "end":
                # Finished! - add a path to the count and don't submit
                # anything for the next iteration
                path_count += 1
            elif poss_end == poss_end.lower():
                # Lower case letters cannot be visited twice - check
                # and if we've not been to poss_end before then push
                # the extended path for further consideration
                if poss_end not in current:
                    q.append(current + [poss_end])
            else:
                # Upper case node can be visited multiple times
                # so just add for further consideration
                q.append(current + [poss_end])

    return path_count


def count_paths_b(input_edges):
    G = nx.Graph()
    for start, end in input_edges:
        G.add_edge(start, end)

    path_count = 0
    q = deque()
    q.append(["start"])

    while len(q) > 0:
        current = q.popleft()
        last_node = current[-1]

        possible_nexts = G.edges(last_node)
        for _, poss_end in possible_nexts:
            if poss_end == "start":
                # no going back to the start!
                continue
            elif poss_end == "end":
                # Finished! - add a path to the count and don't submit
                # anything for the next iteration
                path_count += 1
            elif poss_end == poss_end.lower():
                # Only 1 lower case node can be visited twice - and the
                # others must be at most once.
                visited_lower_case = Counter(
                    node for node in current
                    if node not in ["start", "end"] and node == node.lower()
                )
                # add the next node so we can see what the counts would be
                visited_lower_case[poss_end] += 1

                if visited_lower_case and \
                    sum(visited_lower_case.values()) > len(visited_lower_case) + 1:
                    # if the future path in the counter has more than one 2 in the
                    # values then this is not a valid future path and should be ignored
                    continue
                else:
                    # this is a valid possible extension
                    q.append(current + [poss_end])
            else:
                # Upper case node can be visited multiple times
                # so just add for further consideration
                q.append(current + [poss_end])

    return path_count

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_12_a.txt'
    with open(p, "r", encoding="ascii") as f:
        edges = parse_input(f.read())
        print(f"Answer for a is {count_paths(edges)}.")
        print(f"Answer for b is {count_paths_b(edges)}.")
