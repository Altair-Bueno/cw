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
