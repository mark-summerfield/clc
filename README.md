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

---
