import os
import pathlib
import re

regex_pattern = f'p\d+.py$'
path = pathlib.Path(__file__).parent.resolve()
package_contents = os.listdir(path)

__all__ = [s[:-3] for s in package_contents if re.match(regex_pattern, s)]
