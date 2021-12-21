"""
Advent of Code 2021 - Day 20
See: https://adventofcode.com/2021/day/20
"""

from pathlib import Path

import numpy as np

BINARY_CONV_MATRIX = 2**np.arange(9)[::-1]

def _slice_to_idx(b) -> int:
    """
    Convert a 9 element bit array into a single integer value

    Uses a converstion matrix to convert the numpy 0/1 array
    into an integer value faster than via string & int(s,2)
    References: https://stackoverflow.com/a/41069967 &
                https://stackoverflow.com/a/59273656
    """
    return b.dot(BINARY_CONV_MATRIX)

def parse_input(input_string: str):
    lines = input_string.splitlines()
    enh_algo = lines[0].strip()
    image = np.array([
        [ 1 if c == "#" else 0 for c in line ]
        for line in lines[2:]
    ], dtype=int)
    return enh_algo, image

def process_image(enh_algo, image, fill_value: str):
    # fill_value contains the current value of the infinite space
    output = np.zeros((image.shape[0]+2, image.shape[1]+2), dtype=int)
    for row in range(-1, image.shape[0]+1):
        for col in range(-1, image.shape[1]+1):
            bits = ""
            for source_row in range(row-1, row+2):
                if source_row < 0 or source_row > image.shape[0]-1:
                    bits += fill_value * 3  # OOB -> fill_value
                    continue

                for source_col in range(col-1, col+2):
                    if source_col < 0 or source_col > image.shape[1]-1:
                        bits += fill_value # OOB -> fill_value
                    else:
                        bits += "1" if image[source_row, source_col] else "0"

            bit_value_idx = int(bits, 2)
            output_value = 1 if enh_algo[bit_value_idx] == "#" else 0
            output[row+1, col+1] = output_value

    # Determine the new fill_value
    if fill_value == "0" and enh_algo[0] == "#":
        fill_value = "1"
    elif fill_value == "1" and enh_algo[-1] == ".":
        fill_value = "0"

    return output, fill_value

def part_ab(enh_algo, image) -> int:
    image, fill_value = process_image(enh_algo, image, "0")
    image, fill_value = process_image(enh_algo, image, fill_value)
    a = image.sum()
    for _ in range(2, 50):
        image, fill_value = process_image(enh_algo, image, fill_value)
    return a, image.sum()

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_20_a.txt'
    with open(p, "r", encoding="ascii") as f:
        enh_algo, input_image = parse_input(f.read())
        a, b = part_ab(enh_algo, input_image)
        print(f"Answer for a is {a}.")
        print(f"Answer for b is {b}.")
