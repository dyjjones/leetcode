# import leetcode.problems as problems
from leetcode.problems import *
import re


def main():
    print("Run `pytest` for checking solutions.")

    regex_pattern = f'p\d+$'
    filtered_problems = [p for p in globals() if re.match(regex_pattern, p)]
    filtered_problems.sort(key=lambda s: int(s[1:]))

    if filtered_problems:
        print("Problems solved in Python:")
        print(*filtered_problems, sep='\n')
    else:
        print("No solutions available.")


if __name__ == '__main__':
    main()