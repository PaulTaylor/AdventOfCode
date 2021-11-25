import pytest
import numpy as np

from .day_23 import *
from numba.typed import List

def test_part_a():
    ring = Ring(List([ int(x) for x in "389125467"]))
    assert str(ring) == "25467389"

    for desired in [
        "54673289",
        "32546789",
        "34672589",
        "32584679",
        "36792584",
        "93672584",
        "92583674",
        "58392674",
        "83926574",
        "92658374"]:
        ring.do_round()
        assert str(ring) == desired

    while ring.round < 100:
        ring.do_round()

    assert str(ring) == "67384529"

def test_part_b():
    test_string = "389125467"
    cups = list(map(int, test_string))
    num = max(cups) + 1
    while len(cups) < 1e6:
        cups.append(num)
        num += 1
    assert len(cups) == 1e6

    ring = Ring(cups)
    for _ in tqdm(range(10000000)):
        ring.do_round()

    cup_one = ring.cup_1
    n1 = cup_one.next
    n2 = n1.next
    assert (n1.value * n2.value) == 149245887792