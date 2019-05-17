import re

def main():
    with open('Bairros.sql', 'r') as origin:
        for line in origin:
            if not 'Insert' in line or '--' in line:
                continue

            line = line.strip()
            parts = line.split(',')

            state = parts[-1][1:-3]
            suburb = parts[-2][1:-1]
            _id = parts[-3][14:-1]

            print('{}, {}, {}'.format(
                _id,
                suburb,
                state
            ))

if __name__ == '__main__':
    main()
