"""
Advent of Code 2021 - Day 20
See: https://adventofcode.com/2021/day/20
"""

from pathlib import Path

import numpy as np

BINARY_CONV_MATRIX = 2**np.arange(9)[::-1]

def _slice_to_idx(image_slice) -> int:
    """
    Convert a 9 element bit array into a single integer value

    Uses a converstion matrix to convert the numpy 0/1 array
    into an integer value faster than via string & int(s,2)
    References: https://stackoverflow.com/a/41069967 &
                https://stackoverflow.com/a/59273656
    """
    b = image_slice.reshape((1,9))[0]
    return b.dot(BINARY_CONV_MATRIX)

def parse_input(input_string: str):
    lines = input_string.splitlines()
    enh_algo = lines[0].strip()
    image = np.array([
        [ 1 if c == "#" else 0 for c in line ]
        for line in lines[2:]
    ], dtype=int)
    return enh_algo, image

def process_image(enh_algo, image, fill_value):
    # fill_value contains the current value of the infinite space
    min_row, max_row = 0, image.shape[0]-1
    min_col, max_col = 0, image.shape[1]-1

    output = np.zeros((max_row+3, max_col+3), dtype=int)
    for output_row_idx, row in enumerate(range(min_row-1, max_row+2)):
        for output_col_idx, col in enumerate(range(min_col-1, max_col+2)):
            source_start_row = row-1
            source_end_row = row+2
            source_start_col = col-1
            source_end_col = col+2

            image_slice = np.zeros((3,3), dtype=int)

            for slice_row, source_row in enumerate(range(source_start_row, source_end_row)):
                if source_row < 0 or source_row > max_row:
                    # OOBs are filled with the fill_value
                    image_slice[slice_row] = fill_value
                    continue

                for slice_col, source_col in enumerate(range(source_start_col, source_end_col)):
                    if source_col < 0 or source_col > max_col:
                        # OOBs are filled with the fill_value
                        image_slice[slice_row, slice_col] = fill_value
                        continue

                    image_slice[slice_row, slice_col] = image[source_row, source_col]

            bit_value_idx = _slice_to_idx(image_slice)
            output_value = 1 if enh_algo[bit_value_idx] == "#" else 0
            output[output_row_idx, output_col_idx] = output_value

    # Determine the new fill_value
    if fill_value == 0 and enh_algo[0] == "#":
        # Empty areas would become 1's
        fill_value = 1
    elif fill_value == 1 and enh_algo[-1] == ".":
        # Empty areas change from all 1's to all 0's
        fill_value = 0

    return output, fill_value

def part_a(enh_algo, image) -> int:
    image, fill_value = process_image(enh_algo, image, 0)
    image, fill_value = process_image(enh_algo, image, fill_value)
    return image.sum()

def part_b(enh_algo, image) -> int:
    fill_value = 0
    for _ in range(0, 50):
        image, fill_value = process_image(enh_algo, image, fill_value)
    return image.sum()

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_20_a.txt'
    with open(p, "r", encoding="ascii") as f:
        enh_algo, input_image = parse_input(f.read())
        print(f"Answer for a is {part_a(enh_algo, input_image)}.")
        print(f"Answer for b is {part_b(enh_algo, input_image)}.")
