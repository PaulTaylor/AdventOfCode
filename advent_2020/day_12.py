import math
import re

from dataclasses import dataclass
from pathlib import Path


instruction_pattern = re.compile("([NSEWLRF])([0-9]+)")

@dataclass(repr=True)
class Ship:
    heading: float = 90.0 # Start facing east
    # Current position in the world (we'll start at 0,0)
    X: float = 0.0
    Y: float = 0.0
    # A north move is considered negative, south positive
    #  A west move is considered negative, east positive

    def process(self, instruction):
        action, magntitude = instruction_pattern.match(instruction).groups()
        magntitude = float(magntitude)

        if action == "N":
            # Action N means to move north by the given value.
            self.Y -= magntitude
        elif action == "S":
            # Action S means to move south by the given value.
            self.Y += magntitude
        elif action == "E":
            # Action E means to move east by the given value.
            self.X += magntitude
        elif action == "W":
            # Action W means to move west by the given value.
            self.X -= magntitude
        elif action == "L":
            # Action L means to turn left the given number of degrees.
            self.heading = (self.heading - magntitude) % 360
        elif action == "R":
            # Action R means to turn right the given number of degrees.
            self.heading = (self.heading + magntitude) % 360
        elif action == "F":
            # Action F means to move forward by the given value in the direction the ship is
            # currently facing.  Deal with special cases first to try and avoid trig.
            if self.heading == 0:
                self.process(f"N{magntitude}") # a move north
            elif self.heading == 90:
                self.process(f"E{magntitude}") # a move east
            elif self.heading == 180:
                self.process(f"S{magntitude}") # a move east
            elif self.heading == 270:
                self.process(f"W{magntitude}") # a move east
            else:
                # This'll need trigonometry - we have the hypotenuse distance,
                # we need the opp and adj
                print("** We attempted trigonometry!")
                x_move = math.asin(math.radians(self.heading)) * magntitude
                y_move = math.acos(math.radians(self.heading)) * magntitude
                self.X += x_move
                self.Y += y_move
        else:
            raise Exception("Unknown instruction")

    def manhatten(self):
        return abs(self.X) + abs(self.Y)


@dataclass(repr=True)
class ShipWithWaypoint:
    # Waypoint Position (relative to ship)
    # North is considered negative, south positive
    # West considered negative, east positive
    waypoint_X = 10 # start 10 units east
    waypoint_Y = -1 #   and  1 unit north of the ship

    # Ship position (relative to starting position)
    X: int = 0
    Y: int = 0

    def process(self, instruction):
        action, magntitude = instruction_pattern.match(instruction).groups()
        magntitude = float(magntitude)

        if action == "N":
            # WAYPOINT - Action N means to move north by the given value.
            self.waypoint_Y -= magntitude
        elif action == "S":
            # WAYPOINT - Action S means to move south by the given value.
            self.waypoint_Y += magntitude
        elif action == "E":
            # WAYPOINT - Action E means to move east by the given value.
            self.waypoint_X += magntitude
        elif action == "W":
            # WAYPOINT - Action W means to move west by the given value.
            self.waypoint_X -= magntitude
        elif action == "L":
            # Action L means to turn left the given number of degrees.
            assert magntitude % 90 == 0
            turns = int(magntitude / 90)
            for _ in range(turns):
                new_X = self.waypoint_Y
                new_Y = -self.waypoint_X
                self.waypoint_X = new_X
                self.waypoint_Y = new_Y
        elif action == "R":
            # Action R means to turn right the given number of degrees.
            # What's the current angle to the waypoint?
            assert magntitude % 90 == 0
            turns = int(magntitude / 90)
            for _ in range(turns):
                new_X = -self.waypoint_Y
                new_Y = self.waypoint_X
                self.waypoint_X = new_X
                self.waypoint_Y = new_Y
        elif action == "F":
            self.X += (magntitude * self.waypoint_X)
            self.Y += (magntitude * self.waypoint_Y)
        else:
            raise Exception("Unknown instruction")

    def manhatten(self):
        return abs(self.X) + abs(self.Y)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_12_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        lines = f.readlines()

    s = Ship()
    for i in lines:
        s.process(i.strip())

    print(f"Part A - manhatten distance = {s.manhatten()}")

    s = ShipWithWaypoint()
    for i in lines:
        s.process(i.strip())

    print(f"Part B - manhatten distance = {s.manhatten()}")
