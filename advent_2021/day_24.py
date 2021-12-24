"""
Advent of Code 2021 - Day 24
See: https://adventofcode.com/2021/day/24

Lots of paper logic/manipulation here - the subreddit as more if 
interested!

Looks like there are 14 blocks of code (one per digit)

** Blocks differ only in constants at rows 5 ,6 and 16
represented as a/b/c **

Taking the equations as per the code and simplifying on paper gives

x = int(((z % 26) + b) != d)
z //= a
z = z * (25 * x) + 1
z = z + ((d + c) * x)

Can simplify further for both x cases (x=0 and x=1)

Now we've got a simpler set of equations throw Z3 at it
(which I'll definietly been looking into more!)
"""
import re
from pathlib import Path
from z3 import *

INT_PATTERN = re.compile("-?[0-9]+")

def parse_code(program):
    parsed = []
    for line in program.splitlines():
        parts = line.split()
        if len(parts) == 2:
            parts.extend([None])
        elif INT_PATTERN.match(parts[2]):
            parts[-1] = int(parts[2])
        
        parsed.append(tuple(parts))
    return parsed

def alu(parsed_code, inp: Iterable[int]):
    state = dict(zip("wxyz", (0,0,0,0)))
    stdin = inp.__iter__()

    for instr, a, b in parsed_code:
        if instr == "inp":
            state[a] = next(stdin)
        elif instr == "add":
            state[a] += state.get(b, b)
        elif instr == "mul":
            state[a] *= state.get(b, b)
        elif instr == "div":
            v = state.get(b, b)
            assert v > 0
            state[a] //= v
        elif instr == "mod":
            va = state[a]
            assert va >= 0 
            vb = state.get(b, b)
            assert vb > 0
            state[a] %= vb
        elif instr == "eql":
            state[a] = 1 if state[a] == state.get(b, b) else 0
        else:
            raise Exception("Unknown instruction")
        
    return tuple(state[v] for v in "wxyz")

def find_solution(code, maximise=True):
    # First grab the important parameters from the code
    block_params = []
    for i in range(0, 14):
        a, b, c = 0, 0, 0
        a = code[i*18+4][-1]
        b = code[i*18+5][-1]
        c = code[i*18+15][-1]
        block_params.append((a,b,c))

    # Create and rig the optimiser
    opt = Optimize()
    z, digits = 0, 0
    for idx, (a, b, c) in enumerate(block_params):
        # Set up the input digit - must be between 1 and 9
        d = Int(f"d{idx}") # the input digit for this interation
        digits = (digits*10) + d
        opt.add(And(d > 0, d <= 9))

        z = If(
            (((z % 26) + b) == d),
            # x == 0,
            z/a,
            # x == 1
            ((z/a)*26)+d+c
        )
    
    opt.add(z == 0) # Must finsh with z == 0
    
    if maximise:
        opt.maximize(digits) # Want the largest answer
    else:
        opt.minimize(digits) # Want the smallest answer

    assert opt.check() == sat # Validate model
    return opt.model().eval(digits) # Get answer

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_24_a.txt'
    with open(p, "r", encoding="ascii") as f:
        instructions = f.read()
        code = parse_code(instructions)

    a_ans = find_solution(code)
    b_ans = find_solution(code, maximise=False)

    print(f"Answer for a is {a_ans}.")
    print(f"Answer for b is {b_ans}.")

    # Lets check with our virtual ALU just for fun :)
    assert alu(code, map(int, str(a_ans)))[-1] == 0
    assert alu(code, map(int, str(b_ans)))[-1] == 0
    print("ALU validates both serials")
