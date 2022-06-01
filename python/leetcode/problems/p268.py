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


s = Solution()
assert s.missingNumberFollowup([3, 0, 1]) == 2
assert s.missingNumberFollowup([0, 1]) == 2
assert s.missingNumberFollowup([9, 6, 4, 2, 3, 5, 7, 0, 1]) == 8
