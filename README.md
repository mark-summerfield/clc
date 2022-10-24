# clc (count code lines)

There are two tools here that count newlines in code files.

Both have command line options to specify which languages to consider or
exclude, and which files and folders to include or exclude. They sort
case-insensitive alphabetically within language groups, but can sort by
lines. They use the width needed, or the terminal width if narrower, or the
width specified.

`clc.py` is written in Python 3 and has no dependencies beyond the standard
library.

`clc` is written in Rust—and is _much_ faster—but has to be built and has a
larger executable.

## Example Summary

At one time in ``clc.py``'s folder, `clc.py -S` produced:

```
Python       1 file          284 lines
Rust         8 files         710 lines
```

## Example Full

At one time in ``clc.py``'s folder, `clc.py` produced:

```
━━━━━━━━━━━━━━━━━━━━ Python ━━━━━━━━━━━━━━━━━━━━
/home/mark/app/clc/clc.py                    284
────────────────────────────────────────────────
Python                 1 file          284 lines
━━━━━━━━━━━━━━━━━━━━━ Rust ━━━━━━━━━━━━━━━━━━━━━
/home/mark/app/clc/rs/src/cli.rs              66
/home/mark/app/clc/rs/src/config.rs           97
/home/mark/app/clc/rs/src/consts.rs           45
/home/mark/app/clc/rs/src/display.rs         247
/home/mark/app/clc/rs/src/main.rs            113
/home/mark/app/clc/rs/src/types.rs            33
/home/mark/app/clc/rs/src/util.rs             30
/home/mark/app/clc/rs/src/valid.rs            79
────────────────────────────────────────────────
Rust                   8 files         710 lines
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
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
