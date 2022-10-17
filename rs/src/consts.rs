// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use state::Storage;
use std::collections::{HashMap, HashSet};

pub static EXCLUDE: Storage<Vec<&str>> = Storage::new();
pub static DATA_FOR_LANG: Storage<HashMap<&str, LangData>> = Storage::new();

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
    DATA_FOR_LANG.set(HashMap::from([
        ("c", LangData::new("C", HashSet::from(["h", "c"]))),
        (
            "cpp",
            LangData::new(
                "C++",
                HashSet::from(["hpp", "hxx", "cpp", "cxx"]),
            ),
        ),
        ("d", LangData::new("D", HashSet::from(["d"]))),
        ("go", LangData::new("Go", HashSet::from(["go"]))),
        ("java", LangData::new("Java", HashSet::from(["java"]))),
        ("jl", LangData::new("Julia", HashSet::from(["jl"]))),
        ("nim", LangData::new("Nim", HashSet::from(["nim"]))),
        ("pl", LangData::new("Perl", HashSet::from(["pl", "PL", "pm"]))),
        ("py", LangData::new("Python", HashSet::from(["py", "pyw"]))),
        ("rb", LangData::new("Ruby", HashSet::from(["rb"]))),
        ("rs", LangData::new("Rust", HashSet::from(["rs"]))),
        ("tcl", LangData::new("Tcl", HashSet::from(["tcl"]))),
        ("vala", LangData::new("Vala", HashSet::from(["vala"]))),
    ]));
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
