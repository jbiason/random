# Sparse-Checkout

This code tries to simulate the way `sparse-checkout` works in recent Git
versions.

## The Problem

The problem is that the behaviour of `sparse-checkout` changed somewhere in the
line. In most recent versions, `sparse-checkout init`, on a clone, will hide
every single file; older versions, though, will refuse to init the
`sparse-checkout` if that would leave the current directory empty.

## Solution

The idea is to push `update-index --skip-worktree` to tag a file as "being
ignored" (Git will always assume the file checksum is still the same as in the
index, no matter what you do with the file). Then, when a user wants to
re-enable the directory/file, use `update-index --no-skip-worktree`.

Although that sounds simple, there an issue that Git deals only with files and
not directories, and trying to disable/enable the file in the index requires to
pass the files; for example, on the re-enable, how to find the names of the
files we just deleted?

That requires a combination of use with `ls-files --cached [pattern]`. This way
we can get all the files that need to be pushed back/ignored, reading directly
from the Git information.
