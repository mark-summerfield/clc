# clc (count code lines)

There are three tools here that count newlines in code files.

All three have command line options to specify which languages to consider
or exclude, and which files and folders to include or exclude. They sort
case-insensitive alphabetically within language groups, but can sort by
lines. They use the width needed, or the terminal width if narrower, or the
width specified.

- `clc.py` is written in Python (~320 LOC).
- `go/clc` is written in Go (~600 LOC).
- `rs/clc` is written in Rust (~800 LOC).

For small projects (a few tens of thousands of lines of code) they're all
quick enough not to notice any difference.

For large code bases (millions of lines, tens of thousands of files), the
speed differences are noticable. In these cases, the Python version runs
about 10% faster the second and subsequent times compared to the first time.
The Go version is more than twice as fast the second and subsequent times,
and is about 5x faster than Python. The Rust version runs almost twice as
fast the second and subsequent times and is more than 10x faster than
Python.

I found the Python and Go versions much easier to write than the Rust
version. The Go version is by far the easiest to deploy: you can build a
standalone executable on any supported platform to target any supported
platform. For Rust you need to build on each platform or use a
cross-compilation toolchain. For Python, in this case deployment is tricky
even though there are no dependencies: you either need to ensure that the
target machine has a suitable Python or use a tool to create an executable.

## Example Summary

At one time in ``clc.py``'s folder, `clc.py -S` produced:

```
Go           9 files         612 lines
Python       1 file          321 lines
Rust         8 files         815 lines
```

## Example Full

At one time in ``clc.py``'s folder, `clc.py` produced:

```
━━━━━━━━━━━━━━━━━━━━━━━━━ Go ━━━━━━━━━━━━━━━━━━━━━━━━━
/home/mark/app/clc/go/clc.go                        20
/home/mark/app/clc/go/config.go                    205
/home/mark/app/clc/go/consts.go                      9
/home/mark/app/clc/go/display.go                   149
/home/mark/app/clc/go/files.go                      75
/home/mark/app/clc/go/highlight_unix.go             12
/home/mark/app/clc/go/highlight_windows.go          12
/home/mark/app/clc/go/process.go                   100
/home/mark/app/clc/go/util.go                       30
──────────────────────────────────────────────────────
Go                           9 files         612 lines
━━━━━━━━━━━━━━━━━━━━━━━ Python ━━━━━━━━━━━━━━━━━━━━━━━
/home/mark/app/clc/clc.py                          321
──────────────────────────────────────────────────────
Python                       1 file          321 lines
━━━━━━━━━━━━━━━━━━━━━━━━ Rust ━━━━━━━━━━━━━━━━━━━━━━━━
/home/mark/app/clc/rs/src/cli.rs                    60
/home/mark/app/clc/rs/src/config.rs                226
/home/mark/app/clc/rs/src/consts.rs                 25
/home/mark/app/clc/rs/src/display.rs               247
/home/mark/app/clc/rs/src/main.rs                  113
/home/mark/app/clc/rs/src/types.rs                  35
/home/mark/app/clc/rs/src/util.rs                   30
/home/mark/app/clc/rs/src/valid.rs                  79
──────────────────────────────────────────────────────
Rust                         8 files         815 lines
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

Note that on Windows `=` and `-` are used for the lines and `...` for elided
filenames (e.g., when maxwidth is specified).

## Supported Languages

Out of the box `clc` supports
`c` (C), `cpp` (C++), `d` (D), `go` (Go), `java` (Java), `jl` (Julia),
`nim` (Nim), `pl` (Perl), `py` (Python), `rb` (Ruby), `rs` (Rust),
`tcl` (Tcl), and `vala` (Vala).

But what if you want to change the extensions used to count as a particular
language? Or what if you want to count a language which isn't supported?

From version 1.1.0 both these can be solved by using `clc.dat` data files.
These are plain text files with this format (one entry per line):

    lang|Name|ext1 [ext2 [ext3 ... [extN]]]

For example:

    pas|Pascal|pas pp inc
    sql|SQL|sql

Blank lines and lines beginning with `#` are ignored.

`clc` reads in every `clc.dat` file it finds and for every record it reads
if an entry for the given lang exits it will be replaced by the new data,
and if it doesn't exist, the new lang and data will be inserted.

For example, by default support for Python is built-in with this data:

    py|Python|py pyw

If you don't want to count `.pyw` files you can replace the built-in entry
with, say:

    py|Python|py

`clc` looks for `clc.dat` files in the following locations—in order—so later
entries with the same lang as earlier entries (or defaults) will _replace_
existing ones.

    EXE/clc.dat
    HOME/clc.dat
    HOME/.config/clc.dat
    CWD/clc.dat

where `EXE` is the folder containing the `clc` (or `clc.py`) executable,
`HOME` is the user's home folder, and `CWD` is the current folder.
If any—or all—of these are not found they are silently ignored.

---
