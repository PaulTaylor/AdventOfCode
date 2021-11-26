from .day_08 import run_code_safe, LoopException

def test_code():
    code = """nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"""

    try:
        run_code_safe(code)
    except LoopException as e:
        assert e.acc == 5
