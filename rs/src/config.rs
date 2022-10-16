// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::cli::Cli;
use crate::consts;
use clap::{error, CommandFactory, Parser};
use std::collections::HashSet;

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
        let cli = Cli::parse();
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
