from parsimonious import ParseError
from .day_19 import rules_to_grammar

test_input = """0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"""

def test_part_a():
    rules, input_lines = test_input.split("\n\n")
    grammar = rules_to_grammar(rules)

    success = []
    failures = []
    for idx, line in enumerate(input_lines.splitlines()):
        try:
            grammar.parse(line)
            success.append(idx)
        except ParseError:
            failures.append(idx)

    assert success == [ 0, 2 ]
    assert failures == [ 1, 3, 4 ]

def test_part_b():
    b_input = """42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"""

    rules, input_lines = b_input.split("\n\n")
    grammar = rules_to_grammar(rules)
    rules_b = rules.replace("8: 42", "8: 42 | 42 8").replace("11: 42 31", "11: 42 31 | 42 11 31")

    grammar_b = rules_to_grammar(rules_b)

    successes = 0
    failures = 0
    successes_b = 0
    failures_b = 0
    for line in input_lines.splitlines():
        try:
            grammar.parse(line)
            successes += 1
        except ParseError:
            failures += 1
        try:
            grammar_b.parse(line)
            print(line, " - matched")
            successes_b += 1
        except ParseError as e:
            print(line, " - FAIL")

            if "rule_31" in e.__repr__():
                successes_b += 1
            else:
                failures_b += 1

    assert successes == 3
    assert successes_b == 12
