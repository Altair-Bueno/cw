## cw 2.1.0 26/09/2021

- fix: #18
- new: #16

---

## cw 2.0.0 19/09/2021

This release uses the removes the thread pool executor we were using and 
replaces it with the more advanced [`tokio`](https://tokio.rs) runtime. This
provides `cw` with a boost on performance without the need of multiple threads.
And if you need more threads, you can use the new `--multithread` flag to use
all cores on your machine. It's that simple!

Althought `tokio` is a really fast runtime, it does not provide a performance
gain if you are counting a couple files. So if you want the fastest word counter
you should stick to version 1

---

## libcw 1.1.0 15/09/2021

- Added `tokio` support: Use the feature `tokio` to enable async `io` operations
on a `Parser` instance

---

## libcw 1.0.1 23/08/2021

- Added beta support for UTF16 encoding
- Made some performance optimizations

---

## cw 1.0.0 17/08/2021

First stable release. Provides the basic functionality of classic gwc
(GNU Word Counter) from `coreutils` as well as some adittions. Full list of
features:

- Basic UTF-8 character counting (no zalgo text support yet)
- File max line length
- File byte count
- File word count (see `man isspace`)
- File line count
- Support for LF and CR line breaks
- Multithreading using thread pools
- Human-readable output with colors. No more `man` reading!!

---
