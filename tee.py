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

    with open('tee2.txt', 'w') as output:
        with subprocess.Popen('ls', stdout=subprocess.PIPE, stderr=subprocess.PIPE) as proc:
            for line in proc.stdout:
                print(f'--> {line.decode("utf-8")}', file=output)
    result = subprocess.CompletedProcess(proc.args, proc.returncode, proc.stdout, proc.stderr)
    print(result)


if __name__ == '__main__':
    main()
