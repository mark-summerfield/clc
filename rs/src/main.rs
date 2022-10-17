// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod cli;
mod config;
mod consts;

use config::Config;
use std::{
    collections::HashSet,
    path::{Component, Path, PathBuf},
    time::Instant,
};
use walkdir::{DirEntry, WalkDir};

fn main() {
    consts::initialize(); // NOTE must be first
    let config = Config::new();
    let t = Instant::now();
    process(config);
}

fn process_one(filename: &Path, config: &Config) {
    println!("process_one: {filename:?}");
}

fn process(config: Config) {
    for name in &config.files {
        let filename = abspath(name);
        if filename.is_file() {
            if is_valid_file(&filename, &config) {
                process_one(&filename, &config);
            }
        } else if filename.is_dir() {
            let walker = WalkDir::new(&filename).into_iter();
            for entry in walker
                .filter_entry(|e| is_valid_entry(e, &config))
                .flatten()
            {
                if !entry.file_type().is_dir() {
                    process_one(entry.path(), &config);
                }
            }
        }
    }
}

fn is_valid_entry(entry: &DirEntry, config: &Config) -> bool {
    if entry.file_type().is_dir() {
        is_valid_dir(entry.path(), config)
    } else {
        is_valid_file(entry.path(), config)
    }
}

fn is_valid_file(filename: &Path, config: &Config) -> bool {
    if let Some(name) = filename.file_name() {
        if let Some(name) = name.to_str() {
            if config.include.contains(name) {
                return true;
            }
            if name.starts_with('.') {
                return false;
            }
            let parts: HashSet<String> = filename
                .components()
                .filter_map(|c| match c {
                    Component::Normal(s) => {
                        s.to_str().map(|s| s.to_string())
                    }
                    _ => None,
                })
                .collect();
            if parts.intersection(&config.exclude).count() > 0 {
                return false;
            }
            for part in parts {
                if part.len() > 1 && part.starts_with('.') {
                    return false;
                }
            }
            if let Some((_, ext)) = name.rsplit_once('.') {
                let data_for_lang = consts::DATA_FOR_LANG.get();
                for lang in &config.langs {
                    if data_for_lang[lang.as_str()].exts.contains(ext) {
                        return true;
                    }
                }
            } else {
                return false; // No extension and not in includes
            }
        }
    }
    false
}

fn is_valid_dir(dirname: &Path, config: &Config) -> bool {
    if let Some(name) = dirname.file_name() {
        if let Some(name) = name.to_str() {
            if name.len() > 1 && name.starts_with('.') {
                return false;
            }
        }
    }
    if let Some(parent) = dirname.parent() {
        if let Some(name) = parent.to_str() {
            if config.exclude.contains(name) {
                return false;
            }
        }
    }
    true
}

fn abspath(name: &str) -> PathBuf {
    let filename = PathBuf::from(name);
    if filename.is_absolute() {
        filename
    } else {
        filename.canonicalize().unwrap_or(filename)
    }
}
