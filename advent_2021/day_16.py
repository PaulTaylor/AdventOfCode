"""
Advent of Code 2021 - Day 16
See: https://adventofcode.com/2021/day/16
"""

import io

from math import prod
from pathlib import Path

HEX_TO_BIN = {
    '0': '0000',
    '1': '0001',
    '2': '0010',
    '3': '0011',
    '4': '0100',
    '5': '0101',
    '6': '0110',
    '7': '0111',
    '8': '1000',
    '9': '1001',
    'A': '1010',
    'B': '1011',
    'C': '1100',
    'D': '1101',
    'E': '1110',
    'F': '1111'
}

# Define the functions corresponding to each operator type
OPERATOR_FUNCTIONS = [
    sum,
    prod,
    min,
    max,
    lambda x: x,
    lambda x: 1 if x[0] > x[1] else 0,
    lambda x: 1 if x[0] < x[1] else 0,
    lambda x: 1 if x[0] == x[1] else 0
]

def decode_header(packet_io):
    header_bits = packet_io.read(3)
    type_bits = packet_io.read(3)
    return int(header_bits, 2), int(type_bits, 2)

def decode_literal(packet_io):
    group = packet_io.read(5)
    acc = ""
    while group[0] == "1":
        acc += group[1:]
        group = packet_io.read(5)

    acc += group[1:]
    return int(acc, 2)

def decode_packet(packet_io):
    """
    Decode the next packet and any children
    :returns: the sum of version numbers, and the calculated value
    """

    # read header and type
    version_acc, ptype = decode_header(packet_io)

    if ptype == 4:
        # literal type
        value = decode_literal(packet_io)
    else:
        # Otherwise we're an operator
        subpacket_values = []
        length_type_id = packet_io.read(1)

        # Descend into the substructure appropriately:
        if length_type_id == "0":
            # next 15 bits represent the length of the subpackets
            length = int(packet_io.read(15), 2)
            start_pos = packet_io.tell()
            # Recurse to get the internal packets if we've not reached the
            # end of the specified length
            while (packet_io.tell() - start_pos) != length:
                version, value = decode_packet(packet_io)
                subpacket_values.append(value)
                version_acc += version
        else:
            # Next 11 bits contain the number of subpackets included
            num_subpackets = int(packet_io.read(11), 2)
            for _ in range(num_subpackets):
                version, value = decode_packet(packet_io)
                subpacket_values.append(value)
                version_acc += version

        # Summarise the subpacket values per the operator type
        value = OPERATOR_FUNCTIONS[ptype](subpacket_values)

    return version_acc, value

def part_ab(packet_string):
    # Convert the hex string into a binary character producing StringIO
    packet_io = io.StringIO()
    for c in packet_string:
        packet_io.write(HEX_TO_BIN[c])

    # Return to the beginning and start parsing
    packet_io.seek(0)
    return decode_packet(packet_io)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_16_a.txt'
    with open(p, "r", encoding="ascii") as f:
        hex_packet = f.read()

    a_ans, b_ans = part_ab(hex_packet)

    print(f"Answer for a is {a_ans}.")
    print(f"Answer for b is {b_ans}.")
