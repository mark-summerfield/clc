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
pub struct LangData<'a> {
    pub name: &'a str,
    pub exts: HashSet<&'a str>,
}

impl<'a> LangData<'a> {
    pub fn new(name: &'a str, exts: HashSet<&'a str>) -> Self {
        Self { name, exts }
    }
}
