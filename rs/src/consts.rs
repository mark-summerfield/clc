// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use state::Storage;
use std::collections::{HashMap, HashSet};

pub static EXCLUDE: Storage<Vec<&str>> = Storage::new();
pub static DATA_FOR_LANG: Storage<HashMap<&str, LangData>> =
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
    let mut m = HashMap::new();
    let mut exts = HashSet::new();
    exts.insert(".h");
    exts.insert(".c");
    m.insert("c", LangData::new("C", exts));
    let mut exts = HashSet::new();
    exts.insert(".hpp");
    exts.insert(".hxx");
    exts.insert(".cpp");
    exts.insert(".cxx");
    m.insert("cpp", LangData::new("C++", exts));
    let mut exts = HashSet::new();
    exts.insert(".d");
    m.insert("d", LangData::new("D", exts));
    let mut exts = HashSet::new();
    exts.insert(".go");
    m.insert("go", LangData::new("Go", exts));
    let mut exts = HashSet::new();
    exts.insert(".java");
    m.insert("java", LangData::new("Java", exts));
    let mut exts = HashSet::new();
    exts.insert(".jl");
    m.insert("jl", LangData::new("Julia", exts));
    let mut exts = HashSet::new();
    exts.insert(".nim");
    m.insert("nim", LangData::new("Nim", exts));
    let mut exts = HashSet::new();
    exts.insert(".pl");
    exts.insert(".PL");
    exts.insert(".pm");
    m.insert("pl", LangData::new("Perl", exts));
    let mut exts = HashSet::new();
    exts.insert(".py");
    exts.insert(".pyw");
    m.insert("py", LangData::new("Python", exts));
    let mut exts = HashSet::new();
    exts.insert(".rb");
    m.insert("rb", LangData::new("Ruby", exts));
    let mut exts = HashSet::new();
    exts.insert(".rs");
    m.insert("rs", LangData::new("Rust", exts));
    let mut exts = HashSet::new();
    exts.insert(".tcl");
    m.insert("tcl", LangData::new("Tcl", exts));
    let mut exts = HashSet::new();
    exts.insert(".vala");
    m.insert("vala", LangData::new("Vala", exts));
    DATA_FOR_LANG.set(m);
}

#[derive(Debug)]
pub struct LangData<'a> {
    name: &'a str,
    exts: HashSet<&'a str>,
}

impl<'a> LangData<'a> {
    pub fn new(name: &'a str, exts: HashSet<&'a str>) -> Self {
        Self { name, exts }
    }
}
