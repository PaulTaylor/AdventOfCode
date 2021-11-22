import re

from pathlib import Path

pattern = re.compile("^([0-9]+)-([0-9]+) (\\S+): (\\S+)$")

def single_line_policy1(line):
    m = pattern.match(line)
    min_count, max_count, letter, password = m.groups()
    min_count, max_count = int(min_count), int(max_count)
    
    letter_count = sum(map(lambda c: c == letter, password))
    return letter_count in range(min_count, max_count + 1)

def policy1(lines):
    res = [ single_line_policy1(line) for line in lines ]
    return res

def single_line_policy2(line):
    m = pattern.match(line)
    first_pos, second_pos, letter, password = m.groups()
    first_pos, second_pos = int(first_pos), int(second_pos)

    # first/second_pos are indexed from 1 not 0!
    first_match = (password[first_pos - 1] == letter)
    second_match = (password[second_pos - 1] == letter)

    return (int(first_match) + int(second_match)) == 1

def policy2(lines):
    return [ single_line_policy2(line) for line in lines ]

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_02_a.txt'
    with open(p, "rt") as f:
        lines = f.readlines()
        res = policy1(lines)
        print(f"{sum(res)} passwords are valid by policy 1")
        res = policy2(lines)
        print(f"{sum(res)} passwords are valid by policy 2")