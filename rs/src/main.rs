// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use clap::Parser;

fn main() {
    let config = Config::parse();
    dbg!(config);
}

#[derive(Parser, Debug)]
#[clap(version,
       about = "Counts the lines in the code files for the languages \
       processed.")]
struct Config {
    /// Sort by lines
    #[arg(short, long)]
    sortbylines: bool,
}
