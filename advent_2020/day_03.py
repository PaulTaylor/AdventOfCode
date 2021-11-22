import math

from pathlib import Path

def check_slope(slope, right, down):
    coords = (0, 0)
    count = 0

    # a mod(%) solution would probably work too...
    dupe_count = math.ceil(len(slope) / len(slope[0].strip())) * 200

    while coords[1] < len(slope):

        slope_line = slope[coords[1]].strip() * dupe_count
        
        try:
            count += 1 if slope_line[coords[0]] != '.' else 0
        except IndexError:
            print(len(slope_line), coords[0])

        # Move to next position
        coords = (coords[0] + right, coords[1] + down)

    return count

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_03_a.txt'
    with open(p, "rt") as f:
        lines = f.readlines()
        print(check_slope(lines, 3, 1))

        runs = [
            (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)
        ]

        res = [ check_slope(lines, right, left) for right, left in runs ]
        assert res[1] == 214
        
        output = 1
        for x in res:
            output *= x
        print(output)

