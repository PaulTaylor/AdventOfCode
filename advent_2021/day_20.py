"""
Advent of Code 2021 - Day 20
See: https://adventofcode.com/2021/day/20
"""

from pathlib import Path

import numpy as np

def parse_input(input_string: str):
    lines = input_string.splitlines()
    enh_algo = lines[0].strip()

    # Instead of a dense matrix, we'

    image = np.array([
        [ 1 if c == "#" else 0 for c in line ] 
        for line in lines[2:] 
    ], dtype=int)
    return enh_algo, image

def process_image(enh_algo, image, fill_value):
    # fill_value contains the current value of the infinite space
    min_row, max_row = 0, image.shape[0]-1
    min_col, max_col = 0, image.shape[1]-1

    output_lists = []    
    for row in range(min_row-1, max_row+2):
        output_row = []
        for col in range(min_col-1, max_col+2):
            source_start_row = row-1
            source_end_row = row+2
            source_start_col = col-1
            source_end_col = col+2

            image_slice = np.zeros((3,3))

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

            flattened_slice = image_slice.reshape((1,9))
            bit_value = int("".join( f"{flattened_slice[0,x]:.0f}" for x in range(0, 9)), 2)
            output_value = 1 if enh_algo[bit_value] == "#" else 0
            output_row.append(output_value)

        output_lists.append(output_row)

    # Determine the new fill_value
    if fill_value == 0 and enh_algo[0] == ".":
        # all zeros would become 0's fill_value stays as 0
        pass
    elif fill_value == 0 and enh_algo[0] == "#":
        # Empty areas would become 1's
        fill_value = 1
    elif fill_value == 1 and enh_algo[-1] == ".":
        # Empty areas change from all 1's to all 0's
        fill_value = 0
    elif fill_value == 1 and enh_algo[-1] == "#":
        # empty areas are all 1's and will stay as 1's
        pass

    # Convert the list of output values into a sparse_matrix
    return np.array(output_lists), fill_value

def part_a(enh_algo, image) -> int:
    print(image.shape)
    image, fill_value = process_image(enh_algo, image, 0)
    print(image.shape)
    image, fill_value = process_image(enh_algo, image, fill_value)
    print(image.shape)
    res = image.sum()
    assert res < 5311, res
    return res

def part_b(*a):
    pass

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_20_a.txt'
    with open(p, "r", encoding="ascii") as f:
        enh_algo, input_image = parse_input(f.read())
        print(f"Answer for a is {part_a(enh_algo, input_image)}.")
