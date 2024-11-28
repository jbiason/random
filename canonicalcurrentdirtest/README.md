# CanonincalCurrentDirTest

This is an experiment on trying to understand how "Command" works on Std and
Tokio when the `.current_dir()` is canonicalized.

The base for the experiment is the fact that canonicalized paths on Windows
have a prefix ([used to indicate that the path does not have any special
processing](https://stackoverflow.com/questions/41233684/why-does-my-canonicalized-path-get-prefixed-with))
and it looks like it breaks the `.current_dir()` execution (at least, I saw it
on Tokio, wondering if the same thing happens on Std, or it is my case that it
is broken).
