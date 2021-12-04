"""
Advent of Code 2021 - Day 2
See: https://adventofcode.com/2021/day/2
"""

from collections import deque
from pathlib import Path

def parse_line(line):
    parts = line.split(" ")
    return (parts[0], int(parts[1]))

def parse_lines(in_string):
    return [ parse_line(line) for line in in_string.split("\n") ]

def part_a(instructions):
    horizontal_pos = 0
    vertical_pos = 0

    for direction, num in instructions:
        if direction == "forward":
            horizontal_pos += num
        elif direction == "up":
            vertical_pos -= num
        elif direction == "down":
            vertical_pos += num
        else:
            raise Exception("WTF!")

    return horizontal_pos * vertical_pos

def part_b(instructions):
    horizontal_pos = 0
    vertical_pos = 0
    aim = 0

    for direction, num in instructions:
        if direction == "forward":
            horizontal_pos += num
            vertical_pos += aim * num
        elif direction == "up":
            aim -= num
        elif direction == "down":
            aim += num
        else:
            raise Exception("WTF!")

    return horizontal_pos * vertical_pos

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_02_a.txt'
    with open(p, "r", encoding="ascii") as f:
        instructions = parse_lines(f.read())
        print(f"Answer for a is {part_a(instructions)}.")
        print(f"Answer for b is {part_b(instructions)}.")
