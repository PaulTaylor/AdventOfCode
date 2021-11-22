import re

from collections import defaultdict, deque
from pathlib import Path

def mask_value(value, mask):
    value_string = f"{value:036b}"
    assert len(value_string) == len(mask)

    new_bit_string = "".join([ 
        c if c != "X" else value_string[idx] 
        for idx, c in enumerate(mask) 
    ])
    
    return int(new_bit_string, 2)

def part_a(lines):
    mask_pattern = re.compile("mask = ([01X]{36})")
    instruction_pattern = re.compile("mem\\[([0-9]+)\\] = ([0-9]+)")
    mem = defaultdict(lambda x: 0)
    mask = None
    for line in lines:
        mask_match = mask_pattern.match(line)
        inst_match = instruction_pattern.match(line)
        if mask_match:
            mask = mask_match.group(1)
        elif inst_match:
            idx, value = list(map(int, inst_match.groups()))
            value_to_insert = mask_value(value, mask)
            mem[idx] = value_to_insert
        else:
            raise Exception(line)

    return sum(mem.values())

def mask_address(address, mask):
    addr_string = f"{address:036b}"

    new_addr_bits = []
    for idx, mask_bit in enumerate(mask):
        if mask_bit == "0":
            # passthrough
            new_addr_bits.append(addr_string[idx])
        elif mask_bit == "1":
            # Overwrite this bit with 1
            new_addr_bits.append("1")
        elif mask_bit == "X":
            # Floating bit - will deal with this later!
            new_addr_bits.append("X")
        else:
            raise Exception("Unknown mask bit")

    output_set = []
    processing_queue = deque([new_addr_bits])
    while processing_queue:
        this_entry = processing_queue.pop()
        if "X" in this_entry:
            # Find the index of the first "X"
            x_idx = this_entry.index("X")
            # Output 2 items - one for each possible value of X
            for replacement in range(0, 2):
                new_entry = this_entry.copy()
                new_entry[x_idx] = str(replacement)
                processing_queue.append(new_entry)
        else:
            # If there are no wildcards to replace - we can use this address as is
            output_set.append(this_entry)

    return [ int("".join(x), 2) for x in output_set ]

def part_b(lines):
    mask_pattern = re.compile("mask = ([01X]{36})")
    instruction_pattern = re.compile("mem\\[([0-9]+)\\] = ([0-9]+)")
    mem = defaultdict(lambda x: 0)
    mask = None
    for line in lines:
        mask_match = mask_pattern.match(line)
        inst_match = instruction_pattern.match(line)
        if mask_match:
            mask = mask_match.group(1)
        elif inst_match:
            addr_in, value = list(map(int, inst_match.groups()))
            addresses_to_set = mask_address(addr_in, mask)
            for addr in addresses_to_set:
                mem[addr] = value
        else:
            raise Exception(line)

    return sum(mem.values())

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_14_a.txt'
    with open(p, "rt") as f:
        lines = f.readlines()
    
    # Part A    
    a_result = part_a(lines)
    print(f"sum(mem) for Part A = {a_result}")

    # Part B
    b_result = part_b(lines)
    print(f"sum(mem) for Part B = {b_result}")