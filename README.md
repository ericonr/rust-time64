# Rust and time64

This is a tiny project to showcase the issues caused by
https://github.com/rust-lang/libc/issues/1848

It's reasonable to expect that this code would print

```
value: expected actual_result
sec: 3 3
nsec: 4 4
```

on any system, correct?

Unfortunately, Rust hardcodes libc types, and musl's
[time64](https://musl.libc.org/time64.html) transition (only necessary for
32-bit systems hasn't been dealt with yet. This isn't an issue for *pure Rust*
projects, because the `libc` crate also hardcodes the function names, so it
links to musl's time32 compatibility functions, instead of the modern time64
ones. The only issue here is that the resulting binaries aren't safe for y2038.

When you throw C code into the mix, however, there is a mismatch between what
Rust and C think a certain type is: Rust thinks `time_t` is a 32-bit integer,
and `struct timespec`'s `tv_nsec` can be found 4 bytes after the start of the
struct; C thinks `time_t` is a 64-bit integer, and `tv_nsec` can be found 8
bytes after the start of the struct. Here, the Rust code is reading the 4
higher bytes of `tv_sec` (at least on a little endian machine, the issue would
be even more noticeable on a big endian system) and treating it as `tv_nsec`.

This generates the following output:

```
value: expected actual_result
sec: 3 3
nsec: 4 0
```

Therefore, for "old" 32-bit targets (riscv32 is supposed to default to time64),
any Rust code that interacts with C code built on musl after 1.2.0, using types
based on `time_t` (arguably, the main ones are `struct timespec` and `struct
stat`) in their interface, will be completely miscompiled. It would be
interesting to study how many projects fall into this trap.
