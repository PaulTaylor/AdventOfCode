import numba
import numpy as np

from collections import defaultdict, deque
from numba import njit
from numba.typed import List
from numba.experimental import jitclass
from tqdm import tqdm

ND_SPEC = [
    ('array', numba.int64[:])
]

@jitclass(ND_SPEC)
class NumbaDeque():
    def __init__(self):
        self.array = np.full(2, -1)

    def seen_at_round(self, round):
        self.array[1] = self.array[0]
        self.array[0] = round

    def is_full(self):
        return not(-1 in self.array)

    def next_number(self):
        if self.is_full(): 
            return self.array[0] - self.array[1]
        else:
            return 0

@njit
def numba_game(starting_numbers, rounds):
    "Numba accelerated variant of game(...)"
    memory = {}
 
    for idx, num in enumerate(starting_numbers):
        memory[num] = NumbaDeque()
        memory[num].seen_at_round(idx)

    last_number = starting_numbers[-1]
    for round in range(len(starting_numbers), rounds):
        last_number = memory[last_number].next_number()
        if memory.get(last_number) is None:
            memory[last_number] = NumbaDeque()

        memory[last_number].seen_at_round(round)
        
    return last_number

def game(starting_numbers, rounds):
    memory = defaultdict(lambda: deque(maxlen=2))
    for idx, num in enumerate(starting_numbers):
        memory[num].appendleft(idx)

    last_number = starting_numbers[-1]
    for round in tqdm(range(len(starting_numbers), rounds)):
        previous_occurences = memory[last_number]
        if len(previous_occurences) > 1: 
            # if this is only one - then we must have only said the number in the immediately
            # prior round - so it's not actually a duplicate
            new_number = previous_occurences[0] - previous_occurences[1]
            last_number = new_number
        else:
            last_number = 0
            
        memory[last_number].appendleft(round)
        
    return last_number

if __name__ == "__main__":
    input = List([ 1,0,18,10,19,6 ])
    print(f"Part A answer = {numba_game(input, 2020)}")
    print(f"Part B answer = {numba_game(input, 30000000)}")