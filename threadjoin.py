"""Checking if we can get the result of a thread on a join(), like other
languages.
"""

import time

from threading import Thread


class ThreadWithReturn(Thread):
    def __init__(self, value, *args, **kwargs):
        super().__init__(*args, **kwargs)
        print(f'Created thread with time {value}')
        self._value = value
        self._return = None

    def run(self):
        time.sleep(self._value)
        self._return = self._value + 2
        print(f'Thread {self._value} completed')

    def join(self):
        super().join()
        print(f'Thread {self._value} joined')
        return self._return


def main():
    values = reversed(range(5))
    threads = []
    for value in values:
        t = ThreadWithReturn(value)
        t.start()
        threads.append(t)

    for thread in threads:
        val = thread.join()
        print(f'Returned value: {val}')



if __name__ == '__main__':
    main()
