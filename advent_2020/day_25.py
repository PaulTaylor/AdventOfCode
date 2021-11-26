from numba import njit

MAX_LOOP_SIZE_TO_CHECK = 100_000_000

@njit
def transform_sn(sn, loop_size):
    acc = 1
    for _ in range(loop_size):
        # Set the value to itself multiplied by the subject number.
        acc *= sn
        # Set the value to the remainder after dividing the value by 20201227
        acc = (acc % 20201227)

    return acc

@njit
def find_loop_size(pk):
    sn = 7
    acc = 1
    for ls in range(MAX_LOOP_SIZE_TO_CHECK):
        acc *= sn
        acc = (acc % 20201227)

        if pk == acc:
            return ls + 1

    raise Exception("Loop size not found!")

def do_part_a(card_pk, door_pk):
    card_loop_size = find_loop_size(card_pk)
    door_loop_size = find_loop_size(door_pk)

    card_ek = transform_sn(door_pk, card_loop_size)
    door_ek = transform_sn(card_pk, door_loop_size)

    assert card_ek == door_ek
    return card_ek

if __name__ == "__main__":
    card_pk = 17607508
    door_pk = 15065270

    print(f"EK = {do_part_a(card_pk, door_pk)}")
