import networkx as nx
import sys

from collections import deque
from itertools import product
from networkx.algorithms import connectivity as nxc
from networkx.algorithms import community as nxcomm
from pathlib import Path

class XmasException(Exception):
    pass

def part_a(adapters):
    available = sorted(adapters.copy())
    device_joltage = max(available) + 3
    seq = _part_a_internal(available, [0]) + [device_joltage]

    one_diffs = 0
    three_diffs = 0
    for idx in range(1, len(seq)):
        diff = seq[idx] - seq[idx - 1]
        if diff == 1:
            one_diffs += 1
        elif diff == 3:
            three_diffs += 1

    return one_diffs * three_diffs
    

def _part_a_internal(available, sequence):
    acceptable_min = sequence[-1]
    acceptable_max = sequence[-1] + 3
    
    # Find possible adapters in the remaining list
    q = deque(filter(lambda x: acceptable_min < x <= acceptable_max, available))
    
    if q:
        for idx, item in enumerate(q):
            new_available = available.copy()
            new_available.remove(item)
            new_sequence = (sequence + [item])
            try:
                rv = _part_a_internal(new_available, new_sequence)
                if rv:
                    return rv
            except XmasException:
                return None

    elif len(available) == 0:
        # There are no new adapters to use - we've finished - return the sequence!
        return sequence
    else:
        # This is a dead-end
        raise XmasException(f"Incorrect sequence - {len(available)} left")

def part_b(adapters):
    "This time we don't have to use all the adapters - we just have to get within 3 of 22"
    available = sorted(adapters)
    device_joltage = available[-1] + 3
    
    result_sequences = [0]
    _part_b_internal(available, device_joltage, [0], result_sequences)
    return result_sequences[0]

def _part_b_internal(available, device_joltage, sequence, result_sequences):
    acceptable_min = sequence[-1]
    acceptable_max = sequence[-1] + 3

    if (sequence[-1] + 3) >= device_joltage:
        result_sequences[0] += 1
        if result_sequences[0] % 100000 == 0:
            print("********", result_sequences[0])
    
    # Find possible adapters in the remaining list
    q = deque(filter(lambda x: acceptable_min < x <= acceptable_max, available))

    if q:
        for item in q:
            new_available = available.copy()
            new_available.remove(item)
            new_sequence = (sequence + [item])
            _part_b_internal(new_available, device_joltage, new_sequence, result_sequences)   

def create_graph(s_adapters, device_joltage):
    
    G = nx.DiGraph()
    G.add_node(0)
    G.add_nodes_from(s_adapters)
    G.add_node(device_joltage)

    # Add inter adapter edges
    edges = [ (a,b) for a, b in product(s_adapters, s_adapters) if a != b and (0 < (b-a) <= 3)]
    G.add_edges_from(edges)

    # Add edges from start (0)
    start_edges = [ (0, x) for x in s_adapters if 0 < x <= 3 ]
    G.add_edges_from(start_edges)

    # And end edges
    end_edges = [ (x, device_joltage) for x in s_adapters if  x+3 >= device_joltage ]
    G.add_edges_from(end_edges)

    return G

def find_graph_cuts(G):
    cuts = []
    nodes_cut = set()
    c = nxc.minimum_edge_cut(G, s=min(G.nodes), t=max(G.nodes))
    #while c and G.out_degree(c[0]) < 2 and G.in_degree(c[1]) < 2:
    while len(c) == 1: # this is a cheat and may not work
        cuts.insert(0, next(c.__iter__()))
        for x in c:
            for elem in x:
                nodes_cut.add(elem)
        
        s = min(G.nodes)
        t = min(nodes_cut)
        if s == t:
            break
        else:
            c = nxc.minimum_edge_cut(G, s=s, t=t)
    
    return cuts

def part_b_nx(adapters):
    s_adapters = sorted(adapters)
    device_joltage = s_adapters[-1] + 3
    
    G = create_graph(s_adapters, device_joltage)
    
    # Remove "linear" edges to split the graph into components
    edges_to_remove = find_graph_cuts(G)
    G.remove_edges_from(edges_to_remove)

    #nx.nx_agraph.write_dot(G, "cut-graph.dot")
    
    # Iterate over components of graph
    result = 1
    for G_c in nxcomm.asyn_lpa_communities(G):
        if len(G_c) > 1:
            paths = list(nx.all_simple_paths(G, min(G_c), max(G_c)))
            result *= len(paths)

    return result

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_10_a.txt'
    with open(p, "rt") as f:
        adapters = [ int(x) for x in f.readlines() ]

    print(f"Part A => {part_a(adapters)}")
    print(f"Part B => {part_b_nx(adapters)}")