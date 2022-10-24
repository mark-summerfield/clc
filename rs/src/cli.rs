// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use clap::Parser;
use std::ops::Range;

const MAXSIZE_RANGE: Range<usize> = 20..32767;

#[derive(Parser, Debug)]
#[clap(
    name = "clc",
    version,
    about = "Counts the lines in the code files for the languages \
processed (excluding . folders).

Built-in supported language names: \
c cpp d go java jl nim pl py rb rs tcl vala.

Also supports any languages specified in any clc.dat files that are found. \
These files are looked for in the clc executable's folder, the home \
folder, the home/.config folder, and the current folder. These files have \
the form: 
    lang|Name|ext1 [ext2 [ext3 ... [extN]]]
For example:
    pas|Pascal|pas pp inc
    sql|SQL|sql
Blank lines and lines beginning with `#` are ignored."
)]
pub struct Cli {
    /// Languages to count [default: all known]
    #[arg(short, long, num_args(0..))]
    pub language: Option<Vec<String>>,

    /// Languages not to count [default: none].
    /// For example "-L d cpp" with no "-l" means count all languages except
    /// D and C++.
    #[arg(short = 'L', long, num_args(0..))]
    pub skiplanguage: Option<Vec<String>>,

    /// Files and folders to exclude [default: .hidden and other sensible
    /// ones]
    #[arg(short, long, num_args(0..))]
    pub exclude: Option<Vec<String>>,

    /// Files to include (e.g., those without suffixes)
    #[arg(short, long, num_args(0..))]
    pub include: Option<Vec<String>>,

    /// Maximum line width to use (e.g., for redirected output) [default:
    /// terminal width or needed width if less]
    #[arg(short, long, value_parser=maxsize_in_range)]
    pub maxwidth: Option<usize>,

    /// Sort by lines [the default is to sort by names]
    #[arg(short, long)]
    pub sortbylines: bool,

    /// Summary: output per-language totals and total time if > 0.1 sec
    /// [the default is to output per-language and per-file totals]
    #[arg(short = 'S', long)]
    pub summary: bool,

    /// Files to count or the folders to recursively search [default: .]
    pub file: Option<Vec<String>>,
}

fn maxsize_in_range(s: &str) -> Result<usize, String> {
    let maxsize: usize =
        s.parse().map_err(|_| format!("invalid maxsize: {:?}", s))?;
    if MAXSIZE_RANGE.contains(&maxsize) {
        Ok(maxsize as usize)
    } else {
        Err(format!(
            "maxsize must be in range {}-{}",
            MAXSIZE_RANGE.start, MAXSIZE_RANGE.end
        ))
    }
}
