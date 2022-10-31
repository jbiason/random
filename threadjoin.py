"""Checking if we can get the result of a thread on a join(), like other
languages.
"""

from threading import Thread

class ThreadWithReturn(Thread):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._return = None

    def run(self):
        self._return = self._target(*self._args, **self._kwargs)

    def join(self):
        super().join()
        return self._return


def runner(value):
    return value + 2


def main():
    t = ThreadWithReturn(target=runner, args=[2])
    t.start()
    val = t.join()
    print(val)



if __name__ == '__main__':
    main()
