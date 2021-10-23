from leetcode.problems import p1
test_cases = [
    ([2,7,11,15], 9, [0, 1]),
    ([3,2,4], 6, [1, 2]),
    ([3,3], 6, [0, 1]),
]

bad_test_cases = [
    ([2,7,11,15], 10, None),
    ([3,2,4], 8, None),
    ([3,3], 7, None)
]

class TestP1:
    def test_no_solution(self):
        for nums, target, expected in bad_test_cases:
            assert p1.Solution().twoSum(nums, target) == None
    
    def test_p1(self):
        for nums, target, expected in test_cases:
            assert p1.Solution().twoSum(nums, target) == expected
