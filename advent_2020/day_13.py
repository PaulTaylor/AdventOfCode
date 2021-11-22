import re

from pathlib import Path


digits = re.compile("^[0-9]+$")

def get_buses_at(routes, time):
    return { i for i in routes if time % i == 0}

def parse_input(in_string):
    lines = in_string.split("\n")
    return int(lines[0]), [ int(x) for x in lines[1].split(",") if digits.match(x.strip())  ], [ int(x) if digits.match(x.strip()) else x.strip() for x in lines[1].split(",")  ]

def find_next_bus_for(routes, time):
    while True:
        r = get_buses_at(routes, time)
        if r:
            return next(r.__iter__()), time
        time += 1

def finished(routes, window):
    "checks if we have buses leaving in sequence"

    for r, w in zip(routes, window):
        if r != "x":
            if not r in w:
                return False # mismatch

    return True

def check_timestamp(timestamp, routes_b, indexes_to_check):
    for idx in indexes_to_check:
        route = routes_b[idx]
        if route is None:
            pass
        elif ((timestamp + idx) % route) > 0:
            return False

    return True


def do_part_b(routes_b, start=1, stop_at=None):
    """Taken from a reddit solution:
    https://gist.github.com/joshbduncan/65f810fe821c7a3ea81a1f5a444ea81e
    **not submitted**  only one star for me today :(

    Apparently there are also solutions based on chinese remainder theorem, but I've never heard of it!
    """

    p2 = [ (int(b), idx) for idx, b in enumerate(routes_b) if b != "x" ]

    t, step = 0, 1
    for bus_id, mins in p2:
        # check to see if bus is departing at current time
        while (t + mins) % bus_id != 0:
            t += step
        
        # increase step multiple to find next min for next bus
        step *= bus_id

    return t

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_13_a.txt'
    with open(p, "rt") as f:
        in_str = f.read()

    arrival_time, routes, routes_b = parse_input(in_str)
    dep_bus, dep_time = find_next_bus_for(routes, arrival_time)
    print(f"Part A - should get bus {dep_bus} at {dep_time}")
    wait = dep_time - arrival_time
    part_a_answer = wait * dep_bus
    print(f".. this is a wait of {wait} => answer = {part_a_answer}")

    # Part B - cheated a bit on this one
    timestamp = do_part_b(routes_b)
    print(f"Finished B - timestamp is {timestamp}.")