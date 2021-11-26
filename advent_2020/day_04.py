import re

from pathlib import Path

def check_valid(passport):
    required_fields = [
        "byr", #(Birth Year)
        "iyr", #(Issue Year)
        "eyr", #(Expiration Year)
        "hgt", #(Height)
        "hcl", #(Hair Color)
        "ecl", #(Eye Color)
        "pid", #(Passp
    ]

    try:
        for f in required_fields:
            _ = passport[f]
    except KeyError:
        return False

    return True

height_pattern = re.compile("^([0-9]+)((cm)|(in))$")
hc_pattern = re.compile("^#[0-9a-f]{6}$")
pid_pattern = re.compile("^[0-9]{9}$")

def check_height(val):
    """(Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76."""

    match = height_pattern.match(val)
    if not match:
        return False

    num, unit = match.groups()[0:2]
    if unit == 'cm':
        return 150 <= int(num) <= 193
    elif unit == "in":
        return 56 <= int(num) <= 76
    else:
        return False

def check_valid_strict(passport):
    required_fields = {
        # four digits; at least 1920 and at most 2002.
        "byr": lambda x: (len(x) == 4) and (1920 <= int(x) <= 2002),
        # four digits; at least 2010 and at most 2020.
        "iyr": lambda x: (len(x) == 4) and (2010 <= int(x) <= 2020),
        # four digits; at least 2020 and at most 2030.
        "eyr": lambda x: (len(x) == 4) and (2020 <= int(x) <= 2030),
        # height
        #(Height) - a number followed by either cm or in:
        #  - If cm, the number must be at least 150 and at most 193.
        #  - If in, the number must be at least 59 and at most 76.
        "hgt": check_height, # (Height)
        "hcl": hc_pattern.match, #(Hair Color)
        "ecl": lambda x: x in 'amb blu brn gry grn hzl oth'.split(" "), #(Eye Color)
        "pid": pid_pattern.match, #(Passp
    }

    try:
        for f, check in required_fields.items():
            assert check(passport[f]), f"field {f} with value {passport.get(f, 'None')} fails check"
    except AssertionError:
        return False
    except KeyError:
        return False
    except Exception as e:
        raise e

    return True

def process_lines(lines):
    passports = []
    this_passport = {}
    for line in lines:
        if line.strip() == "":
            # Hit a blank - store the current and reset for next
            passports.append(this_passport)
            this_passport = {}
            continue
        else:
            kv_pairs = line.split(" ")
            for pair in kv_pairs:
                assert len(pair.split(":")) == 2, pair
                k, v = pair.split(":")
                this_passport[k] = v.strip()

    passports.append(this_passport) # deal with the end of the file :)

    validity = sum(check_valid(p) for p in passports)
    strict_validity = sum(check_valid_strict(p) for p in passports)

    return validity, strict_validity

if __name__ == "__main__":

    p = Path(__file__).parent / "input" / 'day_04_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        lines = f.readlines()

    validity, strict_validity = process_lines(lines)

    print(f"{validity} passports are valid")
    print(f"{strict_validity} passports are valid with validation rules")
