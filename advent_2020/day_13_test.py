from .day_13 import *

def test_get_buses_at():
    routes = [7,13,59,31,19]

    desired = [
        set(),
        {31},
        {7,19},
        set(),set(),set(),set(),
        {13},
        set(),
        {7},
        set(),set(),set(),set(),set(),
        {59},
        {7},
        set(),set(),set(),
        {13}
    ]

    for idx, time in enumerate(range(929, 950)):
        assert get_buses_at(routes, time) == desired[idx]

def test_with_input():
    arrival_time, routes, _ = parse_input("""939
7,13,x,x,59,x,31,19""")

    assert arrival_time == 939
    assert routes == [7,13,59,31,19]

def test_find_next_bus_for():
    routes = [7,13,59,31,19]
    time = 939
    dep_bus, dep_time = find_next_bus_for(routes, time)
    assert dep_time - time == 5

def test_finshed():
    routes = [7,13,"x","x",59,"x",31,19]
    window = [
        {7},
        {13},
        set(),
        set(),
        {59},
        set(),
        {31},
        {19}
    ]

    assert finished(routes, window)

def test_do_part_b():
    assert do_part_b([17,"x",13,19]) == 3417
    assert do_part_b([67,7,59,61]) == 754018
    assert do_part_b([67,"x",7,59,61]) == 779210
    assert do_part_b([67,7,"x",59,61]) == 1261476
    assert do_part_b([1789,37,47,1889]) == 1202161486
