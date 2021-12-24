from pathlib import Path
from .day_24 import alu, parse_code

def test_alu():
    example_input = parse_code("""inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2""")

    assert alu(example_input, [1]) == (0,0,0,1)
    assert alu(example_input, [2]) == (0,0,1,0)
    assert alu(example_input, [15]) == (1,1,1,1)

    example_input = parse_code("inp w\nmul w 2\ninp x\neql w x")
    assert alu(example_input, [2,4]) == (1,4,0,0)

def test_check_code():
    p = Path(__file__).parent / "input" / 'day_24_a.txt'
    with open(p, "r", encoding="ascii") as f:
        instructions = f.read()
    parsed = parse_code(instructions)
    assert alu(parsed, map(int, "13579246899999"))