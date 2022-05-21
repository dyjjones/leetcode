from typing import List
from collections import defaultdict


class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        # Python ints have no upper bound; so contrive one
        lowest_delta: int = 2**31 - 1
        sums_set = set()
        delta_to_sum: dict = {}
        bag = defaultdict(int)

        nums.sort()

        first, second, *rest = nums
        bag[first] += 1
        bag[second] += 1
        lowest_bag = min(bag)
        highest_bag = max(bag)

        # if middle in bag then return 0
        # else if middle +- delta in bag, add to deltas

        for right in rest:
            for left in sorted(list(bag.keys())):
                middle = target - (left + right)
                # left can only equal middle if there are 2+ instances in bag
                if (val := bag.get(middle)) and (left != middle or 1 < val):
                    print(
                        f"RETURN!!!, left: {left}, right: {right}, middle: {middle}, val: {val}"
                    )
                    return target
                else:
                    break_to_left = False
                    for delta in range(1, lowest_delta):
                        middle_low = middle - delta
                        middle_high = middle + delta
                        if middle_low < lowest_bag and highest_bag < middle_high:
                            # middle_low and middle_high can't be in bag
                            # break to left for
                            break

                        for possible_middle in [middle_low, middle_high]:
                            # break as soon as a valid middle is found
                            # any higher delta is pointless
                            # as goal is to find closest match

                            # this if condition checks that middle is valid
                            # and if equal to left, there are 2 or more
                            # instances in nums/bag
                            if (val := bag.get(possible_middle)) and (
                                left != possible_middle or 1 < val
                            ):
                                lowest_delta = delta
                                sum_ = left + possible_middle + right
                                print(
                                    f"Target: {target}, Delta: {delta}, Sum: {sum_}, Left: "
                                    f"{left}, Right: {right}, Middle: {possible_middle}"
                                )
                                delta_to_sum[delta] = sum_
                                if not sum_ in sums_set:
                                    sums_set.add(sum_)

                                # there are only 2 iterations in this loop
                                # so if it matches on middle_low, it won't match on
                                # middle_high
                                # and furthermore, you don't want to continue
                                # the delta increment loop because you already found
                                # a new lowest delta
                                break_to_left = True
                                break
                        if break_to_left:
                            break
            bag[right] += 1
            highest_bag = right
        # print("delta: {delta}, sum")
        return delta_to_sum[lowest_delta]
