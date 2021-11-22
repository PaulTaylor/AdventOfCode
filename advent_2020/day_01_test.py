from .day_01 import *

x = [1721,979,366,299,675,1456]

def test_part_a():
    assert worker2(x) == 514579

def test_part_b():
    assert worker3(x) == 241861950

#
# Disabled benchmark code
#
# def test_part_b(benchmark):
#     assert benchmark(worker3, x) == 241861950
#
# def test_part_b_new(benchmark):
#     assert benchmark(worker3_new, x) == 241861950