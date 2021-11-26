from collections import defaultdict
from pathlib import Path


def part_a(lines):
    """
    Each allergen is found in exactly one ingredient. Each ingredient contains zero or one allergen.
    => **Any ingredient that appears along side multiple allergens must not be an allergen**
    """

    allergen_lines = defaultdict(list)
    all_ingredients = set()
    for line in lines:
        line_ingredients = set(line[0:line.index("(")].strip().split(" "))
        line_allergens = line[line.index("contains ")+9:-1].strip().split(" ")
        all_ingredients.update(set(line_ingredients))

        for a in line_allergens:
            allergen_lines[a.replace(",", "")].append(line_ingredients)

    allergen_candidates = {}
    for allergen, a_lines in allergen_lines.items():
        s = None
        for i in a_lines:
            if s:
                s = s.intersection(i)
            else:
                s = i
        allergen_candidates[allergen] = s

    counter = 0
    while sum(len(s) for s in allergen_candidates.values()) > len(allergen_candidates):
        counter += 1
        # If we still have duplicate candidates
        # 1) look for single entries - these must be confirmed
        singletons = { k: v for k, v in allergen_candidates.items() if len(v) == 1 }
        all_singleton_ingredients = { list(v)[0] for v in singletons.values() }

        other_keys = set(allergen_candidates.keys()) - set(singletons.keys())

        for k in other_keys:
            for si_s in all_singleton_ingredients:
                try:
                    allergen_candidates[k].remove(si_s)
                except KeyError:
                    pass

    print(f"Resolved after {counter} rounds.")

    bad_ingredients = set()
    for v in allergen_candidates.values():
        bad_ingredients.update(v)

    good_ingredients = all_ingredients - bad_ingredients
    good_count = 0
    for gi in good_ingredients:
        for l in lines:
            if gi in l.split(" "):
                good_count += 1

    return good_count, allergen_candidates

def part_b(allergen_candidates):
    b_res = [
        next(x[1].__iter__())
        for x in sorted(allergen_candidates.items(), key=lambda x: x[0])
    ]
    return ",".join(b_res)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_21_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        raw = f.read()

    raw_lines = raw.splitlines()
    good_count, allergen_candidates = part_a(raw_lines)
    print(f"Answer to Part A = {good_count}")

    b_res_string = part_b(allergen_candidates)
    print(f"Answer to Part B = {b_res_string}")
