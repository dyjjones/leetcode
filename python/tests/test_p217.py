from leetcode.problems import p217

test_cases = [
    ([1, 2, 3, 1], True),
    ([1, 2, 3, 4], False),
    ([1, 1, 1, 3, 3, 4, 3, 2, 4, 2], True),
]


class TestP217:
    def test_p217(self):
        for nums, expected in test_cases:
            assert p217.Solution().containsDuplicate(nums) == expected
