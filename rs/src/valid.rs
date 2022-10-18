// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::config::Config;
use crate::consts;
use std::{
    collections::HashSet,
    path::{Component, Path},
};
use walkdir::DirEntry;

pub fn is_valid_entry(entry: &DirEntry, config: &Config) -> bool {
    if entry.file_type().is_dir() {
        is_valid_dir(entry.path(), config)
    } else {
        is_valid_file(entry.path(), config)
    }
}

pub fn is_valid_file(filename: &Path, config: &Config) -> bool {
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

pub fn is_valid_dir(dirname: &Path, config: &Config) -> bool {
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
