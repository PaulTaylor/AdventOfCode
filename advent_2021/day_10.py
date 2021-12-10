"""
Advent of Code 2021 - Day 10
See: https://adventofcode.com/2021/day/10
"""

from collections import deque
from pathlib import Path

SCORES = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137
}

B_SCORES = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4
}

PAIRS = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">"
}

def check_line_valid(line):
    stack = deque()
    for c in line:
        if c in "([{<":
            stack.append(c)
        else:
            opener = stack.pop()
            if opener == "(" and c == ")":
                pass
            elif opener == "[" and c == "]":
                pass
            elif opener == "{" and c == "}":
                pass
            elif opener == "<" and c == ">":
                pass
            else:
                # this is a corrupted sequence
                return False, c

    return True, None

def part_a(lines):
    checker_results = [ check_line_valid(line) for line in lines ]
    scores = [ 0 if valid else SCORES[c] for valid, c in checker_results ]
    return sum(scores)

def fix_line(line):
    stack = deque()
    for c in line:
        if c in "([{<":
            stack.append(c)
        else:
            opener = stack.pop()
            if opener == "(" and c == ")":
                pass
            elif opener == "[" and c == "]":
                pass
            elif opener == "{" and c == "}":
                pass
            elif opener == "<" and c == ">":
                pass
            else:
                # this is a corrupted sequence - ignore
                return None

    # The sequence might be incomplete - it's waiting for the matching brackets
    # that are stored (reversed) in the stack - return those characters in the
    # correct order
    return [ PAIRS[c] for c in reversed(stack) ]

def score_fix(fix):
    acc = 0
    for c in fix:
        acc *= 5
        acc += B_SCORES[c]

    return acc

def part_b(lines):
    required_fixes = [ fix_line(line) for line in lines ]
    scores = [ score_fix(fix) for fix in required_fixes if fix ]
    sorted_scores = sorted(scores)
    return sorted_scores[len(sorted_scores) // 2]

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_10_a.txt'
    with open(p, "r", encoding="ascii") as f:
        instructions = [ l.strip() for l in f.readlines() ]
        print(f"Answer for a is {part_a(instructions)}.")
        print(f"Answer for b is {part_b(instructions)}.")
