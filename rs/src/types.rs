// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use std::{collections::HashSet, path::Path};

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

#[derive(Debug)]
pub struct LangData {
    pub name: String,
    pub exts: HashSet<String>,
}

impl LangData {
    pub fn new(name: &str, exts: HashSet<&str>) -> Self {
        let exts: HashSet<String> =
            exts.iter().map(|e| e.to_string()).collect();
        Self { name: name.to_string(), exts }
    }
}
