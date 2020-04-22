[![Build Status](https://github.com/kwyse/euler-solutions/workflows/ci/badge.svg?branch=source)](https://github.com/kwyse/euler-solutions/actions)

# Project Euler Solutions

Project Euler solutions written in Rust.

## Commentary

You can find explanations for the solutions on the [accompanying
site](https://www.kwyse.com/euler-solutions/).

## Testing

Source code for the solutions is
[tangled](https://orgmode.org/manual/Extracting-Source-Code.html) from
`euler.org`.  Each solution invokes a macro defined in `src/lib.rs`
that tests against the answers in `src/answers.rs`.

Emacs is required to tangle the files.  You can then run `cargo test`
as normal.

```shell
$ emacs ./euler.org --batch --eval='(org-babel-tangle)' --kill
$ cargo test
```

## Performance

[Criterion](https://crates.io/crates/criterion) is used for
benchmarking.  Like testing, Emacs is required to tangle the files.

```shell
$ emacs ./euler.org --batch --eval='(org-babel-tangle)' --kill
$ cargo bench
```
