import re

from parsimonious import Grammar, ParseError
from pathlib import Path


def rules_to_grammar(raw_rules):
    rule_strings = raw_rules.split("\n")

    grammar_strings = sorted([
        re.subn("([0-9]+)", 
                   r"rule_\1",
                   rule.replace(": ", " = ( ").replace(" | " , " ) / ( "))[0] + " )"
        for rule in rule_strings
    ], key=lambda x: int(x[(x.index("_")+1):x.index(" =")]))

    grammar_string = "\n".join(grammar_strings)

    return Grammar(grammar_string).default("rule_0")

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_19_a.txt'
    with open(p, "rt") as f:
        raw = f.read().strip()

    rules, input = raw.split("\n\n")

    grammar = rules_to_grammar(rules)

    successes = 0
    failures = 0
    for line in input.splitlines():
        try:
            grammar.parse(line)
            successes += 1
        except ParseError:
            failures += 1

    print(f"Number of correct entries = {successes}")

    # Part B

    rules_b = rules.replace("8: 42", "8: 42 | 42 8").replace("11: 42 31", "11: 42 31 | 42 11 31")
    grammar_b = rules_to_grammar(rules_b)
    successes = 0
    failures = 0
    for line in input.splitlines():
        try:
            grammar_b.parse(line)
            successes += 1
        except ParseError as e:
            # See https://dev.to/meseta/advent-of-code-day-19-abusing-peg-grammar-in-python-the-way-it-s-not-supposed-to-2beg
            # for notes on this hack - not entirely sure if this is parsimonious or a more general issue
            if "rule_31" in e.__repr__():
                successes += 1
            else:
                failures += 1

    assert successes > 163, "163 is not the correct answer"
    print(f"Number of correct entries = {successes}")