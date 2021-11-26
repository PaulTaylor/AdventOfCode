from collections import defaultdict, deque
import numpy as np

from tqdm import tqdm

def game_with_array(starting_numbers, rounds):
    memory = np.full(rounds, -1)
    memory[0:len(starting_numbers)] = starting_numbers

    for current_round in tqdm(range(len(starting_numbers), rounds)):
        current_view = memory[0:(current_round-1)]
        previous_number = current_view=[-1]
        if previous_number in memory[0:current_round-1]:
            # has been previously spoken
            # the next number to speak is the difference between the turn number
            # when it was last spoken and the time it was spoken before that
            previous_indexes = np.argwhere(current_view == previous_number)[-2:, 0]
            second_last_idx, last_idx = previous_indexes
            next_number = last_idx - second_last_idx
            memory[current_round] = next_number
        else:
            # has not been previously spoken
            memory[current_round] = 0

    return memory[-1]

def game(starting_numbers, rounds):
    memory = defaultdict(lambda: deque(maxlen=2))
    for idx, num in enumerate(starting_numbers):
        memory[num].appendleft(idx)

    last_number = starting_numbers[-1]
    for current_round in tqdm(range(len(starting_numbers), rounds)):
        previous_occurences = memory[last_number]
        if len(previous_occurences) > 1:
            # if this is only one - then we must have only said the number in the
            # immediately prior round - so it's not actually a duplicate
            new_number = previous_occurences[0] - previous_occurences[1]
            last_number = new_number
        else:
            last_number = 0

        memory[last_number].appendleft(current_round)

    return last_number


if __name__ == "__main__":
    input_values = [ 1,0,18,10,19,6 ]
    print(f"Part A answer = {game(input_values, 2020)}")
    print(f"Part A answer = {game(input_values, 30000000)}")
