import argparse

def git_init(args):
    print('Init collection:', args.collection)

def git_sync(args):
    print('Sync')

def main():
    parser = argparse.ArgumentParser(description='Main')
    subparsers = parser.add_subparsers()

    git = subparsers.add_parser(name='git', help='Git commands')
    git_subs = git.add_subparsers()

    init = git_subs.add_parser(name='init', help='Init')
    init.set_defaults(func=git_init)
    init.add_argument('collection')

    sync = git_subs.add_parser(name='sync', help='Sync')
    sync.set_defaults(func=git_sync)

    args = parser.parse_args()

    if not hasattr(args, 'func'):
        parser.print_usage()
        return

    args.func(args)


if __name__ == '__main__':
    main()
