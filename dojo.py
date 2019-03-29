def test():
    _input = [100, 101, 102, 103, 104, 105, 110, 111, 112, 113, 114, 115, 150]
    assert [[100, 105], [110, 115], [150]] == interval(_input)

def test_1():
    _input = [100, 101, 102, 103, 104, 105, 110, 111, 112, 113, 114, 115]
    assert [[100, 105], [110, 115]] == interval(_input)

def test_2():
    _input = [100, 110, 111, 112, 113, 114, 115, 150]
    assert [[100], [110, 115], [150]] == interval(_input)

def test_3():
    _input = [100, 101, 102, 103, 104, 105, 107, 110, 111, 112, 113, 114, 115, 150]
    assert [[100, 105], [107], [110, 115], [150]] == interval(_input)

def test_4():
    assert [[100]] == interval([100])

def test_5():
    assert [[1], [3], [5]] == interval([1, 3, 5])

def test_6():
    assert [[1, 2]] == interval([1, 2])

def failure():
    assert [[1]] == interval([1, 2])

def interval(data):
    diffs =  [(data[x] - data[x-1]) != 1
             for x
             in range(len(data))]
    result = []
    for value, _new in zip(data, diffs):
        if _new:
            result.append([value])
        else:
            result[-1] = [result[-1][0], value]
    return result

if __name__ == "__main__":
    test()
    test_1()
    test_2()
    test_3()
    test_4()
    test_5()
    test_6()
    failure()
