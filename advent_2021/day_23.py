"""
Advent of Code 2021 - Day 23
See: https://adventofcode.com/2021/day/23
"""

import heapq
from collections import Counter, namedtuple
from pathlib import Path

Amphipod = namedtuple('Amphipod', 'typ room pos')

MOVE_COSTS={ 'A': 1, 'B': 10, 'C': 100, 'D': 1000 }
FINAL_ROOMS = { 'A': 3, 'B': 5, 'C': 7, 'D': 9 }
HALLWAY_POS = {1,2,4,6,8,10,11}

class HeapWrapper:
    """
    Create a wrapper around heapq that uses a set for "contains" testing.

    Without this the "not in" operation in the a-star algorithm gets slower
    and slower until it virutally grinds to a halt (35s vs ~25+ minutes)
    """
    def __init__(self) -> None:
        self.heap = []
        heapq.heapify(self.heap)
        self.counter = set()

    def push(self, new_v):
        heapq.heappush(self.heap, new_v)
        self.counter.add(new_v[1])

    def pop(self):
        h, v = heapq.heappop(self.heap)
        self.counter.remove(v)
        return h, v

    def __contains__(self, other):
        return other in self.counter

    def __len__(self):
        return len(self.heap)


def parse_input(input_string):
    lines = input_string.splitlines()
    # All amphipods start in rooms
    # 1 is the position nearest the hallway
    if len(lines) == 5:
        locs = [ Amphipod(lines[2][x], x, 1) for x in (3,5,7,9) ] +\
               [ Amphipod(lines[3][x], x, 0) for x in (3,5,7,9) ]
    elif len(lines) == 7:
        locs = [ Amphipod(lines[2][x], x, 3) for x in (3,5,7,9) ] +\
               [ Amphipod(lines[3][x], x, 2) for x in (3,5,7,9) ] +\
               [ Amphipod(lines[4][x], x, 1) for x in (3,5,7,9) ] +\
               [ Amphipod(lines[5][x], x, 0) for x in (3,5,7,9) ]

    return tuple(sorted(locs))

def check_occupancy(pods, room, pos):
    for pod in pods:
        if (pod.room == room) and (pod.pos == pos):
            return True
    return False

def calculate_hallway_occupied(pods):
    return { p.pos for p in pods if p.room == -1 }

def room_clean(state, room, correct_typ):
    in_room = (x for x in state if x.room == room)
    return all(x for x in in_room if x.typ == correct_typ)

def new_tuple(t, index, value):
    # New tuples must be sorted for consistency
    return tuple(sorted(t[:index] + (value,) + t[index+1:]))

def generate_r2h_states(s0, idx, room_size):
    hallway_occupied = calculate_hallway_occupied(s0)
    door_idx = room_size - 1
    base_cost = 0
    a = s0[idx]

    if a.pos < room_size - 1:
        if any(check_occupancy(s0, a.room, p) for p in range(a.pos+1, room_size)):
            return # blocked
        # We'll assume the hallway door is also clear because of the rules
        base_cost += ((door_idx - a.pos) * MOVE_COSTS[a.typ]) # <- the cost to move to pos 1

    # Now assuming we're at the entrance to the room - what hallway
    # positions can we take up - output a state for each
    # Check left
    min_left_pos = max([0] + [ x for x in hallway_occupied if x < a.room ]) + 1
    if min_left_pos not in HALLWAY_POS:
        min_left_pos += 1 # make sure we don't block a door
    for new_pos in range(min_left_pos, a.room):
        if new_pos in HALLWAY_POS:
            new_state = new_tuple(s0, idx, a._replace(room=-1, pos=new_pos))
            cost = base_cost
            cost += MOVE_COSTS[a.typ] # move into hallway
            cost += MOVE_COSTS[a.typ] * (a.room - new_pos)
            yield (new_state, cost)

    # Check right
    max_right_pos = min([11] + [ x-1 for x in hallway_occupied if x > a.room ])
    if max_right_pos not in HALLWAY_POS:
        max_right_pos -= 1 # make sure we don't block a door
    for new_pos in range(a.room + 1, max_right_pos+1):
        if new_pos in HALLWAY_POS:
            new_state = new_tuple(s0, idx, a._replace(room=-1, pos=new_pos))
            cost = base_cost
            cost += MOVE_COSTS[a.typ] # move into hallway
            cost += MOVE_COSTS[a.typ] * (new_pos - a.room)
            yield (new_state, cost)

def generate_h2r_states(s0, idx, room_size):
    assert s0[idx].room == -1, "not already in the hallway"
    hallway_occupied = calculate_hallway_occupied(s0)
    door_idx = room_size - 1
    a = s0[idx]
    target = FINAL_ROOMS[a.typ]

    # is clear
    left = min(a.pos+1, target)
    right = max(a.pos-1, target)
    clear = set(range(left, right+1)).isdisjoint(hallway_occupied)

    if clear:
        others_in_room = [ x for x in s0 if x.room == target ]
        if any( x.typ != a.typ for x in others_in_room ):
            # another 'pod of a different type is in the room
            # so I will not move into it
            pass
        elif others_in_room and others_in_room[-1].pos == door_idx:
            # The other pod is blocking me from entering the room
            # can't do anything in this case
            pass
        else:
            # Find which parts of the room are available - will be anything
            # between the door and any 'pods already in the room
            max_pidx = max((o.pos for o in others_in_room), default=-1)
            for room_idx in range(max_pidx+1, room_size):
                new_state = new_tuple(s0, idx, a._replace(room=target, pos=room_idx))
                cost = MOVE_COSTS[a.typ] * ((right - left)+1)
                cost += (room_size - room_idx) * MOVE_COSTS[a.typ]
                yield (new_state, cost)

def generate_next_states(state, room_size):
    for idx, a in enumerate(state):
        if a.room == FINAL_ROOMS[a.typ] and a.pos == 0:
            # a in it's final room in the innermost position - don't do anything
            pass
        elif a.room == FINAL_ROOMS[a.typ] and a.pos > 0 and not check_occupancy(state, a.room, a.pos-1):
            # could move a step further into the room interior - if it's not occupied
            new_state = new_tuple(state, idx, a._replace(pos=a.pos-1))
            cost = MOVE_COSTS[a.typ]
            yield (cost, new_state)
        elif a.room != -1:
            # This pod is in a room that's not it's own
            # can move into the hallway, but only if the way is clear
            for new_state, cost in generate_r2h_states(state, idx, room_size):
                yield (cost, new_state)
        else:
            # This pod is in the hallway - can only move into it's own
            # room if that room is either empty, or only has same types in
            for new_state, cost in generate_h2r_states(state, idx, room_size):
                yield (cost, new_state)

def heuristic(state):
    "Simple heuristic - there's probably a better one that would increase performance"
    out_of_room = (x for x in state if x.room != FINAL_ROOMS[x.typ])
    acc = 0
    for pod in out_of_room:
        acc += abs(pod.pos - FINAL_ROOMS[pod.typ]) * MOVE_COSTS[pod.typ]
    return acc

def a_star(start, goal, room_size):
    open_set = HeapWrapper()
    open_set.push((0, start))

    came_from = {}
    g_score = { start: 0 }
    f_score = { start: heuristic(start) }

    while len(open_set) > 0:
        _, current = open_set.pop()
        if current == goal:
            return g_score[current]

        for cost, neighbour in generate_next_states(current, room_size):
            tentative_g = g_score[current] + cost
            if tentative_g < g_score.get(neighbour, 100_000_000):
                came_from[neighbour] = current
                g_score[neighbour] = tentative_g
                f_score[neighbour] = tentative_g + heuristic(neighbour)
                if neighbour not in open_set:
                    open_set.push((f_score[neighbour], neighbour))

    raise Exception("No solution found :(")

def part_a(start):
    goal=(
        Amphipod(typ='A', room=3, pos=0),
        Amphipod(typ='A', room=3, pos=1),
        Amphipod(typ='B', room=5, pos=0),
        Amphipod(typ='B', room=5, pos=1),
        Amphipod(typ='C', room=7, pos=0),
        Amphipod(typ='C', room=7, pos=1),
        Amphipod(typ='D', room=9, pos=0),
        Amphipod(typ='D', room=9, pos=1),
    )

    return a_star(start, goal, room_size=len(goal)//4)

def part_b(start):
    goal=(
        Amphipod(typ='A', room=3, pos=0),
        Amphipod(typ='A', room=3, pos=1),
        Amphipod(typ='A', room=3, pos=2),
        Amphipod(typ='A', room=3, pos=3),
        Amphipod(typ='B', room=5, pos=0),
        Amphipod(typ='B', room=5, pos=1),
        Amphipod(typ='B', room=5, pos=2),
        Amphipod(typ='B', room=5, pos=3),
        Amphipod(typ='C', room=7, pos=0),
        Amphipod(typ='C', room=7, pos=1),
        Amphipod(typ='C', room=7, pos=2),
        Amphipod(typ='C', room=7, pos=3),
        Amphipod(typ='D', room=9, pos=0),
        Amphipod(typ='D', room=9, pos=1),
        Amphipod(typ='D', room=9, pos=2),
        Amphipod(typ='D', room=9, pos=3),
    )

    return a_star(start, goal, room_size=len(goal)//4)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_23_a.txt'
    with open(p, "r", encoding="ascii") as f:
        input_string = f.read()

    starting = parse_input(input_string)
    print(f"Answer for a is {part_a(starting)}.")

    new_lines = input_string.splitlines()
    new_lines.insert(3, "  #D#C#B#A#")
    new_lines.insert(4, "  #D#B#A#C#")
    b_input = parse_input("\n".join(new_lines))
    print(f"Answer for b is {part_b(b_input)}.")
