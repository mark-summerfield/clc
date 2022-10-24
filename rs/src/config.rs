// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::cli::Cli;
use crate::consts;
use crate::types::LangData;
use anyhow::Result;
use clap::{error, CommandFactory, FromArgMatches};
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::prelude::*,
    path::Path,
};

#[derive(Debug)]
pub struct Config {
    pub langs: HashSet<String>,
    pub exclude: HashSet<String>,
    pub include: HashSet<String>,
    pub maxwidth: usize,
    pub sortbylines: bool,
    pub summary: bool,
    pub files: HashSet<String>,
}

impl Config {
    pub fn new() -> Self {
        read_config_files(); // NOTE must come first
        let command = Cli::command();
        let cli = Cli::from_arg_matches(
            &command.about(get_about()).get_matches(),
        )
        .unwrap();
        let langs = get_langs(cli.language, cli.skiplanguage);
        let mut exclude = HashSet::from_iter(
            consts::EXCLUDE.get().iter().map(|s| s.to_string()),
        );
        if let Some(excl) = cli.exclude {
            exclude.extend(excl);
        }
        let include = if let Some(incl) = cli.include {
            HashSet::from_iter(incl)
        } else {
            HashSet::new()
        };
        let maxwidth = if let Some(maxwidth) = cli.maxwidth {
            maxwidth // Always in range 20..32767
        } else if let Some((width, _)) = term_size::dimensions() {
            width
        } else {
            80
        };
        // Internally we use this purely to elide filenames, hence the -
        let maxwidth = maxwidth - (consts::LINE_COUNT_WIDTH + 2);
        let files = if let Some(file) = cli.file {
            HashSet::from_iter(file)
        } else {
            HashSet::from([".".to_string()])
        };
        Self {
            langs,
            exclude,
            include,
            maxwidth,
            sortbylines: cli.sortbylines,
            summary: cli.summary,
            files,
        }
    }
}

fn get_langs(
    language: Option<Vec<String>>,
    skiplanguage: Option<Vec<String>>,
) -> HashSet<String> {
    let default_langs = HashSet::from_iter(
        consts::DATA_FOR_LANG.get().keys().map(|s| s.to_string()),
    );
    let mut langs = if let Some(language) = language {
        HashSet::from_iter(language)
    } else {
        default_langs.clone()
    };
    if let Some(language) = skiplanguage {
        for lang in language {
            langs.remove(&lang);
        }
    }
    if langs.is_empty() {
        let mut cmd = Cli::command();
        cmd.error(error::ErrorKind::TooFewValues, "no languages to count")
            .exit();
    }
    let lang_names = Vec::from_iter(langs.iter().map(|s| s.to_string()));
    let mut bad_names = vec![];
    for lang in &lang_names {
        if !default_langs.contains(lang) {
            langs.remove(lang);
            bad_names.push(lang.to_string());
        }
    }
    if !bad_names.is_empty() {
        let s = if bad_names.len() == 1 { "" } else { "s" };
        let names = bad_names.join(" ");
        eprintln!("ignoring unrecognized language{s}: {names}");
    }
    langs
}

fn read_config_files() {
    let mut data_for_lang = initial_data_for_lang();
    // We ignore errors & only care about files we find
    if let Ok(filename) = env::current_exe() {
        if let Some(filename) = filename.parent() {
            let filename = filename.join("clc.dat");
            let _ = read_config_file(&filename, &mut data_for_lang);
        }
    }
    if let Some(home) = dirs::home_dir() {
        let home1 = home.join("clc.dat");
        let _ = read_config_file(&home1, &mut data_for_lang);
        let home2 = home.join(".config/clc.dat");
        let _ = read_config_file(&home2, &mut data_for_lang);
    }
    if let Ok(filename) = env::current_dir() {
        let filename = filename.join("clc.dat");
        let _ = read_config_file(&filename, &mut data_for_lang);
    }
    consts::DATA_FOR_LANG.set(data_for_lang);
}

fn read_config_file(
    filename: &Path,
    data_for_lang: &mut HashMap<String, LangData>,
) -> Result<()> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() == 3 {
            let lang = parts[0].trim();
            let name = parts[1].trim();
            let mut exts: HashSet<&str> = HashSet::new();
            for ext in parts[2].split_whitespace() {
                if let Some(ext) = ext.strip_prefix('.') {
                    exts.insert(ext);
                } else {
                    exts.insert(ext);
                }
            }
            data_for_lang
                .insert(lang.to_string(), LangData::new(name, exts));
        } else {
            eprintln!("ignoring invalid line from {filename:?}: {line}");
        }
    }
    Ok(())
}

fn initial_data_for_lang() -> HashMap<String, LangData> {
    HashMap::from([
        ("c".to_string(), LangData::new("C", HashSet::from(["h", "c"]))),
        (
            "cpp".to_string(),
            LangData::new(
                "C++",
                HashSet::from(["hpp", "hxx", "cpp", "cxx"]),
            ),
        ),
        ("d".to_string(), LangData::new("D", HashSet::from(["d"]))),
        ("go".to_string(), LangData::new("Go", HashSet::from(["go"]))),
        (
            "java".to_string(),
            LangData::new("Java", HashSet::from(["java"])),
        ),
        ("jl".to_string(), LangData::new("Julia", HashSet::from(["jl"]))),
        ("nim".to_string(), LangData::new("Nim", HashSet::from(["nim"]))),
        (
            "pl".to_string(),
            LangData::new("Perl", HashSet::from(["pl", "PL", "pm"])),
        ),
        (
            "py".to_string(),
            LangData::new("Python", HashSet::from(["py", "pyw"])),
        ),
        ("rb".to_string(), LangData::new("Ruby", HashSet::from(["rb"]))),
        ("rs".to_string(), LangData::new("Rust", HashSet::from(["rs"]))),
        ("tcl".to_string(), LangData::new("Tcl", HashSet::from(["tcl"]))),
        (
            "vala".to_string(),
            LangData::new("Vala", HashSet::from(["vala"])),
        ),
    ])
}

fn get_about() -> String {
    // TODO
    //let langs: Vec<&str> = HashSet::from_iter(
    //    consts::DATA_FOR_LANG.get().keys().map(|s| s.to_string()),
    //);

    format!("Counts the lines in the code files for the languages \
processed (excluding . folders).

Supported language names: \
c cpp d go java jl nim pl py rb rs tcl vala.

Also supports any languages specified in any clc.dat files that are found. \
These files are looked for in the clc executable's folder, the home \
folder, the home/.config folder, and the current folder. These files have \
the form: 
    lang|Name|ext1 [ext2 [ext3 ... [extN]]]
For example:
    pas|Pascal|pas pp inc
    sql|SQL|sql
Blank lines and lines beginning with `#` are ignored.")
}
