# import leetcode.problems as problems
from leetcode.problems import *
import re


def main():
    print("Run `pytest` for checking solutions.")

    regex_pattern = f'p\d+$'
    filtered_problems = [int(p[1:]) for p in globals() if re.match(regex_pattern, p)]
    filtered_problems.sort()

    if filtered_problems:
        print("Problems solved in Python:")
        print(*filtered_problems, sep='\n')
    else:
        print("No solutions available.")


if __name__ == '__main__':
    main()