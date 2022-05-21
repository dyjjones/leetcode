from leetcode.problems import p16

test_cases = [
    ([-1, 2, 1, -4], 1, 2),
    ([0, 0, 0], 1, 0),
    ([-3, -2, -5, 3, -4], -1, -2),
    ([-111, -111, 3, 6, 7, 16, 17, 18, 19], 13, 16),
]


class TestP1:
    def test_p16_1(self):
        case = test_cases[0]
        assert p16.Solution().threeSumClosest(case[0], case[1]) == case[2]

    def test_p16_2(self):
        case = test_cases[1]
        assert p16.Solution().threeSumClosest(case[0], case[1]) == case[2]

    def test_p16_3(self):
        case = test_cases[2]
        assert p16.Solution().threeSumClosest(case[0], case[1]) == case[2]

    def test_p16_4(self):
        case = test_cases[3]
        assert p16.Solution().threeSumClosest(case[0], case[1]) == case[2]
