from .day_21 import part_a, part_b

test_string = """mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"""

def test_solution():
    good_count, allergen_candidates = part_a(test_string.splitlines())
    assert good_count == 5
    assert part_b(allergen_candidates) == "mxmxvkd,sqjhc,fvjkl"
