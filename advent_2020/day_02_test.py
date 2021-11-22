from .day_02 import *

def test_ex2a():
    x = [
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    ]

    assert policy1(x) == [True, False, True]

def test_ex2b():
    x = [
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    ]

    assert policy2(x) == [True, False, False]

