import re

from collections import defaultdict, deque
from pathlib import Path

count_and_colour_pattern = re.compile("([0-9]+) (.*)$")

def parse_rules(lines):
    forward_lookup = defaultdict(list)
    reverse_lookup = defaultdict(list)

    for raw in lines:
        rule_fwd_lookup, rule_rev_lookup = parse_single_rule(raw.strip())
        for k, v_list in rule_rev_lookup.items():
            reverse_lookup[k].extend(v_list)
        for k, v_list in rule_fwd_lookup.items():
            forward_lookup[k].extend(v_list)

    return forward_lookup, reverse_lookup

def parse_single_rule(line):
    forward_lookup = defaultdict(list)
    reverse_lookup = defaultdict(list)

    line = line.strip()
    parent, contents = line.split("bags contain")
    parent_colour = parent.strip()

    for content in contents.split(", "):
        content = content.strip()
        content_parts = content.split(" ")

        if content_parts[0] == "no":
            continue

        number = int(content_parts[0])
        colour_name = " ".join(content_parts[1:-1])

        # Want to create a reverse lookup - so inner colour is the key
        # and the value added is the outer colour
        reverse_lookup[colour_name].append(parent_colour)


        # Now forward lookup
        forward_lookup[parent_colour].extend([colour_name] * number)

    return forward_lookup, reverse_lookup

def find_combinations(bag, reverse_lookup):
    # Results initially contain those bags than can directly contain a $bag
    r = set(reverse_lookup[bag])

    # Initialise a queue with those colours to check recursively
    q = deque(r)

    while q:
        next_bag = q.pop()
        new_bags = reverse_lookup[next_bag]
        r.update(new_bags)
        q.extend(new_bags)

    return r

def forward_lookup_for(target_bag, forward_lookup):
    h = list([target_bag])
    q = deque(h)

    # because iterations will immediately be incremented for the
    # "master" bag and we don't count that
    iterations = -1

    while q:
        iterations += 1
        bag = q.popleft()
        q.extend(forward_lookup[bag])
        h.extend(forward_lookup[bag])

    return iterations

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_07_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        rules = f.readlines()

    forward_lookup, reverse_lookup = parse_rules(rules)
    target_bag = 'shiny gold'
    print("Part A = ", len(find_combinations(target_bag, reverse_lookup)))

    # Second part
    iterations = forward_lookup_for(target_bag, forward_lookup)
    print("Part B = ", iterations)
