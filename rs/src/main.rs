// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod cli;
mod config;
mod consts;
mod valid;

use config::Config;
use std::{
    path::{Path, PathBuf},
    time::Instant,
};
use walkdir::WalkDir;

fn main() {
    consts::initialize(); // NOTE must be first
    let config = Config::new();
    let t = Instant::now();
    process(config);
}

fn process_one<'a>(
    filename: &'a Path,
    config: &'a Config,
) -> Option<FileData<'a>> {
    println!("process_one: {filename:?}");
    None
}

fn process(config: Config) {
    for name in &config.files {
        let filename = abspath(name);
        if filename.is_file() {
            if valid::is_valid_file(&filename, &config) {
                process_one(&filename, &config);
            }
        } else if filename.is_dir() {
            let walker = WalkDir::new(&filename).into_iter();
            for entry in walker
                .filter_entry(|e| valid::is_valid_entry(e, &config))
                .flatten()
            {
                if !entry.file_type().is_dir() {
                    process_one(entry.path(), &config);
                }
            }
        }
    }
}

fn abspath(name: &str) -> PathBuf {
    let filename = PathBuf::from(name);
    if filename.is_absolute() {
        filename
    } else {
        filename.canonicalize().unwrap_or(filename)
    }
}

struct FileData<'a> {
    pub lang: &'a str,
    pub filename: &'a Path,
    pub lines: usize,
}
