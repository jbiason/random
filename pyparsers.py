import argparse

def main():
    parser = argparse.ArgumentParser(description='Main')
    subparsers = parser.add_subparsers()

    git = subparsers.add_parser(name='git', help='Git commands')
    git_subs = git.add_subparsers()

    init = git_subs.add_parser(name='init', help='Init')
    init.add_argument('collection')

    sync = git_subs.add_parser(name='sync', help='Sync')

    args = parser.parse_args()


if __name__ == '__main__':
    main()
