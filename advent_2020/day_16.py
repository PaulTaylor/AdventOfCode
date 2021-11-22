import re

from pathlib import Path


rules_pattern = re.compile(r"([a-z ]+): ([0-9]+\-[0-9]+) (?:or ([0-9]+\-[0-9]+))*")

def parse_input(raw):
    lines = raw.splitlines()

    rules = {}
    my_ticket = None
    nearby_tickets = []

    ptr = 0
    nearby_mode = False
    while ptr < len(lines):
        rules_match = rules_pattern.match(lines[ptr])
        if rules_match:
            groups = rules_match.groups()
            rules[groups[0]] = [ tuple(map(int, g.split("-"))) for g in groups[1:] ]
        elif lines[ptr] == "your ticket:":
            ptr += 1
            my_ticket = list(map(int, lines[ptr].split(",")))
        elif lines[ptr] == "nearby tickets:":
            nearby_mode = True
        elif nearby_mode:
            nearby_tickets.append(list(map(int, lines[ptr].split(","))))

        ptr += 1

    return rules, my_ticket, nearby_tickets

def part_a(raw):
    rules, _, other_tickets = parse_input(raw)

    # Check each ticket to see which fields are invalid
    # and sum them up
    invalid_values = []
    invalid_indexes = []
    for idx, t in enumerate(other_tickets):
        for f in t:
            valid = False
            for _, rule_ranges in rules.items():
                valid = valid or any([ min <= f <= max for (min, max) in rule_ranges ])

            if not valid:
                invalid_values.append(f)
                invalid_indexes.append(idx)

    return sum(invalid_values), invalid_indexes

def part_b(raw):
    rules, my_ticket, other_tickets = parse_input(raw)
    
    # Drop invalid tickets
    _, invalid_indexes = part_a(raw)
    for idx in sorted(invalid_indexes, reverse=True):
        del other_tickets[idx]

    # Work out field mapping
    # To start - all fields could be anywhere
    field_mapping = { f: set(range(0, len(my_ticket))) for f in rules.keys() }
    
    # Check each ticket in turn - and remove invalid field mappings
    for t in other_tickets:
        for f_idx, value in enumerate(t):
            for f_name, f_possibles in field_mapping.items(): # checking to see if field f_idx could be f_name
                if f_idx in f_possibles:
                    # this field is still a possibility for this mapping
                    valid = any([ min <= value <= max for (min, max) in rules[f_name] ])

                    if not valid:
                        # This field is not valid under these rules
                        # - which means that this field_idx cannot be the names field
                        field_mapping[f_name].remove(f_idx)

    # Clean-up the mappings
    final_mapping = {}
    while any(map(lambda x: len(x) > 1, field_mapping)):
        # Find the first field that has a single item in it's set
        field = None
        for field, candidates in field_mapping.items():
            if len(candidates) == 1:
                final_mapping[field] = next(candidates.__iter__())
                del field_mapping[field]
                break

        # This means that field MUST be at the index in it's set so we can
        # remove that index from all other sets
        for k in field_mapping.keys():
            field_mapping[k].remove(final_mapping[field])

    return { k: my_ticket[idx] for k, idx in final_mapping.items() }
    

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_16_a.txt'
    with open(p, "rt") as f:
        raw = f.read()
        
    print(f"Part A - ticket scanning error rate = {part_a(raw)[0]}")

    print("My ticket\n=========================================================")
    my_fields = part_b(raw)
    for k, v in my_fields.items():
        print(f"\t{k}: {v}")

    print("=========================================================")
    answer = 1
    for k, v in my_fields.items():
        if k.startswith("departure"):
            answer *= v
    print(f"Answer to submit = {answer}")
    print("=========================================================")