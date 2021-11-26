from .day_12 import Ship, ShipWithWaypoint

def test_ship():
    instructions = ["F10","N3","F7","R90","F11"]
    s = Ship()
    for i in instructions:
        s.process(i)

    assert s.heading == 180
    assert s.X == 17
    assert s.Y == 8

    assert s.manhatten() == 25

def test_ship_with_waypoint():
    instructions = ["F10","N3","F7","R90","F11"]
    s = ShipWithWaypoint()
    for i in instructions:
        s.process(i)

    assert s.manhatten() == 286
