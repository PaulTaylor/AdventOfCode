from .day_14 import mask_value, part_a, part_b

def test_mask_value():
    mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
    test_values = [
        (11, 73),
        (101, 101),
        (0, 64)
    ]

    for input_value, desired in test_values:
        assert mask_value(input_value, mask) == desired

def test_part_a():
    lines = """mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0""".splitlines()

    assert part_a(lines) == 165

def test_part_b():
    lines = """mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1""".splitlines()

    assert part_b(lines) == 208
