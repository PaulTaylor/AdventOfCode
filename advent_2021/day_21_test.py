from .day_21 import PlayerState, the_game, the_quantum_game, move

def test_player_move():
    p = PlayerState(4, 0)
    p = move(p, 6)
    assert p.score == 10
    assert p.position == 10

    p = PlayerState(8, 0)
    p = move(p, 15)
    assert p.score == 3
    assert p.position == 3

def test_the_game():
    assert the_game(4, 8) == 739785

def test_part_b():
    assert the_quantum_game(4, 8) == 444356092776315
