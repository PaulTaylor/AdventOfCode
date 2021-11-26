import math

from pathlib import Path

def decode_seat_string(full_string):
    row_string = full_string[0:7]
    seat_string = full_string [7:]

    # Row
    window_start, window_end = (0, 127)
    for char in row_string[0:-1]:
        mid_point = (window_end - window_start) / 2.0
        if char == "F": # Front half
            window_end = window_start + math.floor(mid_point)
        elif char == "B":
            window_start = window_end - math.floor(mid_point)
        else:
            raise Exception(f"Wat! - {char}")

    # Now use the last char to say which to select
    row = window_start if row_string[-1] == "F" else window_end

    # Seat
    window_start, window_end = 0, 7
    for char in seat_string[0:-1]:
        mid_point = (window_end - window_start) / 2.0
        if char == "L":   # Left
            window_end = window_start + math.floor(mid_point)
        elif char == "R": # Right
            window_start = window_end - math.floor(mid_point)
        else:
            raise Exception(f"Wat! - {char}")

    # Now use the last char to say which to select
    seat = window_start if seat_string[-1] == "L" else window_end

    # Sanity checking
    assert 0 <= row <= 127
    assert 0 <= seat <= 7

    # unique seat ID: multiply the row by 8, then add the column
    seat_id = (row * 8) + seat

    return row, seat, seat_id

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_05_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        lines = [ x.strip() for x in f.readlines() ]

    res = [ decode_seat_string(line)[-1] for line in lines]
    print(f"Max Seat ID = {max(res)}")

    # Looking for exist, not, exist triples in the range of seat ids
    for test_sid in range(1, max(res)):
        if (test_sid - 1) in res:
            if test_sid not in res:
                if (test_sid + 1) in res:
                    print(f"Candiate for the seat id = {test_sid}")
