from leetcode.problems import p2

test_cases = [
    ([2,4,3], [5,6,4], [7,0,8]),
    ([0], [0], [0]),
    ([9,9,9,9,9,9,9], [9,9,9,9], [8,9,9,9,0,0,0,1]),
]

test_cases = [
    ([2,4,3], [5,6,4], [7,0,9]),
    ([0], [0], []),
    ([9,9,9,9,9,9,9], [9,9,9,9], [9,9,9,9,0,0,0,1]),
]

class TestP1:
    def test_no_solution(self):
        for nums, target, expected in bad_test_cases:
            assert p2.Solution().twoSum(nums, target) == None
    
    def test_p1(self):
        for nums, target, expected in test_cases:
            assert p2.Solution().twoSum(nums, target) == expected
