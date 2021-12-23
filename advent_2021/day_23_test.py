import pytest

from .day_23 import parse_input, part_a, part_b, Amphipod, generate_h2r_states, generate_r2h_states
from .day_23 import HALLWAY_POS, check_occupancy

@pytest.fixture
def puzzle_input():
    return parse_input("""#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########""")

@pytest.fixture
def puzzle_input_b():
    return parse_input("""#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########""")

def test_parse_input(puzzle_input, puzzle_input_b):
    assert len(puzzle_input) == 8
    assert puzzle_input[0] == Amphipod(typ='A', room=3, pos=0)
    assert puzzle_input[-1] == Amphipod(typ='D', room=9, pos=1)

    assert len(puzzle_input_b) == 16

def test_check_occupancy():
    state = (
        Amphipod(typ='A', room=3, pos=0),
        Amphipod(typ='A', room=9, pos=0),
        Amphipod(typ='B', room=5, pos=0),
        Amphipod(typ='B', room=5, pos=1),
        Amphipod(typ='C', room=7, pos=0),
        Amphipod(typ='C', room=7, pos=1),
        Amphipod(typ='D', room=-1, pos=8),
        Amphipod(typ='D', room=9, pos=1)
    )
    assert check_occupancy(state, 9, 0)

def test_generate_r2h_states():
    # If a pod is faced with an empty hallway - it can move to any pos
    # except those in front of doors
    s = (Amphipod("B", 3, 0),)
    r = list(generate_r2h_states(s, 0, 2))
    assert len(r) == len(HALLWAY_POS)
    assert sum(cost for _, cost in r) == 40+30+30+50+70+90+100
    s = (Amphipod("B", 3, 1),)
    r = list(generate_r2h_states(s, 0, 2))
    assert sum(cost for _, cost in r) == 30+20+20+40+60+80+90

    # And with bigger rooms
    s = (Amphipod("B", 3, 0),)
    r = list(generate_r2h_states(s, 0, 4))
    assert len(r) == len(HALLWAY_POS)
    assert sum(cost for _, cost in r) == 60+50+50+70+90+110+120

    # If a pod cannot move left - only generate right hand hallway states
    s = (Amphipod("1", -1, 2), Amphipod("B", 3, 0),)
    r = list(generate_r2h_states(s, 1, 2))
    assert len(r) == 5
    assert sum(cost for _, cost in r) == 30+50+70+90+100
    # If a pod cannot move right...
    s = (Amphipod("1", -1, 4), Amphipod("B", 3, 0),)
    r = list(generate_r2h_states(s, 1, 2))
    assert len(r) == 2
    assert sum(cost for _, cost in r) == 40+30

    # And for bigger rooms
    s = (Amphipod("1", -1, 2), Amphipod("B", 3, 2),)
    r = list(generate_r2h_states(s, 1, 4))
    assert len(r) == 5
    assert sum(cost for _, cost in r) == 30+50+70+90+100

    # In this state D in room 9 needs to move out of the way of the
    # A that's trapped behind it
    s = (Amphipod(typ='A', room=3, pos=0),
        Amphipod(typ='A', room=9, pos=0),
        Amphipod(typ='B', room=5, pos=0),
        Amphipod(typ='B', room=5, pos=1),
        Amphipod(typ='C', room=7, pos=0),
        Amphipod(typ='C', room=7, pos=1),
        Amphipod(typ='D', room=-1, pos=6),
        Amphipod(typ='D', room=9, pos=1))
    r = list(generate_r2h_states(s, 7, 2))
    assert len(r) == 3
    assert sum(x[0][-1].pos for x in r) == 8+10+11

def test_generate_h2r_states():
    # A 'pod that's in the hallway can move to it's own room
    room_size = 2
    s = (Amphipod("C", -1, 1),)
    r = list(generate_h2r_states(s, 0, room_size))
    assert len(r) == room_size
    assert all(p.room == 7 for s, _ in r for p in s)
    assert sum(cost for _, cost in r) == 700+800

    # And in bigger rooms
    room_size = 4
    s = (Amphipod("C", -1, 1),)
    r = list(generate_h2r_states(s, 0, room_size))
    assert len(r) == room_size
    assert sum(cost for _, cost in r) == 1000+900+800+700

def test_part_a(puzzle_input):
    assert part_a(puzzle_input) == 12521

@pytest.mark.skip("Takes ~60 seconds - so disable by default")
def test_part_b(puzzle_input_b):
    assert part_b(puzzle_input_b) == 44169
