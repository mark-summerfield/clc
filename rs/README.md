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

---
