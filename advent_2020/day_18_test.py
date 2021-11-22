from .day_18 import *

def test_grammar():
    ast = grammar.parse("1 + 2")
    assert ast

def test_evaluate():
    assert evaluate("1 + 2 * 3 + 4 * 5 + 6") == 71
    assert evaluate("1 + (2 * 3) + (4 * (5 + 6))") == 51
    
    assert evaluate("2 * 3 + (4 * 5)") == 26
    assert evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 437
    assert evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 12240
    assert evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 13632

def test_evaluate_b():
    assert evaluate_b("1 + 2 * 3 + 4 * 5 + 6") == 231
    assert evaluate_b("1 + (2 * 3) + (4 * (5 + 6))") == 51
    assert evaluate_b("2 * 3 + (4 * 5)") == 46
    assert evaluate_b("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 1445.
    assert evaluate_b("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 669060
    assert evaluate_b("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 23340
