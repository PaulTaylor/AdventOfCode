import pytest

from .day_16 import part_ab

@pytest.fixture
def test_packets_for_a():
    return [
        ("D2FE28", 6),
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31)
    ]

@pytest.fixture
def test_packets_for_b():
    return [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1)
    ]

def test_part_a(test_packets_for_a):
    for packet, expected in test_packets_for_a:
        assert part_ab(packet)[0] == expected

def test_part_b(test_packets_for_b):
    for packet, expected in test_packets_for_b:
        assert part_ab(packet)[1] == expected
