from .day_07 import *
from collections import Counter

def test_bag_rules():
    rules = """light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.""".splitlines()

    forward_lookup, reverse_lookup = parse_rules(rules)
    assert set(reverse_lookup['bright white']) == { 'light red', 'dark orange' }

    target_bag = 'shiny gold'
    desired = {'light red', 'bright white', 'dark orange', 'muted yellow'}
    assert find_combinations(target_bag, reverse_lookup) == desired

    assert forward_lookup

    counter = Counter(forward_lookup['shiny gold'])
    assert len(forward_lookup['shiny gold']) == 3
    assert counter['dark olive'] == 1
    assert counter['vibrant plum'] == 2

    assert forward_lookup_for(target_bag, forward_lookup) == 32

def test_second_example():
    rules = """shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.""".splitlines()

    forward_lookup, _ = parse_rules(rules)
    target_bag = 'shiny gold'
    assert forward_lookup_for(target_bag, forward_lookup) == 126