from pathlib import Path
from typing import List

def worker2(x: List[int], sum_target=2020) -> int:
    for a_idx, a in enumerate(x):
        for b_idx, b in enumerate(x):
            if a_idx == b_idx:
                continue

            if (a + b) == sum_target:
                return a * b

    return None

def worker3(x: List[int], sum_target=2020) -> int:
    for a_idx, a in enumerate(x):
        for b_idx, b in enumerate(x):
            for c_idx, c in enumerate(x):
                if (a_idx == b_idx) or (b_idx == c_idx) or (a_idx == c_idx):
                    continue

                if (a + b + c) == sum_target:
                    return a * b * c

    return None

def worker3_new(x: List[int], sum_target=2020) -> int:
    for a_idx, a in enumerate(x):
        for b_idx, b in enumerate(x):
            if a + b >= sum_target:
                continue

            for c_idx, c in enumerate(x):
                if (a_idx == b_idx) or (b_idx == c_idx) or (a_idx == c_idx):
                    continue

                if (a + b + c) == sum_target:
                    return a * b * c

    return None

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_01_a.txt'
    with open(p, "r", encoding="ascii") as f:
        values = [ int(x) for x in f.readlines() ]
        print(f"Answer for a is {worker2(values)}.")
        print(f"Answer for b is {worker3(values)}.")
