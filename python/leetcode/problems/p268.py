from typing import List


class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        nums.sort()
        for i, n in enumerate(nums):
            if i != n:
                return i
        return len(nums)

    def missingNumberFollowup(self, nums: List[int]) -> int:
        expected_middle = (len(nums)) / 2
        expected_last_val = len(nums) + 1
        expected_sum = expected_last_val * expected_middle
        return int(expected_sum - sum(nums))
