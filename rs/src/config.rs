// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::cli::Cli;
use crate::consts;
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
    pub fn new_from_cli(cli: Cli) -> Self {
        let mut langs = HashSet::new();
        if let Some(language) = cli.language {
            for lang in language {
                langs.insert(lang);
            }
        } else {
            // If none specified use all the known languages
            for lang in consts::DATA_FOR_LANG.get().keys() {
                langs.insert(lang.to_string());
            }
        }
        if let Some(language) = cli.skiplanguage {
            for lang in language {
                langs.remove(&lang);
            }
        }
        let mut exclude = HashSet::new();
        if let Some(excl) = cli.exclude {
            for name in excl {
                exclude.insert(name);
            }
        }
        let mut include = HashSet::new();
        if let Some(incl) = cli.include {
            for name in incl {
                include.insert(name);
            }
        }
        let maxwidth = if let Some(maxwidth) = cli.maxwidth {
            maxwidth
        } else {
            if let Some((width, _)) = term_size::dimensions() {
                width
            } else {
                80
            }
        };
        let mut files = HashSet::new();
        if let Some(file) = cli.file {
            for name in file {
                files.insert(name);
            }
        } else {
            files.insert(".".to_string());
        }
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
