import collections

from pathlib import Path

def anyone_result(input_text):
    r = set()
    for line in input_text.split("\n"):
        for c in line:
            r.add(c)

    return r

def everyone_result(input_text):
    input_text = input_text.strip()
    counter = collections.Counter()
    lines = input_text.split("\n")
    for line in lines:
        counter.update(line.strip())

    res = [ q for q, freq in counter.items() if freq == len(lines) ]
    #print("=====================\n", input_text, res)
    return res

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_06_a.txt'

    with open(p, "rt", encoding="ascii") as f:
        group_strings = f.read().split("\n\n")

    group_sets = [ anyone_result(input_text) for input_text in group_strings ]
    count = sum(len(s) for s in group_sets)
    print(f"Total anyone count = {count}")

    everyone_sets = [ everyone_result(input_text) for input_text in group_strings ]
    count = sum(len(s) for s in everyone_sets)
    print(f"Total everyone count = {count}")
