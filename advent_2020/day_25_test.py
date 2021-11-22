from .day_25 import *

def test_transform_sn():
    assert transform_sn(7, 8) == 5764801

def test_part_a():
    card_pk = 5764801
    door_pk = 17807724
    assert 14897079 == do_part_a(card_pk, door_pk)