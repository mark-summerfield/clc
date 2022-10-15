// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    version,
    about = "Counts the lines in the code files for the languages \
processed (excluding . folders).

Supported languages names: c cpp d go java jl nim pl py rb rs tcl vala."
)]
pub struct Config {
    /// Languages to count [default: all known]
    #[arg(short, long)]
    language: Option<Vec<String>>,

    /// Languages not to count [default: none].
    /// For example "-L d cpp" with no "-l" means count all languages except
    /// D and C++.
    #[arg(short = 'L', long)]
    skiplanguage: Option<Vec<String>>,

    /// Files and folders to exclude [default: .hidden and other sensible
    /// ones]
    #[arg(short, long)]
    exclude: Option<Vec<String>>,

    /// Files to include (e.g., those without suffixes)
    #[arg(short, long)]
    include: Option<Vec<String>>,

    /// Maximum line width to use (e.g., for redirected output) [default:
    /// terminal width or needed width if less]
    #[arg(short, long)]
    maxwidth: Option<usize>,

    /// Sort by lines [the default is to sort by names]
    #[arg(short, long)]
    sortbylines: bool,

    /// Summary: output per-language totals and total time [the default is
    /// to output per-language and per-file totals]
    #[arg(short = 'S', long)]
    summary: bool,

    /// Files to count or the folders to recursively search [default: .]
    file: Option<Vec<PathBuf>>,
}
