// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod cli;
mod config;
mod consts;

use config::Config;

fn main() {
    consts::initialize(); // NOTE must be first
    let config = Config::new();
    dbg!(config);
}
