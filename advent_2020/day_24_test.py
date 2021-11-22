from .day_24 import *

test_input = """sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"""

def test_part_a():
    tile_steps = parse_input(test_input)
    assert len(tile_steps) == 20
    assert len(tile_steps[0]) == 20

    _, num_black = do_part_a(tile_steps)
    assert num_black == 10

def test_part_b():
    tile_steps = parse_input(test_input)

    # Day 0
    floor, num_black = do_part_a(tile_steps)

    # Check days 1-10 (inclusive)
    desired = [15,12,25,14,23,28,41,37,49,37]
    desired_tens = [132,259,406,566,788,1106,1373,1844,2208]
    for excl_day in range(100):
        floor, num_black = do_one_day(floor)

        if excl_day < 10:
            assert num_black == desired[excl_day], \
                f"Day {excl_day + 1}: {num_black} and should be {desired[excl_day]}"

        if (excl_day > 10) and (excl_day % 10 == 9):
            desired_idx = int((excl_day) / 10) - 1
            assert num_black == desired_tens[desired_idx], \
                f"Day {excl_day + 1}: {num_black} and should be {desired[excl_day]}"

    
