from .day_04 import *
from pathlib import Path


def test_normal_input():
    p = Path(__file__).parent / "input" / 'day_04_test_input.txt'
    with open(p, "rt") as f:
        lines = f.readlines()
        
    res, _ = process_lines(lines)
    assert res == 2

def test_strict_valid():
    p = Path(__file__).parent / "input" / 'day_04_test_strict_valid.txt'
    with open(p, "rt") as f:
        lines = f.readlines()
        
    _, res = process_lines(lines)
    assert res == 4    

def test_strict_invalid():
    p = Path(__file__).parent / "input" / 'day_04_test_strict_invalid.txt'
    with open(p, "rt") as f:
        lines = f.readlines()
        
    _, res = process_lines(lines)
    assert res == 0
