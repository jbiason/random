import functools
import operator

class E:
    def __init__(self, some_list):
        self.internal_list = some_list

    def __hash__(self):
        calc_hash = functools.reduce(operator.xor, self.internal_list, 0)
        print(f'Calc hash for {self.internal_list} is {calc_hash}')
        return calc_hash

    def __eq__(self, other):
        return self.internal_list == other.internal_list

    def __bool__(self):
        return len(self.internal_list) > 0


content = set()
w1 = E([1, 2])
w2 = E([1, 2])
content.add(w1)
content.add(w2)

print(content)

w3 = E([])
if w3:
    print('It is true')
else:
    print('It is false')
