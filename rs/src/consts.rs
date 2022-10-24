// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::types::LangData;
use state::Storage;
use std::collections::HashMap;

pub const FILE_COUNT_WIDTH: usize = 7;
pub const LINE_COUNT_WIDTH: usize = 11;

pub static EXCLUDE: Storage<Vec<&str>> = Storage::new();
pub static DATA_FOR_LANG: Storage<HashMap<String, LangData>> =
    Storage::new();

pub fn initialize() {
    EXCLUDE.set(vec![
        "__pycache__",
        "build",
        "build.rs",
        "CVS",
        "dist",
        "setup.py",
        "target",
    ]);
}
