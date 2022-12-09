#!/bin/env python

import shutil
import subprocess
import os

from argparse import ArgumentParser
from pathlib import Path
from typing import List
from typing import Optional
from typing import Generator


GIT_EXE = shutil.which("git", mode=os.X_OK)
WORKING_DIRECTORY = Path.cwd() / "test"


def run(command: str, args: List[str]):
    """Run Git with the specified arguments."""
    full_command = [str(GIT_EXE)]
    full_command.extend(command.split())
    full_command.extend(args)

    result = None
    print(f"Git: {full_command}")
    proc = subprocess.Popen(
        full_command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        cwd=WORKING_DIRECTORY,
    )
    result = proc.communicate()
    (out, err) = result
    print(f"Out:\n{out}")
    print(f"Err:\n{err}")
    return result


def files(base: Optional[str] = None) -> List[str]:
    """Return the list of files for the base directory or all the files in the
    repo in case there is no base.
    """
    params = ["--cached"]
    if base:
        params.append(base)
    result = run("ls-files", params)
    return result[0].decode("utf-8").strip().split("\n")


def hide(files: List[str]):
    """Hide a list of files by tagging them to skip the worktree."""
    params = ["--skip-worktree", "--force-remove"]
    params.extend(files)
    run("update-index", params)


def show(files: List[str]):
    """Display files by removing the tagging to skip the worktree."""
    params = ["--no-skip-worktree"]
    params.extend(files)
    run("update-index", params)
    run("checkout", files)


def bases(files: List[str]) -> Generator[str, None, None]:
    """Produces only the bases for the list of files."""
    last_seen = None
    for filename in files:
        as_path = Path(filename)
        base = as_path.parts[0]
        if base != last_seen:
            yield base
            last_seen = base


def clone(args):
    print(f"Clonining {args.url} on {args.branch}")
    shutil.rmtree(WORKING_DIRECTORY, ignore_errors=True)
    WORKING_DIRECTORY.mkdir()
    run(
        "clone",
        ["--no-checkout", args.url, "--branch", args.branch, str(WORKING_DIRECTORY)],
    )
    run("reset", [])
    hide(files())


def enable(args):
    """Make the files under a directory available again."""
    print(f"Enabling {args.path}")
    to_enable = []
    for base in args.path:
        to_enable.extend(files(base))
    show(to_enable)


def disable(args):
    """Make the files under a directory unavailable."""
    print(f"Disabling {args.path}")
    to_enable = []
    for base in args.path:
        to_enable.extend(files(base))
    hide(to_enable)
    for base in args.path:
        shutil.rmtree(WORKING_DIRECTORY / base, ignore_errors=True)


def list(_args):
    """List all the bases available."""
    for filename in bases(files()):
        print(f"{filename}")


def main():
    parser = ArgumentParser()
    subparsers = parser.add_subparsers()

    clone_cmd = subparsers.add_parser("clone")
    clone_cmd.add_argument("url")
    clone_cmd.add_argument("branch")
    clone_cmd.set_defaults(func=clone)

    enable_cmd = subparsers.add_parser("enable")
    enable_cmd.add_argument("path", nargs="+")
    enable_cmd.set_defaults(func=enable)

    disable_cmd = subparsers.add_parser("disable")
    disable_cmd.add_argument("path", nargs="+")
    disable_cmd.set_defaults(func=disable)

    list_cmd = subparsers.add_parser("list")
    list_cmd.set_defaults(func=list)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
