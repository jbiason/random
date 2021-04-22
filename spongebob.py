import sys
from itertools import cycle

functions = cycle([str.upper, str.lower])
phrase = sys.argv[1]

for char in phrase:
    func = next(functions)
    print(func(char), end='')

print('')
