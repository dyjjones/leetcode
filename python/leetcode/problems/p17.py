from typing import *
from itertools import product


nums_to_letters = {
    "2": list("abc"),
    "3": list("def"),
    "4": list("ghi"),
    "5": list("jkl"),
    "6": list("mno"),
    "7": list("pqrs"),
    "8": list("tuv"),
    "9": list("wxyz"),
}


class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        letters = [nums_to_letters[d] for d in digits]
        joined = ["".join(t) for t in product(*letters)]
        return joined
