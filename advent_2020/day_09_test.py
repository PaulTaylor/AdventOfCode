from .day_09 import *

sequence = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95,
            102, 117, 150, 182, 127, 219, 299, 277, 309, 576]

def test_xmas_a():
    
    xd = XmasDecoder(preamble_length=5)
    try:
        for num in sequence:
            xd.process(num)
        assert False, "Should not complete successfully"
    except XmasException as e:
        assert e.number == 127

def test_xmas_b():
    wrong_number = 127
    result = find_weakness(sequence, wrong_number)
    assert result == 62