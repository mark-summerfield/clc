// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod config;
mod consts;

use clap::Parser;

fn main() {
    consts::initialize();
    let config = config::Config::parse();
    dbg!(config);
}
