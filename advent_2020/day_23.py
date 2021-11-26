import numpy as np

from tqdm import tqdm

class Cup(object):
    "This is essentially a linked list element representing a cup in the game"

    def __init__(self, value):
        self.value = value
        self.next = None
        self.prev = None

    def __repr__(self):
        return f"Cup({self.value})"

class Ring(object):
    "This is the state of the cup ring for the game"

    def __init__(self, nums):
        self.head_at_start = None # filled on first round
        self.round = 0
        self.head = Cup(nums[0])
        self.value_cache = {
            nums[0]: self.head
        }
        first_cup = self.head

        for n in nums[1:]:
            new_cup = Cup(n)
            new_cup.prev = self.head
            self.head.next = new_cup
            self.head = new_cup

            self.value_cache[n] = new_cup

            if n == 1:
                self.cup_1 = new_cup

        # Finish connecting the ring
        first_cup.prev = new_cup
        new_cup.next = first_cup

        # And rotate one to get the original head back
        self.head = first_cup

        # Store the min/max values for later reference
        self.min_value = min(nums)
        self.max_value = max(nums)

    def do_round(self):
        # print("start", str(self))

        self.head_at_start = self.head

        snip_head = self.head.next
        snip_end = self.head.next.next.next

        removed_values = (
            snip_head.value,
            snip_head.next.value,
            snip_head.next.next.value
        )

        # print("removed: ", ",".join(map(str, removed_values)))

        # put the ring back without the above 3
        self.head.next = snip_end.next
        snip_end.next.prev = self.head

        # We won't None out the references for the snipped set
        # because those will be overwritten on reinsert anyway

        # Calculate the destination value
        destination_value = self.head.value - 1
        while (destination_value in removed_values) or (destination_value < self.min_value):
            destination_value -= 1
            if destination_value < self.min_value:
                destination_value = self.max_value

        # print("destination: ", destination_value)

        # Move head to the value that we should be inserting after
        self.head = self.value_cache[destination_value]
        # while self.head.value != destination_value:
        # self.head = self.head.next

        # Insert the snipped cups
        after_snipped = self.head.next
        self.head.next = snip_head
        snip_head.prev = self.head
        snip_end.next = after_snipped
        after_snipped.prev = snip_end

        # Advance head by one position
        self.head = self.head_at_start.next

        # print("end: ", str(self), "\n**************************\n")

        self.round += 1

    def __repr__(self):
        buf = []
        ptr = self.cup_1.next
        while ptr.value != 1:
            buf.append(str(ptr.value))
            ptr = ptr.next

        return "".join(buf)

if __name__ == "__main__":
    puzzle_input = "739862541"

    a_input = np.array([ int(x) for x in puzzle_input ])
    ring = Ring(a_input)
    for _ in range(100):
        ring.do_round()

    ans = str(ring)
    assert ans != "76345298", "Incorrect answer"
    print("The answer to part A is:", ans)

    # Part B
    cups = list(map(int, puzzle_input))
    num = max(cups) + 1
    while len(cups) < 1e6:
        cups.append(num)
        num += 1
    assert len(cups) == 1e6

    ring = Ring(cups)
    for _ in tqdm(range(10000000)):
        ring.do_round()

    cup_one = ring.cup_1
    n1 = cup_one.next
    n2 = n1.next
    print("The answer to part B is:", n1.value * n2.value)
