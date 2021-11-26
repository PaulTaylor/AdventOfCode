from collections import deque
from pathlib import Path


def parse_input(raw):
    lines = raw.splitlines()
    p1 = deque()
    p2 = deque()

    current = p1
    for line in lines:
        if line.startswith("Player"):
            if line[-2:-1] == "1":
                current = p1
            elif line[-2:-1] == "2":
                current = p2
        elif line.strip() == "":
            continue
        else:
            current.append(int(line))

    return p1, p2


def play_game(deck_1, deck_2):
    while deck_1 and deck_2:
        p1 = deck_1.popleft()
        p2 = deck_2.popleft()

        if p1 == p2:
            deck_1.append(p1)
            deck_2.append(p2)
        elif max(p1, p2) == p1:
            deck_1.append(p1)
            deck_1.append(p2)
        else:
            deck_2.append(p2)
            deck_2.append(p1)


def part_a(raw):
    d1, d2 = parse_input(raw)
    play_game(d1, d2)

    if d1:
        winner = d1
    else:
        winner = d2

    factor = len(winner)
    acc = 0
    for idx, v in enumerate(winner):
        acc += (v * (factor - idx))

    return acc


def play_recursive_game(d1, d2, history, game=1):
    while d1 and d2:
        winner = None
        if (tuple(d1), tuple(d2)) in history:
            # Same cards in same order - p1 wins
            return 1

        history.add((tuple(d1), tuple(d2)))

        # Different game - continue:
        c1 = d1.popleft()
        c2 = d2.popleft()

        if len(d1) >= c1 and len(d2) >= c2:
            # recurse!
            winner = play_recursive_game(
                deque(list(d1)[0:c1]),
                deque(list(d2)[0:c2]),
                set(), game=game+1)
        else:
            # winner is person with highest card
            if max(c1, c2) == c1:
                winner = 1
            elif max(c1, c2) == c2:
                winner = 2
            else:
                raise Exception("Equal cards!")


        if not winner:
            raise Exception("No winner!!!")
        elif winner == 1:
            d1.append(c1)
            d1.append(c2)
        else: # winner == 2:
            d2.append(c2)
            d2.append(c1)

    if d1 and not d2:
        cached_rv = 1
    elif not d1 and d2:
        cached_rv = 2

    return cached_rv


def part_b(raw):
    d1, d2 = parse_input(raw)
    history = set()
    play_recursive_game(d1, d2, history)

    if d1:
        winner = d1
    else:
        winner = d2

    factor = len(winner)
    acc = 0
    for idx, v in enumerate(winner):
        acc += (v * (factor - idx))

    return acc


if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_22_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        raw = f.read()

    a_res = part_a(raw)
    print(a_res)
    print(part_b(raw))
