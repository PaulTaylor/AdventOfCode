"""
Advent of Code 2021 - Day 19
See: https://adventofcode.com/2021/day/19
"""

import itertools
from collections import namedtuple
from pathlib import Path

import numpy as np
from scipy.spatial.transform import Rotation as R

# NamedTuple for sensor data
# each will be initialised with sensor_coords as [(0,0,0)]
# because each sensor is at that location from it's own perspective
SensorData = namedtuple('BeaconSet', ['beacon_coords', 'sensor_coords'])

def parse_input(lines):
    scanners = []
    current_scanner = None
    for line in lines:
        line = line.strip()
        if not line:
            continue

        if "scanner" in line:
            if current_scanner:
                scanners.append(
                    SensorData(beacon_coords=current_scanner, sensor_coords=[(0,0,0)])
                )
            current_scanner = []
        else:
            x, y, z = [ int(v) for v in line.split(",") ]
            current_scanner.append((x,y,z))

    scanners.append(
        SensorData(beacon_coords=current_scanner, sensor_coords=[(0,0,0)])
    )
    return scanners

def calculate_distance(p1, p2):
    "Manhatten distance between p1 and p2"
    return sum( abs(x - y) for x,y in zip(p1, p2) )

def generate_distance_lists(s):
    """
    # For each point in s
    # compute the distances between p and the other points
    """
    acc = []
    for p in s:
        dists = [ calculate_distance(p, o) for o in s ]
        acc.append(dists)
    return acc

def check_alignment(equiv_pairs, x_rot, y_rot, z_rot):
    # create our desired rotation
    r = R.from_euler("xyz", [ z_rot, y_rot, x_rot ], degrees=True)
    # Calculate the translation using the first item in s2
    translation = None
    correct_count = 0
    for p1, p2 in equiv_pairs:
        p2r = r.apply(p2) # Rotate
        if translation is None:
            translation = p1 - p2r
            correct_count += 1
        else:
            p2rt = p2r + translation
            # Need to be careful here with precision and conversion to/from float
            # cannot just do p2rt.astype(int) because it floors things
            # ie. 12.9999999997 becomes 12 and not 13
            # ( even though when you print the array it displays 13 :/ )
            p2rt = np.round(p2rt).astype(int)

            if np.array_equal(p2rt, p1):
                correct_count += 1
            elif np.allclose(p2rt, p1):
                raise Exception("Numeric precision issues - need to investigate")

    if correct_count == len(equiv_pairs):
        return r, translation
    else:
        return None

def generate_equiv_pairs(s1, s2, s1_point_distances, s2_point_distances):
    equiv_pairs = []
    for s1_idx, d1 in enumerate(s1_point_distances):
        d1_set = set(d1)
        for s2_idx, d2 in enumerate(s2_point_distances):
            # Count the matching items
            match_count = 0
            for d in d2:
                match_count += d in d1_set

            if match_count >= 12:
                # Now we work out which point in both sets is the same
                equiv_pairs.append((s1[s1_idx], s2[s2_idx]))

    return equiv_pairs

def find_alignment(s1, s2, equiv_pairs):
    # Alignment finding
    # Each of the 3 axes can be 90 degree rotated
    # For each configuration need to:
    #   perform axis rotation on s2
    #   translate the s2 co-ords to the matching s1 co-ords
    #   validate that all 12 match

    for x_rot in (0,90,180,270):
        for y_rot in (0,90,180,270):
            for z_rot in (0,90,180,270):
                align_result = check_alignment(equiv_pairs, x_rot, y_rot, z_rot)
                if align_result:
                    # We've got it!
                    # Now we can return a list of all the points translated into
                    # s1's co-ordinate space. Now unpack the alignment result:
                    r, translation = align_result

                    # Need to create a new SensorData that contains the combined
                    # coordinates for the beacons.  It will also need to translate
                    # and include the sensor locations from s2 into itself
                    new_beacon_coords = s1.beacon_coords +\
                        [ tuple(np.round(r.apply(p) + translation)) for p in s2.beacon_coords ]
                    new_sensor_coords = s1.sensor_coords +\
                        [ tuple(np.round(r.apply(p) + translation)) for p in s2.sensor_coords ]
                    # Dedupe the beacons and return
                    return SensorData(list(set(new_beacon_coords)), new_sensor_coords)

    raise Exception("Found overlap, but failed to align.")

def merge(s1: SensorData, s2: SensorData):
    # Overlap must be exactly 12 points
    # for each session set s1, s2
    #   for each point
    #     calculate the distances between this point and all others in the same s
    # check to see if any of the distance sets in s1 and s2 overlap by exactly 12
    # if they do - then there's an overlap

    s1_point_distances = generate_distance_lists(s1.beacon_coords)
    s2_point_distances = generate_distance_lists(s2.beacon_coords)

    # check if there are points with the same difference matrixes
    # in both s1 and s2
    equiv_pairs = generate_equiv_pairs(
        s1.beacon_coords, s2.beacon_coords, s1_point_distances, s2_point_distances)

    if not equiv_pairs:
        # No overlap - don't need to try and find an alignment - bail out here
        return None

    return find_alignment(s1, s2, equiv_pairs)

def part_ab(the_input):
    with_ids = dict(enumerate(the_input))
    remaining_ids = list(with_ids.keys())
    while len(remaining_ids) > 1:
        start_size = len(with_ids)
        for id1, id2 in itertools.product(remaining_ids, remaining_ids):
            if (id1 != id2) and (id2 > id1):
                res = merge(with_ids[id1], with_ids[id2])
                if res:
                    with_ids[id1] = res
                    del with_ids[id2]
                    remaining_ids = list(with_ids.keys())
                    break

        if len(with_ids) == start_size:
            raise Exception("No merges worked :(")

    # If we get here then we've merged successfully!
    # Need to return the number of unique co-ordinates
    # and the maximum manhatten distance between sensors
    assert len(with_ids) == 1
    for _, merged_data in with_ids.items():
        return len(merged_data.beacon_coords), part_b(merged_data.sensor_coords)

def part_b(sensor_coords):
    max_mdistance = 0
    for c1, c2 in itertools.product(sensor_coords, sensor_coords):
        max_mdistance = max(max_mdistance, calculate_distance(c1, c2))
    return int(max_mdistance)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_19_a.txt'
    with open(p, "r", encoding="ascii") as f:
        puzzle_input = parse_input(f.readlines())

    a_res, b_res = part_ab(puzzle_input)
    print(f"Answer for a is {a_res}.")
    print(f"Answer for b is {b_res}.")
