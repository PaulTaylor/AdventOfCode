from .day_05 import decode_seat_string

def test_decode_seat_string():
    test_strings = {
        'FBFBBFFRLR': (44, 5, 357),
        'BFFFBBFRRR': (70, 7, 567),
        'FFFBBBFRRR': (14, 7, 119),
        'BBFFBBFRLL': (102, 4, 820)
    }
    
    for s, res in test_strings.items():
        assert res == decode_seat_string(s)