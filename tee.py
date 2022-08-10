"""Experimental file-like object that captures output but still send content to
a real file."""

import io
import subprocess


class Tee(io.FileIO):
    def close(self):
        print('TeeClose')
        return super().close()

    def readline(self, size):
        print('TeeReadLine')
        super().readline(size)

    def readlines(self, hint):
        print('TeeReadLines')
        super().readlines(size)

    def writelines(self, lines):
        print('TeeWriteLines')
        return super().writelines(lines)

    def write(self, b):
        print('TeeWite:', b)
        return super().write(b)


def main():
    with Tee('tee.txt', 'w') as target:
        subprocess.run('ls', stdout=target, check=False)


if __name__ == '__main__':
    main()
