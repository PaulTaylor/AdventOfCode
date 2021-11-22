from .day_16 import *

test_input = """class: 1-3 or 5-7
row: 6-11 or 33-44
seat no: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"""

test_b_input = """class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"""

def test_parse_input():
    rules, my_ticket, other_tickets = parse_input(test_input)
    
    assert len(rules) == 3
    assert rules["class"] == [(1, 3), (5, 7)]
    
    assert my_ticket == [7,1,14]

    assert len(other_tickets) == 4
    assert other_tickets[0] == [7,3,47]

def test_part_a():
    assert part_a(test_input) == (71, [1,2,3])

def test_part_b():
    assert part_b(test_b_input) == { 'class': 12, 'row': 11, 'seat': 13 }