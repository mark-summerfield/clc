# clc (count code lines)

clc is a quick rust tool that counts newlines in code files.

clc has command line options to specify which languages to consider or
exclude, and which files and folders to include or exclude. clc sorts
case-insensitive alphabetically within language groups, but can sort by
lines. clc uses the width needed, or the terminal width if narrower, or the
width specified.

_Note: clc will only build on Windows using clang >= 3.9; it isn't tested on
Windows._

[crates.io](https://crates.io/crates/qtrac-clc) •
[docs](https://docs.rs/clc/latest/qtrac-clc/)

(Due to a name conflict the cargo crate is `qtrac-clc` but the executable is
`clc`.)

## Example Summary

At one time in ``clc``'s folder, `clc -S` produced:

```
Python       1 file          284 lines
Rust         8 files         710 lines
```

## Example Full

At one time in ``clc``'s folder, `clc` produced:

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
These are plain text files with this format:

    lang|Name|ext1 [ext2 [ext3 ... [extN]]]

For example:

    pas|Pascal|pas pp inc

`clc` reads in every `clc.dat` file it finds and for every record it reads
if an entry for the given lang exits it will be replaced by the new data,
and if it doesn't exist, the new lang and data will be inserted.

For example, by default support for Python in built-in with this data:

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

where `EXE` is the folder containing the `clc` executable, `HOME` is the
user's home folder, and `CWD` is the current folder. If any—or all—of these
are not found they are silently ignored.

---
