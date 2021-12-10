import pytest

from .day_10 import *

@pytest.fixture
def valid_input():
    return """([])
{()()()}
<([{}])>
[<>({}){}[([])<>]]
(((((((((())))))))))"""

@pytest.fixture
def invalid_input():
    return """{([(<{}[<>[]}>{[]{[(<()>
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{"""

@pytest.fixture
def incomplete_input():
    return """[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
(((({<>}<{<{<>}{[]{[]{}
{<[[]]>}<{[{[{[]{()[[[]
<{([{{}}[<[[[<>{}]]]>[]]"""

def test_part_a(valid_input, invalid_input):
    assert part_a(valid_input.splitlines()) == 0
    assert part_a(invalid_input.splitlines()) == 26397

def test_part_b(valid_input, incomplete_input):
    assert part_b(incomplete_input.splitlines()) == 288957
