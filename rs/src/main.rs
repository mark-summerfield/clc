// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod cli;
mod config;
mod consts;

use clap::Parser;
use cli::Cli;
use config::Config;

fn main() {
    consts::initialize();
    let cli = Cli::parse();
    let config = Config::new_from_cli(cli);
    dbg!(config);
}
