import re, sys
from pathlib import Path

instruction_pattern = re.compile(r'(nop|acc|jmp) ([-+][0-9]+)')

class LoopException(Exception):
    def __init__(self, ptr, acc):
        self.ptr = ptr
        self.acc = acc

def run_code_safe(raw_code):
    ptr = 0
    acc = 0

    code_lines = raw_code.splitlines()
    visited_lines = set()
    ptr_history = list()

    while ptr < len(code_lines): 
        # the pointer hasn't dropped off the end of the code

        if ptr in visited_lines:
            raise LoopException(ptr, acc)
        visited_lines.add(ptr)
        ptr_history.append(ptr)

        line = code_lines[ptr]
        op, num = instruction_pattern.match(line).groups()
        num = int(num)

        if op == 'acc':
            acc += num
            ptr += 1
        elif op == 'jmp':
            ptr += num
        elif op == 'nop':
            ptr += 1
    
    print("Terminated normally")
    return acc

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_08_a.txt'
    with open(p, "rt") as f:
        raw_code = f.read()

    try:
        run_code_safe(raw_code)
    except LoopException as e:
        print(e.__repr__())

    print("Trying to modify code:")

    # Trying to edit the code to find a terminating version
    code_lines = raw_code.splitlines()
    
    for idx in range(len(code_lines)):
        modified_code = code_lines.copy()
        if "jmp" in code_lines[idx]:
            modified_code[idx] = modified_code[idx].replace("jmp", "nop")
        elif "nop" in code_lines[idx]:
            modified_code[idx] = modified_code[idx].replace("nop", "jmp")
        else:
            # cannot change this line so skip the check
            continue

        try:
            acc = run_code_safe("\n".join(modified_code))
            print(f"**This version terminated with acc = {acc}**")
            break
        except:
            pass # this version didn't terminate

    