import numpy as np

from collections import deque
from dataclasses import dataclass
from numba import njit
from tqdm import tqdm
from typing import Tuple

@njit
def generate_dest_label(cl, lowest_cup, highest_cup):
    dl = cl - 1
    if dl < lowest_cup:
        return highest_cup
    else:
        return dl

@njit
def do_round(cups, current_label):
    current_index, = np.where(cups == current_label)[0]
    lowest_cup = min(cups)
    highest_cup = max(cups)

    # Make sure current cup is at position 0 before this method is called
    cups = np.roll(cups, -current_index)

    # remove 3 cups immediately to the right of the current cup
    removed = np.copy(cups[1:4])
    cups[1:4] = -1
    cups[1:] = np.roll(cups[1:], -3)

    # Calculate destination
    destination_label = generate_dest_label(current_label, lowest_cup, highest_cup)
    while destination_label in removed:
        destination_label = generate_dest_label(destination_label, lowest_cup, highest_cup)

    #print("Destination label: ", destination_label)
    destination_index, = np.where(cups == destination_label)[0]

    # Add back the removed cups after the destination index
    cups[destination_index + 1:] = np.roll(cups[destination_index + 1:], 3)
    cups[destination_index+1:destination_index+4] = removed
    #print("rv = ", cups)
    
    # Calculate the new "current" value and return
    return cups, cups[1]

@dataclass
class Cup(object):
    value: int
    prev: "Cup"
    next: "Cup"

    def __init__(self, value, prev=None, next=None):
        self.value = value
        self.prev = prev
        self.next = next

    def current_item(self):
        return self.value

    def remove_next_3(self):
        "Disconnects the next n elements from the list and returns a reference to them"
        rem_first = self.next
        rem_last = self.next.next.next
        rem_last.next.prev = self
        rem_first.prev = None
        self.next = rem_last.next
        rem_last.next = None

        return rem_first

    def insert_after(self, cups):
        "Adds the cups in the cups paramter to the list immediately after this position"
        old_next = self.next

        incoming_last = cups
        while incoming_last.next:
            incoming_last = incoming_last.next

        self.next = cups
        assert not incoming_last.next 
        incoming_last.next = old_next

        if old_next:
            old_next.prev = incoming_last

    def __repr__(self):
        return f"Cup({self.value})"

    def list_string(self):
        q = deque([self.value])
        ptr = self
        while ptr.next and ptr.next is not self:
            ptr = ptr.next
            q.append(ptr.value)

        return ",".join(map(str, q))

    def create_answer_string(self):
        assert self.value == 1
        one = self
        head = one.next
        str_parts = []
        while head.value != 1:
            str_parts.append(str(head.value))
            head = head.next

        return "".join(str_parts)

def create_ring(string, sep=""):

    head = None
    for c in string.split(sep) if sep != "" else string:
        if head:
            new_cup = Cup(int(c), prev=head)
            head.insert_after(new_cup)
            head = new_cup
            min_value = min(min_value, new_cup.value)
            max_value = max(max_value, new_cup.value)
        else:
            head = Cup(int(c))
            min_value = head.value
            max_value = head.value

    # Turn this into a loop by linking the start/end
    first_elem = head
    while first_elem.prev:
        first_elem = first_elem.prev

    head.next = first_elem
    first_elem.prev = head
    
    # Reset head to the first element to start the game
    return first_elem, min_value, max_value

def do_round_ll(head: Cup, min_v, max_v, value_lookup) -> Cup:
    """Re-implementation of do_round() using a Linked List
    
    :param cups: cups is a pointer to into the circular LL structure of cups
    :returns: the point to the next \"current\" value"""

    taken = head.remove_next_3()
    taken_values = list(map(int, taken.list_string().split(",")))

    # The crab selects a destination cup
    dest_value = head.value - 1
    while dest_value in taken_values or dest_value < 1:
        dest_value -= 1
        if dest_value < min_v:
            dest_value = max_v

    # The crab places the cups it just picked up so that they are immediately
    # clockwise of the destination cup.
    dest_cup = value_lookup[dest_value]
    dest_cup.insert_after(taken)

    return head.next

def create_string(cups):
    idx_of_one, = np.where(cups == 1)[0]
    cups = [ str(x) for x in cups ]
    return "".join(cups[idx_of_one + 1:]) + "".join(cups[0:idx_of_one])

if __name__ == "__main__":
    puzzle_input = "739862541"
    cups = np.array([ int(x) for x in puzzle_input ])
    current = cups[0]
    for _ in range(100):
        cups, current = do_round(cups, current)

    ans = create_string(cups)
    assert ans != "76345298", "Incorrect answer"
    print(ans)

    # Part B
    cups = list(map(int, puzzle_input))
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
    print(n1, n2)
    print(n1.value * n2.value)
