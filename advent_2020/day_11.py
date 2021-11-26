from pathlib import Path

import numpy as np


def prepare_grid(floor_string):
    return np.array([ list(line.strip()) for line in floor_string.split("\n") ])

def do_seat(fp, col, row):
    top_left = (max(0, col - 1), max(0, row - 1))
    # add 2 here because the indexing below is exclusive!
    bottom_right = (min(col + 2, fp.shape[0]), min(row + 2, fp.shape[1]))

    # get the slice of the floor plan between those bounds
    fp_slice = fp[top_left[0]:bottom_right[0], top_left[1]:bottom_right[1]]
    #print("\n", top_left, bottom_right, "\n", fp_slice, "\n--------------------------")

    # Is the actual seat empty?
    if fp[col, row] == "L":
        # Yes - are there any occupied seats next to us? (ie. any occupied in fp_slice)
        if np.sum(fp_slice == "#") == 0:
            # No occupied seats - so we sit down
            return '#'
    elif fp[col, row] == "#":
        # No, it's occupied
        if np.sum(fp_slice == "#") >= 5:
            # four or more adjacent seats are occupied - so we vacate this seat
            # The test about is 5 because we count ourselves in that count
            return 'L'
    else:
        # It's the floor - don't do anything
        pass

    # If we're here - return a noop
    return fp[col, row]

def simulate(floor_plan):
    result = np.copy(floor_plan)

    for col_idx in range(floor_plan.shape[0]):
        for row_idx in range(floor_plan.shape[1]):
            r = do_seat(floor_plan, col_idx, row_idx)
            result[col_idx, row_idx] = r

    return result

def do_seat_b(fp, col, row):

    # The floor never changes :)
    if fp[col, row] == '.':
        return '.'

    # Need to look in all eight directions to find the first seat in that direction
    seats_to_consider = []

    # Top-Left - keep stepping (-1, -1) until we hit a seat or the edge
    cand_col = col
    cand_row = row
    while cand_col > 0 and cand_row > 0:
        cand_col = cand_col - 1
        cand_row = cand_row - 1
        # Do the check
        if fp[cand_col, cand_row] in "#L":
            seats_to_consider.append(fp[cand_col, cand_row])
            break


    # Top-Middle - only decrease row
    cand_row = row
    while cand_row > 0:
        cand_row = cand_row - 1
        # Do the check
        if fp[col, cand_row] in "#L":
            seats_to_consider.append(fp[col, cand_row])
            break

    # Top-Right
    cand_col = col
    cand_row = row
    while cand_col < (fp.shape[0] - 1) and cand_row > 0:
        cand_col = cand_col + 1
        cand_row = cand_row - 1
        # Do the check
        if fp[cand_col, cand_row] in "#L":
            seats_to_consider.append(fp[cand_col, cand_row])
            break

    # Middle-Right
    cand_col = col
    while cand_col < (fp.shape[0] - 1):
        cand_col = cand_col + 1
        # Do the check
        if fp[cand_col, row] in "#L":
            seats_to_consider.append(fp[cand_col, row])
            break

    # Bottom-Right
    cand_col = col
    cand_row = row
    while cand_col < (fp.shape[0] - 1) and cand_row < (fp.shape[1] - 1):
        cand_col = cand_col + 1
        cand_row = cand_row + 1
        # Do the check
        if fp[cand_col, cand_row] in "#L":
            seats_to_consider.append(fp[cand_col, cand_row])
            break

    # Bottom-Middle
    cand_row = row
    while cand_row < (fp.shape[1] - 1):
        cand_row = cand_row + 1
        # Do the check
        if fp[col, cand_row] in "#L":
            seats_to_consider.append(fp[col, cand_row])
            break

    # Bottom-Left
    cand_col = col
    cand_row = row
    while cand_col > 0 and cand_row < (fp.shape[1] - 1):
        cand_col = cand_col - 1
        cand_row = cand_row + 1
        # Do the check
        if fp[cand_col, cand_row] in "#L":
            seats_to_consider.append(fp[cand_col, cand_row])
            break

    # Middle-Left
    cand_col = col
    while cand_col > 0:
        cand_col = cand_col - 1
        # Do the check
        if fp[cand_col, row] in "#L":
            seats_to_consider.append(fp[cand_col, row])
            break

    # Now we should have (maximum of) eight elements in the seats_to_consider list
    num_occupied = sum(1 for x in seats_to_consider if x == '#')
    if num_occupied >= 5:
        return 'L'
    elif num_occupied < 1:
        return '#'

    # Return unchanged if nothing else has happened here
    return fp[col, row]


def simulate_b(floor_plan):
    result = np.copy(floor_plan)

    for col_idx in range(floor_plan.shape[0]):
        for row_idx in range(floor_plan.shape[1]):
            r = do_seat_b(floor_plan, col_idx, row_idx)
            result[col_idx, row_idx] = r

    return result

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_11_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        floor_plan = f.read()

    previous_res = prepare_grid(floor_plan)

    print(f"Floor plan is of shape {previous_res.shape}")

    count = 0
    res = previous_res
    while count < 1 or not np.array_equal(res, previous_res):
        previous_res = res
        res = simulate(previous_res)
        count += 1

    occupied_seats = np.sum(res == "#")

    print(f"Converged in {count} iterations")
    print(f"There are {occupied_seats} occupied seats in the final result")

    # Reset for Simulation B

    previous_res = prepare_grid(floor_plan)
    count = 0
    res = previous_res
    while count < 1 or not np.array_equal(res, previous_res):
        previous_res = res
        res = simulate_b(previous_res)
        count += 1

    occupied_seats = np.sum(res == "#")
    print(f"(Sim B) Converged in {count} iterations")
    print(f"(Sim B) There are {occupied_seats} occupied seats in the final result")
