from .day_06 import *

test_input = """abc

a
b
c

ab
ac

a
a
a
a

b""".split("\n\n")

def test_anyone():
    test_results = [
        set('abc'),
        set('abc'),
        set('abc'),
        set('a'),
        set('b')
    ]

    count = 0
    for input, desired in zip(test_input, test_results):
        actual = anyone_result(input)
        assert actual == desired
        count += len(actual)

    assert count == 11

def test_everyone():
    test_results = [
        list('abc'),
        [],
        list('a'),
        list('a'),
        list('b')
    ]

    count = 0
    for input, desired in zip(test_input, test_results):
        actual = everyone_result(input)
        assert actual == desired
        count += len(actual)

    assert count == 6