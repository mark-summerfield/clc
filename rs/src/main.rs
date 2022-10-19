// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod cli;
mod config;
mod consts;
mod display;
mod valid;

use anyhow::Result;
use config::Config;
use rayon::prelude::*;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    time::Instant,
};
use walkdir::WalkDir;

fn main() {
    consts::initialize(); // NOTE must be first
    let config = Config::new();
    let t = Instant::now();
    let filenames = get_filenames(&config);
    let file_data = filenames
        .par_iter()
        .filter_map(|filename| process_one(filename).ok())
        .collect();
    if config.summary {
        display::display_summary(file_data, config, t);
    } else {
        display::display_full(file_data, config);
    }
}

fn process_one(filename: &Path) -> Result<FileData> {
    let mut file = File::open(&filename)?;
    let (count, lang) = if let Some(lang) = lang_for_name(filename) {
        let mmap = unsafe { memmap2::Mmap::map(&file)? };
        let count = mmap.iter().filter(|&b| *b == b'\n').count();
        (count, lang)
    } else {
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        let count = text.bytes().filter(|&b| b == b'\n').count();
        let lang = if text.starts_with("#!") {
            if let Some(i) = text.find('\n') {
                lang_for_line(&text[..i])
            } else {
                ""
            }
        } else {
            ""
        };
        (count, lang.to_string())
    };
    Ok(FileData::new(lang, filename, count))
}

fn get_filenames(config: &Config) -> Vec<PathBuf> {
    let mut filenames = Vec::with_capacity(1000);
    for name in &config.files {
        let filename = abspath(name);
        if filename.is_file() {
            if valid::is_valid_file(&filename, config) {
                filenames.push(filename);
            }
        } else if filename.is_dir() {
            for entry in WalkDir::new(&filename)
                .into_iter()
                .filter_entry(|e| valid::is_valid_entry(e, config))
                .flatten()
            {
                if !entry.file_type().is_dir() {
                    filenames.push(entry.into_path());
                }
            }
        }
    }
    filenames
}

fn abspath(name: &str) -> PathBuf {
    let filename = PathBuf::from(name);
    if filename.is_absolute() {
        filename
    } else {
        filename.canonicalize().unwrap_or(filename)
    }
}

fn lang_for_name(name: &Path) -> Option<String> {
    if let Some(ext) = name.extension() {
        let ext = ext.to_string_lossy().to_string();
        let data_for_lang = consts::DATA_FOR_LANG.get();
        for (lang, lang_data) in data_for_lang.iter() {
            if lang_data.exts.contains(ext.as_str()) {
                return Some(lang.to_string());
            }
        }
    }
    None
}

fn lang_for_line(line: &str) -> &str {
    if line.contains("julia") {
        "jl"
    } else if line.contains("perl") {
        "pl"
    } else if line.contains("python") {
        "py"
    } else if line.contains("ruby") {
        "rb"
    } else if line.contains("tcl") {
        "tcl"
    } else {
        ""
    }
}

#[derive(Debug)]
pub struct FileData {
    pub lang: String,
    pub filename: String,
    pub lines: usize,
}

impl FileData {
    pub fn new(lang: String, filename: &Path, lines: usize) -> Self {
        Self {
            lang,
            filename: filename.to_string_lossy().to_string(),
            lines,
        }
    }
}
