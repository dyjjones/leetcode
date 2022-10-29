from leetcode.problems import p268

test_cases = [
    ([3, 0, 1], 2),
    ([0, 1], 2),
    ([9, 6, 4, 2, 3, 5, 7, 0, 1], 8),
]


class TestP217:
    def test_p268_original(self):
        for nums, expected in test_cases:
            assert p268.Solution().missingNumber(nums) == expected

    def test_p268_followup(self):
        for nums, expected in test_cases:
            assert p268.Solution().missingNumberFollowup(nums) == expected
