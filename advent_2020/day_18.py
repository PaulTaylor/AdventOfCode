from pathlib import Path
from parsimonious.grammar import Grammar, NodeVisitor

# pylint: disable=eval-used

grammar = Grammar("""
    formula         = num_or_bracket (ws* op ws* num_or_bracket)+
    bracketed       = "(" formula ")"
    num_or_bracket  = (num / bracketed)
    num             = ~"[0-9]+"
    op              = "+" / "-" / "*" / "/"
    ws              = ~"\\\\s+"
""")

class FormulaVisitor(NodeVisitor):
    "AST walker that computes the answer for Part A"

    def generic_visit(self, node, visited_children):
        raise NotImplementedError(
            f'No visitor method was defined for this expression: {node.expr_name}')

    def visit_formula(self, _, visited_children):
        # At this point, visited children will already be evaluated (including any nested brackets)
        # all we need to do is follow the rules from the scenario (strict left/right evaluation)
        # with no BODMAS
        # The grammer/walker will ensure that brackets are all evaluated first - as we want :)

        first, the_rest = visited_children
        acc = first
        for op, num in the_rest:
            acc = eval(f"{acc} {op} {num}")

        return acc

    def visit_bracketed(self, _, visited_children):
        # Drop the surrounding bracket tokens and just return the result
        # of the internal formula evaluation
        _, result, _ = visited_children
        return result

    def visit_num_or_bracket(self, _, visited_children):
        # Just return the child - that'll be what we actually want to consider
        return visited_children[0]

    def visit_ws(self, *_):
        pass # no-one cares about whitespace

    def visit_(self, _, visited_children):
        # This is the whitespace wrapper - filter out any None's in the return values
        return list(filter(lambda x: x, visited_children))

    def visit_op(self, node, _):
        # Op just needs the return the symbol
        return node.text

    def visit_num(self, node, _):
        # nums just return the integer value
        return int(node.text)

class PartBVisitor(FormulaVisitor):
    "Alternative AST walker that computes the answer for Part B"

    def generic_visit(self, node, visited_children):
        raise NotImplementedError(
            f'No visitor method was defined for this expression: {node.expr_name}')

    def visit_formula(self, _, visited_children):
        # Customise for adjusted precedence rules
        # Addition is now more important than multiplication
        first, the_rest = visited_children

        # Flatten the visted_children list and evaluate the +'s first
        ops = [first]
        for op in the_rest:
            ops.extend(op)

        while "+" in ops:
            idx = ops.index("+")
            prior = ops[idx - 1]
            post = ops[idx + 1]
            ops[idx] = prior + post

            del ops[idx + 1]
            del ops[idx - 1]

        # now eval ltr
        acc = ops[0]
        ptr = 1
        while ptr < len(ops):
            op = ops[ptr]
            num = ops[ptr + 1]
            acc = eval(f"{acc} {op} {num}")
            ptr += 2

        return acc

def evaluate(expression):
    ast = grammar.parse(expression)
    return FormulaVisitor().visit(ast)

def evaluate_b(expression):
    ast = grammar.parse(expression)
    return PartBVisitor().visit(ast)

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_18_a.txt'
    with open(p, "rt", encoding="ascii") as f:
        raw = f.read().strip()

    # Part A
    acc = 0
    for idx, line in enumerate(raw.splitlines()):
        res = evaluate(line)
        acc += res
    print("====================================")
    print(f"summed total = {acc}")

    # Part B
    acc = 0
    for idx, line in enumerate(raw.splitlines()):
        res = evaluate_b(line)
        acc += res
    print("====================================")
    print(f"summed total = {acc}")
