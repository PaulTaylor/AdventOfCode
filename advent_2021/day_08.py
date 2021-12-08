"""
Advent of Code 2021 - Day 8
See: https://adventofcode.com/2021/day/8
"""

from itertools import chain, product
from pathlib import Path
from typing import List, Set


UNIQUE_LENGTHS = {
    2: 1, # only 1 uses 2 segments
    3: 7, # only 7 uses 3 segments
    4: 4, # only 4 uses 4 segments
    7: 8, # only 8 uses all 7 segments
}

# reference positions
#  00
# 1  2
#  33
# 4  5
#  66
LIT_SEGMENTS = {
    0: [0,1,2,4,5,6],
    1: [2,5],
    2: [0,2,3,4,6],
    3: [0,2,3,5,6],
    4: [1,2,3,5],
    5: [0,1,3,5,6],
    6: [0,1,3,4,5,6],
    7: [0,2,5],
    8: [0,1,2,3,4,5,6],
    9: [0,1,2,3,5,6]
}

def parse_input(input_string):
    return [
        list(map(str.split, line.split("|")))
        for line in input_string.splitlines()
    ]

def part_a(the_patterns):
    """Find the total number of unique length digits in the output patterns"""
    acc = 0
    for _, out_patterns in the_patterns:
        for number in out_patterns:
            if len(number) in UNIQUE_LENGTHS:
                acc += 1

    return acc

def find_total(in_patterns, out_patterns):
    """
    find the total represented by the out_patterns given the specified input and
    output patterns
    """

    # Start with a config where all letters could be any
    # position
    config = [ set('abcdefg') for _ in range(7) ]

    # First sweep the patterns looking for the easy ones
    for p in chain(in_patterns, out_patterns):
        if len(p) == 2:
            # this pattern must be a number 1
            refine_config(config, p, LIT_SEGMENTS[1])
        elif len(p) == 3:
            # Similarly this has to be a 7 where 0/2/5 are lit
            refine_config(config, p, LIT_SEGMENTS[7])
        elif len(p) == 4:
            # 4 where 1/2/3/5 are lit
            refine_config(config, p, LIT_SEGMENTS[4])
        elif len(p) == 7:
            # This is actually a noop - because in an 8 we wouldn't remove
            # anything from any of the sets
            pass

    patterns_as_sets = [ set(x) for x in in_patterns + out_patterns ]

    # Here we have to start looking at possibilities find the first element
    # can use itertools.product to look for a configuration which works:
    for candidate in product(*config):
        is_valid, digits = test_config(candidate, patterns_as_sets)
        if is_valid:
            config = candidate
            break

    assert digits, "Couldn't find an appropriate configuration :,("

    # Use the digits from the correct config to decode the value of the
    # output number
    return decode_pattern(out_patterns, digits)

def decode_pattern(patterns, digits):
    "decode the given pattern using the supplied digit mapping"
    num_string = ""
    for pattern in patterns:
        pattern_set = set(pattern)
        num_string += str(digits.index(pattern_set))

    return int(num_string)

def test_config(config, patterns: List[Set[str]]) -> bool:
    """Test a config by generating all 10 digits, and checking that there are no
    patterns in the input patterns that are not present - if there are input patterns
    not in our generated list - then the config is incorrect"""

    assert isinstance(patterns[0], set)

    if len(set(config)) != len(config):
        # duplicate letters
        return False, None

    digits = []
    for indicies in LIT_SEGMENTS.values():
        digit = set()
        for i in indicies:
            digit.add(config[i])
        digits.append(digit)

    for p in patterns:
        if not p in digits:
            return False, None

    return True, digits


def refine_config(config, pattern, LIT_SEGMENTS):
    """
    refine the config assuming the pattern corresponds to the specified LIT_SEGMENTS

    For example - if this pattern is a number 1
    # therefore elements 2 and 5 must be those in this pattern (parameter LIT_SEGMENTS)
    # therefore we can remove these 2 letters from any of the other elements
    #       and we can set elements 2 and 5 to have only these 2 possibilities
    """
    # Apply intersection to those elements that are lit
    for pos in LIT_SEGMENTS:
        config[pos] &= set(pattern) # &= is set intersection (and assign)

    # Remove the latters from pattern from all other segments
    for pos in set(range(7)) - set(LIT_SEGMENTS):
        for char in pattern:
            if char in config[pos]:
                config[pos].remove(char)

def part_b(the_patterns):
    "Sum the values returned for each pattern"
    return sum(find_total(*p) for p in the_patterns)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_08_a.txt'
    with open(p, "r", encoding="ascii") as f:
        patterns = parse_input(f.read())
        print(f"Answer for a is {part_a(patterns)}.")
        print(f"Answer for b is {part_b(patterns)}.")
