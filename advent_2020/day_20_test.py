from pathlib import Path

import pytest

from .day_20 import create_tiles, do_part_a, do_part_b


@pytest.fixture
def tiles():
    p = Path(__file__).parent / "input" / 'day_20_test_input.txt'
    with open(p, "rt", encoding="ascii") as f:
        raw = f.read()
        return create_tiles(raw)

def test_parts_ab(tiles):
    a_ans, sub_G = do_part_a(tiles)
    assert a_ans == 20899048083289

    _, roughness = do_part_b(tiles, sub_G)
    assert roughness == 273
