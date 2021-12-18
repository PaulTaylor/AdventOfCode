"""
Advent of Code 2021 - Day 18
See: https://adventofcode.com/2021/day/18
"""

import math
import re

from collections import deque
from itertools import product
from pathlib import Path

NUMBER_PATTERN = re.compile(r"([0-9]+)")
TWO_DIGIT_NUMBER = re.compile(r"([0-9]{2,})")


def calculate_magnitude(input_string):
    list_rep = eval(input_string, {}, {})
    return _calculate_magnitude(list_rep)

def _calculate_magnitude(input_list):
    left, right = input_list
    if isinstance(left, list):
        left = _calculate_magnitude(left)
    if isinstance(right, list):
        right = _calculate_magnitude(right)

    return left*3 + right*2

def explode(input_string):
    depth = 0
    for idx, c in enumerate(input_string):
        if c == "[":
            depth += 1
        elif c == "]":
            depth -= 1

        if depth > 4:
            # The next several characters must have the [x,y] pattern
            closing_idx = input_string.find("]", idx)
            prior_string = input_string[:idx]
            pair_string = input_string[idx:closing_idx+1]
            post_string = input_string[closing_idx+1:]

            left_num, right_num = eval(pair_string, {}, {})

            # look in the prior string for the previous integer
            # we'll need the last match
            match_deque = deque(NUMBER_PATTERN.finditer(prior_string), maxlen=1)
            if match_deque:
                prior_match = match_deque.pop()
                new_number = int(prior_match.group()) + left_num
                prior_string = prior_string[:prior_match.start()] + str(new_number) + prior_string[prior_match.end():]

            # Now deal with the post string and the right hand number
            # Look for the next number
            post_match = NUMBER_PATTERN.search(post_string)
            if post_match:
                # Found a number
                new_number = int(post_match.group()) + right_num
                post_string = post_string[:post_match.start()] + str(new_number) + post_string[post_match.end():]

            # We've exploded the first eligible item - so return the new string
            return prior_string + "0" + post_string

    # No return if no change required
    return None

def split(input_string):
    match = TWO_DIGIT_NUMBER.search(input_string)
    if match:
        prior_string = input_string[:match.start()]
        post_string = input_string[match.end():]
        old_number = int(match.group())
        left_number = math.floor(old_number / 2)
        right_number = math.ceil(old_number / 2)

        return prior_string + f"[{left_number},{right_number}]" + post_string

    return input_string

def add_and_reduce(left, right):
    current = f"[{left},{right}]"

    previous = ""
    while current != previous:
        previous = current

        # First try and explode
        exploded = explode(current)
        if exploded:
            current = exploded
            continue

        # If cannot explode, try and split
        current = split(current)

    return current


def part_a(lines):
    root = lines[0]
    for line in lines[1:]:
        root = add_and_reduce(root, line)
    return calculate_magnitude(root)

def part_b(lines):
    max_magnitude = 0
    for l1, l2 in product(lines, lines):
        if l1 != l2:
            max_magnitude = max(max_magnitude, part_a([l1, l2]))
    return max_magnitude

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_18_a.txt'
    with open(p, "r", encoding="ascii") as f:
        lines = f.readlines()
        print(f"Answer for a is {part_a(lines)}.")
        print(f"Answer for b is {part_b(lines)}.")
