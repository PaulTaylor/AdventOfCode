"""
Advent of Code 2021 - Day 21
See: https://adventofcode.com/2021/day/21
"""
import asyncio

from collections import namedtuple
from dataclasses import dataclass
from functools import reduce
from itertools import cycle, product

@dataclass
class DeterministicDice:
    "A Deterministic Dice that rolls 1->100 in sequence forever"
    rolls: int

    def __init__(self):
        self.rolls = 0
        self.generator = (x+1 for x in cycle(range(100)))

    def next(self):
        self.rolls += 1
        return next(self.generator)

PlayerState = namedtuple('PlayerState', [ 'position', 'score' ])
QuantumState = namedtuple('QuantumState', ['p1', 'p2', 'next_player'])
Q_TARGET = 21

def move(player, total_roll):
    "Return a new PlayerState with the move made"
    position = (player.position + total_roll)
    while position > 10:
        position -= 10

    return PlayerState(position=position, score=player.score + position)

def the_game(p1_start, p2_start):
    "Play the game with a deterministic dice"
    dice = DeterministicDice()
    p1 = PlayerState(p1_start, 0)
    p2 = PlayerState(p2_start, 0)

    game_round = 0
    while p1.score < 1000 and p2.score < 1000:
        if game_round % 2 == 0:
            p1 = move(p1, dice.next() + dice.next() + dice.next())
        else:
            p2 = move(p2, dice.next() + dice.next() + dice.next())

        game_round += 1

    return min(p1.score, p2.score) * dice.rolls

async def quantum_future(state, result_futures):
    """
    Co-routine that schedules decendent states to be computed
    and returns the number of p1/p2 wins for all of its sub-states
    """

    # If this state is a winning state for one of the player we return
    # immediately
    if state.p1.score >= Q_TARGET:
        return (1, 0)
    if state.p2.score >= Q_TARGET:
        return (0, 1)

    # Otherwise, generate the next states and either create or join
    # the existing computation as necessary
    sub_state_futures = []
    for r1, r2, r3 in product(range(1,4), range(1,4), range(1,4)):
        if state.next_player == 1:
            new_state = QuantumState(
                p1=move(state.p1, r1 + r2 +r3),
                p2=state.p2,
                next_player=2
            )
        elif state.next_player == 2:
            new_state = QuantumState(
                p1=state.p1,
                p2=move(state.p2, r1 + r2 + r3),
                next_player=1
            )

        # Check if this state has already been scheduled for computation
        # and if not, create a new co-routine and schedule it for execution
        if new_state not in result_futures:
            new_corout = quantum_future(new_state, result_futures)
            result_futures[new_state] = asyncio.ensure_future(new_corout)

        # And the pending result to our list of results
        sub_state_futures.append(result_futures[new_state])

    # Compute the composite result for this state once all available
    results = await asyncio.gather(*sub_state_futures)
    return reduce(
        lambda acc, v: (acc[0] + v[0], acc[1] + v[1]),
        results, (0,0))

def the_quantum_game(p1_start, p2_start):
    "Run a quantum game where each possible dice roll is modelled out"

    # dict that will be used to cache state computations
    result_futures = {}

    # Define the initial state of the board
    start_state = QuantumState(
        p1=PlayerState(p1_start, 0),
        p2=PlayerState(p2_start, 0),
        next_player=1
    )

    # Start the computation tree running
    p1_wins, p2_wins = asyncio.run(
        quantum_future(start_state, result_futures))
    return max(p1_wins, p2_wins)


if __name__ == "__main__":
    p1_start, p2_start = 4, 2
    print(f"Answer for a is {the_game(p1_start, p2_start)}.")
    print(f"Answer for b is {the_quantum_game(p1_start, p2_start)}.")
