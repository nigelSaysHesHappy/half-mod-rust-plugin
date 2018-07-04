# halfMod Rust Plugin Demo

Work in progress demo of a Rust plugin for
[halfMod](https://github.com/nigelSaysHesHappy/halfMod).

Expects the `include` folder from `halfMod/src` to be present next to
`build.rs`, or symbolically linked. A folder `half_mod` is also expected, which
should be a copy or symlink of `halfMod/src/o`.

You should also have a `clang++` version of 3.8 or higher accessible at
`/usr/bin/clang++-3.8`, as required by `halfMod`.

The build process should be more automatic in the future.