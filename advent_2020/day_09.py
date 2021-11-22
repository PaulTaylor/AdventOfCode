from collections import deque
from pathlib import Path

class XmasException(Exception):
    def __init__(self, number):
        self.number = number

class XmasDecoder:
    def __init__(self, preamble_length):
        self.preamble_length = preamble_length
        self.sliding_window = deque(maxlen=preamble_length)

    def process(self, number):
        if len(self.sliding_window) < self.preamble_length:
            # can just add numbers - we've not filled the buffer yet
            self.sliding_window.append(number)
        elif self.is_valid_number(number):
                self.sliding_window.popleft()
                self.sliding_window.append(number)
        else:
            # Not a valid input
            raise XmasException(number)

    def is_valid_number(self, number):
        for a_idx, a in enumerate(self.sliding_window):
            for b_idx, b in enumerate(self.sliding_window):
                if (a_idx != b_idx) and ((a + b) == number):
                    return True
        
        return False

def find_weakness(sequence, wrong_number):
    for a_idx in range(len(sequence)):
        for b_idx in range(a_idx + 1, len(sequence)):
            candidate = sequence[a_idx:b_idx]
            sequence_sum = sum(candidate)
            if sequence_sum == wrong_number:
                return min(candidate) + max(candidate)
    
    raise Exception(f"Couldn't find the correct subsequence summing to {wrong_number}")

if __name__ == "__main__":
    wrong_number = -1
    
    p = Path(__file__).parent / "input" / 'day_09_a.txt'
    with open(p, "rt") as f:
        input_numbers = [ int(x) for x in f.readlines() ]

    try:
        xd = XmasDecoder(preamble_length=25)
        for num in input_numbers:
            xd.process(num)
        assert False, "Should not complete successfully"
    except XmasException as e:
        print(f"First number with an error is {e.number}")
        wrong_number =  e.number

    # Part B
    result = find_weakness(input_numbers, wrong_number)
    print(result)
