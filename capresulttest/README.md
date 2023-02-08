# CapResultTest

It is known that a `.collect()` in which the `Item` is a `Result` will stop
processing as soon as it finds an `Err`. But what if we capture this before
`.collect()`?
