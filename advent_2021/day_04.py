"""
Advent of Code 2021 - Day 4
See: https://adventofcode.com/2021/day/4
"""

from pathlib import Path
import numpy as np

class Board():

    def __init__(self, board) -> None:
        self.board = np.array(board, dtype=int)
        self.covered = np.zeros_like(board, dtype=bool)

    def play(self, num) -> bool:
        "Play a round of bingo, return true if we're a winner"
        matches = self.board == num
        self.covered = np.logical_or(self.covered, matches, out=self.covered)

        # Check for columns
        col_sums = np.sum(self.covered, axis=0)
        row_sums = np.sum(self.covered, axis=1)

        return (self.covered.shape[1] in col_sums) or (self.covered.shape[0] in row_sums)

    def score(self, last_played_number):
        uncovered_sum = np.sum(self.board[np.logical_not(self.covered)])
        return uncovered_sum * last_played_number

def parse_input(input_string: str):
    lines = input_string.splitlines()

    numbers = [ int(x) for x in lines[0].split(",") ]

    boards = []
    for board_start in range(2, len(lines), 6):
        board_lines = lines[board_start:board_start+5]
        board = [ b_line.split() for b_line in board_lines ]
        boards.append(Board(board))

    return numbers, boards

def part_a(numbers, boards):
    for number in numbers:
        for board in boards:
            if board.play(number):
                return board.score(number)

    raise Exception("No winner :(")

def part_b(numbers, input_boards):
    boards = dict(enumerate(input_boards))
    for idx, number in enumerate(numbers):
        to_remove = []
        for board_id, board in boards.items():
            if board.play(number):
                to_remove.append(board_id)

        for board_id in to_remove:
            if len(boards) > 1:
                del boards[board_id]

        if len(boards) == 1:
            break

        if len(boards) < 1:
            raise Exception("No boards remaining :(")

    # To calculate the final score, we can reuse the logic from part_a to
    # complete the game on the last remaining board
    return part_a(numbers[idx:], boards.values())

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_04_a.txt'
    with open(p, "r", encoding="ascii") as f:
        in_numbers, in_boards = parse_input(f.read())

    print(f"Answer for a is {part_a(in_numbers, in_boards)}.")
    print(f"Answer for b is {part_b(in_numbers, in_boards)}.")
