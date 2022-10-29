from typing import List
from collections import defaultdict


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        num_indices = defaultdict(list)
        for i, n in enumerate(nums):
            num_indices[n].append(i)
        for m, indices in num_indices.items():
            n = target - m
            if n in num_indices:
                if n == m:
                    if 2 <= len(indices):
                        return indices[:2]
                else:
                    return [indices[0], num_indices[n][0]]
