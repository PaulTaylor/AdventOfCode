import pytest
import numpy as np

from .day_23 import *

def check_equal(actual, desired):
    a_actual = np.array(actual)
    a_desired = np.array(desired)

    # roll desired so that it looks like actual
    idx, = np.where(a_desired == a_actual[0])[0]
    a_desired = np.roll(a_desired, -idx)

    return np.array_equal(a_actual, a_desired)

def test_remove_next_3():
    current, minv, maxv = create_ring("389125467")
    assert current.list_string() == "3,8,9,1,2,5,4,6,7"
    assert minv == 1
    assert maxv == 9

    removed = current.remove_next_3()
    assert not removed.prev 
    assert removed.list_string() == "8,9,1"
    assert current.list_string() == "3,2,5,4,6,7"

def test_part_a():
    current, min_v, max_v = create_ring("389125467")

    head = current
    value_lookup = { current.value : current }
    while head.next is not current:
        head = head.next
        value_lookup[head.value] = head
    del head
    
    # Round 1
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Verify the start of R2 state
    assert value_lookup.get(3).list_string() == "3,2,8,9,1,5,4,6,7"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R3
    assert value_lookup.get(3).list_string() == "3,2,5,4,6,7,8,9,1"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R4
    assert value_lookup.get(7).list_string() == "7,2,5,8,9,1,3,4,6"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R5
    assert value_lookup.get(3).list_string() == "3,2,5,8,4,6,7,9,1", "SR5"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R6
    assert value_lookup.get(9).list_string() == "9,2,5,8,4,1,3,6,7", "SR6"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R7
    assert value_lookup.get(7).list_string() == "7,2,5,8,4,1,9,3,6", "SR7"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R8
    assert value_lookup.get(8).list_string() == "8,3,6,7,4,1,9,2,5", "SR8"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R9
    assert value_lookup.get(7).list_string() == "7,4,1,5,8,3,9,2,6", "SR9"
    current = do_round_ll(current, min_v, max_v, value_lookup)
    # Start of R10
    assert value_lookup.get(5).list_string() == "5,7,4,1,8,3,9,2,6", "SR10"
    current = do_round_ll(current, min_v, max_v, value_lookup)

    assert "92658374" == value_lookup[1].create_answer_string()

    for _ in range(90):
        current = do_round_ll(current, min_v, max_v, value_lookup)

    assert value_lookup[1].create_answer_string() == "67384529"

@pytest.mark.skip(reason="skip test because it takes a long time")
def test_part_b():
    test_string = "389125467"
    cups = list(map(int, test_string))
    num = max(cups) + 1
    while len(cups) < 1e6:
        cups.append(num)
        num += 1
    assert len(cups) == 1e6

    current, min_v, max_v = create_ring(",".join(list(map(str, cups))), sep=",")
    
    head = current
    value_lookup = { current.value : current }
    while head.next is not current:
        head = head.next
        value_lookup[head.value] = head
    assert len(value_lookup) == len(cups)

    del cups, head

    for _ in tqdm(range(10000000)):
        current = do_round_ll(current, min_v, max_v, value_lookup)

    cup_one = value_lookup[1]
    n1 = cup_one.next
    n2 = n1.next
    assert (n1.value * n2.value) == 149245887792